use crate::{
    functions::{format_timestamp, now, TimestampFormat},
    statics::CACHE,
    structs::gateway::events::handler::EventHandler,
};
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
                    format!("[{}](https://cdn.discordapp.com/emojis/{id})", name.as_deref().unwrap_or("<unknown>")),
                ReactionType::Unicode { name } => name,
            },
            format_timestamp(now(), TimestampFormat::Full),
        ));

        // Limit
        while reactions.join("\n\n").len() > 4096 {
            reactions.rotate_left(1);
            reactions.pop();
        }
    }
}
