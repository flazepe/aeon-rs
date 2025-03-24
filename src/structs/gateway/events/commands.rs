use crate::{
    statics::CONFIG,
    structs::{
        database::guilds::Guilds,
        gateway::{commands, events::EventHandler},
    },
};
use anyhow::Result;
use twilight_gateway::MessageSender;
use twilight_model::gateway::payload::incoming::MessageCreate;

impl EventHandler {
    pub async fn handle_commands(event: &MessageCreate, sender: &MessageSender) -> Result<()> {
        let Some(guild_id) = &event.guild_id else { return Ok(()) };
        let guild = Guilds::get(guild_id).await?;
        let mention_prefix = format!("<@{}>", CONFIG.bot.client_id);
        let prefix = match guild.prefixes.iter().find(|prefix| event.content.to_lowercase().starts_with(*prefix)) {
            Some(prefix) => prefix,
            None => {
                if event.content.starts_with(&mention_prefix) {
                    &mention_prefix
                } else {
                    return Ok(());
                }
            },
        };
        let prefixless = event.content.chars().skip(prefix.len()).collect::<String>().trim().to_string();
        let (command, args) = prefixless.split_once(' ').unwrap_or((&prefixless, ""));

        commands::run(event, sender, command, args.trim()).await
    }
}
