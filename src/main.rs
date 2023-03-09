use std::str::{from_utf8, FromStr};

use discord_chatbot::models::{
    request::InteractionRequest,
    response::{InteractionMessage, InteractionResponse},
};
use ed25519_dalek::{PublicKey, Signature};
use env::DISCORD_BOT_PUBLIC_KEY;
use lambda_http::{http::Method, run, service_fn, Body, Error, Request, RequestExt, Response};
use tracing::{error, info, instrument};

pub mod env;
pub mod error;
pub mod models;

#[instrument(ret, err)]
async fn get_response(_req: &Request) -> Result<Response<Body>, Error> {
    Ok(Response::new(Body::from("Hello world!")))
}

#[instrument(ret, err)]
async fn post_interactions_handler(req: &Request) -> Result<Response<Body>, Error> {
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
            let response = InteractionResponse::new(
                4,
                Some(InteractionMessage {
                    tts: None,
                    content: Some("hello world!".to_string()),
                }),
            );
            Ok(Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&response)?))
                .unwrap())
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

    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => get_response(&req).await,
        (&Method::POST, "/api/interactions") => post_interactions_handler(&req).await,
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
