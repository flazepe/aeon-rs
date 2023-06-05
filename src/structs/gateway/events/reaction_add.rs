use crate::{
    statics::{CONFIG, REST},
    structs::gateway::events::handler::EventHandler,
};
use slashook::structs::channels::Message;
use twilight_model::{channel::message::ReactionType, gateway::payload::incoming::ReactionAdd};

impl EventHandler {
    pub async fn on_reaction_add(reaction: Box<ReactionAdd>) {
        let reaction = reaction.0;
        let Ok(message) = Message::fetch(&REST, reaction.channel_id,  reaction.message_id).await else { return; };
        let Some(interaction) = message.interaction.as_ref() else { return; };

        if message.author.id == CONFIG.bot.client_id
            && interaction.user.id == reaction.user_id.to_string()
            && ["🗑️", "❌", "🇽", "delete"].contains(
                &match reaction.emoji {
                    ReactionType::Custom { name, animated: _, id: _ } => name.unwrap_or("".into()),
                    ReactionType::Unicode { name } => name,
                }
                .as_str(),
            )
        {
            message.delete(&REST).await.ok();
        }
    }
}
