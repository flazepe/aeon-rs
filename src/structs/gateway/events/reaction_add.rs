use crate::{
    statics::{CACHE, CONFIG, REST},
    structs::gateway::events::handler::EventHandler,
};
use slashook::structs::channels::Message;
use twilight_model::{channel::message::ReactionType, gateway::payload::incoming::ReactionAdd};

impl EventHandler {
    pub async fn on_reaction_add(reaction: Box<ReactionAdd>) {
        let reaction = reaction.0;

        if !["🗑️", "❌", "🇽", "delete"].contains(
            &match reaction.emoji {
                ReactionType::Custom { name, animated: _, id: _ } => name.unwrap_or("".into()),
                ReactionType::Unicode { name } => name,
            }
            .as_str(),
        ) {
            return;
        }

        let mut author_id = None;
        let mut user_id = None;

        // Find message in cache
        {
            let channels = CACHE.channels.read().unwrap();

            if let Some(message) = channels
                .get(&reaction.channel_id.to_string())
                .and_then(|messages| messages.iter().find(|message| message.id == reaction.message_id))
            {
                let Some(interaction) = message.interaction.as_ref() else {
                    return;
                };

                author_id = Some(message.author.id.to_string());
                user_id = Some(interaction.user.id.to_string());
            }
        }

        // Fetch message if not in cache
        if author_id.is_none() || user_id.is_none() {
            let Ok(message) = Message::fetch(&REST, reaction.channel_id, reaction.message_id).await else {
                return;
            };

            let Some(interaction) = message.interaction else {
                return;
            };

            author_id = Some(message.author.id);
            user_id = Some(interaction.user.id);
        }

        if author_id.unwrap() == CONFIG.bot.client_id && user_id.unwrap() == reaction.user_id.to_string() {
            REST.delete::<()>(format!("channels/{}/messages/{}", reaction.channel_id, reaction.message_id)).await.ok();
        }
    }
}
