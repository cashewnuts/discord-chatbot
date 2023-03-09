use reqwest::Response;
use tracing::{info, instrument};

use crate::{
    endpoint::{
        get_channel_endpoint, get_register_application_command_endpoint,
        get_register_guild_command_endpoint,
    },
    env::DISCORD_BOT_TOKEN,
    error::Error,
    models::application_command::{ApplicationCommand, ApplicationCommandOption},
};

pub fn generate_chat_command() -> ApplicationCommand {
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
pub async fn post_create_application_command(client: &reqwest::Client) -> Result<Response, Error> {
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
        .await?;

    Ok(resp)
}

#[instrument(skip(client), ret, err)]
pub async fn post_create_guild_command(
    client: &reqwest::Client,
    guild_id: &str,
) -> Result<Response, Error> {
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
        .await?;

    Ok(resp)
}

#[instrument(skip(client), ret, err)]
pub async fn get_get_channel(
    client: &reqwest::Client,
    channel_id: &str,
) -> Result<Response, Error> {
    let resp = client
        .get(get_channel_endpoint(channel_id))
        .header(
            "Authorization",
            format!("Bot {}", DISCORD_BOT_TOKEN.unwrap()),
        )
        .send()
        .await?;

    Ok(resp)
}
