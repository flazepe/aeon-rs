use crate::{
    statics::{REQWEST, REST},
    structs::command_context::{AeonCommandContext, AeonCommandInput},
};
use anyhow::{Context, Result};
use slashook::{
    commands::MessageResponse,
    structs::{users::User, utils::File},
};
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };

    ctx.defer(false).await?;

    let user = User::fetch(&REST, &ctx.get_user_arg("user").unwrap_or(&input.user).id).await?; // Fetch the user again to make sure banner exists
    let user_id = &user.id;
    let user_banner_url =
        user.banner.as_ref().and_then(|banner| user.banner_url(if banner.starts_with("a_") { "gif" } else { "png" }, 4096));
    let guild_banner_url = input.guild_id.as_ref().and_then(|guild_id| {
        input.member.as_ref().and_then(|member| {
            member
                .banner
                .as_ref()
                .map(|banner| format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/banners/{banner}?size=4096"))
        })
    });
    let banner_url =
        if ctx.get_bool_arg("force-user-banner").unwrap_or(false) { user_banner_url } else { guild_banner_url.or(user_banner_url) }
            .context("User has no banner set.")?;

    ctx.respond(
        MessageResponse::from(format!(
            "{}<{banner_url}>",
            if banner_url.contains("guild") {
                "**Showing member's server banner**. To view member's user banner, set `force-user-banner` to `true`.\n"
            } else {
                ""
            },
        ))
        .add_file(File::new(
            format!("image.{}", if banner_url.contains("a_") { "gif" } else { "png" }),
            REQWEST.get(banner_url).send().await?.bytes().await?,
        )),
        false,
    )
    .await
}
