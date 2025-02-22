use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::{Result, Value};

#[derive(Debug,Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")] 
enum DotaAttribute {
    Strength,
    Agility,
    Intelligence,
    Universal,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")] 
enum DotaPosition {
    Carry,
    Midlane,
    Offlane,
    SoftSupport,
    HardSupport
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")] 
enum DotaAttackType {
    Melee,
    Ranged,
    Both
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DotaEntry {
    pub id: Value,
    pub name: Value,
    pub attribute: DotaAttribute,
    pub position: Vec<DotaPosition>,
    pub attack_type: DotaAttackType,
    pub release_year: Value,
}

