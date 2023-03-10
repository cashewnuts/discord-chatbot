use reqwest::Response;
use serde_json::json;
use tracing::instrument;

use crate::{endpoint::chatgpt_completions_endpoint, env::CHATGPT_API_KEY, error::Error};

/**
 * https://discord.com/developers/docs/resources/channel#get-channel
 */
#[instrument(skip(client), ret, err)]
pub async fn post_chat_completions(
    client: &reqwest::Client,
    system_text: &str,
    text: &str,
) -> Result<Response, Error> {
    let resp = client
        .post(chatgpt_completions_endpoint())
        .header(
            "Authorization",
            format!("Bearer {}", CHATGPT_API_KEY.unwrap()),
        )
        .json(&json!({
            "model": "gpt-3.5-turbo-0301",
            "messages": [
                { "role": "system", "content": system_text },
                { "role": "user", "content": text }
            ]
        }))
        .send()
        .await?;

    Ok(resp)
}
