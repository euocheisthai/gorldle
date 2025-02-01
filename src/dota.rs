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
pub struct DotaEntry {
    name: Value,
    attribute: DotaAttribute,
    position: Vec<DotaPosition>,
    attack_type: DotaAttackType,
    release_year: Value,
}
