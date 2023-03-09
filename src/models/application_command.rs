use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationCommand {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: u32,
    pub description: String,
    pub options: Option<Vec<ApplicationCommandOption>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationCommandOption {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: u32,
    pub description: String,
    pub required: Option<bool>,
    pub min_length: Option<u32>,
}
