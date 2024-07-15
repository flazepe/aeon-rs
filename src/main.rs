mod commands;
mod functions;
mod macros;
pub mod statics;
mod structs;
mod traits;

use crate::{
    statics::{CONFIG, MONGODB},
    structs::{api::ordr::OrdrRender, client::AeonClient, database::reminders::Reminders, gateway::client::GatewayClient},
};
use anyhow::Result;
use mongodb::{options::ClientOptions, Client as MongoDBClient};
use slashook::main;
use tokio::spawn;

#[main]
async fn main() -> Result<()> {
    let mut mongodb_options = ClientOptions::parse(&CONFIG.database.mongodb_uri).await?;
    mongodb_options.min_pool_size = Some(1);

    let mongodb = MongoDBClient::with_options(mongodb_options)?;
    mongodb.warm_connection_pool().await;

    MONGODB.set(mongodb.database("aeon")).expect("Could not set MongoDB client.");
    println!("[DATABASE] Connected to MongoDB.");

    spawn(Reminders::poll());
    println!("[REMINDERS] Started polling reminders.");

    spawn(GatewayClient::new().create_shards());
    println!("[GATEWAY] Spawned client.");

    spawn(OrdrRender::connect());
    println!("[ORDR] Spawned socket client.");

    let mut client = AeonClient::new();

    match client.register_commands().await {
        Ok(_) => println!("[CLIENT] Registered commands."),
        Err(error) => println!("[CLIENT] An error occurred while registering commands: {error}"),
    };

    client.start().await;

    Ok(())
}
