#![allow(clippy::single_match)]

use gslib::Datapoint;
use gslib::Datatype;
use gslib::Info;
use gslib::Message;
use gslib::COMMAND_HASH;
use gslib::CONFIG_HASH;
use gslib::DATA_HASH;

use crate::data::process::process;
use crate::MessageSender;

pub async fn handle_incoming_data(
    data: Datapoint,
    msg_sender: MessageSender,
) -> anyhow::Result<()> {
    msg_sender.send(Message::Data(process(&data)))?;

    match data.datatype {
        Datatype::CommandHash => {
            if data.value != COMMAND_HASH {
                msg_sender.send(Message::Error("Command Hash Mismatch".to_string()))?;
            } else {
                msg_sender.send(Message::Status(Info::CommandHashPassed))?;
            }
        },
        Datatype::DataHash => {
            if data.value != DATA_HASH {
                msg_sender.send(Message::Error("Data Hash Mismatch".to_string()))?;
            } else {
                msg_sender.send(Message::Status(Info::DataHashPassed))?;
            }
        },
        Datatype::ConfigHash => {
            if data.value != CONFIG_HASH {
                msg_sender.send(Message::Error("Config Hash Mismatch".to_string()))?;
            } else {
                msg_sender.send(Message::Status(Info::ConfigHashPassed))?;
            }
        },
        Datatype::ResetFSM => {
            msg_sender.send(Message::Status(Info::ResettingMainPCB))?;
        },
        Datatype::Prop1SystemCheckSuccess => {
            msg_sender.send(Message::Status(Info::Propulsion1SystemCheckSuccess))?;
        },
        Datatype::Prop2SystemCheckSuccess => {
            msg_sender.send(Message::Status(Info::Propulsion2SystemCheckSuccess))?;
        },
        Datatype::LeviSystemCheckSuccess => {
            msg_sender.send(Message::Status(Info::LeviSystemCheckSuccess))?;
        },
        Datatype::LeviSystemCheckFailure => {
            msg_sender.send(Message::Error("Levi System Check Failure".to_string()))?;
        },
        Datatype::Prop1SystemCheckFailure => {
            msg_sender.send(Message::Error("Prop 1 System Check Failure".to_string()))?;
        },
        Datatype::Prop2SystemCheckFailure => {
            msg_sender.send(Message::Error("Prop 2 System Check Failure".to_string()))?;
        },
        _ => {},
    }

    Ok(())
}
