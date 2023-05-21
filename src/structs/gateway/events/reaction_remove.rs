use crate::{
    functions::{format_timestamp, TimestampFormat},
    statics::CACHE,
    structs::gateway::events::handler::EventHandler,
};
use std::time::{SystemTime, UNIX_EPOCH};
use twilight_model::{channel::message::ReactionType, gateway::payload::incoming::ReactionRemove};

impl EventHandler {
    pub fn on_reaction_remove(reaction: Box<ReactionRemove>) {
        let reaction = reaction.0;

        if let Some(guild_id) = reaction.guild_id {
            let mut messages = CACHE.reaction_snipes.write().unwrap();
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
                            match animated {
                                true => "gif",
                                false => "png",
                            },
                        )
                    },
                    ReactionType::Unicode { name } => {
                        name
                    },
                },
                format_timestamp(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(), TimestampFormat::Full)
            ));

            // Limit
            while reactions.join("\n\n").len() > 4000 {
                reactions.rotate_left(1);
                reactions.pop();
            }
        }
    }
}
