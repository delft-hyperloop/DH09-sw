#![allow(non_snake_case)]

extern crate serde;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use goose_utils::commands::generate_commands_from_config;
use goose_utils::datatypes::generate_data_types_from_config;
use goose_utils::events::generate_events;
use goose_utils::fmt::run_fmt;
use goose_utils::fsm_states::FSMState;
use goose_utils::hash_config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    gs: GS,
    pod: Pod,
    FSMState: Vec<FSMState>,
}

#[derive(Debug, Deserialize)]
struct Pod {
    net: NetConfig,
}
#[derive(Debug, Deserialize)]
struct NetConfig {
    ip: [u8; 4],
    port: u16,
}

#[derive(Debug, Deserialize)]
struct GS {
    ips: Vec<[u8; 4]>,
    port: u16,
    buffer_size: usize,
    timeout: u64,
    heartbeat: u64,
    status_channel: String,
    warning_channel: String,
    info_channel: String,
    error_channel: String,
    shortcut_channel: String,
}

pub const CONFIG_PATH: &str = "../../config/config.toml";
pub const EVENTS_PATH: &str = "../../config/events.toml";
pub const DATAFLOW_PATH: &str = "../../config/dataflow.yaml";

fn main() -> Result<()> {
    tauri_build::build();
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("config.rs");
    let gs_file = fs::read_to_string(CONFIG_PATH)?;

    let config: Config = toml::from_str(&gs_file)?;

    let mut content = String::from("//@generated\n");

    // content.push_str(&check_config(EVENTS_PATH, CONFIG_PATH)?);
    content.push_str(&hash_config(CONFIG_PATH)?);

    content.push_str(&configure_gs(&config));
    content.push_str(&configure_gs_ips(&config.gs.ips, config.gs.port));
    let df = fs::read_to_string(DATAFLOW_PATH)?;
    let df = goose_utils::dataflow::parse_from(&df);
    let dt = goose_utils::dataflow::collect_data_types(&df);
    let dt = generate_data_types_from_config(&dt, true)?;
    content.push_str(&dt);
    let commands = goose_utils::dataflow::collect_commands(&df);
    content.push_str(&generate_commands_from_config(&commands, false));
    content.push_str(&generate_events(EVENTS_PATH, false)?);
    content.push_str(&generate_fsm_states(&config));
    content.push_str(&configure_channels(&config));
    content.push_str(&goose_utils::info::generate_info(CONFIG_PATH, true)?);
    content.push_str(&goose_utils::dataflow::make_gs_code(&df));

    fs::write(dest_path.clone(), content).unwrap_or_else(|_| {
        panic!("Couldn't write to {}! Build failed.", dest_path.to_str().unwrap());
    });

    // format the generated file so it's readable
    run_fmt(
        &dest_path,
        // rustfmt.toml is at the workspace root,
        // exactly 2 directories up from station/build.rs
        &PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap().to_string())
            .ancestors()
            .nth(2)
            .unwrap()
            .join("rustfmt.toml"),
    )?;

    println!("cargo::rerun-if-changed={CONFIG_PATH}");
    println!("cargo::rerun-if-changed={EVENTS_PATH}");
    println!("cargo::rerun-if-changed={DATAFLOW_PATH}");
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=../../util");

    Ok(())
}

fn configure_gs(config: &Config) -> String {
    // format!("pub fn gs_socket() -> std::net::SocketAddr {{ std::net::SocketAddr::new(std::net::IpAddr::from([{},{},{},{}]),{}) }}\n", config.gs.ip[0], config.gs.ip[1], config.gs.ip[2], config.gs.ip[3], config.gs.port)
    format!(
        "pub static POD_IP_ADDRESS: ([u8;4],u16) = ([{},{},{},{}],{});\n",
        config.pod.net.ip[0],
        config.pod.net.ip[1],
        config.pod.net.ip[2],
        config.pod.net.ip[3],
        config.pod.net.port
    ) + &*format!("pub const NETWORK_BUFFER_SIZE: usize = {};\n", config.gs.buffer_size)
        + &format!("pub const IP_TIMEOUT: u64 = {};\n", config.gs.timeout)
        + &format!("pub const HEARTBEAT: u64 = {};\n", config.gs.heartbeat)
}

fn configure_channels(config: &Config) -> String {
    format!("\npub const STATUS_CHANNEL: &str = \"{}\";\n", config.gs.status_channel)
        + &*format!("pub const WARNING_CHANNEL: &str = \"{}\";\n", config.gs.warning_channel)
        + &*format!("pub const INFO_CHANNEL: &str = \"{}\";\n", config.gs.info_channel)
        + &*format!("pub const ERROR_CHANNEL: &str = \"{}\";\n", config.gs.error_channel)
        + &*format!("pub const SHORTCUT_CHANNEL: &str = \"{}\";\n", config.gs.shortcut_channel)
}

fn generate_fsm_states(config: &Config) -> String {
    format!(
        "\n\n/// Enum representing the different states that the `MainFSM` will be in
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
#[cfg_attr(target_os = \"none\", derive(defmt::Format))]
#[allow(dead_code)]
pub enum States {{
{}
}}

impl States {{
    pub fn from_index(index: u8) -> States {{
        match index {{
{},
            _ => States::UnknownState,
        }}
    }}

    pub fn to_index(&self) -> u8 {{
        match self {{
{}
        }}
    }}
}}",
        config
            .FSMState
            .iter()
            .map(|x| format!("\t/// {}\n\t{}", x.doc, x.state))
            .collect::<Vec<String>>()
            .join(",\n"),
        config
            .FSMState
            .iter()
            .filter(|x| x.state != "UnknownState")
            .map(|x| format!("\t\t\t{} => States::{}", x.index, x.state))
            .collect::<Vec<String>>()
            .join(",\n"),
        config
            .FSMState
            .iter()
            .map(|x| format!("\t\t\tStates::{} => {}", x.state, x.index))
            .collect::<Vec<String>>()
            .join(",\n")
    )
}

/// Generates the IPv4 addresses from the provided list of (IP, port) tuples
fn configure_gs_ips(ips: &Vec<[u8; 4]>, port: u16) -> String {
    let mut result: String = String::from("");

    for ip in ips {
        result.push_str(&format!("\t([{}, {}, {}, {}], {}),\n", ip[0], ip[1], ip[2], ip[3], port));
    }

    format!(
        "\npub const IP_ADDRESS_COUNT: usize = {};\npub const GS_IP_ADDRESSES: [([u8;4], u16); {}] = [\n\
        {}\n];\n\n",
        ips.len(), ips.len(), result
    )
}
