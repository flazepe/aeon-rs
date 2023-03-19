mod commands;
mod constants;
mod macros;
mod structs;
mod traits;

use anyhow::Result;
use slashook::main;
use structs::client::AeonClient;

#[main]
async fn main() -> Result<()> {
    let mut client = AeonClient::new().await?;

    client.register_commands().await?;
    client.start().await;

    Ok(())
}
