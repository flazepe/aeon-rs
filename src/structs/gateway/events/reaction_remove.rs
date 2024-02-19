use crate::{
    functions::{format_timestamp, TimestampFormat},
    statics::CACHE,
    structs::gateway::events::handler::EventHandler,
};
use std::time::{SystemTime, UNIX_EPOCH};
use twilight_model::{channel::message::ReactionType, gateway::payload::incoming::ReactionRemove};

impl EventHandler {
    pub async fn on_reaction_remove(reaction: Box<ReactionRemove>) {
        let reaction = reaction.0;

        let mut messages = CACHE.reaction_snipes.write().unwrap();
        let key = format!("{}/{}", reaction.channel_id, reaction.message_id);

        if !messages.contains_key(&key) {
            messages.insert(key.clone(), vec![]);
        }

        let reactions = messages.get_mut(&key).unwrap();

        reactions.push(format!(
            "<@{}> - {}\n{}",
            reaction.user_id,
            match reaction.emoji {
                ReactionType::Custom { name, id, animated: _ } =>
                    format!("[{}](https://cdn.discordapp.com/emojis/{})", name.unwrap_or("<unknown>".into()), id),
                ReactionType::Unicode { name } => name,
            },
            format_timestamp(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(), TimestampFormat::Full),
        ));

        // Limit
        while reactions.join("\n\n").len() > 4000 {
            reactions.rotate_left(1);
            reactions.pop();
        }
    }
}
