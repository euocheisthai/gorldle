use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize, Deserialize)]
pub struct EntryId {
    pub id: i16,
}
