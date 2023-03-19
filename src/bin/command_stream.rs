use aws_lambda_events::event::{dynamodb::Event, streams::DynamoDbEventResponse};
use discord_chatbot::{
    models::{
        chatgpt::chat_completion::ChatCompletionRequest,
        discord::{message::Message, webhook_request::WebhookRequest},
        dynamo::discord_command::{CommandType, DiscordCommand},
    },
    service::ServiceFn,
    services::{
        chatgpt_service::{post_chat_completions, response_extract_stream},
        discord_service::{edit_followup_message, post_followup_message},
    },
};
use futures_util::{pin_mut, StreamExt};
use lambda_runtime::{run, Error, LambdaEvent};
use std::sync::Arc;
use tokio::task::JoinSet;
use tracing::{error, info, warn};

struct Service {
    client: Arc<reqwest::Client>,
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(
    event: LambdaEvent<Event>,
    service: &Service,
) -> Result<DynamoDbEventResponse, Error> {
    let response = DynamoDbEventResponse {
        batch_item_failures: Vec::new(),
    };

    let mut set = JoinSet::new();

    // Extract some useful information from the request
    for record in event.payload.records.into_iter() {
        let record_box = Box::new(record.clone());
        let client = service.client.clone();
        match record.event_name.as_str() {
            // MODIFY is for replay usage
            "INSERT" | "MODIFY" => {
                set.spawn(async move {
                    let event_id = record_box.event_id.clone();
                    info!("processing event ({event_id})");

                    let map_err_event_id = |e| {
                        error!("error occurred {e:?}");
                        event_id.clone()
                    };
                    let map_err_json_event_id = |e| {
                        error!("json serialize error occurred {e:?}");
                        event_id.clone()
                    };

                    let new_image = record.change.new_image;
                    let command_try: Result<DiscordCommand, _> = serde_dynamo::from_item(new_image);
                    let command = match command_try {
                        Ok(c) => c,
                        Err(err) => {
                            warn!("unsupported command: {err:?}");
                            return Ok(());
                        }
                    };

                    match command.clone().command_type {
                        CommandType::Chat(chat_command) => {
                            let response = post_chat_completions(
                                &client,
                                &ChatCompletionRequest::from(chat_command.clone()),
                            )
                            .await
                            .map_err(map_err_event_id)?;

                            let chat_response = if response.status().is_success() {
                                response
                            } else {
                                let err_text =
                                    response.text().await.map_err(map_err_json_event_id)?;
                                error!("chatgpt error response: {err_text:?}");
                                post_followup_message(
                                    &client,
                                    &chat_command.interaction_token,
                                    &WebhookRequest { content: err_text },
                                )
                                .await
                                .map_err(map_err_event_id)?;
                                return Ok(());
                            };
                            let stream = response_extract_stream(chat_response, 10);
                            pin_mut!(stream); // needed for iteration
                            let mut buffer = String::new();
                            let mut message: Option<Message> = None;
                            while let Some(value) = stream.next().await {
                                let value = value.map_err(|err| {
                                    error!("stream error: {err:?}");
                                    event_id.clone()
                                })?;
                                buffer.push_str(&value);
                                if let Some(msg) = message.clone() {
                                    let message_id = msg.id;
                                    edit_followup_message(
                                        &client,
                                        &message_id,
                                        &chat_command.interaction_token,
                                        &WebhookRequest {
                                            content: buffer.clone(),
                                        },
                                    )
                                    .await
                                    .map_err(map_err_event_id)?;
                                } else {
                                    let msg = post_followup_message(
                                        &client,
                                        &chat_command.interaction_token,
                                        &WebhookRequest {
                                            content: buffer.clone(),
                                        },
                                    )
                                    .await
                                    .map_err(map_err_event_id)?
                                    .json::<Message>()
                                    .await
                                    .map_err(map_err_json_event_id)?;
                                    message = Some(msg);
                                }
                            }
                        }
                    }

                    println!("command: {command:?}");
                    info!("processed event ({event_id})");
                    Result::<(), String>::Ok(())
                });
            }
            _ => info!("Do nothing on Delete"),
        }
    }

    while let Some(res) = set.join_next().await {
        match res {
            Ok(_) => info!("done"),
            Err(_) => error!("error"),
        }
    }

    Ok(response)
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

    let client = Arc::new(reqwest::Client::new());

    let svs = &Service { client };

    let service = ServiceFn::new(function_handler, svs);
    // Our Filter...
    run(service).await
}
