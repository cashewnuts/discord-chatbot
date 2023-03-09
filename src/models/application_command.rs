use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationCommand {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: u32,
    pub description: String,
}
