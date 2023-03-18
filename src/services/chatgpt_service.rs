use reqwest::Response;
use tracing::instrument;

use crate::{
    endpoint::chatgpt_completions_endpoint, environment::CHATGPT_API_KEY, error::Error,
    models::chatgpt::chat_completion::ChatCompletionRequest,
};

/**
 * https://discord.com/developers/docs/resources/channel#get-channel
 */
#[instrument(skip(client), ret, err)]
pub async fn post_chat_completions(
    client: &reqwest::Client,
    request: &ChatCompletionRequest,
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
