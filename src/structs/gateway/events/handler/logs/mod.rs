mod ban_add;
mod ban_remove;
mod channel_create;
mod channel_delete;
mod channel_pins_update;
mod channel_update;
mod guild_emojis_update;
mod guild_stickers_update;
mod guild_update;
mod invite_create;
mod invite_delete;
mod member_add;
mod member_remove;
mod member_update;
mod message_delete;
mod message_delete_bulk;
mod message_update;
mod reaction_remove_all;
mod reaction_remove_emoji;
mod role_create;
mod role_delete;
mod role_update;
mod thread_create;
mod thread_delete;
mod thread_update;

use crate::{
    statics::REST,
    structs::{database::guilds::Guilds, gateway::events::handler::EventHandler},
};
use anyhow::Result;
use slashook::structs::messages::Message;
use twilight_gateway::Event;

impl EventHandler {
    pub async fn handle_logs(event: &Event) -> Result<()> {
        let (guild_id, embed) = match event {
            Event::BanAdd(event) => ban_add::log(event).await?,
            Event::BanRemove(event) => ban_remove::log(event).await?,
            Event::ChannelCreate(event) => channel_create::log(event).await?,
            Event::ChannelDelete(event) => channel_delete::log(event).await?,
            Event::ChannelPinsUpdate(event) => channel_pins_update::log(event).await?,
            Event::ChannelUpdate(event) => channel_update::log(event).await?,
            Event::GuildEmojisUpdate(event) => guild_emojis_update::log(event).await?,
            Event::GuildStickersUpdate(event) => guild_stickers_update::log(event).await?,
            Event::GuildUpdate(event) => guild_update::log(event).await?,
            Event::InviteCreate(event) => invite_create::log(event).await?,
            Event::InviteDelete(event) => invite_delete::log(event).await?,
            Event::MemberAdd(event) => member_add::log(event).await?,
            Event::MemberRemove(event) => member_remove::log(event).await?,
            Event::MemberUpdate(event) => member_update::log(event).await?,
            Event::MessageDelete(event) => message_delete::log(event).await?,
            Event::MessageDeleteBulk(event) => message_delete_bulk::log(event).await?,
            Event::MessageUpdate(event) => message_update::log(event).await?,
            Event::ReactionRemoveAll(event) => reaction_remove_all::log(event).await?,
            Event::ReactionRemoveEmoji(event) => reaction_remove_emoji::log(event).await?,
            Event::RoleCreate(event) => role_create::log(event).await?,
            Event::RoleDelete(event) => role_delete::log(event).await?,
            Event::RoleUpdate(event) => role_update::log(event).await?,
            Event::ThreadCreate(event) => thread_create::log(event).await?,
            Event::ThreadDelete(event) => thread_delete::log(event).await?,
            Event::ThreadUpdate(event) => thread_update::log(event).await?,
            _ => return Ok(()),
        };

        let (Some(guild_id), Some(embed)) = (guild_id, embed) else { return Ok(()) };
        let guild = Guilds::get(guild_id).await?;
        let Some(logs_channel_id) = guild.logs_channel_id else { return Ok(()) };
        let _ = Message::create(&REST, logs_channel_id, embed).await;

        Ok(())
    }
}
