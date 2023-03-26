use crate::{commands::get_commands, statics::CONFIG};
use anyhow::Result;
use slashook::{Client as SlashookClient, Config as SlashookConfig};

pub struct AeonClient {
    pub slashook: SlashookClient,
}

impl AeonClient {
    pub fn new() -> Self {
        Self {
            slashook: SlashookClient::new(SlashookConfig {
                bot_token: Some(String::from(&CONFIG.bot.token)),
                client_id: Some(String::from(&CONFIG.bot.client_id)),
                public_key: String::from(&CONFIG.bot.public_key),
                port: 2931,
                ..Default::default()
            }),
        }
    }

    pub async fn register_commands(&mut self) -> Result<()> {
        self.slashook.register_commands(get_commands());

        if let Some(guild_id) = &CONFIG.bot.guild_id {
            self.slashook.sync_guild_commands(&guild_id).await?;
        } else {
            self.slashook.sync_commands().await?;
        }

        Ok(())
    }

    pub async fn start(self) {
        self.slashook.start().await;
    }
}
