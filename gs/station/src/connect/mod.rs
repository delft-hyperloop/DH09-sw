mod handle_incoming_data;
mod queueing;
mod tcp_reader;
mod tcp_writer;

use anyhow::Result;
use gslib::socket;
use gslib::Info;
use gslib::Message;
use gslib::ProcessedData;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::task::JoinHandle;

use crate::connect::tcp_reader::get_messages_from_tcp;
use crate::connect::tcp_writer::transmit_commands_to_tcp;
use crate::CommandReceiver;
use crate::MessageSender;

pub type DataReceiver = tokio::sync::broadcast::Receiver<ProcessedData>;
pub type DataSender = tokio::sync::broadcast::Sender<ProcessedData>;

pub async fn connect_main(
    message_transmitter: MessageSender,
    command_receiver: CommandReceiver,
) -> Result<()> {
    // connect the stream to the address
    message_transmitter.send(Message::Warning(format!("trying to connect... {:?}", socket())))?;
    // let connection = TcpStream::connect(socket()).await?;
    let connection = TcpListener::bind(socket()).await?;
    let (connection, x) = connection.accept().await?;
    message_transmitter.send(Message::Warning(format!("connected with {x:?}")))?;
    message_transmitter.send(Message::Status(Info::ConnectionEstablished))?;
    let (x, y) =
        process_stream(connection, message_transmitter.clone(), command_receiver.resubscribe())
            .await?;

    x.await?;
    y.await?;

    Ok(())
}

async fn process_stream(
    socket: TcpStream,
    message_transmitter: MessageSender,
    command_receiver: CommandReceiver,
) -> Result<(JoinHandle<()>, JoinHandle<()>)> {
    let (reader, writer) = socket.into_split();
    let transmit = message_transmitter.clone();
    let a = tokio::spawn(async move {
        match get_messages_from_tcp(reader, transmit.clone()).await {
            Ok(_) => {
                transmit
                    .send(Message::Warning(
                        "[get_messages_from_tcp] finished with no errors.".to_string(),
                    ))
                    .expect("messaging channel closed... this is irrecoverable");
            },
            Err(e) => {
                transmit
                    .send(Message::Error(format!(
                        "[get_messages_from_tcp] finished with errors: {e:?}"
                    )))
                    .expect("messaging channel closed... this is irrecoverable");
            },
        }
    });
    let transmit = message_transmitter.clone();
    let b = tokio::spawn(async move {
        match transmit_commands_to_tcp(command_receiver, transmit.clone(), writer).await {
            Ok(_) => {
                transmit
                    .send(Message::Warning(
                        "[transmit_commands_to_tcp] finished with no errors.".to_string(),
                    ))
                    .expect("messaging channel closed... this is irrecoverable");
            },
            Err(e) => {
                transmit
                    .send(Message::Error(format!(
                        "[transmit_commands_to_tcp] finished with errors: {e:?}"
                    )))
                    .expect("messaging channel closed... this is irrecoverable");
            },
        }
    });

    Ok((a, b))
}
