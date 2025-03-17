use std::collections::HashMap;
use std::fmt::format;
use std::fmt::Display;
use std::fmt::Write;
use std::str::FromStr;

use crate::datatypes::Limit;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct DataflowSpec {
    pub procedures: HashMap<String, ProcedureSpec>,
    pub standard_datapoints: Vec<StandardDatapointSpec>,
    pub message_processing: Vec<MessageProcessingSpec>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct StandardDatapointSpec {
    pub datapoint: DatapointSpec,
    pub priority: Option<usize>,
}

#[derive(serde::Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
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
}

impl Ty {
    pub fn ty_size(self) -> usize {
        match self {
            Self::U8 | Self::I8 => 1,
            Self::U16 | Self::I16 | Self::F16 => 2,
            Self::U32 | Self::I32 | Self::F32 => 4,
            Self::U64 | Self::I64 | Self::F64 => 8,
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
    Can1 { id: u32 },
    Can2 { id: u32 },
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
}

#[derive(serde::Deserialize, Debug)]
#[serde(try_from = "String")]
pub struct GetterSpec {
    pub ty: Ty,
    pub can_payload_range: std::ops::Range<usize>,
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
                CanSpec::Can1 { id } => {
                    if let Some(old_event) = can1_ids_to_events.insert(*id, &fsm.event) {
                        panic!(
                            "duplicate event for CAN1 id: {} ({} and {})",
                            id, old_event, fsm.event
                        );
                    }
                },
                CanSpec::Can2 { id } => {
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

    writeln!(&mut code, "pub fn event_for_can_1_id(id: u32) -> fsm::Event {{ match id {{")
        .unwrap();

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

    writeln!(&mut code, "pub fn event_for_can_2_id(id: u32) -> fsm::Event {{ match id {{")
        .unwrap();

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
        if let CanSpec::Can1 { id } = mp.can {
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
        if let CanSpec::Can2 { id } = mp.can {
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

    code
}

pub fn make_logging_pcb_code(df: &DataflowSpec) -> String { format!("") }

pub fn make_gs_code(df: &DataflowSpec) -> String { format!("") }

pub fn parse_from(data: &str) -> DataflowSpec { serde_yaml::from_str(data).unwrap() }
