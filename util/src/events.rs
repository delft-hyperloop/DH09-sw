#![allow(non_snake_case)]
use std::fs;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Hash)]
pub struct Config {
    Event: Vec<Event>,
}

#[derive(Deserialize, Hash)]
pub struct Event {
    name: String,
    doc: String,
    params: Option<String>,
}

pub fn get_events_config(path: &str) -> Result<Config> {
    let config_str = fs::read_to_string(path)?;
    Ok(toml::from_str(&config_str)?)
}

pub fn generate_events(path: &str, drv: bool) -> Result<String> {
    let config: Config = get_events_config(path)?;

    let mut hasher = DefaultHasher::new();
    config.hash(&mut hasher);
    let hash = hasher.finish();

    let mut enum_definitions = String::new();
    let mut to_str = String::new();

    let event_count = config.Event.len();
    for event in config.Event.iter() {
        match &event.params {
            None => {
                enum_definitions.push_str(&format!("    {},\n", event.name));
            },
            Some(x) => {
                enum_definitions.push_str(&format!("    {}({}),\n", event.name, x));
            },
        }
        to_str.push_str(&format!("\"{}\",", event.name));
    }

    Ok(format!(
        "\n\npub const EVENTS_DISPLAY: [&str; {}] = [{}\"Unknown\"];\n",
        event_count + 1,
        to_str
    ) + &*format!(
        "
{}
pub enum Event {{
{}
}}
",
        if drv {
            "#[derive(Debug, PartialEq, Eq, defmt::Format)]"
        } else {
            "#[derive(Debug, PartialEq, Eq)]"
        },
        enum_definitions,
    ) + &format!("\npub const EVENTS_HASH: u64 = {hash};"))
}
