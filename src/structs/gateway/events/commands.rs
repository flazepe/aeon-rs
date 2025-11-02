use crate::{
    commands,
    statics::{DEFAULT_PREFIXES, MONGODB},
    structs::gateway::events::EventHandler,
};
use anyhow::Result;
use twilight_gateway::MessageSender;
use twilight_model::channel::Message;

impl EventHandler {
    pub async fn handle_commands(message: &Message, sender: &MessageSender) -> Result<()> {
        let lowercased_content = message.content.to_lowercase();
        let prefixes = match &message.guild_id {
            Some(guild_id) => {
                let mongodb = MONGODB.get().unwrap();
                mongodb.guilds.get(guild_id).await?.prefixes
            },
            None => vec![],
        };
        let Some(prefix) = prefixes
            .iter()
            .find(|prefix| lowercased_content.starts_with(*prefix))
            .or(DEFAULT_PREFIXES.iter().find(|prefix| lowercased_content.starts_with(*prefix)))
        else {
            return Ok(());
        };
        let prefixless = message.content.chars().skip(prefix.len()).skip_while(|char| char.is_whitespace()).collect::<String>();
        let (command, content) = prefixless.split_once(char::is_whitespace).unwrap_or((&prefixless, ""));

        commands::run(message, sender, command.to_lowercase(), content.trim()).await
    }
}
