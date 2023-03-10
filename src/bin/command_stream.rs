use aws_lambda_events::event::{dynamodb::Event, streams::DynamoDbEventResponse};
use aws_sdk_dynamodb::Client;
use discord_chatbot::service::ServiceFn;
use lambda_runtime::{run, Error, LambdaEvent};
use tracing::info;

const PRIMARY_KEY: &str = "Id";

struct Service<'a> {
    client: &'a Client,
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler<'a>(
    event: LambdaEvent<Event>,
    service: &'a Service<'a>,
) -> Result<DynamoDbEventResponse, Error> {
    let response = DynamoDbEventResponse {
        batch_item_failures: Vec::new(),
    };

    // Extract some useful information from the request
    for record in event.payload.records.into_iter() {
        let event_id = record.event_id.clone();
        let keys = record.change.keys.clone();
        info!("success {}: {:?}", event_id, keys);
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
        .init();

    let config = &aws_config::load_from_env().await;
    let client = &Client::new(config);

    let svs = &Service { client };

    let service = ServiceFn::new(function_handler, svs);
    // Our Filter...
    run(service).await
}
