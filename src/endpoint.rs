use tracing::instrument;

use crate::constants::{CHATGPT_BASE_URL, DISCORD_BASE_URL};
use crate::environment::DISCORD_APPLICATION_ID;

#[instrument(ret)]
pub fn application_commands_endpoint() -> String {
    format!(
        "{DISCORD_BASE_URL}/applications/{}/commands",
        DISCORD_APPLICATION_ID.unwrap()
    )
}

#[instrument(ret)]
pub fn application_command_item_endpoint(command_id: &str) -> String {
    format!(
        "{DISCORD_BASE_URL}/applications/{}/commands/{command_id}",
        DISCORD_APPLICATION_ID.unwrap()
    )
}

#[instrument(ret)]
pub fn guild_commands_endpoint(guild_id: &str) -> String {
    format!(
        "{DISCORD_BASE_URL}/applications/{}/guilds/{guild_id}/commands",
        DISCORD_APPLICATION_ID.unwrap()
    )
}

#[instrument(ret)]
pub fn guild_command_item_endpoint(guild_id: &str, command_id: &str) -> String {
    format!(
        "{DISCORD_BASE_URL}/applications/{}/guilds/{guild_id}/commands/{command_id}",
        DISCORD_APPLICATION_ID.unwrap()
    )
}

#[instrument(ret)]
pub fn channel_item_endpoint(channel_id: &str) -> String {
    format!("{DISCORD_BASE_URL}/channels/{channel_id}",)
}

#[instrument(ret)]
pub fn chatgpt_completions_endpoint() -> String {
    format!("{CHATGPT_BASE_URL}/chat/completions",)
}

/**
 * https://discord.com/developers/docs/resources/channel#create-message
 */
#[instrument(ret)]
pub fn get_create_message_endpoint(channel_id: &str) -> String {
    format!("{DISCORD_BASE_URL}/channels/{channel_id}/messages")
}

/**
 * https://discord.com/developers/docs/interactions/receiving-and-responding#create-followup-message
 */
#[instrument(ret)]
pub fn get_followup_endpoint(interaction_token: &str) -> String {
    format!(
        "{DISCORD_BASE_URL}/webhooks/{}/{interaction_token}",
        DISCORD_APPLICATION_ID.unwrap()
    )
}

/**
 * https://discord.com/developers/docs/resources/channel#start-thread-without-message
 */
#[instrument(ret)]
pub fn get_start_thread_endpoint(channel_id: &str) -> String {
    format!("{DISCORD_BASE_URL}/channels/{channel_id}/threads")
}
