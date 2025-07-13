use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Write;
use std::str::FromStr;

use crate::commands;
use crate::datatypes::Limit;
use crate::datatypes::StoreInfo;

pub mod frontend;
pub mod gs;
pub mod levi;
pub mod mainpcb;
pub mod procedures;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct DataflowSpec {
    pub procedures: HashMap<String, ProcedureSpec>,
    pub standard_datapoints: Vec<StandardDatapointSpec>,
    pub message_processing: Vec<MessageProcessingSpec>,
    pub commands: Vec<CommandSpec>,
    pub beckhoff: BeckhoffTaskSpec,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct BeckhoffTaskSpec {
    pub task_period: u32,
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
    U8LE,
    U16,
    U16LE,
    U32,
    U32LE,
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
            Self::U8 | Self::U8LE | Self::I8 => 1,
            Self::U16 | Self::U16LE | Self::I16 | Self::F16 => 2,
            Self::U32 | Self::U32LE | Self::I32 | Self::F32 => 4,
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
            "u8le" => Ok(Self::U8LE),
            "u16" => Ok(Self::U16),
            "u16le" => Ok(Self::U16LE),
            "u32" => Ok(Self::U32),
            "u32le" => Ok(Self::U32LE),
            "u64" => Ok(Self::U64),
            "i8" => Ok(Self::I8),
            "i16" => Ok(Self::I16),
            "i32" => Ok(Self::I32),
            "i64" => Ok(Self::I64),
            "f16" => Ok(Self::F16),
            "f32" => Ok(Self::F32),
            "f64" => Ok(Self::F64),
            s if s.starts_with("[u8;") && s.ends_with(']') => {
                let n = s[4..s.len() - 1].trim().parse().map_err(|_| "invalid array size").unwrap();
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
            Self::U8LE => write!(f, "u8le"),
            Self::U16 => write!(f, "u16"),
            Self::U16LE => write!(f, "u16le"),
            Self::U32 => write!(f, "u32"),
            Self::U32LE => write!(f, "u32le"),
            Self::U64 => write!(f, "u64"),
            Self::I8 => write!(f, "i8"),
            Self::I16 => write!(f, "i16"),
            Self::I32 => write!(f, "i32"),
            Self::I64 => write!(f, "i64"),
            Self::F16 => write!(f, "f16"),
            Self::F32 => write!(f, "f32"),
            Self::F64 => write!(f, "f64"),
            Self::U8Arr(n) => write!(f, "[u8; {n}]"),
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
    pub severity: Option<String>,
    #[serde(rename = "beckhoff")]
    pub comes_from_levi_info: Option<DatapointComesFromLeviInfo>,
}

#[derive(serde::Deserialize, Debug)]
pub struct DatapointComesFromLeviInfo {
    pub name: String,
    pub levi_type: StructuredTy,
    pub formula: String,
}

#[derive(serde::Deserialize, Debug)]
pub enum StructuredTy {
    Byte,
    Integer,
    Real,
    LReal,
}

impl StructuredTy {
    fn make_input(&self, name: &str) -> String {
        match self {
            Self::Byte => format!("{name}: BYTE;"),
            Self::Integer => format!("{name}: INT;"),
            Self::Real => format!("{name}: REAL;"),
            Self::LReal => format!("{name}: LREAL;"),
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((proc_name, proc_ty)) = s.split_once(':') else {
            return Err(format!("missing colon in {s:?}"));
        };

        let Some((input_ty, output_ty)) = proc_ty.split_once("->") else {
            return Err(format!("missing conversion arrow in {s:?}"));
        };

        let input = input_ty.parse::<Ty>().unwrap();
        let output = output_ty.parse::<Ty>().unwrap();

        Ok(Self { proc_name: proc_name.to_string(), input, output })
    }
}

impl TryFrom<String> for CanConversionSpec {
    type Error = String;

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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(':').map_or(Err(format!("missing colon in {s:?}")), |(suffix, ty_str)| {
            let input = ty_str.parse::<Ty>().unwrap();
            Ok(Self { procedure_suffix: suffix.to_string(), input })
        })
    }
}

impl TryFrom<String> for ConversionGsSpec {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> { s.parse() }
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct DatapointSpec {
    pub name: String,
    pub id: u16,
    pub critical: Option<bool>,
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
    #[serde(default = "zero_trim")]
    pub trim: Trim,
}

fn zero_trim() -> Trim { Trim(0) }

#[derive(serde::Deserialize, Debug)]
#[serde(try_from = "usize")]
pub struct Trim(usize);

impl TryFrom<usize> for Trim {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value > 8 {
            Err(format!("trim value must be between 0 and 8, found {value}."))
        } else {
            Ok(Trim(value))
        }
    }
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
            Ty::U8 | Ty::U8LE => format!("{data_slice}[{start}]"),
            Ty::U16 => {
                format!("u16::from_be_bytes({})", size_2_bytes(data_slice, start))
            },
            Ty::U16LE => {
                format!("u16::from_le_bytes({})", size_2_bytes(data_slice, start))
            },
            Ty::U32 => format!("u32::from_be_bytes({})", size_4_bytes(data_slice, start)),
            Ty::U32LE => format!("u32::from_le_bytes({})", size_4_bytes(data_slice, start)),
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
                    s.push_str(&format!("{data_slice}[{start} + {i}], "));
                }
                s.push(']');
                s
            },
        }
    }
}

impl FromStr for GetterSpec {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((ty_str, range_and_closing_delim)) = s.split_once('[') else {
            return Err(format!("missing opening bracket `[` in {s:?}"));
        };

        let ty = ty_str.parse::<Ty>().unwrap();
        let Some(range) = range_and_closing_delim.strip_suffix(']') else {
            return Err(format!("missing closing bracket `]` in {s:?}"));
        };

        let Some((start, end)) = range.split_once("..") else {
            return Err(format!("missing range delimiter `..` in {s:?}"));
        };

        let start =
            start.parse().map_err(|e| format!("invalid range start ({e}) in {s:?}")).unwrap();
        let end = end.parse().map_err(|e| format!("invalid range end ({e}) in {s:?}")).unwrap();

        if end - start != ty.ty_size() {
            return Err(format!(
                "range size does not match type size: expected {} but found {} in {s:?}",
                end - start,
                ty.ty_size()
            ));
        }

        Ok(Self { ty, can_payload_range: start..end })
    }
}

impl TryFrom<String> for GetterSpec {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> { s.parse() }
}

fn make_datapoint_parser(spec: &MessageProcessingSpec) -> String {
    let mut code = String::new();
    for dpc in &spec.datapoint_conversion {
        let s = dpc.getter.get_from_can_frame("data");
        writeln!(&mut code, "let d = {s};").unwrap();

        if dpc.getter.ty != dpc.can_conversion.input {
            panic!(
                "getter type does not match can conversion input type ({} ≠ {}) for datapoint {}",
                dpc.getter.ty, dpc.can_conversion.input, dpc.datapoint.name
            );
        }

        writeln!(&mut code, "let c = {}(d);", dpc.can_conversion.proc_name).unwrap();

        if dpc.can_conversion.output != dpc.gs.conversion.input {
            panic!(
                "can conversion output type does not match gs conversion input type\
                    ({} ≠ {}) for datapoint {}",
                dpc.can_conversion.output, dpc.gs.conversion.input, dpc.datapoint.name
            );
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
            let mut critical = false;
            if dpc.datapoint.critical.is_some() {
                critical = dpc.datapoint.critical.unwrap();
                if critical {
                    data_types.criticalDatapoints.push(dpc.datapoint.name.clone())
                }
            }
            data_types.Datatype.push(crate::datatypes::Datatype {
                id: dpc.datapoint.id,
                name: dpc.datapoint.name.clone(),
                lower: dpc.limits.as_ref().map(|l| l.lower).unwrap_or(Limit::No),
                upper: dpc.limits.as_ref().map(|l| l.upper).unwrap_or(Limit::No),
                critical,
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
            critical: false,
            display_units: None,
            priority: sd.priority,
            store: sd.datapoint.store.clone(),
        });
    }

    data_types
}

pub fn collect_commands(df: &DataflowSpec) -> commands::Config {
    let mut commands = commands::Config { Command: Vec::new() };
    for cmd in &df.commands {
        commands.Command.push(commands::Command { id: cmd.id, name: cmd.name.clone() });
    }
    commands
}

pub fn parse_from(data: &str) -> DataflowSpec { serde_yaml::from_str(data).unwrap() }
