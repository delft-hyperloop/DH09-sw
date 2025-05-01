use std::collections::HashMap;
use std::fmt::format;
use std::fmt::Display;
use std::fmt::Write;
use std::str::FromStr;

use crate::commands;
use crate::datatypes::Limit;
use crate::datatypes::StoreInfo;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct DataflowSpec {
    pub procedures: HashMap<String, ProcedureSpec>,
    pub standard_datapoints: Vec<StandardDatapointSpec>,
    pub message_processing: Vec<MessageProcessingSpec>,
    pub commands: Vec<CommandSpec>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct StandardDatapointSpec {
    pub datapoint: DatapointSpec,
    pub priority: Option<usize>,
}

#[derive(serde::Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(try_from = "String")]
pub enum Ty {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F16,
    F32,
    F64,
    U8Arr(usize),
}

impl Ty {
    pub fn ty_size(self) -> usize {
        match self {
            Self::U8 | Self::I8 => 1,
            Self::U16 | Self::I16 | Self::F16 => 2,
            Self::U32 | Self::I32 | Self::F32 => 4,
            Self::U64 | Self::I64 | Self::F64 => 8,
            Self::U8Arr(n) => n,
        }
    }
}

impl FromStr for Ty {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "u8" => Ok(Self::U8),
            "u16" => Ok(Self::U16),
            "u32" => Ok(Self::U32),
            "u64" => Ok(Self::U64),
            "i8" => Ok(Self::I8),
            "i16" => Ok(Self::I16),
            "i32" => Ok(Self::I32),
            "i64" => Ok(Self::I64),
            "f16" => Ok(Self::F16),
            "f32" => Ok(Self::F32),
            "f64" => Ok(Self::F64),
            s if s.starts_with("[u8;") && s.ends_with(']') => {
                let n = s[4..s.len() - 1].trim().parse().map_err(|_| "invalid array size")?;
                Ok(Self::U8Arr(n))
            },
            _ => Err("invalid type"),
        }
    }
}

impl TryFrom<String> for Ty {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> { value.parse() }
}

impl Display for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U8 => write!(f, "u8"),
            Self::U16 => write!(f, "u16"),
            Self::U32 => write!(f, "u32"),
            Self::U64 => write!(f, "u64"),
            Self::I8 => write!(f, "i8"),
            Self::I16 => write!(f, "i16"),
            Self::I32 => write!(f, "i32"),
            Self::I64 => write!(f, "i64"),
            Self::F16 => write!(f, "f16"),
            Self::F32 => write!(f, "f32"),
            Self::F64 => write!(f, "f64"),
            Self::U8Arr(n) => write!(f, "[u8; {}]", n),
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct ProcedureSpec {
    pub input: Ty,
    pub output: Ty,
    pub formula: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct MessageProcessingSpec {
    pub name: String,
    pub can: CanSpec,
    pub fsm: Option<FsmSpec>,
    pub datapoint_conversion: Vec<DatapointConversionSpec>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(tag = "bus", rename_all = "lowercase")]
pub enum CanSpec {
    Can1 {
        id: u32,
    },
    Can2 {
        id: u32,
        #[serde(flatten)]
        comes_from_levi: Option<Can2ComesFromLevi>,
    },
}

/// for auto-generation of the code for the levi Beckhoff pc.
#[derive(serde::Deserialize, Debug)]
pub struct Can2ComesFromLevi {
    /// once how many milliseconds the data is logged
    pub log_period: u32,
}

#[derive(serde::Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FsmSpec {
    pub event: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct DatapointConversionSpec {
    pub datapoint: DatapointSpec,
    pub getter: GetterSpec,
    pub can_conversion: CanConversionSpec,
    pub display_units: Option<String>,
    pub limits: Option<LimitsSpec>,
    pub gs: DatapointConversionGsSpec,
    #[serde(rename = "beckhoff")]
    pub comes_from_levi_info: Option<DatapointComesFromLeviInfo>,
}

#[derive(serde::Deserialize, Debug)]
pub struct DatapointComesFromLeviInfo {
    pub name: String,
    #[serde(default = "default_input_spec")]
    pub input_spec: String,
    pub levi_type: StructuredTy,
}

fn default_input_spec() -> String { "%I*".to_string() }

#[derive(serde::Deserialize, Debug)]
pub enum StructuredTy {
    Byte,
    Integer,
    Real,
}

impl StructuredTy {
    fn make_input(&self, name: &str, addr: &str) -> String {
        match self {
            Self::Byte => format!("{name} AT {addr}: BYTE;"),
            Self::Integer => format!("{name} AT {addr}: INT;"),
            Self::Real => format!("{name} AT {addr}: REAL;"),
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct LimitsSpec {
    pub lower: Limit,
    pub upper: Limit,
}

#[derive(serde::Deserialize, Debug)]
#[serde(try_from = "String")]
pub struct CanConversionSpec {
    pub proc_name: String,
    pub input: Ty,
    pub output: Ty,
}

impl FromStr for CanConversionSpec {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((proc_name, proc_ty)) = s.split_once(':') else {
            return Err("missing colon");
        };

        let Some((input_ty, output_ty)) = proc_ty.split_once("->") else {
            return Err("missing conversion arrow");
        };

        let input = input_ty.parse::<Ty>()?;
        let output = output_ty.parse::<Ty>()?;

        Ok(Self { proc_name: proc_name.to_string(), input, output })
    }
}

impl TryFrom<String> for CanConversionSpec {
    type Error = &'static str;

    fn try_from(s: String) -> Result<Self, Self::Error> { s.parse() }
}

#[derive(serde::Deserialize, Debug)]
pub struct DatapointConversionGsSpec {
    pub conversion: ConversionGsSpec,
}

#[derive(serde::Deserialize, Debug)]
#[serde(try_from = "String")]
pub struct ConversionGsSpec {
    pub procedure_suffix: String,
    pub input: Ty,
}

impl FromStr for ConversionGsSpec {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(':').map_or(Err("missing colon"), |(suffix, ty_str)| {
            let input = ty_str.parse::<Ty>()?;
            Ok(Self { procedure_suffix: suffix.to_string(), input })
        })
    }
}

impl TryFrom<String> for ConversionGsSpec {
    type Error = &'static str;

    fn try_from(s: String) -> Result<Self, Self::Error> { s.parse() }
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct DatapointSpec {
    pub name: String,
    pub id: u16,

    pub store: Option<StoreInfo>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(try_from = "String")]
pub struct GetterSpec {
    pub ty: Ty,
    pub can_payload_range: std::ops::Range<usize>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct CommandSpec {
    pub name: String,
    pub id: u16,
    pub can: Option<CanCommandSpec>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct CanCommandSpec {
    #[serde(flatten)]
    pub can: CanSpec,
    pub conversion: Option<String>,
}

impl GetterSpec {
    fn get_from_can_frame(&self, data_slice: &str) -> String {
        let start = self.can_payload_range.start;

        fn size_2_bytes(data_slice: &str, start: usize) -> String {
            format!("[{data_slice}[{start}], {data_slice}[{}]]", start + 1)
        }

        fn size_4_bytes(data_slice: &str, start: usize) -> String {
            format!(
                "[{data_slice}[{start}], {data_slice}[{}], {data_slice}[{}], {data_slice}[{}]]",
                start + 1,
                start + 2,
                start + 3
            )
        }

        fn size_8_bytes(data_slice: &str, start: usize) -> String {
            format!("[{data_slice}[{start}], {data_slice}[{}], {data_slice}[{}], {data_slice}[{}], {data_slice}[{}], {data_slice}[{}], {data_slice}[{}], {data_slice}[{}]]", start + 1, start + 2, start + 3, start + 4, start + 5, start + 6, start + 7)
        }

        match self.ty {
            Ty::U8 => format!("{data_slice}[{start}]"),
            Ty::U16 => {
                format!("u16::from_be_bytes({})", size_2_bytes(data_slice, start))
            },
            Ty::U32 => format!("u32::from_be_bytes({})", size_4_bytes(data_slice, start)),
            Ty::U64 => format!("u64::from_be_bytes({})", size_8_bytes(data_slice, start)),
            Ty::I8 => todo!(),
            Ty::I16 => format!("i16::from_be_bytes({})", size_2_bytes(data_slice, start)),
            Ty::I32 => format!("i32::from_be_bytes({})", size_4_bytes(data_slice, start)),
            Ty::I64 => format!("i64::from_be_bytes({})", size_8_bytes(data_slice, start)),
            Ty::F16 => {
                format!("f16::from_be_bytes({})", size_2_bytes(data_slice, start))
            },
            Ty::F32 => format!("f32::from_be_bytes({})", size_4_bytes(data_slice, start)),
            Ty::F64 => format!("f64::from_be_bytes({})", size_8_bytes(data_slice, start)),
            Ty::U8Arr(n) => {
                let mut s = String::from("[");
                for i in 0..n {
                    s.push_str(&format!("{data_slice}[{start} + {i}], ", i = i));
                }
                s.push(']');
                s
            },
        }
    }
}

impl FromStr for GetterSpec {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((ty_str, range_and_closing_delim)) = s.split_once('[') else {
            return Err("missing opening bracket");
        };

        let ty = ty_str.parse::<Ty>()?;
        let Some(range) = range_and_closing_delim.strip_suffix(']') else {
            return Err("missing closing bracket");
        };

        let Some((start, end)) = range.split_once("..") else {
            return Err("missing range delimiter");
        };

        let start = start.parse().map_err(|_| "invalid range start")?;
        let end = end.parse().map_err(|_| "invalid range end")?;

        if end - start != ty.ty_size() {
            return Err("range size does not match type size");
        }

        Ok(Self { ty, can_payload_range: start..end })
    }
}

impl TryFrom<String> for GetterSpec {
    type Error = &'static str;

    fn try_from(s: String) -> Result<Self, Self::Error> { s.parse() }
}

fn make_procedures(df: &DataflowSpec) -> String {
    let mut code = String::new();

    for (name, spec) in &df.procedures {
        code.push_str(&format!(
            "fn {}(x: {}) -> {} {{\n",
            name.replace('-', "_"),
            spec.input,
            spec.output,
        ));
        code.push_str(&spec.formula);
        code.push_str("}\n\n");
    }

    code
}

fn make_datapoint_parser(spec: &MessageProcessingSpec) -> String {
    let mut code = String::new();
    for dpc in &spec.datapoint_conversion {
        let s = dpc.getter.get_from_can_frame("data");
        writeln!(&mut code, "let d = {};", s).unwrap();

        if dpc.getter.ty != dpc.can_conversion.input {
            panic!("getter type does not match can conversion input type");
        }

        writeln!(&mut code, "let c = {}(d);", dpc.can_conversion.proc_name).unwrap();

        if dpc.can_conversion.output != dpc.gs.conversion.input {
            panic!("can conversion output type does not match gs conversion input type");
        }

        writeln!(
            &mut code,
            "f(Datapoint::new(
                Datatype::{},
                dump_{}(c),
                embassy_time::Instant::now().as_ticks(),
            )).await;",
            dpc.datapoint.name, dpc.gs.conversion.procedure_suffix
        )
        .unwrap();
    }
    code
}

pub fn collect_data_types(df: &DataflowSpec) -> crate::datatypes::Config {
    let mut data_types = crate::datatypes::Config::default();
    for mp in &df.message_processing {
        for dpc in &mp.datapoint_conversion {
            data_types.Datatype.push(crate::datatypes::Datatype {
                id: dpc.datapoint.id,
                name: dpc.datapoint.name.clone(),
                lower: dpc.limits.as_ref().map(|l| l.lower).unwrap_or(Limit::No),
                upper: dpc.limits.as_ref().map(|l| l.upper).unwrap_or(Limit::No),
                display_units: dpc.display_units.clone(),
                priority: None,
                store: dpc.datapoint.store.clone(),
            });
        }
    }
    for sd in &df.standard_datapoints {
        data_types.Datatype.push(crate::datatypes::Datatype {
            id: sd.datapoint.id,
            name: sd.datapoint.name.clone(),
            lower: Limit::No,
            upper: Limit::No,
            display_units: None,
            priority: sd.priority,
            store: sd.datapoint.store.clone(),
        });
    }

    data_types
}

pub fn make_main_pcb_code(df: &DataflowSpec) -> String {
    let mut code = String::from(
        r#"
use crate::gs_master::Datapoint;
use core::future::Future;
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

    writeln!(&mut code, "pub fn event_for_can_1_id(id: u32) -> fsm::Event {{ match id {{").unwrap();

    for (id, event) in &can1_ids_to_events {
        writeln!(&mut code, "{} => fsm::Event::{},", id, event).unwrap();
    }

    writeln!(
        &mut code,
        "_ => fsm::Event::NoEvent,
        }}
    }}",
    )
    .unwrap();

    writeln!(&mut code, "pub fn event_for_can_2_id(id: u32) -> fsm::Event {{ match id {{").unwrap();

    for (id, event) in &can2_ids_to_events {
        writeln!(&mut code, "{} => fsm::Event::{},", id, event).unwrap();
    }

    writeln!(
        &mut code,
        "_ =>fsm::Event::NoEvent,
        }}
    }}",
    )
    .unwrap();

    writeln!(&mut code, "pub async fn parse_datapoints_can_1<F, Fut>(id: u32, data: &[u8], mut f: F) where F: FnMut(Datapoint) -> Fut, Fut: Future<Output=()> {{ {proc} match id {{").unwrap();
    for mp in &df.message_processing {
        if let CanSpec::Can1 { id, .. } = mp.can {
            writeln!(
                &mut code,
                "{} => {{
                    ",
                id
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
                "{} => {{
                    ",
                id
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

    let mut can1commands = vec![];
    let mut can2commands = vec![];

    for command in &df.commands {
        if let Some(CanCommandSpec { can: CanSpec::Can1 { id, .. }, conversion }) = &command.can {
            can1commands.push((&command.name, *id, conversion));
        } else if let Some(CanCommandSpec { can: CanSpec::Can2 { id, .. }, conversion }) =
            &command.can
        {
            can2commands.push((&command.name, *id, conversion));
        }
    }

    writeln!(&mut code, "pub async fn gs_to_can1<F, Fut>(command: Command, mut f: F) where F: FnMut(crate::can::can1::CanEnvelope) -> Fut, Fut: Future<Output=()> {{ {proc}\n\nmatch command {{").unwrap();
    for (command_name, id, conversion) in &can1commands {
        writeln!(
            &mut code,
            r#"Command::{command_name}(v) => {{
                let data = {conversion}(v);
                f(crate::can::can1::CanEnvelope::new_from_frame(embassy_stm32::can::frame::FdFrame::new_extended({id}, &data).expect("Invalid frame!"))).await;
            }}"#,
            conversion = conversion.as_deref().unwrap_or("default_command_process"),
        )
        .unwrap();
    }
    writeln!(&mut code, "_ => {{}}}}}}").unwrap();

    writeln!(&mut code, "pub async fn gs_to_can2<F, Fut>(command: Command, mut f: F) where F: FnMut(crate::can::can2::CanEnvelope) -> Fut, Fut: Future<Output=()> {{ {proc}\n\nmatch command {{").unwrap();
    for (command_name, id, conversion) in &can2commands {
        writeln!(
            &mut code,
            r#"Command::{command_name}(v) => {{
                let data = {conversion}(v);
                f(crate::can::can2::CanEnvelope::new_from_frame(embassy_stm32::can::frame::Frame::new_standard({id}, &data).expect("Invalid frame!"))).await;
            }}"#,
            conversion = conversion.as_deref().unwrap_or("default_command_process"),
        )
        .unwrap();
    }
    writeln!(&mut code, "_ => {{}}}}}}").unwrap();

    code
}

pub fn make_levi_beckhoff_code(df: &DataflowSpec) -> String {
r#"
VAR
	i: UINT;
	
	test_real: LREAL_AND_BYTES;
	test_Quint: QUINT_Reals;
	
	send_messages: BOOL := FALSE;
END_VAR

IF CAN_INPUTS.RxCounter <> CAN_OUTPUTS.RxCounter THEN
	FOR i:= 0 TO (CAN_INPUTS.NoOfRxMessages - 1) DO
		Incoming_messages[i] := CAN_INPUTS.RxMessages[i];
	END_FOR
	CAN_OUTPUTS.RxCounter := CAN_INPUTS.RxCounter;
	
END_IF

IF send_messages THEN

	//Messages_To_Send[0].length := 1;
	//Messages_To_Send[0].cobId := 450;
	//Messages_To_Send[0].txData[0] := 123;
	
	test_real.value := 123.0;
	//Messages_To_Send[1].length := 8;
	//Messages_To_Send[1].cobId := 460;
	//Messages_To_Send[1].txData := test_real.bytes;
	
	test_Quint.values[0] := 41241.25;
	test_Quint.values[1] := 0;
	Messages_To_Send[0].length := 8;
	Messages_To_Send[0].cobId := 420;
	Messages_To_Send[0].txData := test_Quint.bytes;
	
	No_Messages_Queued := 1;
END_IF

//Send new messages
IF (CAN_OUTPUTS.TxCounter = CAN_INPUTS.TxCounter) AND (No_Messages_Queued <> 0) THEN
	FOR i:= 0 TO (No_Messages_Queued - 1) DO
		CAN_OUTPUTS.TxMessages[i] := Messages_To_Send[i];
	END_FOR
	//Tell interface how many messages to send
	CAN_Outputs.NoOfTxMessages := No_Messages_Queued;
	CAN_OUTPUTS.TxCounter := CAN_INPUTS.TxCounter + 1;
	No_Messages_Queued := 0;
END_IF

"#;

    let mut ordinary_vars = String::new();
    let mut in_out_vars = String::from("VAR_IN_OUT\n");
    
    let mut code = String::new();

    writeln!(&mut ordinary_vars, r#"
VAR
        clock_time: TIME := ...;
        can_out_msgs: INT := 0;
        Incoming_messages: ARRAY[0..10] OF EXTCANTXQUEUE;
        Messages_To_Send: ARRAY[0..10] OF EXTCANTXQUEUE;
        No_Messages_Queued: UINT := 0;
        tx_data: ARRAY[0..7] OF BYTE;
        
        CAN_INPUTS AT %I*: CANRXQUEUESTRUCT_T_10;
        CAN_OUTPUTS AT %Q*: CANTXQUEUESTRUCT_X_10;
"#).unwrap();

    for mp in &df.message_processing {
        if let CanSpec::Can2 { id, comes_from_levi: Some(l) } = &mp.can {
            let mut tx_data_create = String::new();
            for dp in &mp.datapoint_conversion {
                let Some(levi_info) = &dp.comes_from_levi_info else {
                    panic!("no");
                };

                writeln!(&mut ordinary_vars, "        {}", levi_info.levi_type.make_input(&levi_info.name, &levi_info.input_spec)).unwrap();
                match dp.getter.ty {
                    Ty::U8 => {
                        writeln!(&mut tx_data_create, "    tx_data[{}] := {};", dp.getter.can_payload_range.start, levi_info.name).unwrap();
                    },
                    Ty::U16 => {
                        writeln!(&mut tx_data_create, "    tx_data[{}] := {}.bytes[0];", dp.getter.can_payload_range.start, levi_info.name).unwrap();
                        writeln!(&mut tx_data_create, "    tx_data[{}] := {}.bytes[1];", dp.getter.can_payload_range.start + 1, levi_info.name).unwrap();
                    },
                    Ty::U32 => {
                        writeln!(&mut tx_data_create, "    tx_data[{}] := {}.bytes[0];", dp.getter.can_payload_range.start, levi_info.name).unwrap();
                        writeln!(&mut tx_data_create, "    tx_data[{}] := {}.bytes[1];", dp.getter.can_payload_range.start + 1, levi_info.name).unwrap();
                        writeln!(&mut tx_data_create, "    tx_data[{}] := {}.bytes[2];", dp.getter.can_payload_range.start + 2, levi_info.name).unwrap();
                        writeln!(&mut tx_data_create, "    tx_data[{}] := {}.bytes[3];", dp.getter.can_payload_range.start + 3, levi_info.name).unwrap();
                    },
                    _ => panic!("not supported"),
                }
            }
            writeln!(&mut in_out_vars, "    can_{id}_last_log : TIME;").unwrap();
            writeln!(&mut code, "IF (clock_time - can_{id}_last_log > {} (ms)) THEN", l.log_period).unwrap();
            writeln!(&mut code, "    can_{id}_last_log := clock_time;").unwrap();
            writeln!(&mut code, "    Messages_To_Send[No_Messages_Queued].length := 8;").unwrap();
            writeln!(&mut code, "    Messages_To_Send[No_Messages_Queued].cobId := {id};").unwrap();
            writeln!(&mut code, "{}", tx_data_create).unwrap();
            writeln!(&mut code, "    Messages_To_Send[No_Messages_Queued].txData := tx_data;").unwrap();
            writeln!(&mut code, "    No_Messages_Queued := No_Messages_Queued + 1;").unwrap();
            writeln!(&mut code, "END IF;").unwrap();
        }
    }

    writeln!(&mut ordinary_vars, "END_VAR").unwrap();
    writeln!(&mut in_out_vars, "END_VAR").unwrap();

    format!("
{ordinary_vars}
{in_out_vars}

{code}
//Send new messages
IF (CAN_OUTPUTS.TxCounter = CAN_INPUTS.TxCounter) AND (No_Messages_Queued <> 0) THEN
	FOR i:= 0 TO (No_Messages_Queued - 1) DO
		CAN_OUTPUTS.TxMessages[i] := Messages_To_Send[i];
	END_FOR
	//Tell interface how many messages to send
	CAN_Outputs.NoOfTxMessages := No_Messages_Queued;
	CAN_OUTPUTS.TxCounter := CAN_INPUTS.TxCounter + 1;
	No_Messages_Queued := 0;
END_IF
    ")
}

pub fn make_logging_pcb_code(df: &DataflowSpec) -> String { format!("") }

pub fn make_gs_code(df: &DataflowSpec) -> String {
    let mut code = String::new();

    code.push_str(
        r#"
pub fn process_input_datatype(datatype: Datatype, data: u64) -> f64 {
"#,
    );
    code.push_str(&make_procedures(df));
    code.push_str("match datatype {");

    for dp in df.message_processing.iter().flat_map(|p| p.datapoint_conversion.iter()) {
        let x = format!("parse_{}", dp.gs.conversion.procedure_suffix);
        let dtn = &dp.datapoint.name;
        writeln!(&mut code, "Datatype::{dtn} => {x}(data),").unwrap();
    }
    writeln!(&mut code, "_ => data as f64,}}}}").unwrap();

    code
}

pub fn collect_commands(df: &DataflowSpec) -> commands::Config {
    let mut commands = commands::Config { Command: Vec::new() };
    for cmd in &df.commands {
        commands.Command.push(commands::Command { id: cmd.id, name: cmd.name.clone() });
    }
    commands
}

pub fn output_gs_frontend_code(df: &DataflowSpec) -> String {
    let mut code = String::new();
    write!(
        &mut code,
        r#"
/* AUTO GENERATED USING npm run generate:gs */
export type NamedCommand = "#
    )
    .unwrap();

    for (id, command) in df.commands.iter().enumerate() {
        if id != 0 {
            write!(&mut code, " | ").unwrap();
        }
        write!(&mut code, "\"{}\"", command.name).unwrap();
    }

    writeln!(
        &mut code,
        r#";
export const NamedCommandValues:NamedCommand[] = ["#
    )
    .unwrap();
    for (id, command) in df.commands.iter().enumerate() {
        if id != 0 {
            write!(&mut code, ", ").unwrap();
        }
        write!(&mut code, "\"{}\"", command.name).unwrap();
    }
    write!(
        &mut code,
        r#"];

export type NamedDatatype = "#
    )
    .unwrap();

    let dt = collect_data_types(df);

    dt.Datatype.iter().enumerate().for_each(|(id, datatype)| {
        if id != 0 {
            write!(&mut code, " | ").unwrap();
        }
        write!(&mut code, "\"{}\"", datatype.name).unwrap();
    });

    writeln!(
        &mut code,
        r#";

export const NamedDatatypeValues = ["#
    )
    .unwrap();

    dt.Datatype.iter().enumerate().for_each(|(id, datatype)| {
        if id != 0 {
            write!(&mut code, ", ").unwrap();
        }
        write!(&mut code, "\"{}\"", datatype.name).unwrap();
    });

    writeln!(
        &mut code,
        r#"];


        // gdd stores registration
        // auto-generated with npm run generate:gs
        "#
    )
    .unwrap();

    for d in &dt.Datatype {
        if let Some(store) = &d.store {
            write!(
                &mut code,
                r#"
            gdd.stores.registerStore<{type}>("{name}", {default}"#,
                type = store.ty,
                name = d.name,
                default = store.default,
            )
            .unwrap();
            if let Some(callback) = &store.callback {
                write!(&mut code, ", {callback}").unwrap();
            }

            writeln!(&mut code, ");").unwrap();
        }
    }

    code
}

pub fn parse_from(data: &str) -> DataflowSpec { serde_yaml::from_str(data).unwrap() }
