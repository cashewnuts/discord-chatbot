use serde::{Deserialize, Serialize};

/**
 * https://discord.com/developers/docs/interactions/application-commands#application-command-object
 */

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationCommand {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: u32,
    pub description: Option<String>,
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
    pub max_value: Option<u32>,
}
