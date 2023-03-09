use reqwest::Response;
use serde_json::json;
use tracing::instrument;

use crate::{endpoint::get_chatgpt_endpoint, env::CHATGPT_API_KEY, error::Error};

#[instrument(skip(client), ret, err)]
pub async fn post_get_channel(client: &reqwest::Client, text: &str) -> Result<Response, Error> {
    let resp = client
        .post(get_chatgpt_endpoint())
        .header(
            "Authorization",
            format!("Bearer {}", CHATGPT_API_KEY.unwrap()),
        )
        .json(&json!({
            "model": "gpt-3.5-turbo-0301",
            "messages": [{ "role": "user", "content": text }]
        }))
        .send()
        .await?;

    Ok(resp)
}
