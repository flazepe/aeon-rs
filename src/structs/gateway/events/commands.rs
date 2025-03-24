use crate::{
    commands,
    statics::CONFIG,
    structs::{database::guilds::Guilds, gateway::events::EventHandler},
};
use anyhow::Result;
use twilight_gateway::MessageSender;
use twilight_model::channel::Message;

impl EventHandler {
    pub async fn handle_commands(message: &Message, sender: &MessageSender) -> Result<()> {
        let Some(guild_id) = &message.guild_id else { return Ok(()) };
        let guild = Guilds::get(guild_id).await?;

        let lowercased_content = message.content.to_lowercase();
        let default_prefixes = [format!("<@{}>", CONFIG.bot.client_id), "aeon".into()];
        let Some(prefix) = guild
            .prefixes
            .iter()
            .find(|prefix| lowercased_content.starts_with(*prefix))
            .or(default_prefixes.iter().find(|prefix| lowercased_content.starts_with(*prefix)))
        else {
            return Ok(());
        };

        let prefixless = message.content.chars().skip(prefix.len()).collect::<String>().trim().to_string();
        let (command, args) = prefixless.split_once(' ').unwrap_or((&prefixless, ""));

        commands::run(message, sender, command, args.trim()).await
    }
}
