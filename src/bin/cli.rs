use std::str::from_utf8;

use clap::Parser;
use discord_chatbot::{
    error::Error,
    models::chatgpt::chat_completion::ChatCompletionResponse,
    models::{
        chatgpt::chat_completion::ChatCompletionChunkResponse,
        discord::webhook_request::WebhookRequest,
    },
    services::{
        chatgpt_service::post_chat_completions,
        discord_service::{
            delete_application_command, delete_guild_command, generate_chat_command,
            generate_chata_command, generate_chats_command, get_application_commands,
            get_get_channel, get_get_message, get_get_messages, get_guild_commands,
            post_create_application_chat_command, post_create_application_message_command,
            post_create_guild_chat_command, post_create_guild_message_command,
            post_followup_message,
        },
    },
};
use futures_util::StreamExt;
use serde_json::json;
use tracing::{error, info};

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
    GetCommands {
        #[arg(short, long)]
        guild_id: Option<String>,
    },
    DeleteCommand {
        command_id: String,
        #[arg(short, long)]
        guild_id: Option<String>,
    },
    GetChannel {
        channel_id: String,
    },
    GetMessages {
        channel_id: String,
        #[arg(short, long, default_value_t = 10)]
        limit: u32,
        #[arg(short, long)]
        before: Option<String>,
    },
    GetMessage {
        channel_id: String,
        message_id: String,
    },
    FollowUp {
        #[arg(short, long)]
        token: String,
    },
    Chat {
        text: String,
        #[arg(short, long)]
        stream: bool,
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
                let response =
                    post_create_guild_chat_command(&client, &guild_id, &generate_chat_command())
                        .await?;
                println!("(GUILD)chat command created: {:?}", response.text().await?);
                let response =
                    post_create_guild_chat_command(&client, &guild_id, &generate_chats_command())
                        .await?;
                println!("(GUILD)chats command created: {:?}", response.text().await?);
                let response =
                    post_create_guild_chat_command(&client, &guild_id, &generate_chata_command())
                        .await?;
                println!("(GUILD)chata command created: {:?}", response.text().await?);
                let response = post_create_guild_message_command(&client, &guild_id).await?;
                println!(
                    "(GUILD)message command created: {:?}",
                    response.text().await?
                );
            } else {
                info!("create application command");
                let response =
                    post_create_application_chat_command(&client, &generate_chat_command()).await?;
                println!("chat command created: {:?}", response.text().await?);
                let response =
                    post_create_application_chat_command(&client, &generate_chats_command())
                        .await?;
                println!("chats command created: {:?}", response.text().await?);
                let response =
                    post_create_application_chat_command(&client, &generate_chata_command())
                        .await?;
                println!("chata command created: {:?}", response.text().await?);
                let response = post_create_application_message_command(&client).await?;
                println!("message command created: {:?}", response.text().await?);
            }
        }
        Action::GetCommands { guild_id } => match guild_id {
            Some(g_id) => {
                let response = get_guild_commands(&client, &g_id).await?;
                println!("guild commands: {:#?}", response.text().await?);
            }
            None => {
                let response = get_application_commands(&client).await?;
                println!("application commands: {:#?}", response.text().await?);
            }
        },
        Action::DeleteCommand {
            command_id,
            guild_id,
        } => match guild_id {
            Some(g_id) => {
                let response = delete_guild_command(&client, &g_id, &command_id).await?;
                println!("guild command deleted: {:#?}", response.text().await?);
            }
            None => {
                let response = delete_application_command(&client, &command_id).await?;
                println!("application command deleted: {:#?}", response.text().await?);
            }
        },
        Action::GetChannel { channel_id } => {
            info!("get channel: {channel_id}");
            let response = get_get_channel(&client, &channel_id).await?;
            println!("{:?}", response.text().await?);
        }
        Action::GetMessages {
            channel_id,
            before,
            limit,
        } => {
            info!("get channel messages: {channel_id}");
            let response = get_get_messages(&client, &channel_id, before, Some(limit)).await?;
            println!("{:?}", response.text().await?);
        }
        Action::GetMessage {
            channel_id,
            message_id,
        } => {
            info!("get channel message: {channel_id}:{message_id}");
            let response = get_get_message(&client, &channel_id, &message_id).await?;
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
        Action::Chat { text, stream } => {
            info!("chat: {text}");
            let response = post_chat_completions(
                &client,
                &serde_json::from_value(json!({
                    "model": "gpt-3.5-turbo",
                    "messages": [
                        {"role": "system", "content": "You are a helpful assistant."},
                        {"role": "user", "content": text},
                    ],
                    "stream": stream,
                }))?,
            )
            .await?;
            if stream {
                let mut bytes_stream = response.bytes_stream();
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
                            if let Some(choice) = chunk.choices.first() {
                                if let Some(content) = choice.delta.clone().content {
                                    print!("{content}");
                                }
                            }
                            buf = "".to_string();
                        }
                    }
                }
            } else {
                let response = response.json::<ChatCompletionResponse>().await?;
                println!("{:?}", response);
            }
        }
    }
    Ok(())
}
