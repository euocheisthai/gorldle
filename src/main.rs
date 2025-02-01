use std::env;
use std::fs;

use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::{Result, Value};

#[derive(Debug,Serialize, Deserialize)]
#[serde(rename_all = "lowercase")] 
enum DotaAttribute {
    Strength,
    Agility,
    Intelligence,
    Universal,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")] 
enum DotaPosition {
    Carry,
    Midlane,
    Offlane,
    SoftSupport,
    HardSupport
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")] 
enum DotaAttackType {
    Melee,
    Ranged,
    Both
}

#[derive(Debug, Serialize, Deserialize)]
struct DotaEntry {
    name: Value, // String,
    attribute: DotaAttribute,
    position: Vec<DotaPosition>,
    attack_type: DotaAttackType,
    release_year: Value, // u8
}

fn load_profile(profile_path: &str) -> Value {
    let profile: String = fs::read_to_string(profile_path)
        .expect("Did you move the config somewhere?");

    let profile_json: Value = serde_json::from_str(&profile)
        .expect("That's no JSON");

    return profile_json
}

fn main() {
    let current_profile: Value = load_profile("profile_1.json");
    let profile_id: &Value = &current_profile["profile_id"];

    if let Value::Array(items) = &current_profile["items"] {
        for item in items {
            match serde_json::from_value::<DotaEntry>(item.clone()) {
                Ok(dota_entry) => println!("{:?}", dota_entry),
                Err(e) => eprintln!("Failed to parse entry: {}", e),
            }
        }
    }
}
