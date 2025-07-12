#![allow(non_snake_case, non_camel_case_types)]
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Hash, Default)]
pub struct Config {
    pub(crate) Datatype: Vec<Datatype>,
    pub(crate) criticalDatapoints: Vec<String>,
}

#[derive(Deserialize, Hash)]
pub struct Datatype {
    pub name: String,
    pub id: u16,
    pub lower: Limit,
    pub upper: Limit,
    pub critical: bool,
    pub display_units: Option<String>,
    pub priority: Option<usize>,
    pub store: Option<StoreInfo>,
}

#[derive(Deserialize, Hash, Debug, Clone)]
pub struct StoreInfo {
    pub default: String,
    #[serde(rename = "type", default = "number_s")]
    pub ty: String,
    pub callback: Option<String>,
}

fn number_s() -> String { "number".into() }

#[derive(Debug, Hash, Clone, Copy)]
pub enum Limit {
    No,
    Single(u64),
    Multiple(Severities),
}

impl Display for Limit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match *self {
            Limit::No => write!(f, "Limit::No"),
            Limit::Single(x) => write!(f, "Limit::Single({x})"),
            Limit::Multiple(y) => {
                write!(
                    f,
                    "Limit::Multiple(Severities {{ warn: {}, err: {}, brake: {} }})",
                    y.warn.map(|x| format!("Some({x})")).unwrap_or("None".into()),
                    y.err.map(|x| format!("Some({x})")).unwrap_or("None".into()),
                    y.brake.map(|x| format!("Some({x})")).unwrap_or("None".into()),
                )
            },
        }
    }
}

#[derive(Deserialize, Hash, Clone, Copy, Debug)]
pub struct Severities {
    pub warn: Option<u64>,
    pub err: Option<u64>,
    pub brake: Option<u64>,
}

pub fn get_data_config(path: &str) -> Result<Config> {
    let config_str = fs::read_to_string(path)?;
    Ok(toml::from_str(&config_str)?)
}

pub fn get_data_items(path: &str) -> Result<Vec<(u16, String)>> {
    Ok(get_data_config(path)?.Datatype.iter().map(|x| (x.id, x.name.clone())).collect())
}

pub fn generate_data_types_from_config(config: &Config, drv: bool) -> Result<String> {
    let mut hasher = DefaultHasher::new();
    config.hash(&mut hasher);
    let hash = hasher.finish();

    let criticalCount = config.criticalDatapoints.len();
    let criticalDatapoints = &config.criticalDatapoints;
    let mut criticalDatapointResult = String::new();
    let mut enum_definitions = String::new();
    let mut match_to_id = String::new();
    let mut match_from_id = String::new();
    let mut data_ids = vec![];
    let mut from_str = String::new();
    let mut bounds = String::new();
    let mut units = String::from("    pub fn unit(&self) -> String {\n        match *self {\n");

    let mut priorities = String::new();

    for dtp in criticalDatapoints {
        criticalDatapointResult.push_str(&format!("\n\t\t\tDatatype::{dtp} => true,"));
    }

    for dtype in &config.Datatype {
        data_ids.push(dtype.id);
        enum_definitions.push_str(&format!("    {},\n", dtype.name));
        match_to_id.push_str(&format!("            Datatype::{} => {},\n", dtype.name, dtype.id));
        match_from_id.push_str(&format!("            {} => Datatype::{},\n", dtype.id, dtype.name));
        from_str.push_str(&format!("            {:?} => Datatype::{},\n", dtype.name, dtype.name));
        bounds.push_str(&format!(
            "            Datatype::{} => ({}, {}),\n",
            dtype.name, dtype.upper, dtype.lower,
        ));
        if let Some(u) = &dtype.display_units {
            units.push_str(&format!(
                "            Datatype::{} => String::from({:?}),\n",
                dtype.name, u
            ));
        }
        if let Some(p) = dtype.priority {
            priorities.push_str(&format!("            Datatype::{} => {},\n", dtype.name, p));
        }
    }

    units.push_str("            _ => String::new(),\n        }\n    }");

    Ok(format!(
        "\n
pub enum Limit {{
    No,
    Single(u64),
    Multiple(Severities)
}}

pub struct Severities {{
    pub warn: Option<u64>,
    pub err: Option<u64>,
    pub brake: Option<u64>,
}}

pub enum ValueCheckResult {{
    Fine,
    Warn,
    Error,
    BrakeNow,
}}

pub const CRITICAL_DATATYPE_COUNT: usize = {criticalCount};

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
{}
pub enum Datatype {{
{enum_definitions}
}}\n
impl Datatype {{
    pub fn to_id(&self)->u16 {{
        #[allow(unreachable_patterns)]
        match *self {{
{match_to_id}
        }}
    }}

    pub fn is_critical(&self) -> bool {{
        match self {{{criticalDatapointResult}
            _ => false,
        }}
    }}

    pub fn from_id(id:u16) -> Self {{
        #[allow(unreachable_patterns)]
        match id {{
{match_from_id}
            _ => Datatype::DefaultDatatype,
        }}
    }}

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {{
        match s {{
{from_str}
            _ => Datatype::DefaultDatatype,
        }}
    }}


    pub fn bounds(&self) -> (Limit, Limit) {{
        match *self {{
{bounds}
        }}
    }}

    pub fn check_bounds(&self, value: u64) -> ValueCheckResult {{
        let (up, low) = self.bounds();
        let ok_up = match up {{
            Limit::No => 0,
            Limit::Single(x) => if value > x {{ 1 }} else {{ 0 }},
            Limit::Multiple(Severities {{
                warn: a,
                err: b,
                brake: c,
            }}) => {{
                if let Some(cc) = c {{
                    if value > cc {{ 100 }} else {{ 0 }}
                }} else if let Some(bb) = b {{
                    if value > bb {{ 10 }} else {{ 0 }}
                }} else if let Some(aa) = a {{
                    if value > aa {{ 1 }} else {{ 0 }}
                }} else {{ 0 }}
            }}
        }};
        let ok_low = match low {{
            Limit::No => 0,
            Limit::Single(x) => if value < x {{ 1 }} else {{ 0 }},
            Limit::Multiple(Severities {{
                warn: a,
                err: b,
                brake: c,
            }}) => {{
                if let Some(cc) = c {{
                    if value < cc {{ 100 }} else {{ 0 }}
                }} else if let Some(bb) = b {{
                    if value < bb {{ 10 }} else {{ 0 }}
                }} else if let Some(aa) = a {{
                    if value < aa {{ 1 }} else {{ 0 }}
                }} else {{ 0 }}
            }}
        }};
        match ok_up + ok_low {{
            0 => ValueCheckResult::Fine,
            1..=9 => ValueCheckResult::Warn,
            10..=99 => ValueCheckResult::Error,
            _ => ValueCheckResult::BrakeNow,
        }}
    }}
{}

    pub fn priority(&self) -> usize {{
        match *self {{
{priorities}
            _ => 0,
        }}
    }}
}}
",
        if drv {
            "#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, PartialOrd, Ord)]"
        } else {
            "#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, defmt::Format)]"
        },
        if drv { units } else { "".into() },
    ) + &format!(
        "pub static DATA_IDS : [u16;{}] = [{}];\n",
        data_ids.len(),
        data_ids.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ")
    ) + "

impl Default for Datatype {
    fn default() -> Self {
        Self::DefaultDatatype
    }
}" + &format!("\npub const DATA_HASH: u64 = {hash};"))
}

pub fn generate_datatypes(path: &str, drv: bool) -> Result<String> {
    let config: Config = get_data_config(path)?;

    generate_data_types_from_config(&config, drv)
}
