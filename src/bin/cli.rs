use awc::Client;
use clap::Parser;
use discord_chatbot::constants::DISCORD_BASE_URL;
use discord_chatbot::env::{DISCORD_APPLICATION_ID, DISCORD_BOT_TOKEN};
use discord_chatbot::models::application_command::ApplicationCommand;
use tracing::{info, instrument};

type Error = Box<dyn std::error::Error + Send + Sync>;

/**
 * https://discord.com/developers/docs/interactions/application-commands#create-global-application-command
 */
#[instrument(ret)]
fn get_register_application_command_endpoint() -> String {
    format!(
        "{DISCORD_BASE_URL}/applications/{}/commands",
        DISCORD_APPLICATION_ID.unwrap()
    )
}

/**
 * https://discord.com/developers/docs/interactions/application-commands#create-guild-application-command
 */
#[instrument(ret)]
fn get_register_guild_command_endpoint(guild_id: &str) -> String {
    format!(
        "{DISCORD_BASE_URL}/applications/{}/guilds/{guild_id}/commands",
        DISCORD_APPLICATION_ID.unwrap()
    )
}

#[instrument(skip(client), ret, err)]
async fn post_create_application_command(client: &Client) -> Result<(), Error> {
    let command = ApplicationCommand {
        name: "chat".to_string(),
        type_: 1,
        description: "ChatGPT command".to_string(),
    };
    info!("{command:?}");

    let resp = client
        .post(get_register_application_command_endpoint())
        .insert_header((
            "Authorization",
            format!("Bot {}", DISCORD_BOT_TOKEN.unwrap()),
        ))
        .send_json(&command)
        .await
        .map_err(|_e| Error::from("Error occuerred".to_string()))?;
    info!("{:?}", resp);
    Ok(())
}

#[instrument(skip(client), ret, err)]
async fn post_create_guild_command(client: &Client, guild_id: &str) -> Result<(), Error> {
    let command = ApplicationCommand {
        name: "chat".to_string(),
        type_: 1,
        description: "ChatGPT command".to_string(),
    };
    info!("{command:?}");

    let resp = client
        .post(get_register_guild_command_endpoint(guild_id))
        .insert_header((
            "Authorization",
            format!("Bot {}", DISCORD_BOT_TOKEN.unwrap()),
        ))
        .send_json(&command)
        .await
        .map_err(|_e| Error::from("Error occuerred".to_string()))?;

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

    let client = Client::default();
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
    }
    Ok(())
}
