use crate::statics::CONFIG;
use anyhow::{Context, Result};
use redis::{AsyncTypedCommands, Client, HashFieldExpirationOptions, SetExpiry, aio::MultiplexedConnection};
use std::{collections::HashMap, fmt::Display};

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

    pub async fn hset<T: Display, U: Display, V: Display>(&self, key: T, field: U, value: V, ttl_secs: Option<u64>) -> Result<()> {
        let key = format!("{PREFIX}{key}");
        let field = field.to_string();
        let value = value.to_string();

        if let Some(ttl_secs) = ttl_secs {
            let hash_field_expiration_options = HashFieldExpirationOptions::default().set_expiration(SetExpiry::EX(ttl_secs));
            self.connection.clone().hset_ex(key, &hash_field_expiration_options, &[(field, value)]).await?;
        } else {
            self.connection.clone().hset(key, field, value).await?;
        }

        Ok(())
    }

    pub async fn hset_many<T: Display, U: IntoIterator<Item = (V, W)>, V: Display, W: Display>(
        &self,
        key: T,
        fields_values: U,
        ttl_secs: Option<u64>,
    ) -> Result<()> {
        let key = format!("{PREFIX}{key}");
        let fields_values = fields_values.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect::<Vec<(String, String)>>();

        if let Some(ttl_secs) = ttl_secs {
            let hash_field_expiration_options = HashFieldExpirationOptions::default().set_expiration(SetExpiry::EX(ttl_secs));
            self.connection.clone().hset_ex(key, &hash_field_expiration_options, &fields_values).await?;
        } else {
            self.connection.clone().hset_multiple(key, &fields_values).await?;
        }

        Ok(())
    }

    pub async fn hget_many<T: Display>(&self, key: T) -> Result<HashMap<String, String>> {
        let key = format!("{PREFIX}{key}");
        Ok(self.connection.clone().hgetall(key).await?)
    }
}
