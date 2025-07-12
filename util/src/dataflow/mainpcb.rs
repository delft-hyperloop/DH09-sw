use crate::dataflow::procedures::make_procedures;
use crate::dataflow::*;

pub fn make_main_pcb_code(df: &DataflowSpec) -> String {
    let mut code = String::from(
        r#"
use crate::Datapoint;
use core::future::Future;

#[inline(always)]
fn apply_trim_0(data: [u8; 8], _ctxt: &str) -> [u8; 8] {
    data
}

#[inline(always)]
fn apply_trim_1(data: [u8; 8], ctxt: &str) -> [u8; 7] {
    let mut trimmed = [0; 7];
    trimmed.copy_from_slice(&data[1..]);
    for i in 0..1 {
        if data[i] != 0 {
            defmt::warn!("trimming non-zero byte at index {} ({})", i, ctxt);
        }
    }
    trimmed
}

#[inline(always)]
fn apply_trim_2(data: [u8; 8], ctxt: &str) -> [u8; 6] {
    let mut trimmed = [0; 6];
    trimmed.copy_from_slice(&data[2..]);
    for i in 0..2 {
        if data[i] != 0 {
            defmt::warn!("trimming non-zero byte at index {} ({})", i, ctxt);
        }
    }
    trimmed
}

#[inline(always)]
fn apply_trim_3(data: [u8; 8], ctxt: &str) -> [u8; 5] {
    let mut trimmed = [0; 5];
    trimmed.copy_from_slice(&data[3..]);
    for i in 0..3 {
        if data[i] != 0 {
            defmt::warn!("trimming non-zero byte at index {} ({})", i, ctxt);
        }
    }
    trimmed
}

#[inline(always)]
fn apply_trim_4(data: [u8; 8], ctxt: &str) -> [u8; 4] {
    let mut trimmed = [0; 4];
    trimmed.copy_from_slice(&data[4..]);
    for i in 0..4 {
        if data[i] != 0 {
            defmt::warn!("trimming non-zero byte at index {} ({})", i, ctxt);
        }
    }
    trimmed
}

#[inline(always)]
fn apply_trim_5(data: [u8; 8], ctxt: &str) -> [u8; 3] {
    let mut trimmed = [0; 3];
    trimmed.copy_from_slice(&data[5..]);
    for i in 0..5 {
        if data[i] != 0 {
            defmt::warn!("trimming non-zero byte at index {} ({})", i, ctxt);
        }
    }
    trimmed
}

#[inline(always)]
fn apply_trim_6(data: [u8; 8], ctxt: &str) -> [u8; 2] {
    let mut trimmed = [0; 2];
    trimmed.copy_from_slice(&data[6..]);
    for i in 0..6 {
        if data[i] != 0 {
            defmt::warn!("trimming non-zero byte at index {} ({})", i, ctxt);
        }
    }
    trimmed
}

#[inline(always)]
fn apply_trim_7(data: [u8; 8], ctxt: &str) -> [u8; 1] {
    let mut trimmed = [0; 1];
    trimmed.copy_from_slice(&data[7..]);
    for i in 0..7 {
        if data[i] != 0 {
            defmt::warn!("trimming non-zero byte at index {} ({})", i, ctxt);
        }
    }
    trimmed
}

#[inline(always)]
fn apply_trim_8(data: [u8; 8], ctxt: &str) -> [u8; 0] {
    let mut trimmed = [0; 0];
    for i in 0..8 {
        if data[i] != 0 {
            defmt::warn!("trimming non-zero byte at index {} ({})", i, ctxt);
        }
    }
    trimmed
}
"#,
    );
    let mut can1_ids_to_events = HashMap::new();
    let mut can2_ids_to_events = HashMap::new();
    let proc = make_procedures(df);

    for mp in &df.message_processing {
        if let Some(fsm) = &mp.fsm {
            match &mp.can {
                CanSpec::Can1 { id, .. } => {
                    if let Some(old_event) = can1_ids_to_events.insert(*id, &fsm.event) {
                        panic!(
                            "duplicate event for CAN1 id: {} ({} and {})",
                            id, old_event, fsm.event
                        );
                    }
                },
                CanSpec::Can2 { id, .. } => {
                    if let Some(old_event) = can2_ids_to_events.insert(*id, &fsm.event) {
                        panic!(
                            "duplicate event for CAN2 id: {} ({} and {})",
                            id, old_event, fsm.event
                        );
                    }
                },
            }
        }
    }

    writeln!(&mut code, "pub fn event_for_can_1_id(id: u32) -> crate::Event {{ match id {{")
        .unwrap();

    for (id, event) in &can1_ids_to_events {
        writeln!(&mut code, "{id} => crate::Event::{event},").unwrap();
    }

    writeln!(
        &mut code,
        "_ => crate::Event::NoEvent,
        }}
    }}",
    )
    .unwrap();

    writeln!(&mut code, "pub fn event_for_can_2_id(id: u32) -> crate::Event {{ match id {{")
        .unwrap();

    for (id, event) in &can2_ids_to_events {
        writeln!(&mut code, "{id} => crate::Event::{event},").unwrap();
    }

    writeln!(
        &mut code,
        "_ =>crate::Event::NoEvent,
        }}
    }}",
    )
    .unwrap();

    writeln!(&mut code, "pub async fn parse_datapoints_can_1<F, Fut>(id: u32, data: &[u8], mut f: F) where F: FnMut(Datapoint) -> Fut, Fut: Future<Output=()> {{ {proc} match id {{").unwrap();
    for mp in &df.message_processing {
        if let CanSpec::Can1 { id, .. } = mp.can {
            writeln!(
                &mut code,
                "{id} => {{
                    "
            )
            .unwrap();

            code.push_str(&make_datapoint_parser(mp));

            writeln!(
                &mut code,
                "}}
                "
            )
            .unwrap();
        }
    }
    writeln!(&mut code, "_ => {{}}}}}}").unwrap();

    writeln!(&mut code, "pub async fn parse_datapoints_can_2<F, Fut>(id: u32, data: &[u8], mut f: F) where F: FnMut(Datapoint) -> Fut, Fut: Future<Output=()> {{ {proc} match id {{").unwrap();
    for mp in &df.message_processing {
        if let CanSpec::Can2 { id, .. } = mp.can {
            writeln!(
                &mut code,
                "{id} => {{
                    "
            )
            .unwrap();

            code.push_str(&make_datapoint_parser(mp));

            writeln!(
                &mut code,
                "}}
                "
            )
            .unwrap();
        }
    }
    writeln!(&mut code, "_ => {{}}}}}}").unwrap();

    writeln!(&mut code, "pub fn match_can_to_datatypes(id: u32) -> [Datatype; 8] {{ match id {{")
        .unwrap();
    for mp in &df.message_processing {
        if let CanSpec::Can2 { id, .. } = mp.can {
            writeln!(&mut code, "{id} => [ ").unwrap();

            let mut count = 0;
            for dpc in &mp.datapoint_conversion {
                write!(&mut code, "Datatype::{}, ", dpc.datapoint.name).unwrap();
                count += 1;
            }

            loop {
                if count < 8 {
                    write!(&mut code, "Datatype::DefaultDatatype, ").unwrap();
                } else {
                    break;
                }
                count += 1;
            }

            writeln!(&mut code, "],").unwrap();
        }
    }

    writeln!(
        &mut code,
        "_ => [{}Datatype::DefaultDatatype],}}}}",
        "Datatype::DefaultDatatype, ".repeat(7)
    )
    .unwrap();

    let mut can1commands = vec![];
    let mut can2commands = vec![];

    for command in &df.commands {
        if let Some(CanCommandSpec { can: CanSpec::Can1 { id, .. }, conversion, trim }) =
            &command.can
        {
            can1commands.push((&command.name, *id, conversion, trim));
        } else if let Some(CanCommandSpec { can: CanSpec::Can2 { id, .. }, conversion, trim }) =
            &command.can
        {
            can2commands.push((&command.name, *id, conversion, trim));
        }
    }

    writeln!(&mut code, "pub async fn gs_to_can1<F, Fut>(command: Command, mut f: F) where F: FnMut(crate::can::fdcan::CanEnvelope) -> Fut, Fut: Future<Output=()> {{ {proc}\n\nmatch command {{").unwrap();
    for (command_name, id, conversion, trim) in &can1commands {
        writeln!(
            &mut code,
            r#"Command::{command_name}(v) => {{
                let data = {apply_trim}({conversion}(v), "{command_name}");
                f(crate::can::can1::CanEnvelope::new_from_frame(embassy_stm32::can::frame::FdFrame::new_extended({id}, &data).expect("Invalid frame!"))).await;
            }}"#,
            conversion = conversion.as_deref().unwrap_or("default_command_process"),
            apply_trim = format_args!("apply_trim_{trim}", trim = trim.0),
        )
            .unwrap();
    }
    writeln!(&mut code, "_ => {{}}}}}}").unwrap();

    writeln!(&mut code, "pub async fn gs_to_can2<F, Fut>(command: Command, mut f: F) where F: FnMut(crate::can::can2::CanEnvelope) -> Fut, Fut: Future<Output=()> {{ {proc}\n\nmatch command {{").unwrap();
    for (command_name, id, conversion, trim) in &can2commands {
        writeln!(
            &mut code,
            r#"Command::{command_name}(v) => {{
                let data = {apply_trim}({conversion}(v), "{command_name}");
                f(crate::can::can2::CanEnvelope::new_from_frame(embassy_stm32::can::frame::Frame::new_standard({id}, &data).expect("Invalid frame!"))).await;
            }}"#,
            conversion = conversion.as_deref().unwrap_or("default_command_process"),
            apply_trim = format_args!("apply_trim_{trim}", trim = trim.0),
        )
            .unwrap();
    }
    writeln!(&mut code, "_ => {{}}}}}}").unwrap();

    code
}
