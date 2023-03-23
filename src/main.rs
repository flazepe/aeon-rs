mod commands;
mod constants;
mod macros;
mod structs;
mod traits;

use crate::{
    constants::*,
    structs::{client::AeonClient, gateway::client::GatewayClient},
};
use anyhow::Result;
use mongodb::{options::ClientOptions as MongoDBClientOptions, Client as MongoDBClient};
use slashook::main;
use tokio::spawn;

#[main]
async fn main() -> Result<()> {
    MONGODB
        .get_or_init(async {
            MongoDBClient::with_options(
                MongoDBClientOptions::parse(&CONFIG.db.mongodb_uri)
                    .await
                    .unwrap(),
            )
            .unwrap()
            .database("aeon")
        })
        .await;

    // Spawn gateway client
    spawn(GatewayClient::new().create_shards());

    let mut client = AeonClient::new();

    client.register_commands().await?;
    client.start().await;

    Ok(())
}
