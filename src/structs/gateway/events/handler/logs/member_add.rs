use crate::statics::colors::SUCCESS_COLOR;
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::{
    gateway::payload::incoming::MemberAdd,
    id::{marker::GuildMarker, Id},
};

pub async fn log(event: &MemberAdd) -> Result<(Option<Id<GuildMarker>>, Embed)> {
    let embed = Embed::new()
        .set_color(SUCCESS_COLOR)
        .unwrap_or_default()
        .set_title("Member Joined")
        .set_description(format!("<@{}>", event.user.id))
        .add_field("Username", format!("{} ({})", event.user.name, event.user.id), false);

    Ok((event.guild_id.into(), embed))
}
