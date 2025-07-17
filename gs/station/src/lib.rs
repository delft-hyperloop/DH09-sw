#![allow(dead_code, clippy::match_like_matches_macro)]
use std::collections::BTreeMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
#[cfg(feature = "tui")]
use std::str::FromStr;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::time::Instant;

use anyhow::anyhow;
use chrono::Local;
#[cfg(feature = "tui")]
use ratatui::prelude::Color;

// use crate::POD_IP_ADDRESS;

include!(concat!(env!("OUT_DIR"), "/config.rs"));

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Copy)]
pub struct Datapoint {
    pub datatype: Datatype,
    pub value: u64,
    pub timestamp: u64,
}

impl Datapoint {
    pub fn new(datatype: Datatype, value: u64, timestamp: u64) -> Self {
        Self { datatype, value, timestamp }
    }

    pub fn from_bytes(buf: &[u8; 20]) -> Self {
        Datapoint::new(
            Datatype::from_id(u16::from_be_bytes([buf[1], buf[2]])),
            u64::from_le_bytes([buf[3], buf[4], buf[5], buf[6], buf[7], buf[8], buf[9], buf[10]]),
            u64::from_le_bytes([
                buf[11], buf[12], buf[13], buf[14], buf[15], buf[16], buf[17], buf[18],
            ]),
        )
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct ProcessedData {
    pub datatype: Datatype,
    pub value: f64,
    pub timestamp: u64,
    pub style: String,
    pub units: String,
    pub lower: Option<u64>,
    pub upper: Option<u64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Message {
    Data(ProcessedData),
    Status(Info),
    Info(String),
    Warning(String),
    Error(String),
}

pub struct Log {
    pub start_time: Instant,
    pub path: PathBuf,
    pub messages: Vec<(Message, Instant)>,
    pub commands: Vec<(Command, Instant)>,
}

pub struct LogRow {
    pub since_mainpcb_boot: u64,
    pub datatypes: BTreeMap<Datatype, f64>,
    pub status: String,
    pub info: String,
    pub warning: String,
    pub error: String,
    pub command: String,
}

impl LogRow {
    fn print_datatypes(dmap: &BTreeMap<Datatype, f64>) -> String {
        let mut out = String::new();
        let mut list = dmap.iter().map(|(x, y)| (*x, *y)).collect::<Vec<(Datatype, f64)>>();
        list.sort_by_key(|e| e.0);

        for (_, v) in &list {
            if v.is_nan() {
                out.push(',');
            } else {
                out.push_str(&format!("{v},"));
            }
        }

        out
    }

    pub fn to_csv_string(&self) -> String {
        let mut out = format!("{},", self.since_mainpcb_boot);
        out.push_str(&Self::print_datatypes(&self.datatypes));
        out.push_str(&format!(
            "{},{},{},{},{}",
            self.status, self.info, self.warning, self.error, self.command
        ));
        out
    }
}

impl Log {
    pub fn now() -> Self {
        let now = Local::now().naive_local();
        let formatted_time = now.format("%d_%m_%Y_at_%H_%M_%S").to_string();

        let path = dirs::download_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap())
            .join(format!("log-{formatted_time}.csv"));

        let data_header = DATA_IDS
            .iter()
            .map(|x| Datatype::from_id(*x))
            .map(|x| format!("{x:?}"))
            .collect::<Vec<String>>()
            .join(",");

        let header = format!("μs_since_gs_boot,ticks_since_pcb_boot,{data_header},status,info,warning,error,command_sent\n");

        let mut f = File::create(&path)
            .map_err(|e| {
                anyhow!("could not open file. error: {e:?}.\nraw log output:\n\n{header}\n\n")
            })
            .unwrap();

        f.write_all(header.as_bytes())
            .map_err(|e| {
                anyhow!("could not write to file. error: {e:?}.\nraw log output:\n\n{header}\n\n")
            })
            .unwrap();

        Log { start_time: Instant::now(), messages: vec![], commands: vec![], path }
    }

    pub fn save_csv(&mut self) -> anyhow::Result<()> {
        // for quick writing during a run,
        // we save timestamps as Instants,
        // and convert to microseconds since boot here.
        // We save the data in a time series,
        // with one row per timestamp and one column per datatype.

        // Columns:
        // μs since gs boot | message timestamp (ticks since mainpcb boot) | datatype name |

        let mut table: BTreeMap<u128, LogRow> = BTreeMap::new();

        for (msg, t) in &self.messages {
            let ts = t.duration_since(self.start_time).as_micros();
            match msg {
                Message::Data(d) => {
                    table
                        .entry(ts)
                        .and_modify(|e| {
                            e.datatypes.insert(d.datatype, d.value);
                            e.since_mainpcb_boot = d.timestamp;
                        })
                        .or_insert(LogRow {
                            since_mainpcb_boot: d.timestamp,
                            datatypes: BTreeMap::from([(d.datatype, d.value)]),
                            status: String::new(),
                            info: String::new(),
                            warning: String::new(),
                            error: String::new(),
                            command: String::new(),
                        });
                },
                Message::Status(s) => {
                    table
                        .entry(ts)
                        .and_modify(|e| {
                            e.status = format!("{s:?}");
                        })
                        .or_insert(LogRow {
                            since_mainpcb_boot: 0,
                            datatypes: BTreeMap::new(),
                            status: format!("{s:?}"),
                            info: String::new(),
                            warning: String::new(),
                            error: String::new(),
                            command: String::new(),
                        });
                },
                Message::Info(i) => {
                    table
                        .entry(ts)
                        .and_modify(|e| {
                            e.info = i.clone();
                        })
                        .or_insert(LogRow {
                            since_mainpcb_boot: 0,
                            datatypes: BTreeMap::new(),
                            status: String::new(),
                            info: i.clone(),
                            warning: String::new(),
                            error: String::new(),
                            command: String::new(),
                        });
                },
                Message::Warning(w) => {
                    table
                        .entry(ts)
                        .and_modify(|e| {
                            e.warning = w.clone();
                        })
                        .or_insert(LogRow {
                            since_mainpcb_boot: 0,
                            datatypes: BTreeMap::new(),
                            status: String::new(),
                            info: String::new(),
                            warning: w.clone(),
                            error: String::new(),
                            command: String::new(),
                        });
                },
                Message::Error(ee) => {
                    table
                        .entry(ts)
                        .and_modify(|e| {
                            e.error = ee.clone();
                        })
                        .or_insert(LogRow {
                            since_mainpcb_boot: 0,
                            datatypes: BTreeMap::new(),
                            status: String::new(),
                            info: String::new(),
                            warning: String::new(),
                            error: ee.clone(),
                            command: String::new(),
                        });
                },
            }
        }

        for (cmd, t) in &self.commands {
            let ts = t.duration_since(self.start_time).as_micros();

            let cmd_name = format!("{cmd:?}").split_once("(").unwrap().0.to_string();
            table
                .entry(ts)
                .and_modify(|e| {
                    e.command = cmd_name.clone();
                })
                .or_insert(LogRow {
                    since_mainpcb_boot: 0,
                    datatypes: BTreeMap::new(),
                    status: String::new(),
                    info: String::new(),
                    warning: String::new(),
                    error: String::new(),
                    command: cmd_name,
                });
        }

        for (_k, e) in table.iter_mut() {
            for d in DATA_IDS {
                let dt = Datatype::from_id(d);
                e.datatypes.entry(dt).or_insert(f64::NAN);
            }
        }

        let mut content = table.into_iter().collect::<Vec<_>>();
        content.sort_by_key(|(k, _)| *k);
        let text_content = content
            .into_iter()
            .map(|(k, v)| format!("{k},{}\n", v.to_csv_string()))
            .collect::<String>();

        // let mut path = to;
        // let mut n = 1;
        // while path.is_file() {
        //     path = PathBuf::from(format!(
        //         "{}_{n}{}",
        //         path.parent().unwrap().join(path.file_stem().unwrap()).display(),
        //         path.extension().unwrap_or_default().display()
        //     ));
        //     n += 1;
        // }

        let mut f = OpenOptions::new().append(true).open(&self.path).map_err(|e| {
            anyhow!("could not open file. error: {e:?}.\nraw log output:\n\n{text_content}\n\n")
        })?;

        f.write_all(text_content.as_bytes()).map_err(|e| {
            anyhow!("could not write to file. error: {e:?}.\nraw log output:\n\n{text_content}\n\n")
        })?;

        self.messages.clear();
        self.commands.clear();

        Ok(())
    }
}

impl Info {
    #[cfg(feature = "tui")]
    pub fn colour(&self) -> Color { Color::from_str(self.to_colour_str()).unwrap() }
}

pub struct ApiStruct {
    pub command_sender: Sender<Command>,
    pub command_receiver: Receiver<Command>,
    pub message_sender: Sender<Message>,
    pub message_receiver: Receiver<Message>,
}

pub fn socket() -> std::net::SocketAddr {
    std::net::SocketAddr::new(std::net::IpAddr::from([0, 0, 0, 0]), GS_IP_ADDRESSES[0].1)
}
