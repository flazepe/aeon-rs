use crate::{statics::colors::SUCCESS_EMBED_COLOR, structs::database::guilds::Guilds};
use anyhow::Result;
use slashook::{chrono::Utc, structs::embeds::Embed};
use twilight_model::gateway::payload::incoming::BanRemove;

pub async fn handle(event: &BanRemove) -> Result<()> {
    let embed = Embed::new()
        .set_color(SUCCESS_EMBED_COLOR)
        .unwrap_or_default()
        .set_title("User Unbanned")
        .set_description(format!("<@{}>", event.user.id))
        .add_field("Username", format!("{} ({})", event.user.name, event.user.id), false)
        .set_timestamp(Utc::now());

    Guilds::send_log(event.guild_id, embed).await
}
