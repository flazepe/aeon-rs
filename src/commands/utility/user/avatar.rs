use crate::{
    statics::REQWEST,
    structs::command_context::{AeonCommandContext, AeonCommandInput},
    traits::UserExt,
};
use anyhow::Result;
use slashook::{commands::MessageResponse, structs::utils::File};
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };

    ctx.defer(false).await?;

    let user = ctx.get_user_arg("user").unwrap_or(&input.user);
    let user_id = &user.id;
    let user_avatar_url = user.display_avatar_url("gif", 4096);
    let guild_avatar_url = input.guild_id.as_ref().and_then(|guild_id| {
        input.member.as_ref().and_then(|member| {
            member
                .avatar
                .as_ref()
                .map(|avatar| format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{avatar}?size=4096"))
        })
    });
    let avatar_url =
        if ctx.get_bool_arg("force-user-avatar").unwrap_or(false) { user_avatar_url } else { guild_avatar_url.unwrap_or(user_avatar_url) };

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
