pub mod commands;
pub mod constants;
pub mod macros;
pub mod structs;
pub mod traits;

use anyhow::Result;
use slashook::main;
use structs::client::AeonClient;

#[main]
async fn main() -> Result<()> {
    let mut client = AeonClient::new()?;

    client.register_commands().await?;
    client.start().await;

    Ok(())
}
