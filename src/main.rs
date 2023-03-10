use std::str::{from_utf8, FromStr};

use discord_chatbot::{
    models::{
        request::InteractionRequest,
        response::{InteractionMessage, InteractionResponse},
    },
    services::{discord_service::{post_start_thread, post_message}, chatgpt_service::post_chat_completions},
};
use ed25519_dalek::{PublicKey, Signature};
use env::DISCORD_BOT_PUBLIC_KEY;
use lambda_http::{http::Method, run, service_fn, Body, Error, Request, RequestExt, Response};
use serde_json::json;
use tracing::{error, info, instrument};

use crate::models::{channel::Channel, chat_completion::ChatCompletionResponse};

pub mod env;
pub mod error;
pub mod models;

#[instrument(ret, err)]
fn get_response(_req: &Request) -> Result<Response<Body>, Error> {
    Ok(Response::new(Body::from("Hello world!")))
}

#[instrument(skip(client), ret, err)]
async fn post_interactions_handler(
    client: &reqwest::Client,
    req: &Request,
) -> Result<Response<Body>, Error> {
    info!("{DISCORD_BOT_PUBLIC_KEY:?}");
    let request: InteractionRequest = if let Ok(Some(req)) = req.payload() {
        req
    } else {
        let resp = Response::builder()
            .status(400)
            .header("content-type", "text/html")
            .body("invalid request signature".into())
            .map_err(Box::new)?;
        return Ok(resp);
    };
    info!("{request:?}");
    match request.type_ {
        1u32 => {
            let response = InteractionResponse::<String>::new(1, None);
            Ok(Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&response)?))
                .unwrap())
        }
        2u32 => {
            let data = request.data.unwrap();
            if data.name.as_str() == "chat" {
                let options = data.options.unwrap();
                let option = options.first().unwrap();
                let text = option.value.clone().unwrap();
                let response =
                    post_start_thread(client, &request.channel_id.unwrap(), &format!("{text:<20}")).await?;
                let channel: Channel = response.json().await?;
                info!("post_start_thread: {:?}", channel);
                post_message(client, &channel.id, &json!({
                    "content": text,
                })).await?;
                let chat: ChatCompletionResponse = post_chat_completions(client, "You are a helpful assistant.", &text).await?.json().await?;
                info!("total_token_usage: {:?}", chat.get_total_token_usage());
                let chat_choice = chat.choices.first().unwrap();
                let chat_message_content = chat_choice.message.content.clone();
                post_message(client, &channel.id, &json!({
                    "content": chat_message_content,
                })).await?;
                let response = InteractionResponse::new(
                    4,
                    Some(InteractionMessage {
                        tts: None,
                        content: Some("Thread started! Check out blow".to_string()),
                    }),
                );
                Ok(Response::builder()
                    .status(200)
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&response)?))
                    .unwrap())
            } else {
                Ok(Response::builder()
                    .status(400)
                    .header("content-type", "application/json")
                    .body(Body::from("Unsupported commands"))
                    .unwrap())
            }
        }
        _ => Ok(Response::new(Body::from("unsupported type"))),
    }
}

#[instrument(ret, err)]
fn validate_request(req: &Request) -> Result<(), Error> {
    let headers = req.headers();
    let signature = headers
        .get("X-Signature-Ed25519")
        .ok_or("Header not found: X-Signature-Ed25519")?
        .to_str()?;
    let timestamp = headers
        .get("X-Signature-Timestamp")
        .ok_or("Header not found: X-Signature-Timestamp")?
        .to_str()?;
    let body: String = from_utf8(req.body())?.to_string();

    let raw_public_str = DISCORD_BOT_PUBLIC_KEY.unwrap();
    let public_key = PublicKey::from_bytes(&hex::decode(raw_public_str)?).unwrap();
    let signature = Signature::from_str(signature)?;
    let msg = format!("{timestamp}{body}");

    public_key
        .verify_strict(msg.as_bytes(), &signature)
        .map_err(|e| Error::from(e.to_string()))
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(req: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    if validate_request(&req).is_err() {
        let resp = Response::builder()
            .status(401)
            .header("content-type", "text/html")
            .body("invalid request signature".into())
            .map_err(Box::new)?;
        return Ok(resp);
    }

    let client = reqwest::Client::new();
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => get_response(&req),
        (&Method::POST, "/api/interactions") => post_interactions_handler(&client, &req).await,
        _ => {
            error!("{req:?}");
            let resp = Response::builder()
                .status(404)
                .header("content-type", "text/html")
                .body("Not found".into())
                .map_err(Box::new)?;
            Ok(resp)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .with_file(true)
        .with_line_number(true)
        .init();

    run(service_fn(function_handler)).await
}
