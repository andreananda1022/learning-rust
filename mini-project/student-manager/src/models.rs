use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    pub id: u32,
    pub name: String,
    pub age: u8,
    pub score: f32,
}
