use crate::statics::{CONFIG, FLAZEPE_ID, REDIS, REST};
use anyhow::{Error, Result};
use serde_json::from_str;
use slashook::structs::messages::Message as SlashookMessage;
use twilight_model::{
    channel::Message as TwilightMessage, channel::message::EmojiReactionType, gateway::payload::incoming::ReactionAdd, id::Id,
};

pub async fn handle(event: &ReactionAdd) -> Result<()> {
    let reaction = event.0.clone();

    let reaction_emoji_name = match reaction.emoji {
        EmojiReactionType::Custom { name, .. } => name,
        EmojiReactionType::Unicode { name } => Some(name),
    };

    if !["🗑️", "❌", "🇽", "delete"].contains(&reaction_emoji_name.as_deref().unwrap_or_default()) {
        return Ok(());
    }

    let Some(guild_id) = event.guild_id else { return Ok(()) };
    let channel_id = event.channel_id;
    let message_id = event.message_id;

    let mut author_id = None;
    let mut user_id = None;

    let message = REDIS
        .get()
        .unwrap()
        .get(format!("guilds_{guild_id}_channels_{channel_id}_messages_{message_id}"))
        .await
        .and_then(|message| from_str::<TwilightMessage>(&message).map_err(Error::msg));

    if let Ok(message) = message {
        author_id = Some(message.author.id.to_string());

        if let Some(interaction_metadata) = message.interaction_metadata.as_ref()
            && interaction_metadata.id != Id::new(1202934262123470899)
        {
            user_id = Some(interaction_metadata.user.id.to_string());
        }
    }

    if author_id.is_none() || user_id.is_none() {
        let Ok(message) = SlashookMessage::fetch(&REST, reaction.channel_id, reaction.message_id).await else { return Ok(()) };
        author_id = message.author.map(|author| author.id);

        if let Some(interaction_metadata) = message.interaction_metadata
            && interaction_metadata.id != "1202934262123470899"
        {
            user_id = Some(interaction_metadata.user.id);
        }
    }

    let Some(author_id) = author_id else { return Ok(()) };

    if user_id.as_deref().unwrap_or(FLAZEPE_ID) == reaction.user_id.to_string() && author_id == CONFIG.bot.client_id {
        _ = REST.delete::<()>(format!("channels/{channel_id}/messages/{message_id}")).await;
    }

    Ok(())
}
