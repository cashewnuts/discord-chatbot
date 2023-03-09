use clap::Parser;

use discord_chatbot::{
    error::Error,
    services::{
        chatgpt_service::post_get_channel,
        discord_service::{
            get_get_channel, post_create_application_command, post_create_guild_command,
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
    CreateCommand {
        #[arg(short, long)]
        guild_id: Option<String>,
    },
    GetChannel {
        #[arg(short, long)]
        channel_id: String,
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
        Action::CreateCommand { guild_id } => {
            if let Some(guild_id) = guild_id {
                info!("create guild command: {guild_id}");
                let response = post_create_guild_command(&client, &guild_id).await?;
                println!("{:?}", response.text().await?);
            } else {
                info!("create application command");
                let response = post_create_application_command(&client).await?;
                println!("{:?}", response.text().await?);
            }
        }
        Action::GetChannel { channel_id } => {
            info!("get channel: {channel_id}");
            let response = get_get_channel(&client, &channel_id).await?;
            println!("{:?}", response.text().await?);
        }
        Action::Chat { text } => {
            info!("chat: {text}");
            let response = post_get_channel(&client, &text).await?;
            println!("{:?}", response.text().await?);
        }
    }
    Ok(())
}
