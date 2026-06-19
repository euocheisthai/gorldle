use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::vec::IntoIter;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DotaAttribute {
    Strength,
    Agility,
    Intelligence,
    Universal,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DotaPosition {
    Carry,
    Midlane,
    Offlane,
    SoftSupport,
    HardSupport,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DotaAttackType {
    Melee,
    Ranged,
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
            ("name", self.name.clone()),
            ("attribute", serde_json::to_value(&self.attribute).unwrap()),
            ("position", serde_json::to_value(&self.position).unwrap()),
            (
                "attack_type",
                serde_json::to_value(&self.attack_type).unwrap(),
            ),
            ("release_year", self.release_year.clone()),
        ]
        .into_iter()
    }
}

impl DotaEntry {
    pub fn get(&self, key: &str) -> Option<Value> {
        self.into_iter()
            .find(|(field_name, _)| *field_name == key)
            .map(|(_, value)| value)
    }
}
