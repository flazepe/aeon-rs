use anyhow::Result;
use slashook::main;

pub mod commands;
pub mod constants;
pub mod structs;
pub mod macros;
pub mod traits;

#[main]
async fn main() -> Result<()> {
    let mut client = structs::client::AeonClient::new()?;

    client.register_commands().await?;
    client.start().await;

    Ok(())
}
