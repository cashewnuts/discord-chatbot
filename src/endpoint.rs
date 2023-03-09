use tracing::instrument;

use crate::constants::{CHATGPT_BASE_URL, DISCORD_BASE_URL};
use crate::env::DISCORD_APPLICATION_ID;

/**
 * https://discord.com/developers/docs/interactions/application-commands#create-global-application-command
 */
#[instrument(ret)]
pub fn get_register_application_command_endpoint() -> String {
    format!(
        "{DISCORD_BASE_URL}/applications/{}/commands",
        DISCORD_APPLICATION_ID.unwrap()
    )
}

/**
 * https://discord.com/developers/docs/interactions/application-commands#create-guild-application-command
 */
#[instrument(ret)]
pub fn get_register_guild_command_endpoint(guild_id: &str) -> String {
    format!(
        "{DISCORD_BASE_URL}/applications/{}/guilds/{guild_id}/commands",
        DISCORD_APPLICATION_ID.unwrap()
    )
}

/**
 * https://discord.com/developers/docs/resources/channel#get-channel
 */
#[instrument(ret)]
pub fn get_channel_endpoint(channel_id: &str) -> String {
    format!("{DISCORD_BASE_URL}/channels/{channel_id}",)
}

/**
 * https://discord.com/developers/docs/resources/channel#get-channel
 */
#[instrument(ret)]
pub fn get_chatgpt_endpoint() -> String {
    format!("{CHATGPT_BASE_URL}/chat/completions",)
}
