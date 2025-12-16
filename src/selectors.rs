use std::{collections::HashMap, fs, path::Path};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct SelectorEntry {
    selector: String,
    signature: String,
}

pub fn load_selectors<P: AsRef<Path>>(path: P) -> HashMap<u32, String> {
    let content = fs::read_to_string(path).unwrap();

    let entries: Vec<SelectorEntry> = serde_json::from_str(&content).unwrap();

    let mut map = HashMap::new();
    for entry in entries {
        let stripped_hex = entry.selector.trim_start_matches("0x");
        if let Ok(selector) = u32::from_str_radix(stripped_hex, 16) {
            map.insert(selector, entry.signature);
        }
    }

    map
}
