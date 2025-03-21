use crate::{
    statics::{CACHE, CONFIG, FLAZEPE_ID, REST},
    structs::gateway::events::handler::EventHandler,
};
use anyhow::Result;
use slashook::structs::messages::Message;
use twilight_model::{channel::message::EmojiReactionType, gateway::payload::incoming::ReactionAdd, id::Id};

impl EventHandler {
    pub async fn on_reaction_add(reaction: Box<ReactionAdd>) -> Result<()> {
        let reaction = reaction.0;
        let reaction_emoji_name = match reaction.emoji {
            EmojiReactionType::Custom { name, animated: _, id: _ } => name,
            EmojiReactionType::Unicode { name } => Some(name),
        };

        if !["🗑️", "❌", "🇽", "delete"].contains(&reaction_emoji_name.as_deref().unwrap_or("")) {
            return Ok(());
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

                if let Some(interaction_metadata) = message.interaction_metadata.as_ref() {
                    if interaction_metadata.id != Id::new(1202934262123470899) {
                        user_id = Some(interaction_metadata.user.id.to_string());
                    }
                }
            }
        }

        // Fetch message if not in cache
        if author_id.is_none() || user_id.is_none() {
            let Ok(message) = Message::fetch(&REST, reaction.channel_id, reaction.message_id).await else { return Ok(()) };
            author_id = Some(message.author.id);

            if let Some(interaction_metadata) = message.interaction_metadata {
                if interaction_metadata.id != "1202934262123470899" {
                    user_id = Some(interaction_metadata.user.id);
                }
            }
        }

        let Some(author_id) = author_id else { return Ok(()) };

        if user_id.as_deref().unwrap_or(FLAZEPE_ID) == reaction.user_id.to_string() && author_id == CONFIG.bot.client_id {
            let channel_id = reaction.channel_id;
            let message_id = reaction.message_id;
            let _ = REST.delete::<()>(format!("channels/{channel_id}/messages/{message_id}")).await;
        }

        Ok(())
    }
}
