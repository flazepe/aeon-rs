use crate::{
    statics::{MONGODB, REDIS},
    structs::database::{mongodb::MongoDB, redis::Redis},
};
use anyhow::{Context, Result};
use tracing::info;

pub mod mongodb;
pub mod redis;

pub struct Database;

impl Database {
    pub async fn init() -> Result<()> {
        MONGODB.set(MongoDB::new().await?).expect("Could not set MongoDB.");
        info!(target: "Database", "Connected to MongoDB.");

        REDIS.set(Redis::new().await?).expect("Could not set Redis.");
        info!(target: "Database", "Connected to Redis.");

        info!(target: "Database", "Initialized.");

        Ok(())
    }

    pub fn get_mongodb() -> Result<&'static MongoDB> {
        MONGODB.get().context("Could not get MongoDB.")
    }

    pub fn get_redis() -> Result<&'static Redis> {
        REDIS.get().context("Could not get Redis.")
    }
}
