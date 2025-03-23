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
mod voice_state_update;

use crate::structs::gateway::events::EventHandler;
use anyhow::Result;
use twilight_gateway::Event;

impl EventHandler {
    pub async fn handle_logs(event: &Event) -> Result<()> {
        match event {
            Event::BanAdd(event) => ban_add::handle(event).await,
            Event::BanRemove(event) => ban_remove::handle(event).await,
            Event::ChannelCreate(event) => channel_create::handle(event).await,
            Event::ChannelDelete(event) => channel_delete::handle(event).await,
            Event::ChannelPinsUpdate(event) => channel_pins_update::handle(event).await,
            Event::ChannelUpdate(event) => channel_update::handle(event).await,
            Event::GuildEmojisUpdate(event) => guild_emojis_update::handle(event).await,
            Event::GuildStickersUpdate(event) => guild_stickers_update::handle(event).await,
            Event::GuildUpdate(event) => guild_update::handle(event).await,
            Event::InviteCreate(event) => invite_create::handle(event).await,
            Event::InviteDelete(event) => invite_delete::handle(event).await,
            Event::MemberAdd(event) => member_add::handle(event).await,
            Event::MemberRemove(event) => member_remove::handle(event).await,
            Event::MemberUpdate(event) => member_update::handle(event).await,
            Event::MessageDelete(event) => message_delete::handle(event).await,
            Event::MessageDeleteBulk(event) => message_delete_bulk::handle(event).await,
            Event::MessageUpdate(event) => message_update::handle(event).await,
            Event::ReactionRemoveAll(event) => reaction_remove_all::handle(event).await,
            Event::ReactionRemoveEmoji(event) => reaction_remove_emoji::handle(event).await,
            Event::RoleCreate(event) => role_create::handle(event).await,
            Event::RoleDelete(event) => role_delete::handle(event).await,
            Event::RoleUpdate(event) => role_update::handle(event).await,
            Event::ThreadCreate(event) => thread_create::handle(event).await,
            Event::ThreadDelete(event) => thread_delete::handle(event).await,
            Event::ThreadUpdate(event) => thread_update::handle(event).await,
            Event::VoiceStateUpdate(event) => voice_state_update::handle(event).await,
            _ => Ok(()),
        }
    }
}
