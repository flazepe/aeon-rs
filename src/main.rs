mod commands;
mod functions;
mod macros;
pub mod statics;
mod structs;
mod traits;

use crate::{
    statics::EMOJIS,
    structs::{
        client::AeonClient,
        database::{Database, mongodb::reminders::Reminders},
        emoji_manager::EmojiManager,
        gateway::client::GatewayClient,
    },
};
use anyhow::Result;
use slashook::main;
use tokio::spawn;
use tracing::{error, info, subscriber::set_global_default};
use tracing_subscriber::FmtSubscriber;

#[main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder().finish();
    set_global_default(subscriber)?;

    Database::init().await?;

    spawn(Reminders::poll());
    info!(target: "Reminders", "Spawned poller.");

    spawn(GatewayClient::new().create_shards());
    info!(target: "Gateway", "Spawned client.");

    let mut emojis = EmojiManager::new();

    match emojis.sync().await {
        Ok(_) => info!(target: "Emojis", "Synced."),
        Err(error) => error!(target: "Emojis", "An error occurred while syncing: {error:#?}"),
    }

    EMOJIS.set(emojis).expect("Could not set EmojiManager.");

    let mut client = AeonClient::new();

    match client.register_commands().await {
        Ok(_) => info!(target: "Slashook", "Registered commands."),
        Err(error) => error!(target: "Slashook", "An error occurred while registering commands: {error:#?}"),
    };

    client.start().await;

    Ok(())
}
