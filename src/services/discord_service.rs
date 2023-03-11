use reqwest::Response;
use serde::Serialize;
use serde_json::json;
use tracing::{info, instrument};

use crate::{
    endpoint::{
        application_command_item_endpoint, application_commands_endpoint, channel_item_endpoint,
        get_create_message_endpoint, get_followup_endpoint, get_start_thread_endpoint,
        guild_command_item_endpoint, guild_commands_endpoint,
    },
    environment::DISCORD_BOT_TOKEN,
    error::Error,
    models::application_command::{ApplicationCommand, ApplicationCommandOption},
};

pub fn generate_chat_command() -> ApplicationCommand {
    ApplicationCommand {
        name: "chat".to_string(),
        type_: 1,
        description: Some("ChatGPT command".to_string()),
        options: Some(vec![ApplicationCommandOption {
            name: "text".to_string(),
            type_: 3,
            description: "question text".to_string(),
            required: Some(true),
            min_length: Some(5u32),
        }]),
    }
}

pub fn generate_message_command() -> ApplicationCommand {
    ApplicationCommand {
        name: "Summarize".to_string(),
        type_: 3, // Message
        description: None,
        options: None,
    }
}

/**
 * https://discord.com/developers/docs/interactions/application-commands#create-global-application-command
 */
#[instrument(skip(client), ret, err)]
pub async fn post_create_application_chat_command(
    client: &reqwest::Client,
) -> Result<Response, Error> {
    let command = generate_chat_command();
    info!("{command:?}");

    let resp = client
        .post(application_commands_endpoint())
        .header(
            "Authorization",
            format!("Bot {}", DISCORD_BOT_TOKEN.unwrap()),
        )
        .json(&command)
        .send()
        .await?;

    Ok(resp)
}

/**
 * https://discord.com/developers/docs/interactions/application-commands#create-global-application-command
 */
#[instrument(skip(client), ret, err)]
pub async fn post_create_application_message_command(
    client: &reqwest::Client,
) -> Result<Response, Error> {
    let command = generate_message_command();
    info!("{command:?}");

    let resp = client
        .post(application_commands_endpoint())
        .header(
            "Authorization",
            format!("Bot {}", DISCORD_BOT_TOKEN.unwrap()),
        )
        .json(&command)
        .send()
        .await?;

    Ok(resp)
}

/**
 * https://discord.com/developers/docs/interactions/application-commands#create-guild-application-command
 */
#[instrument(skip(client), ret, err)]
pub async fn post_create_guild_chat_command(
    client: &reqwest::Client,
    guild_id: &str,
) -> Result<Response, Error> {
    let command = generate_chat_command();
    info!("{command:?}");

    let resp = client
        .post(guild_commands_endpoint(guild_id))
        .header(
            "Authorization",
            format!("Bot {}", DISCORD_BOT_TOKEN.unwrap()),
        )
        .json(&command)
        .send()
        .await?;

    Ok(resp)
}

/**
 * https://discord.com/developers/docs/interactions/application-commands#create-guild-application-command
 */
#[instrument(skip(client), ret, err)]
pub async fn post_create_guild_message_command(
    client: &reqwest::Client,
    guild_id: &str,
) -> Result<Response, Error> {
    let command = generate_message_command();
    info!("{command:?}");

    let resp = client
        .post(guild_commands_endpoint(guild_id))
        .header(
            "Authorization",
            format!("Bot {}", DISCORD_BOT_TOKEN.unwrap()),
        )
        .json(&command)
        .send()
        .await?;

    Ok(resp)
}

/**
 * https://discord.com/developers/docs/interactions/application-commands#get-guild-application-commands
 */
#[instrument(skip(client), ret, err)]
pub async fn get_application_commands(client: &reqwest::Client) -> Result<Response, Error> {
    let resp = client
        .get(application_commands_endpoint())
        .header(
            "Authorization",
            format!("Bot {}", DISCORD_BOT_TOKEN.unwrap()),
        )
        .send()
        .await?;

    Ok(resp)
}

/**
 * https://discord.com/developers/docs/interactions/application-commands#get-guild-application-commands
 */
#[instrument(skip(client), ret, err)]
pub async fn get_guild_commands(
    client: &reqwest::Client,
    guild_id: &str,
) -> Result<Response, Error> {
    let resp = client
        .get(guild_commands_endpoint(guild_id))
        .header(
            "Authorization",
            format!("Bot {}", DISCORD_BOT_TOKEN.unwrap()),
        )
        .send()
        .await?;

    Ok(resp)
}

/**
 * https://discord.com/developers/docs/interactions/application-commands#delete-global-application-command
 */
#[instrument(skip(client), ret, err)]
pub async fn delete_guild_command(
    client: &reqwest::Client,
    guild_id: &str,
    command_id: &str,
) -> Result<Response, Error> {
    let resp = client
        .delete(guild_command_item_endpoint(guild_id, command_id))
        .header(
            "Authorization",
            format!("Bot {}", DISCORD_BOT_TOKEN.unwrap()),
        )
        .send()
        .await?;

    Ok(resp)
}

/**
 * https://discord.com/developers/docs/interactions/application-commands#delete-global-application-command
 */
#[instrument(skip(client), ret, err)]
pub async fn delete_application_command(
    client: &reqwest::Client,
    command_id: &str,
) -> Result<Response, Error> {
    let resp = client
        .delete(application_command_item_endpoint(command_id))
        .header(
            "Authorization",
            format!("Bot {}", DISCORD_BOT_TOKEN.unwrap()),
        )
        .send()
        .await?;

    Ok(resp)
}

/**
 * https://discord.com/developers/docs/resources/channel#get-channel
 */
#[instrument(skip(client), ret, err)]
pub async fn get_get_channel(
    client: &reqwest::Client,
    channel_id: &str,
) -> Result<Response, Error> {
    let resp = client
        .get(channel_item_endpoint(channel_id))
        .header(
            "Authorization",
            format!("Bot {}", DISCORD_BOT_TOKEN.unwrap()),
        )
        .send()
        .await?;

    Ok(resp)
}

#[instrument(skip(client, payload), ret, err)]
pub async fn post_message<T: Serialize + ?Sized>(
    client: &reqwest::Client,
    channel_id: &str,
    payload: &T,
) -> Result<Response, Error> {
    let resp = client
        .post(get_create_message_endpoint(channel_id))
        .header(
            "Authorization",
            format!("Bot {}", DISCORD_BOT_TOKEN.unwrap()),
        )
        .json(payload)
        .send()
        .await?;

    Ok(resp)
}

#[instrument(skip(client, payload), ret, err)]
pub async fn post_followup_message<T: Serialize + ?Sized>(
    client: &reqwest::Client,
    interaction_token: &str,
    payload: &T,
) -> Result<Response, Error> {
    let resp = client
        .post(get_followup_endpoint(interaction_token))
        .header(
            "Authorization",
            format!("Bot {}", DISCORD_BOT_TOKEN.unwrap()),
        )
        .json(payload)
        .send()
        .await?;

    Ok(resp)
}

#[instrument(skip(client), ret, err)]
pub async fn post_start_thread(
    client: &reqwest::Client,
    channel_id: &str,
    name: &str,
) -> Result<Response, Error> {
    let resp = client
        .post(get_start_thread_endpoint(channel_id))
        .header(
            "Authorization",
            format!("Bot {}", DISCORD_BOT_TOKEN.unwrap()),
        )
        .json(&json!({
            "name": name,
            // https://discord.com/developers/docs/resources/channel#channel-object-channel-types
            "type": 11,
            "auto_archive_duration": 60
        }))
        .send()
        .await?;

    Ok(resp)
}
