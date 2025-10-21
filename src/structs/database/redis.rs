use crate::statics::CONFIG;
use anyhow::{Context, Result};
use redis::{AsyncTypedCommands, Client, aio::MultiplexedConnection};
use std::fmt::Display;

static PREFIX: &str = "aeon_";

#[derive(Debug)]
pub struct Redis {
    connection: MultiplexedConnection,
}

impl Redis {
    pub async fn new() -> Result<Self> {
        let client = Client::open(CONFIG.database.redis_uri.as_str())?;
        let connection = client.get_multiplexed_async_connection().await?;

        Ok(Self { connection })
    }

    pub async fn set<T: Display, U: Display>(&self, key: T, value: U, ttl_secs: Option<u64>) -> Result<()> {
        let key = format!("{PREFIX}{key}");

        if let Some(ttl_secs) = ttl_secs {
            self.connection.clone().set_ex(key, value.to_string(), ttl_secs).await?;
        } else {
            self.connection.clone().set(key, value.to_string()).await?;
        }

        Ok(())
    }

    pub async fn get<T: Display>(&self, key: T) -> Result<String> {
        self.connection.clone().get(format!("{PREFIX}{key}")).await?.context("Key not found")
    }

    pub async fn get_many<T: Display>(&self, keys: Vec<T>) -> Result<Vec<String>> {
        let keys = keys.into_iter().map(|key| format!("{PREFIX}{key}")).collect::<Vec<String>>();
        Ok(self.connection.clone().mget(keys).await?.into_iter().flatten().collect())
    }

    pub async fn delete<T: Display>(&self, key: T) -> Result<usize> {
        Ok(self.connection.clone().del(format!("{PREFIX}{key}")).await?)
    }

    pub async fn delete_many<T: Display>(&self, keys: Vec<T>) -> Result<usize> {
        let keys = keys.into_iter().map(|key| format!("{PREFIX}{key}")).collect::<Vec<String>>();
        Ok(self.connection.clone().del(keys).await?)
    }
}
