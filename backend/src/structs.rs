use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Suvi {
    pub _id: String,
    pub value: i32,
}
