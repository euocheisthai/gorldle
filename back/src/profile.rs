use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryId {
    pub id: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Correctness {
    Correct,
    PartiallyCorrect,
    Incorrect,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldComparison {
    pub field: String,
    pub value: serde_json::Value,
    pub correct: Correctness,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuessResponse {
    pub name: String,
    pub fields: Vec<FieldComparison>,
}
