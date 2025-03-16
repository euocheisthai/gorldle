use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::{Result, Value};
use std::vec::IntoIter;

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

impl<'a> IntoIterator for &'a DotaEntry {
    type Item = (&'static str, Value);
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            ("id", self.id.clone()),
            ("name", self.name.clone()),
            ("attribute", serde_json::to_value(&self.attribute).unwrap()),
            ("position", serde_json::to_value(&self.position).unwrap()),
            ("attack_type", serde_json::to_value(&self.attack_type).unwrap()),
            ("release_year", self.release_year.clone()),
        ].into_iter()
    }
}

impl DotaEntry {
    pub fn get(&self, key: &str) -> Option<Value> {
        self.clone()  
            .into_iter()
            .find(|(field_name, _)| *field_name == key)  
            .map(|(_, value)| value)
    }
}