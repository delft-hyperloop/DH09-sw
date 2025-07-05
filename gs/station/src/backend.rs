use std::collections::VecDeque;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::anyhow;
use gslib::Command;
use gslib::Info;
use gslib::Log;
use gslib::Message;
use gslib::ProcessedData;
use regex::Regex;
use tokio::sync::{Mutex, RwLock};
use tokio::task::AbortHandle;
use tokio::time::{interval, timeout};

use crate::connect::DataReceiver;
use crate::connect::DataSender;
use crate::CommandReceiver;
use crate::CommandSender;
use crate::MessageReceiver;
use crate::MessageSender;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    ShuttingDown,
}

#[derive(Clone)]
pub struct ConnectionConfig {
    pub heartbeat_interval: Duration,
    pub heartbeat_timeout: Duration,
    pub command_timeout: Duration,
    pub max_reconnect_attempts: u32,
    pub reconnect_delay: Duration,
    pub channel_capacity: usize,
    pub max_log_size: usize,
    pub max_queue_size: usize,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            heartbeat_interval: Duration::from_secs(30),
            heartbeat_timeout: Duration::from_secs(5),
            command_timeout: Duration::from_secs(10),
            max_reconnect_attempts: 5,
            reconnect_delay: Duration::from_secs(2),
            channel_capacity: 512,
            max_log_size: 10000,
            max_queue_size: 100,
        }
    }
}

pub trait ConnectionStateCallback: Send + Sync {
    fn on_connection_state_changed(&self, old_state: ConnectionState, new_state: ConnectionState);
    fn on_connection_error(&self, error: String);
    fn on_reconnection_attempt(&self, attempt: u32, max_attempts: u32);
}

pub struct Backend {
    pub server_handle: Option<AbortHandle>,
    pub levi_handle: Option<(AbortHandle, AbortHandle)>,
    pub message_transmitter: MessageSender,
    pub message_receiver: MessageReceiver,
    pub command_transmitter: CommandSender,
    pub command_receiver: CommandReceiver,
    pub processed_data_sender: DataSender,
    pub processed_data_receiver: DataReceiver,
    pub log: Log,
    pub save_path: PathBuf,
    pub connection_health_handle: Option<AbortHandle>,
    pub connection_state: Arc<RwLock<ConnectionState>>,
    pub reconnect_attempts: u32,
    pub config: ConnectionConfig,
    pub command_queue: Arc<Mutex<VecDeque<Command>>>,
    pub last_heartbeat_response: Arc<RwLock<Option<Instant>>>,
    pub callbacks: Vec<Box<dyn ConnectionStateCallback>>,
    pub auto_reconnect: bool,
}

impl Default for Backend {
    fn default() -> Self { Self::new() }
}

impl Backend {
    /// # Create a backend instance
    /// this creates two new broadcast channels, one for messages and one for commands.
    ///
    /// this function should only be called once, in main.rs::main()
    pub fn new() -> Self {
        Self::with_config(ConnectionConfig::default())
    }

    pub fn with_config(config: ConnectionConfig) -> Self {
        let (message_transmitter, message_receiver) =
            tokio::sync::broadcast::channel::<Message>(config.channel_capacity);
        let (command_transmitter, command_receiver) =
            tokio::sync::broadcast::channel::<Command>(config.channel_capacity);
        let (processed_data_sender, processed_data_receiver) =
            tokio::sync::broadcast::channel::<ProcessedData>(config.channel_capacity);

        Self {
            server_handle: None,
            levi_handle: None,
            message_transmitter,
            message_receiver,
            command_transmitter,
            command_receiver,
            processed_data_sender,
            processed_data_receiver,
            log: Log { messages: vec![], commands: vec![] },
            save_path: PathBuf::from_str("/Users/andtsa/Desktop/log.txt").unwrap(),
            connection_health_handle: None,
            connection_state: Arc::new(RwLock::new(ConnectionState::Disconnected)),
            reconnect_attempts: 0,
            config,
            command_queue: Arc::new(Mutex::new(VecDeque::new())),
            last_heartbeat_response: Arc::new(RwLock::new(None)),
            callbacks: Vec::new(),
            auto_reconnect: true,
        }
    }

    pub fn add_connection_callback(&mut self, callback: Box<dyn ConnectionStateCallback>) {
        self.callbacks.push(callback);
    }

    async fn set_connection_state(&self, new_state: ConnectionState) {
        let old_state = {
            let mut state = self.connection_state.write().await;
            let old = *state;
            *state = new_state;
            old
        };

        if old_state != new_state {
            for callback in &self.callbacks {
                callback.on_connection_state_changed(old_state, new_state);
            }
        }
    }

    async fn notify_connection_error(&self, error: String) {
        for callback in &self.callbacks {
            callback.on_connection_error(error.clone());
        }
    }

    async fn notify_reconnection_attempt(&self, attempt: u32, max_attempts: u32) {
        for callback in &self.callbacks {
            callback.on_reconnection_attempt(attempt, max_attempts);
        }
    }

    /// # Start a TCP server with connection monitoring and retry logic
    /// ### Uses the backend's existing broadcast channels
    pub async fn start_server(&mut self) -> bool {
        let current_state = *self.connection_state.read().await;
        if current_state == ConnectionState::Connected || current_state == ConnectionState::Connecting {
            self.warn("Server already running or connecting".to_string());
            return false;
        }

        self.set_connection_state(ConnectionState::Connecting).await;

        let m = self.message_transmitter.clone();
        let c = self.command_receiver.resubscribe();
        let t = self.command_transmitter.clone();
        let s = self.processed_data_sender.clone();
        let r = self.processed_data_receiver.resubscribe();
        let state = self.connection_state.clone();
        let heartbeat_response = self.last_heartbeat_response.clone();
        let queue = self.command_queue.clone();
        let config = self.config.clone();
        
        // Start the main server task with retry logic
        self.server_handle = Some(
            tokio::spawn(async move {
                let mut retry_count = 0;
                
                loop {
                    let result = Self::connect_with_retry(
                        m.clone(),
                        c.resubscribe(),
                        t.clone(),
                        r.resubscribe(),
                        s.clone(),
                        state.clone(),
                        heartbeat_response.clone(),
                        queue.clone(),
                        &config,
                    ).await;
                    
                    match result {
                        Ok(_) => {
                            // Connection successful
                            let _ = state.write().await;
                            break;
                        }
                        Err(e) => {
                            retry_count += 1;
                            let _ = m.send(Message::Error(format!("Connection attempt {} failed: {:?}", retry_count, e)));
                            
                            if retry_count >= config.max_reconnect_attempts {
                                let _ = m.send(Message::Error("Maximum connection attempts exceeded".to_string()));
                                let mut conn_state = state.write().await;
                                *conn_state = ConnectionState::Disconnected;
                                break;
                            }
                            
                            let _ = m.send(Message::Info(format!("Retrying connection in {:?}...", config.reconnect_delay)));
                            tokio::time::sleep(config.reconnect_delay).await;
                        }
                    }
                }
            })
            .abort_handle(),
        );

        // Start connection health monitoring
        self.start_connection_health_monitor().await;
        
        self.info("Server started with health monitoring and retry logic".to_string());
        true
    }

    async fn connect_with_retry(
        msg_tx: MessageSender,
        cmd_rx: CommandReceiver,
        cmd_tx: CommandSender,
        data_rx: DataReceiver,
        data_tx: DataSender,
        state: Arc<RwLock<ConnectionState>>,
        heartbeat_response: Arc<RwLock<Option<Instant>>>,
        queue: Arc<Mutex<VecDeque<Command>>>,
        config: &ConnectionConfig,
    ) -> anyhow::Result<()> {
        // Set state to connecting
        {
            let mut conn_state = state.write().await;
            *conn_state = ConnectionState::Connecting;
        }

        // Attempt connection with timeout
        let connect_result = timeout(
            config.command_timeout,
            crate::connect::connect_main(msg_tx.clone(), cmd_rx, cmd_tx.clone(), data_rx, data_tx)
        ).await;

        match connect_result {
            Ok(Ok(_)) => {
                // Connection successful
                {
                    let mut conn_state = state.write().await;
                    *conn_state = ConnectionState::Connected;
                }
                
                // Update heartbeat response time
                {
                    let mut heartbeat = heartbeat_response.write().await;
                    *heartbeat = Some(Instant::now());
                }

                // Process queued commands
                Self::process_command_queue(queue, cmd_tx, msg_tx.clone()).await;
                
                let _ = msg_tx.send(Message::Info("Connection established successfully".to_string()));
                Ok(())
            }
            Ok(Err(e)) => {
                let _ = msg_tx.send(Message::Error(format!("Connection failed: {:?}", e)));
                Err(e)
            }
            Err(_) => {
                let error = anyhow!("Connection timeout");
                let _ = msg_tx.send(Message::Error("Connection timed out".to_string()));
                Err(error)
            }
        }
    }

    async fn process_command_queue(
        queue: Arc<Mutex<VecDeque<Command>>>,
        cmd_tx: CommandSender,
        msg_tx: MessageSender,
    ) {
        let mut queue_guard = queue.lock().await;
        let queue_size = queue_guard.len();
        
        if queue_size > 0 {
            let _ = msg_tx.send(Message::Info(format!("Processing {} queued commands", queue_size)));
            
            while let Some(cmd) = queue_guard.pop_front() {
                if let Err(e) = cmd_tx.send(cmd) {
                    let _ = msg_tx.send(Message::Error(format!("Failed to send queued command: {:?}", e)));
                }
            }
        }
    }

    async fn start_connection_health_monitor(&mut self) {
        let msg_tx = self.message_transmitter.clone();
        let cmd_tx = self.command_transmitter.clone();
        let state = self.connection_state.clone();
        let heartbeat_response = self.last_heartbeat_response.clone();
        let config = self.config.clone();
        
        self.connection_health_handle = Some(
            tokio::spawn(async move {
                let mut health_interval = interval(config.heartbeat_interval);
                let mut heartbeat_counter = 0u64;
                
                loop {
                    health_interval.tick().await;
                    
                    // Only check health if we think we're connected
                    let current_state = *state.read().await;
                    if current_state != ConnectionState::Connected {
                        continue;
                    }
                    
                    heartbeat_counter += 1;
                    
                    // Send heartbeat command (not async, so no timeout)
                    let heartbeat_result = cmd_tx.send(Command::Heartbeat(heartbeat_counter));
                    
                    match heartbeat_result {
                        Ok(_) => {
                            // Check if we received a response recently
                            let last_response = *heartbeat_response.read().await;
                            if let Some(last_time) = last_response {
                                if last_time.elapsed() > config.heartbeat_timeout * 2 {
                                    let _ = msg_tx.send(Message::Warning("Heartbeat response delayed - connection may be unstable".to_string()));
                                }
                            }
                        }
                        Err(e) => {
                            let _ = msg_tx.send(Message::Error(format!("Heartbeat send failed: {:?}", e)));
                            // Mark connection as potentially lost
                            let mut conn_state = state.write().await;
                            *conn_state = ConnectionState::Disconnected;
                        }
                    }
                }
            })
            .abort_handle(),
        );
    }

    pub async fn send_command(&mut self, cmd: Command) -> bool {
        let current_state = *self.connection_state.read().await;
        
        // If not connected, queue the command
        if current_state != ConnectionState::Connected {
            return self.queue_command(cmd).await;
        }

        #[cfg(all(feature = "backend", not(feature = "tui")))]
        if cmd != Command::FrontendHeartbeat(0) && cmd != Command::Heartbeat(0) {
            eprintln!("[backend] sending command {:?}", &cmd);
        }
        
        // Use timeout for sending commands to prevent hanging
        let cmd_tx = self.command_transmitter.clone();
        let msg_tx = self.message_transmitter.clone();
        let state = self.connection_state.clone();
        let queue = self.command_queue.clone();
        let config = self.config.clone();
        
        let cmd_copy = cmd; // Copy for logging
        // broadcast::Sender::send is not async, so just call directly
        let result = cmd_tx.send(cmd);
        
        match result {
            Ok(_) => {
                self.log_cmd(&cmd_copy);
                true
            }
            Err(e) => {
                let _ = msg_tx.send(Message::Error(format!("Failed to send command: {:?}", e)));
                // Queue the command for retry
                self.queue_command_internal(queue, cmd_copy).await;
                false
            }
        }
    }

    async fn queue_command(&mut self, cmd: Command) -> bool {
        let mut log_msgs = Vec::new();
        let mut queue = self.command_queue.lock().await;
        
        if queue.len() >= self.config.max_queue_size {
            log_msgs.push(format!(
                "Command queue full ({}), dropping oldest command",
                self.config.max_queue_size
            ));
            queue.pop_front();
        }
        
        queue.push_back(cmd);
        log_msgs.push(format!("Command queued (queue size: {})", queue.len()));
        
        drop(queue); // Release the lock before logging
        for msg in log_msgs {
            self.info(msg);
        }
        
        // Trigger reconnection if auto-reconnect is enabled
        if self.auto_reconnect {
            let current_state = *self.connection_state.read().await;
            if current_state == ConnectionState::Disconnected {
                tokio::spawn(async move {
                    // Implement reconnection logic here
                });
            }
        }
        
        true
    }

    async fn queue_command_internal(&self, queue: Arc<Mutex<VecDeque<Command>>>, cmd: Command) {
        let mut queue_guard = queue.lock().await;
        
        if queue_guard.len() >= self.config.max_queue_size {
            queue_guard.pop_front();
        }
        
        queue_guard.push_back(cmd);
    }

    pub async fn attempt_reconnect(&mut self) -> bool {
        let current_state = *self.connection_state.read().await;
        if current_state == ConnectionState::Connected || current_state == ConnectionState::Connecting {
            return true; // Already connected or connecting
        }

        if self.reconnect_attempts >= self.config.max_reconnect_attempts {
            self.err(format!("Maximum reconnection attempts ({}) exceeded", self.config.max_reconnect_attempts));
            return false;
        }

        self.reconnect_attempts += 1;
        self.set_connection_state(ConnectionState::Reconnecting).await;
        self.notify_reconnection_attempt(self.reconnect_attempts, self.config.max_reconnect_attempts).await;
        
        self.info(format!("Attempting reconnection... (attempt {}/{})", self.reconnect_attempts, self.config.max_reconnect_attempts));
        
        // Clean up existing connections
        self.quit_server().await;
        
        // Wait before reconnecting
        tokio::time::sleep(self.config.reconnect_delay).await;
        
        // Try to restart server
        let success = self.start_server().await;
        
        if success {
            self.info("Reconnection successful".to_string());
            self.reconnect_attempts = 0; // Reset counter on success
        } else {
            self.warn(format!("Reconnection attempt {} failed", self.reconnect_attempts));
        }
        
        success
    }

    pub fn info(&mut self, msg: String) {
        if let Err(e) = self.message_transmitter.send(Message::Info(msg.clone())) {
            eprintln!("Failed to send info message: {:?}, original message: {}", e, msg);
        }
    }

    pub fn warn(&mut self, msg: String) {
        if let Err(e) = self.message_transmitter.send(Message::Warning(msg.clone())) {
            eprintln!("Failed to send warning message: {:?}, original message: {}", e, msg);
        }
    }

    pub fn err(&mut self, msg: String) {
        if let Err(e) = self.message_transmitter.send(Message::Error(msg.clone())) {
            eprintln!("Failed to send error message: {:?}, original message: {}", e, msg);
        }
    }

    pub fn status(&mut self, status: Info) {
        if let Err(e) = self.message_transmitter.send(Message::Status(status)) {
            eprintln!("Failed to send status message: {:?}", e);
        }
    }

    pub async fn quit_server(&mut self) {
        self.set_connection_state(ConnectionState::ShuttingDown).await;
        
        // Stop health monitoring first
        if let Some(health_handle) = self.connection_health_handle.take() {
            self.info("Stopping connection health monitor".into());
            health_handle.abort();
        }

        // Gracefully shutdown server
        if let Some(sh) = self.server_handle.take() {
            self.info("Gracefully shutting down server".into());
            
            // Send shutdown command (not async, so no timeout)
            let cmd_tx = self.command_transmitter.clone();
            let shutdown_result = cmd_tx.send(Command::Shutdown(0));
            if shutdown_result.is_err() {
                self.warn("Shutdown command failed to send".into());
            }
            // Give the server time to shutdown gracefully
            tokio::time::sleep(Duration::from_secs(1)).await;
            sh.abort();
        }
        
        self.set_connection_state(ConnectionState::Disconnected).await;
    }

    pub fn save(&self) -> anyhow::Result<Message> {
        Self::save_to_path(&self.log, self.save_path.clone())
    }

    pub fn save_to_path(log: &Log, path: PathBuf) -> anyhow::Result<Message> {
        let json = serde_json::to_string_pretty(log)?;
        match std::fs::write(path.clone(), json) {
            Ok(_) => Ok(Message::Info(format!("Saved to {:?}", &path))),
            Err(e) => Ok(Message::Error(format!("Failed to save at {:?}: {:?}", &path, e))),
        }
    }

    pub fn log_msg(&mut self, msg: &Message) { 
        self.log.messages.push(msg.clone()); 
        
        // Limit log size to prevent memory issues
        if self.log.messages.len() > self.config.max_log_size {
            let drain_count = self.config.max_log_size / 10; // Remove 10% of messages
            self.log.messages.drain(0..drain_count);
        }
    }

    pub fn log_cmd(&mut self, cmd: &Command) { 
        self.log.commands.push(*cmd); 
        
        // Limit log size to prevent memory issues
        if self.log.commands.len() > self.config.max_log_size {
            let drain_count = self.config.max_log_size / 10; // Remove 10% of commands
            self.log.commands.drain(0..drain_count);
        }
    }

    pub async fn is_connected(&self) -> bool {
        *self.connection_state.read().await == ConnectionState::Connected
    }

    pub async fn get_connection_state(&self) -> ConnectionState {
        *self.connection_state.read().await
    }

    pub fn set_auto_reconnect(&mut self, enabled: bool) {
        self.auto_reconnect = enabled;
    }

    pub async fn get_queue_size(&self) -> usize {
        self.command_queue.lock().await.len()
    }

    pub async fn clear_queue(&mut self) {
        let mut log_msgs = Vec::new();
        let mut queue = self.command_queue.lock().await;
        let size = queue.len();
        queue.clear();
        log_msgs.push(format!("Cleared {} commands from queue", size));
        drop(queue);
        for msg in log_msgs {
            self.info(msg);
        }
    }

    // Rest of the methods remain the same...
    pub fn load_procedures(folder: PathBuf) -> anyhow::Result<Vec<[String; 6]>> {
        let mut r = vec![];

        for entry in std::fs::read_dir(folder)? {
            let f = entry?;

            let f_name = f
                .file_name()
                .to_str()
                .ok_or_else(|| anyhow!("Invalid procedure file name: {:?}", f))?
                .to_string();
            if f_name.ends_with(".procedure") {
                let contents = std::fs::read_to_string(f.path())?;
                let mut lines = contents.lines();
                let title = lines
                    .next()
                    .ok_or_else(|| anyhow!("Missing procedure title"))?
                    .trim_start_matches([' ', '#'])
                    .to_string();

                let (id, people, equipment, content) = Self::parse_procedure_content(
                    lines.fold(String::new(), |acc, x| acc + x + "\n"),
                )?;

                r.push([
                    f_name.trim_end_matches(".procedure").to_string(),
                    title,
                    id,
                    people,
                    equipment,
                    content,
                ]);
            }
        }

        Ok(r)
    }

    fn parse_procedure_content(
        content: String,
    ) -> anyhow::Result<(String, String, String, String)> {
        let mut x = (String::new(), String::new(), String::new(), String::new());

        let all = r#"[a-zA-Z0-9\n .;!,:(){}\"'_/+-=\[\]\t%<>]"#;

        x.0 = Regex::new("\n[iI][dD][: ]*\n([a-zA-Z0-9\n ._]*)\n\n")
            .unwrap()
            .captures(&content)
            .ok_or_else(|| anyhow!("\nMissing \"ID\" field for procedure:\n{:?}", content))?
            .get(1)
            .unwrap()
            .as_str()
            .to_string();

        x.1 = Regex::new(&format!("\nPeople[: ]*\n({}*)\nItems", all))
            .unwrap()
            .captures(&content)
            .ok_or_else(|| anyhow!("\nMissing \"People\" field for procedure:\n{:?}", content))?
            .get(1)
            .unwrap()
            .as_str()
            .to_string();

        x.2 = Regex::new(&format!("\nItems[: ]*\n({}*)\nProcedures", all))
            .unwrap()
            .captures(&content)
            .ok_or_else(|| anyhow!("\nMissing \"Items\" field for procedure:\n{:?}", content))?
            .get(1)
            .unwrap()
            .as_str()
            .to_string();

        x.3 = Regex::new(&format!("\nProcedures[: ]*\n({}*)\n", all))
            .unwrap()
            .captures(&content)
            .ok_or_else(|| anyhow!("\nMissing \"Procedures\" field for procedure:\n{:?}", content))?
            .get(1)
            .unwrap()
            .as_str()
            .lines()
            .map(|x| format!("<p><input type=\"checkbox\"> {x}</p>"))
            .collect::<Vec<String>>()
            .join("\n");

        Ok(x)
    }
}

// Example callback implementation
pub struct DefaultConnectionCallback;

impl ConnectionStateCallback for DefaultConnectionCallback {
    fn on_connection_state_changed(&self, old_state: ConnectionState, new_state: ConnectionState) {
        println!("Connection state changed: {:?} -> {:?}", old_state, new_state);
    }

    fn on_connection_error(&self, error: String) {
        eprintln!("Connection error: {}", error);
    }

    fn on_reconnection_attempt(&self, attempt: u32, max_attempts: u32) {
        println!("Reconnection attempt {}/{}", attempt, max_attempts);
    }
}

#[cfg(test)]
#[path = "tests/backend.rs"]
mod tests;