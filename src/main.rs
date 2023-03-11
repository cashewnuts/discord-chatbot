use std::{
    env,
    str::{from_utf8, FromStr},
    sync::Arc,
};

use discord_chatbot::{
    models::{
        channel::Channel,
        dynamo::discord_command::{ChatCommandMessage, DiscordCommand},
        message::Message,
        request::InteractionRequest,
        response::InteractionResponse,
    },
    services::discord_service::{get_get_channel, get_get_messages},
};
use ed25519_dalek::{PublicKey, Signature};
use environment::DISCORD_BOT_PUBLIC_KEY;
use lambda_http::{http::Method, run, service_fn, Body, Error, Request, RequestExt, Response};
use tracing::{error, info, instrument};

pub mod environment;
pub mod error;
pub mod models;

#[instrument(ret, err)]
fn get_response(_req: &Request) -> Result<Response<Body>, Error> {
    Ok(Response::new(Body::from("Hello world!")))
}

#[instrument(skip(http_client, dynamo_client), ret, err)]
async fn post_interactions_handler(
    req: &Request,
    http_client: &reqwest::Client,
    dynamo_client: &aws_sdk_dynamodb::Client,
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
                let channel_id = request.channel_id.unwrap();
                let channel = get_get_channel(http_client, &channel_id)
                    .await?
                    .json::<Channel>()
                    .await?;
                let topic = if channel.type_ == 0u32 {
                    channel.topic
                } else if let Some(p_channel_id) = channel.parent_id {
                    let parent_channel = get_get_channel(http_client, &p_channel_id)
                        .await?
                        .json::<Channel>()
                        .await?;
                    parent_channel.topic
                } else {
                    None
                };
                let messages =
                    get_get_messages(http_client, &channel_id, channel.last_message_id, Some(1))
                        .await?
                        .json::<Vec<Message>>()
                        .await?;
                info!("messages: {messages:?}");
                let message = messages.first().unwrap();
                let content = message.content.clone().unwrap();
                let res = dynamo_client
                    .put_item()
                    .table_name(env::var("DISCORD_COMMAND_TABLE")?)
                    .set_item(Some(serde_dynamo::to_item(DiscordCommand::chat_command(
                        &request.id,
                        &channel_id,
                        &request.token,
                        topic,
                        vec![ChatCommandMessage::user(content)],
                        10,
                    ))?))
                    .send()
                    .await?;
                let response = InteractionResponse::new(5, Option::<String>::None);
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
async fn function_handler(
    req: &Request,
    http_client: &reqwest::Client,
    dynamo_client: &aws_sdk_dynamodb::Client,
) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    if validate_request(&req).is_err() {
        let resp = Response::builder()
            .status(401)
            .header("content-type", "text/html")
            .body("invalid request signature".into())
            .map_err(Box::new)?;
        return Ok(resp);
    }

    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => get_response(&req),
        (&Method::POST, "/api/interactions") => {
            post_interactions_handler(&req, http_client, dynamo_client).await
        }
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

    let http_client = Arc::new(reqwest::Client::new());
    let config = aws_config::load_from_env().await;
    let dynamo_client = Arc::new(aws_sdk_dynamodb::Client::new(&config));
    // Define a closure here that makes use of the shared client.
    let handler_func_closure = move |event: Request| {
        let http_client = http_client.clone();
        let dynamo_client = dynamo_client.clone();
        async move { function_handler(&event, &http_client, &dynamo_client).await }
    };

    run(service_fn(handler_func_closure)).await
}
