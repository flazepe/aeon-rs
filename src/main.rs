mod commands;
mod functions;
mod macros;
pub mod statics;
mod structs;
mod traits;

use crate::{
    statics::{CONFIG, MONGODB},
    structs::{api::ordr::OrdrRender, client::AeonClient, database::reminders::Reminders, gateway::client::GatewayClient, ocr::Ocr},
};
use anyhow::Result;
use mongodb::{options::ClientOptions as MongoDBClientOptions, Client as MongoDBClient};
use slashook::main;
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
    spawn(Reminders::poll());
    println!("[REMINDERS] Started polling reminders.");

    // Spawn gateway client
    spawn(GatewayClient::new().create_shards());
    println!("[GATEWAY] Spawned client.");

    // Spawn ordr socket
    spawn(OrdrRender::connect());
    println!("[ORDR] Spawned socket client.");

    // Download OCR trained data
    Ocr::download_trained_data().await?;
    println!("[OCR] Downloaded all trained data.");

    let mut client = AeonClient::new();

    match client.register_commands().await {
        Ok(_) => println!("[CLIENT] Registered commands."),
        Err(error) => println!("[CLIENT] An error occurred while registering commands: {error}"),
    };

    client.start().await;

    Ok(())
}
