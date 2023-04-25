mod commands;
mod functions;
mod macros;
mod statics;
mod structs;
mod traits;

use crate::{
    statics::{CONFIG, MONGODB},
    structs::{client::AeonClient, gateway::client::GatewayClient},
};
use anyhow::Result;
use mongodb::{options::ClientOptions as MongoDBClientOptions, Client as MongoDBClient};
use slashook::main;
use structs::reminders::Reminders;
use tokio::spawn;

#[main]
async fn main() -> Result<()> {
    MONGODB
        .get_or_init(async {
            MongoDBClient::with_options(MongoDBClientOptions::parse(&CONFIG.database.mongodb_uri).await.unwrap()).unwrap().database("aeon")
        })
        .await;

    println!("[DATABASE] Connected to MongoDB.");

    // Reminders
    spawn(Reminders::new().poll());
    println!("[REMINDERS] Started polling reminders.");

    // Spawn gateway client
    spawn(GatewayClient::new().create_shards());
    println!("[GATEWAY] Spawned gateway client.");

    let mut client = AeonClient::new();

    if let Err(error) = client.register_commands().await {
        println!("[CLIENT] An error occurred while registering commands: {error}");
    } else {
        println!("[CLIENT] Registered commands.");
    }

    client.start().await;

    Ok(())
}
