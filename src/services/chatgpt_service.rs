use std::str::from_utf8;

use reqwest::Response;
use tracing::{error, instrument};

use crate::{
    endpoint::chatgpt_completions_endpoint,
    environment::CHATGPT_API_KEY,
    error::Error,
    models::chatgpt::chat_completion::{ChatCompletionChunkResponse, ChatCompletionRequest},
};

use futures_util::{Stream, StreamExt};

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

#[instrument(skip(response))]
pub fn response_extract_stream(
    response: Response,
    count: usize,
) -> impl Stream<Item = Result<String, Error>> {
    let mut bytes_stream = response.bytes_stream();
    async_stream::try_stream! {
        let mut stream_buffer: Vec<ChatCompletionChunkResponse> = vec![];
        let yield_buffer = |stream_buffer: Vec<ChatCompletionChunkResponse>| {
                let mut yield_string = String::new();
                for chunk in stream_buffer.iter() {
                    if let Some(choice) = chunk.choices.first() {
                        if let Some(content) = choice.delta.clone().content {
                            yield_string.push_str(&content.to_string());
                        }
                    }
                }
                yield_string
        };
        while let Some(item) = bytes_stream.next().await {
            let bytes = item?;
            let str_bytes = from_utf8(&bytes[..])?;
            if str_bytes.len() < 6 {
                continue;
            }
            let trim_start = str_bytes.split("data: ");
            let mut buf = "".to_string();
            for spl in trim_start {
                if spl.len() == 0 || spl == "[DONE]\n\n" {
                    if buf.len() > 0 {
                        error!("chunk buffer: {buf:?}");
                    }
                    buf = "".to_string();
                    continue;
                }
                buf.push_str(spl);
                let chunk: Result<ChatCompletionChunkResponse, _> =
                    serde_json::from_str(&buf);
                if let Ok(chunk) = chunk {
                    stream_buffer.push(chunk);
                    buf = "".to_string();
                }
            }
            if stream_buffer.len() > count {
                yield yield_buffer(stream_buffer);
                stream_buffer = Vec::new();
            }
        }
        if stream_buffer.len() > 0 {
            yield yield_buffer(stream_buffer);
        }
    }
}
