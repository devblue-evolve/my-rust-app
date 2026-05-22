use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LlmInfo {
    pub id: i32,
    pub model_name: String,
    pub version: String,
    pub provider: String,
}