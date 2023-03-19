mod commands;
mod constants;
mod macros;
mod structs;
mod traits;

use crate::structs::{client::AeonClient, gateway::client::GatewayClient};
use anyhow::Result;
use slashook::main;
use tokio::spawn;

#[main]
async fn main() -> Result<()> {
    // Spawn gateway client
    spawn(GatewayClient::new().create_shards());

    let mut client = AeonClient::new().await?;

    client.register_commands().await?;
    client.start().await;

    Ok(())
}
