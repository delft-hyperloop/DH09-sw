//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.
//!
//! The build script also sets the linker flags to tell it which link script to
//! use.

#![allow(non_snake_case)]

use anyhow::anyhow;

extern crate serde;

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::Path;

use anyhow::Result;
use goose_utils::fsm_states::FSMState;
use goose_utils::hash_config;
use serde::Deserialize;
/*
   BUILD CONFIGURATION
   POD MAIN APPLICATION

*/

#[derive(Debug, Deserialize)]
struct Config {
    gs: GS,
    pod: Pod,
    FSMState: Vec<FSMState>,
}

#[derive(Debug, Deserialize)]
struct GS {
    ips: Vec<[u8; 4]>,
    port: u16,
    buffer_size: usize,
    timeout: u64,
    heartbeat: u64,
}

#[derive(Debug, Deserialize)]
struct Pod {
    net: NetConfig,
    internal: InternalConfig,
    comm: Comm,
    heartbeats: BTreeMap<String, u64>,
}

#[derive(Debug, Deserialize)]
struct Comm {
    bms_lv_ids: Vec<u16>,
    bms_hv_ids: Vec<u16>,
    gfd_ids: Vec<u16>,
}

#[derive(Debug, Deserialize)]
struct NetConfig {
    ip: [u8; 4],
    port: u16,
    dhcp: bool,
    // udp_port: u16,
    mac_addr: [u8; 6],
    keep_alive: u64,
}

#[derive(Debug, Deserialize)]
struct InternalConfig {
    event_queue_size: usize,
    data_queue_size: usize,
    can_queue_size: usize,
}

/// Path to config file
pub const CONFIG_PATH: &str = "../../config/config.toml";
/// Path to events file
pub const EVENTS_PATH: &str = "../../config/events.toml";
/// Path to dataflow file
pub const DATAFLOW_PATH: &str = "../../config/dataflow.yaml";

fn main() -> Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("config.rs");

    let ip_file = fs::read_to_string(CONFIG_PATH)?;
    let config: Config = toml::from_str(&ip_file)?;

    let mut content = String::from("//@generated\n");

    let df = std::fs::read_to_string(DATAFLOW_PATH)?;
    let df = goose_utils::dataflow::parse_from(&df);

    // content.push_str(&check_config(DATAFLOW_PATH, )?);
    content.push_str(&hash_config(CONFIG_PATH)?);

    content.push_str(&configure_ip(&config));
    content.push_str(&configure_gs_ips(
        &config.gs.ips,
        config.gs.port
    ));
    content.push_str(&configure_pod(&config));
    content.push_str(&configure_internal(&config));
    let commands = goose_utils::dataflow::collect_commands(&df);
    content.push_str(&goose_utils::commands::generate_commands_from_config(
        &commands, true,
    ));
    content.push_str(&generate_fsm_states(&config));
    content.push_str(&goose_utils::events::generate_events(EVENTS_PATH, true)?);
    content.push_str(&goose_utils::info::generate_info(CONFIG_PATH, false)?);
    let dt = goose_utils::dataflow::collect_data_types(&df);
    let dt = goose_utils::datatypes::generate_data_types_from_config(&dt, false)?;
    content.push_str(&dt);
    content.push_str(&configure_heartbeats(&config, &dt)?);

    content.push_str(&goose_utils::dataflow::make_main_pcb_code(&df));
    // content.push_str(&*can::main(&id_list));

    fs::write(dest_path.clone(), content).unwrap_or_else(|e| {
        panic!(
            "Couldn't write to {}! Build failed with error: {}",
            dest_path.to_str().unwrap(),
            e
        )
    });
    println!("cargo::rerun-if-changed={CONFIG_PATH}");
    println!("cargo::rerun-if-changed={EVENTS_PATH}");
    println!("cargo::rerun-if-changed={DATAFLOW_PATH}");

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo::rerun-if-changed=memory.x");
    println!("cargo::rerun-if-changed=build.rs");

    // linking
    // println!("cargo::rustc-link-arg-bins=--nmagic");
    // println!("cargo::rustc-link-arg-bins=-Tlink.x");
    // println!("cargo::rustc-link-arg-bins=-Tdefmt.x");
    println!("cargo::rerun-if-changed=../../util");

    Ok(())
}

fn configure_heartbeats(config: &Config, dt: &str) -> Result<String> {
    let mut x = format!(
        "\npub const HEARTBEATS_LEN: usize = {};\npub const HEARTBEATS: [(Datatype, u64); HEARTBEATS_LEN] = [",
        config.pod.heartbeats.len()
    );
    for (key, val) in &config.pod.heartbeats {
        if !dt.contains(key) {
            return Err(anyhow!(
                "\n\nFound heartbeat for non-existing datatype: {:?}\nYou can only add a timeout for datatypes present in /config/datatypes.toml (check your spelling)\n",
                key
            ));
        }
        x.push_str(&format!("(Datatype::{key}, {val}), "));
    }
    x.push_str("];\n");
    Ok(x)
}

fn configure_ip(config: &Config) -> String {
    format!(
        "pub const NETWORK_BUFFER_SIZE: usize = {};\n",
        config.gs.buffer_size
    ) + &*format!("pub const IP_TIMEOUT: u64 = {};\n", config.gs.timeout)
}

fn configure_pod(config: &Config) -> String {
    format!(
        "pub const POD_IP_ADDRESS: ([u8;4],u16) = ([{},{},{},{}],{});\n",
        config.pod.net.ip[0],
        config.pod.net.ip[1],
        config.pod.net.ip[2],
        config.pod.net.ip[3],
        config.pod.net.port
    )
        + &format!("\npub const USE_DHCP: bool = {};\n", config.pod.net.dhcp)
        //     + &*format!(
        //     "pub static POD_UDP_IP_ADDRESS: ([u8;4],u16) = ([{},{},{},{}],{});\n",
        //     config.pod.net.ip[0],
        //     config.pod.net.ip[1],
        //     config.pod.net.ip[2],
        //     config.pod.net.ip[3],
        //     config.pod.net.udp_port
        // ) +
        + &format!(
        "pub const POD_MAC_ADDRESS: [u8;6] = [{},{},{},{},{},{}];\n",
        config.pod.net.mac_addr[0],
        config.pod.net.mac_addr[1],
        config.pod.net.mac_addr[2],
        config.pod.net.mac_addr[3],
        config.pod.net.mac_addr[4],
        config.pod.net.mac_addr[5]
    ) + &format!("pub const KEEP_ALIVE: u64 = {};\n", config.pod.net.keep_alive)
        + &format!("pub const HEARTBEAT: u64 = {};\n", config.gs.heartbeat)
}

fn configure_internal(config: &Config) -> String {
    format!(
        "pub const EVENT_QUEUE_SIZE: usize = {};\n",
        config.pod.internal.event_queue_size
    ) + &*format!(
        "pub const DATA_QUEUE_SIZE: usize = {};\n",
        config.pod.internal.data_queue_size
    ) + &*format!(
        "pub const CAN_QUEUE_SIZE: usize = {};\n",
        config.pod.internal.can_queue_size
    ) + &*format!(
        "pub const LV_IDS: [u16;{}] = [{}];\n",
        config.pod.comm.bms_lv_ids.len(),
        config
            .pod
            .comm
            .bms_lv_ids
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    ) + &*format!(
        "pub const HV_IDS: [u16;{}] = [{}];\n",
        config.pod.comm.bms_hv_ids.len(),
        config
            .pod
            .comm
            .bms_hv_ids
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    ) + &*format!(
        "pub const GFD_IDS: [u16;{}] = [{}];\n",
        config.pod.comm.gfd_ids.len(),
        config
            .pod
            .comm
            .gfd_ids
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    ) + &*format!(
        "pub const BATTERY_GFD_IDS: [u16;{}] = [{},{},{}];\n",
        config.pod.comm.bms_lv_ids.len()
            + config.pod.comm.bms_hv_ids.len()
            + config.pod.comm.gfd_ids.len(),
        config
            .pod
            .comm
            .bms_lv_ids
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", "),
        config
            .pod
            .comm
            .bms_hv_ids
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", "),
        config
            .pod
            .comm
            .gfd_ids
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    )
}

fn generate_fsm_states(config: &Config) -> String {
    format!(
        "\n\n/// Enum representing the different states that the `MainFSM` will be in
#[derive(Eq, PartialEq, Debug, Clone, Copy, defmt::Format)]
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
