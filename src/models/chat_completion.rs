use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionChoice {
    pub index: u32,
    pub finish_reason: String,
    pub message: ChatCompletionMessage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub usage: ChatCompletionUsage,
    pub choices: Vec<ChatCompletionChoice>,
}

impl ChatCompletionResponse {
    pub fn get_total_token_usage(&self) -> u32 {
        self.usage.total_tokens
    }
}
