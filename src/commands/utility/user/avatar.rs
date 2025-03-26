use crate::{
    statics::REQWEST,
    structs::command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt},
    traits::UserExt,
};
use anyhow::Result;
use slashook::{
    commands::MessageResponse,
    structs::{guilds::GuildMember, utils::File},
};
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };

    ctx.defer(false).await?;

    let user = input.get_user_arg("user").unwrap_or(&input.user);
    let user_id = &user.id;

    let guild_avatar = match input.guild_id.as_ref() {
        Some(guild_id) => input.rest.get::<GuildMember>(format!("guilds/{guild_id}/members/{user_id}")).await.ok().and_then(|member| {
            member.avatar.map(|avatar| format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{avatar}?size=4096"))
        }),
        None => None,
    };

    let avatar_url = if input.get_bool_arg("force-user-avatar").unwrap_or(false) {
        user.display_avatar_url("gif", 4096)
    } else {
        guild_avatar.unwrap_or_else(|| user.display_avatar_url("gif", 4096))
    };

    ctx.respond(
        MessageResponse::from(format!(
            "{}<{avatar_url}>",
            if avatar_url.contains("guild") {
                "**Showing member's server avatar**. To view member's user avatar, set `force-user-avatar` to `true`.\n"
            } else {
                ""
            },
        ))
        .add_file(File::new(
            format!("image.{}", if avatar_url.contains("a_") { "gif" } else { "png" }),
            REQWEST.get(avatar_url).send().await?.bytes().await?,
        )),
        false,
    )
    .await
}
