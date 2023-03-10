use clap::Parser;

use discord_chatbot::{
    error::Error,
    models::webhook_request::WebhookRequest,
    services::{
        chatgpt_service::post_chat_completions,
        discord_service::{
            get_get_channel, post_create_application_chat_command,
            post_create_application_message_command, post_create_guild_chat_command,
            post_create_guild_message_command, post_followup_message,
        },
    },
};
use tracing::info;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    CreateCommands {
        #[arg(short, long)]
        guild_id: Option<String>,
    },
    GetChannel {
        #[arg(short, long)]
        channel_id: String,
    },
    FollowUp {
        #[arg(short, long)]
        token: String,
    },
    Chat {
        #[arg(short, long)]
        text: String,
    },
}

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .with_file(true)
        .with_line_number(true)
        .init();

    let client = reqwest::Client::new();
    match args.action {
        Action::CreateCommands { guild_id } => {
            if let Some(guild_id) = guild_id {
                info!("create guild command: {guild_id}");
                let response = post_create_guild_chat_command(&client, &guild_id).await?;
                println!("(GUILD)chat command created: {:?}", response.text().await?);
                let response = post_create_guild_message_command(&client, &guild_id).await?;
                println!(
                    "(GUILD)message command created: {:?}",
                    response.text().await?
                );
            } else {
                info!("create application command");
                let response = post_create_application_chat_command(&client).await?;
                println!("chat command created: {:?}", response.text().await?);
                let response = post_create_application_message_command(&client).await?;
                println!("message command created: {:?}", response.text().await?);
            }
        }
        Action::GetChannel { channel_id } => {
            info!("get channel: {channel_id}");
            let response = get_get_channel(&client, &channel_id).await?;
            println!("{:?}", response.text().await?);
        }
        Action::FollowUp { token } => {
            info!("follow up: {token}");
            let response = post_followup_message(
                &client,
                &token,
                &WebhookRequest {
                    content: "Follow up!".to_string(),
                },
            )
            .await?;
            println!("{:?}", response.text().await?);
        }
        Action::Chat { text } => {
            info!("chat: {text}");
            let response =
                post_chat_completions(&client, "You are a helpful assistant.", &text).await?;
            println!("{:?}", response.text().await?);
        }
    }
    Ok(())
}
