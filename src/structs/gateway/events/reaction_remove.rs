use crate::{statics::*, structs::gateway::events::handler::EventHandler, *};
use std::time::{SystemTime, UNIX_EPOCH};
use twilight_model::{channel::message::ReactionType, gateway::payload::incoming::ReactionRemove};

impl EventHandler {
    pub fn on_reaction_remove(reaction: Box<ReactionRemove>) {
        let reaction = reaction.0;

        if let Some(guild_id) = reaction.guild_id {
            let mut cache = CACHE.lock().unwrap();
            let messages = &mut cache.reaction_snipes;
            let key = format!("{}/{}", guild_id.to_string(), reaction.message_id);

            if !messages.contains_key(&key) {
                messages.insert(key.clone(), vec![]);
            }

            let reactions = messages.get_mut(&key).unwrap();

            reactions.push(format!(
                "<@{}> - {}\n{}",
                reaction.user_id,
                match reaction.emoji {
                    ReactionType::Custom { name, id, animated } => {
                        format!(
                            "[{}](https://cdn.discordapp.com/emojis/{}.{})",
                            name.unwrap_or("<deleted>".into()),
                            id,
                            if_else!(animated, "gif", "png")
                        )
                    }
                    ReactionType::Unicode { name } => {
                        name
                    }
                },
                format_timestamp!(SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs())
            ));

            // Limit
            while reactions.join("\n\n").len() > 4000 {
                reactions.rotate_left(1);
                reactions.pop();
            }
        }
    }
}