use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize, Deserialize)]
pub struct EntryId {
    pub id: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Correctness {
    Correct,
    PartiallyCorrect,
    Incorrect
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldComparison {
    // field: &'static str,
    pub(crate) field: String,
    pub(crate) value: String,
    pub(crate) correct: Correctness,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuessResponse {
    pub(crate) fields: Vec<FieldComparison>,
}