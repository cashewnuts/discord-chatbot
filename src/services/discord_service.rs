use reqwest::Response;
use serde::Serialize;
use serde_json::json;
use tracing::{info, instrument};

use crate::{
    endpoint::{
        application_command_item_endpoint, application_commands_endpoint, channel_item_endpoint,
        get_channel_message_item_endpoint, get_channel_messages_endpoint, get_followup_endpoint,
        get_start_thread_endpoint, guild_command_item_endpoint, guild_commands_endpoint,
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
        options: None,
    }
}

pub fn generate_chats_command() -> ApplicationCommand {
    ApplicationCommand {
        name: "chats".to_string(),
        type_: 1,
        description: Some("ChatGPT command".to_string()),
        options: Some(vec![ApplicationCommandOption {
            name: "read_count".to_string(),
            type_: 4,
            description: "Read messages count. default is 3".to_string(),
            required: Some(false),
            min_length: None,
            max_value: Some(100),
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

/**
 * https://discord.com/developers/docs/resources/channel#get-channel-messages
 */
#[instrument(skip(client), ret, err)]
pub async fn get_get_messages(
    client: &reqwest::Client,
    channel_id: &str,
    before: Option<String>,
    limit: Option<u32>,
) -> Result<Response, Error> {
    let limit = limit.or_else(|| Some(10)).unwrap();
    let query_params = if let Some(before) = before {
        json!({
            "before": before,
            "limit": limit,
        })
    } else {
        json!({
            "limit": limit,
        })
    };
    let resp = client
        .get(get_channel_messages_endpoint(channel_id))
        .header(
            "Authorization",
            format!("Bot {}", DISCORD_BOT_TOKEN.unwrap()),
        )
        .query(&query_params)
        .send()
        .await?;

    Ok(resp)
}

/**
 * https://discord.com/developers/docs/resources/channel#get-channel-message
 */
#[instrument(skip(client), ret, err)]
pub async fn get_get_message(
    client: &reqwest::Client,
    channel_id: &str,
    message_id: &str,
) -> Result<Response, Error> {
    let resp = client
        .get(get_channel_message_item_endpoint(channel_id, message_id))
        .header(
            "Authorization",
            format!("Bot {}", DISCORD_BOT_TOKEN.unwrap()),
        )
        .send()
        .await?;

    Ok(resp)
}

/**
 * https://discord.com/developers/docs/resources/channel#create-message
 */
#[instrument(skip(client, payload), ret, err)]
pub async fn post_message<T: Serialize + ?Sized>(
    client: &reqwest::Client,
    channel_id: &str,
    payload: &T,
) -> Result<Response, Error> {
    let resp = client
        .post(get_channel_messages_endpoint(channel_id))
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
