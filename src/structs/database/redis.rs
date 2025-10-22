use crate::statics::CONFIG;
use anyhow::{Context, Result};
use redis::{AsyncTypedCommands, Client, HashFieldExpirationOptions, SetExpiry, aio::MultiplexedConnection};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{from_str, to_string};
use std::{collections::BTreeMap, fmt::Display};

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

    pub async fn set<T: Display, U: Serialize>(&self, key: T, value: U, ttl_secs: Option<u64>) -> Result<()> {
        let key = format!("{PREFIX}{key}");

        if let Some(ttl_secs) = ttl_secs {
            self.connection.clone().set_ex(key, to_string(&value)?, ttl_secs).await?;
        } else {
            self.connection.clone().set(key, to_string(&value)?).await?;
        }

        Ok(())
    }

    pub async fn get<T: DeserializeOwned>(&self, key: impl Display) -> Result<T> {
        self.connection
            .clone()
            .get(format!("{PREFIX}{key}"))
            .await?
            .context("Key not found")
            .and_then(|data| from_str(&data).context("Could not deserialize data"))
    }

    pub async fn get_many<T: DeserializeOwned>(&self, keys: Vec<impl Display>) -> Result<Vec<T>> {
        let keys = keys.into_iter().map(|key| format!("{PREFIX}{key}")).collect::<Vec<String>>();
        Ok(self.connection.clone().mget(keys).await?.into_iter().flat_map(|data| from_str(&data?).ok()).collect())
    }

    pub async fn hset<T: Serialize>(&self, key: impl Display, field: impl Display, value: T, ttl_secs: Option<u64>) -> Result<()> {
        let key = format!("{PREFIX}{key}");
        let field = field.to_string();
        let value = to_string(&value)?;

        if let Some(ttl_secs) = ttl_secs {
            let hash_field_expiration_options = HashFieldExpirationOptions::default().set_expiration(SetExpiry::EX(ttl_secs));
            self.connection.clone().hset_ex(key, &hash_field_expiration_options, &[(field, value)]).await?;
        } else {
            self.connection.clone().hset(key, field, value).await?;
        }

        Ok(())
    }

    pub async fn hset_many<T: Serialize, U: Serialize>(
        &self,
        key: impl Display,
        fields_values: impl IntoIterator<Item = (T, U)>,
        ttl_secs: Option<u64>,
    ) -> Result<()> {
        let key = format!("{PREFIX}{key}");
        let fields_values = fields_values
            .into_iter()
            .flat_map(|(k, v)| {
                let k = to_string(&k).ok()?;
                let v = to_string(&v).ok()?;
                Some((k, v))
            })
            .collect::<Vec<(String, String)>>();

        if let Some(ttl_secs) = ttl_secs {
            let hash_field_expiration_options = HashFieldExpirationOptions::default().set_expiration(SetExpiry::EX(ttl_secs));
            self.connection.clone().hset_ex(key, &hash_field_expiration_options, &fields_values).await?;
        } else {
            self.connection.clone().hset_multiple(key, &fields_values).await?;
        }

        Ok(())
    }

    pub async fn hget_many<T: DeserializeOwned + Ord, U: DeserializeOwned>(&self, key: impl Display) -> Result<BTreeMap<T, U>> {
        let key = format!("{PREFIX}{key}");
        Ok(self.connection.clone().hgetall(key).await?).map(|data| {
            BTreeMap::from_iter(data.into_iter().flat_map(|(k, v)| {
                let k = from_str(&k).ok()?;
                let v = from_str(&v).ok()?;
                Some((k, v))
            }))
        })
    }
}
