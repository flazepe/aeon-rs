use crate::{commands::get_commands, structs::config::CONFIG};
use anyhow::Result;
use slashook::{Client, Config as SlashookConfig};

pub struct AeonClient {
    pub client: Client,
}

impl AeonClient {
    pub fn new() -> Result<Self> {
        let client = Client::new(SlashookConfig {
            bot_token: Some(String::from(&CONFIG.bot.token)),
            client_id: Some(String::from(&CONFIG.bot.client_id)),
            public_key: String::from(&CONFIG.bot.public_key),
            ..Default::default()
        });

        Ok(Self { client })
    }

    pub async fn register_commands(&mut self) -> Result<()> {
        self.client.register_commands(get_commands());

        if let Some(guild_id) = &CONFIG.bot.guild_id {
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
