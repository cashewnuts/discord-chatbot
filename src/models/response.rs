use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct InteractionMessage {
    pub tts: Option<bool>,
    pub content: Option<String>,
}

impl InteractionMessage {
    pub fn new<S: Into<String>>(content: S) -> Self {
        Self {
            tts: None,
            content: Some(content.into()),
        }
    }
}

/**
 * https://discord.com/developers/docs/interactions/receiving-and-responding#responding-to-an-interaction
 */
#[derive(Serialize, Deserialize)]
pub struct InteractionResponse<T> {
    #[serde(rename = "type")]
    type_: u32,
    data: Option<T>,
}

impl<T> InteractionResponse<T> {
    pub fn new(type_: u32, data: Option<T>) -> Self {
        Self { type_, data }
    }
}
