use crate::{
    statics::{CACHE, CONFIG, FLAZEPE_ID, REST},
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
                author_id = Some(message.author.id.to_string());

                if let Some(interaction) = message.interaction.as_ref() {
                    if !message.content.contains("voice message") {
                        user_id = Some(interaction.user.id.to_string());
                    }
                }
            }
        }

        // Fetch message if not in cache
        if author_id.is_none() || user_id.is_none() {
            let Ok(message) = Message::fetch(&REST, reaction.channel_id, reaction.message_id).await else { return };
            author_id = Some(message.author.id);

            if let Some(interaction) = message.interaction {
                if !message.content.contains("voice message") {
                    user_id = Some(interaction.user.id);
                }
            }
        }

        let Some(author_id) = author_id else { return };

        if user_id.unwrap_or(FLAZEPE_ID.to_string()) == reaction.user_id.to_string() && author_id == CONFIG.bot.client_id {
            REST.delete::<()>(format!("channels/{}/messages/{}", reaction.channel_id, reaction.message_id)).await.ok();
        }
    }
}
