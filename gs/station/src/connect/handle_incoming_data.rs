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
        // TODO: Add 'events' here as datapoints, but probably generated in config
        Datatype::CommandHash => {
            if data.value != COMMAND_HASH {
                msg_sender.send(Message::Error("Command hash mismatch".to_string()))?;
                msg_sender.send(Message::Status(Info::CommandHashFailed))?;
            } else {
                msg_sender.send(Message::Status(Info::CommandHashPassed))?;
            }
        },
        Datatype::DataHash => {
            if data.value != DATA_HASH {
                msg_sender.send(Message::Error("Data hash mismatch".to_string()))?;
                msg_sender.send(Message::Status(Info::DataHashFailed))?;
            } else {
                msg_sender.send(Message::Status(Info::DataHashPassed))?;
            }
        },
        Datatype::ConfigHash => {
            if data.value != CONFIG_HASH {
                msg_sender.send(Message::Error("Config hash mismatch".to_string()))?;
                msg_sender.send(Message::Status(Info::ConfigHashFailed))?;
            } else {
                msg_sender.send(Message::Status(Info::ConfigHashPassed))?;
            }
        },
        Datatype::ResetFSM => {
            msg_sender.send(Message::Status(Info::ResettingMainPCB))?;
        }
        _ => {},
    }

    Ok(())
}
