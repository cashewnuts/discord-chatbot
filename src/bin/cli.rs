use std::vec;

use clap::Parser;

use discord_chatbot::endpoint::{
    get_channel_endpoint, get_register_application_command_endpoint,
    get_register_guild_command_endpoint,
};
use discord_chatbot::env::DISCORD_BOT_TOKEN;
use discord_chatbot::models::application_command::{ApplicationCommand, ApplicationCommandOption};
use tracing::{info, instrument};

type Error = Box<dyn std::error::Error + Send + Sync>;

fn generate_chat_command() -> ApplicationCommand {
    ApplicationCommand {
        name: "chat".to_string(),
        type_: 1,
        description: "ChatGPT command".to_string(),
        options: Some(vec![ApplicationCommandOption {
            name: "text".to_string(),
            type_: 3,
            description: "question text".to_string(),
            required: Some(true),
            min_length: Some(5u32),
        }]),
    }
}

#[instrument(skip(client), ret, err)]
async fn post_create_application_command(client: &reqwest::Client) -> Result<(), Error> {
    let command = generate_chat_command();
    info!("{command:?}");

    let resp = client
        .post(get_register_application_command_endpoint())
        .header(
            "Authorization",
            format!("Bot {}", DISCORD_BOT_TOKEN.unwrap()),
        )
        .json(&command)
        .send()
        .await?
        .text()
        .await?;
    info!("{:?}", resp);
    Ok(())
}

#[instrument(skip(client), ret, err)]
async fn post_create_guild_command(client: &reqwest::Client, guild_id: &str) -> Result<(), Error> {
    let command = generate_chat_command();
    info!("{command:?}");

    let resp = client
        .post(get_register_guild_command_endpoint(guild_id))
        .header(
            "Authorization",
            format!("Bot {}", DISCORD_BOT_TOKEN.unwrap()),
        )
        .json(&command)
        .send()
        .await?
        .text()
        .await?;

    info!("{:?}", resp);
    Ok(())
}

#[instrument(skip(client), ret, err)]
async fn get_get_channel(client: &reqwest::Client, channel_id: &str) -> Result<(), Error> {
    let resp = client
        .get(get_channel_endpoint(channel_id))
        .header(
            "Authorization",
            format!("Bot {}", DISCORD_BOT_TOKEN.unwrap()),
        )
        .send()
        .await?
        .text()
        .await?;

    info!("{:?}", resp);
    Ok(())
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    CreateCommand {
        #[arg(short, long)]
        guild_id: Option<String>,
    },
    GetChannel {
        #[arg(short, long)]
        channel_id: String,
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
        Action::CreateCommand { guild_id } => {
            if let Some(guild_id) = guild_id {
                info!("create guild command: {guild_id}");
                post_create_guild_command(&client, &guild_id).await?;
            } else {
                info!("create application command");
                post_create_application_command(&client).await?;
            }
        }
        Action::GetChannel { channel_id } => {
            info!("get channel: {channel_id}");
            get_get_channel(&client, &channel_id).await?;
        }
    }
    Ok(())
}
