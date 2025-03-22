use crate::statics::colors::NOTICE_COLOR;
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::{
    gateway::payload::incoming::MemberUpdate,
    id::{marker::GuildMarker, Id},
};

pub async fn log(event: &MemberUpdate) -> Result<(Option<Id<GuildMarker>>, Option<Embed>)> {
    let embed = Embed::new()
        .set_color(NOTICE_COLOR)
        .unwrap_or_default()
        .set_title("Member Updated")
        .set_description(format!("<@{}>", event.user.id))
        .add_field("Username", format!("{} ({})", event.user.name, event.user.id), false);

    Ok((event.guild_id.into(), embed.into()))
}
