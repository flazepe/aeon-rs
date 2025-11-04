use crate::{commands::get_slashook_commands, statics::CONFIG};
use anyhow::Result;
use slashook::{Client as SlashookClient, Config as SlashookConfig, structs::interactions::ApplicationCommand};

pub struct AeonClient {
    slashook: SlashookClient,
}

impl AeonClient {
    pub fn new() -> Self {
        Self {
            slashook: SlashookClient::new(SlashookConfig {
                bot_token: Some(CONFIG.bot.token.clone()),
                client_id: Some(CONFIG.bot.client_id.clone()),
                public_key: CONFIG.bot.public_key.clone(),
                port: 2931,
                ..Default::default()
            }),
        }
    }

    pub async fn register_commands(&mut self) -> Result<Vec<ApplicationCommand>> {
        self.slashook.register_commands(get_slashook_commands());

        Ok(match CONFIG.bot.guild_id.as_ref() {
            Some(guild_id) => self.slashook.sync_guild_commands(guild_id).await?,
            None => self.slashook.sync_commands().await?,
        })
    }

    pub async fn start(self) {
        self.slashook.start().await;
    }
}
