use crate::{commands, structs::config::*};
use anyhow::Result;
use slashook::{Client, Config as SlashookConfig};
use std::fs::read_to_string;
use toml::from_str;

pub struct AeonClient {
    client: Client,
    config: Config,
}

impl AeonClient {
    pub fn new() -> Result<Self> {
        let config: Config = from_str(&read_to_string("config.toml")?)?;

        let client = Client::new(SlashookConfig {
            bot_token: Some(String::from(&config.bot.token)),
            client_id: Some(String::from(&config.bot.client_id)),
            public_key: String::from(&config.bot.public_key),
            ..Default::default()
        });

        Ok(Self { config, client })
    }

    pub async fn register_commands(&mut self) -> Result<()> {
        self.client
            .register_commands(commands::utils::Utils::init().get_commands());

        if let Some(guild_id) = &self.config.bot.guild_id {
            self.client.sync_guild_commands(&guild_id).await?;
        } else {
            self.client.sync_commands().await?;
        }

        Ok(())
    }

    pub async fn start(self) {
        self.client.start().await;
    }
}
