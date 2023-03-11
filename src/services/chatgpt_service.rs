use reqwest::Response;
use serde::Serialize;
use serde_json::json;
use tracing::instrument;

use crate::{endpoint::chatgpt_completions_endpoint, environment::CHATGPT_API_KEY, error::Error};

/**
 * https://discord.com/developers/docs/resources/channel#get-channel
 */
#[instrument(skip(client, request), ret, err)]
pub async fn post_chat_completions<T: Serialize + ?Sized>(
    client: &reqwest::Client,
    request: &T,
) -> Result<Response, Error> {
    let resp = client
        .post(chatgpt_completions_endpoint())
        .header(
            "Authorization",
            format!("Bearer {}", CHATGPT_API_KEY.unwrap()),
        )
        .json(request)
        .send()
        .await?;

    Ok(resp)
}
