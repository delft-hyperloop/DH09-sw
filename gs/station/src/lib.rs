#![allow(dead_code)]
use std::str::FromStr;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use ratatui::prelude::Color;

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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProcessedData {
    pub datatype: Datatype,
    pub value: f64,
    pub timestamp: u64,
    pub style: String,
    pub units: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Message {
    Data(ProcessedData),
    Status(Info),
    Info(String),
    Warning(String),
    Error(String),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Log {
    pub messages: Vec<Message>,
    pub commands: Vec<Command>,
}

impl Info {
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
