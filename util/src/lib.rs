#![feature(let_chains)]

use std::cmp::min;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

pub mod commands;
pub mod dataflow;
pub mod datatypes;
pub mod events;
pub mod fsm_states;
pub mod info;
pub mod ip;
pub mod limits;

// mod shared;
use anyhow::Result;

pub fn check_config(ep: &str, conf: &str) -> Result<String> {
    let mut items = vec![];
    let e = events::get_event_items(ep)?;
    items.extend(&e);
    let mut seen = HashSet::new();
    let mut seen_names = HashSet::new();
    for (id, name) in &items {
        if !seen.insert(*id) {
            panic!(
                "\nDuplicate id found: {} ({}). Closest available id is: {}\n",
                format_args!("{:#05x}", id),
                format_args!("{:#05x}", id),
                format_args!(
                    "{:#05x}",
                    nearest_id(*id, &items.iter().map(|x| x.0).collect::<Vec<u16>>())
                ),
            );
        } else if *id > 2u16.pow(11) {
            panic!(
                "\nId out of range: {}. Allowed ids are 0x000-0x7ff. Closest available id is: {}\n",
                format_args!("{:#05x}", id),
                format_args!(
                    "{:#05x}",
                    nearest_id(*id, &items.iter().map(|x| x.0).collect::<Vec<u16>>())
                ),
            );
        }
        if !seen_names.insert(name) {
            let other = items.iter().find(|x| x.1.eq(name)).unwrap_or_else(|| {
                unreachable!(
                    "The hash set should only contain entries that exist in the items vector"
                )
            });
            panic!(
                "\nDuplicate entry found:\n1: {} {}->{} \n2: {} {}->{}\n",
                category(&e.iter().map(|x| x.0).collect::<Vec<u16>>(), *id),
                name,
                format_args!("{:#05x}", id),
                category(&e.iter().map(|x| x.0).collect::<Vec<u16>>(), other.0),
                other.1,
                format_args!("{:#05x}", other.0),
            );
        }
    }

    let cs = read_to_string(conf)?;
    let cs = cs.lines().collect::<Vec<_>>();
    let mut hasher = DefaultHasher::new();
    cs.hash(&mut hasher);
    let hash = hasher.finish();
    Ok(format!("\npub const CONFIG_HASH: u64 = {};\n", hash))
}

fn nearest_id(id: u16, ids: &[u16]) -> u16 {
    for i in min(id, 2u16.pow(11))..2u16.pow(11) {
        if !ids.contains(&i) {
            return i;
        }
    }
    for j in (0u16..2u16.pow(11)).rev() {
        if !ids.contains(&j) {
            return j;
        }
    }
    panic!("There are no more available ids!")
}

fn category(e: &[u16], i: u16) -> String {
    if e.contains(&i) {
        "[[Event]]".into()
    } else {
        unreachable!("This ID had to come from somewhere...")
    }
}
