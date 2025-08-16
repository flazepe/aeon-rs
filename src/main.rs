mod commands;
mod functions;
mod macros;
pub mod statics;
mod structs;
mod traits;

use crate::{
    statics::MONGODB,
    structs::{client::AeonClient, database::reminders::Reminders, gateway::client::GatewayClient},
};
use anyhow::Result;
use slashook::main;
use tokio::spawn;

#[main]
async fn main() -> Result<()> {
    MONGODB.set(AeonClient::connect_to_database().await?).expect("Could not set MongoDB client.");
    println!("[DATABASE] Connected to MongoDB.");

    spawn(Reminders::poll());
    println!("[REMINDERS] Started polling reminders.");

    spawn(GatewayClient::new().create_shards());
    println!("[GATEWAY] Spawned client.");

    let mut client = AeonClient::new();

    match client.register_commands().await {
        Ok(_) => println!("[CLIENT] Registered commands."),
        Err(error) => println!("[CLIENT] An error occurred while registering commands: {error}"),
    };

    client.start().await;

    Ok(())
}
