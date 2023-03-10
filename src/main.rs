use anyhow::Result;
use slashook::main;

pub mod client;
pub mod commands;
pub mod config;
pub mod constants;
pub mod structs;

#[main]
async fn main() -> Result<()> {
    let mut client = client::AeonClient::new()?;

    client.register_commands().await?;
    client.start().await;

    Ok(())
}
