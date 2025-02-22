use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize, Deserialize)]
pub struct EntryId {
    pub id: u8,
}

#[derive(Serialize, Deserialize)]
pub enum Correctness {
    Correct,
    PartiallyCorrect,
    Incorrect
}

#[derive(Serialize, Deserialize)]
pub struct FieldComparison {
    // field: &'static str,
    pub(crate) field: String,
    pub(crate) value: String,
    pub(crate) correct: Correctness,
}

#[derive(Serialize)]
pub struct GuessResponse {
    fields: Vec<FieldComparison>,
}