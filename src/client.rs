use crate::commands;
use crate::config::{self, Config};
use anyhow::Result;
use slashook::{Client, Config as SlashookConfig};

pub struct AeonClient {
    client: Client,
    config: Config,
}

impl AeonClient {
    pub fn new() -> Result<Self> {
        let config = config::load_config()?;

        let client = Client::new(SlashookConfig {
            bot_token: Some(String::from(&config.bot.token)),
            client_id: Some(String::from(&config.bot.client_id)),
            public_key: String::from(&config.bot.public_key),
            ..Default::default()
        });

        Ok(Self { config, client })
    }

    pub async fn register_commands(&mut self) -> Result<()> {
        commands::utils::Utils::init(&mut self.client).register();

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
