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

    /*
        We have to fetch the user directly from the API to make sure that the banner value is populated.

        Weirdly enough, the resolved object from the application command data still does not have the
        user banner populated (it does for member banners though...), so we can't use that.
    */
    let user = User::fetch(&REST, &ctx.get_user_arg("user").unwrap_or(&input.user).id).await?;
    let user_id = &user.id;

    let user_banner_url = user.banner_url("png", Some("gif"), 4096);

    /*
        The only realistic way to get the member banner is by using the resolved object from the application command data.
        We can't directly fetch the guild member object from the API anyway if the bot isn't in the server, so this method is better.
    */
    let member_banner_url = input.guild_id.as_ref().and_then(|guild_id| {
        input
            .as_ref()
            .resolved
            .as_ref()
            .and_then(|resolved| {
                resolved.members.as_ref().and_then(|members| {
                    members.get(user_id).and_then(|member| member.banner_url(guild_id, user_id, "png", Some("gif"), 4096))
                })
            })
            .or_else(|| {
                if &input.user.id == user_id {
                    input.member.as_ref().and_then(|member| member.banner_url(guild_id, user_id, "png", Some("gif"), 4096))
                } else {
                    None
                }
            })
    });

    let banner_url =
        if ctx.get_bool_arg("force-main-banner").unwrap_or(false) { user_banner_url } else { member_banner_url.or(user_banner_url) }
            .context("User has no banner set.")?;

    ctx.respond(
        MessageResponse::from(format!(
            "{}<{banner_url}>",
            if banner_url.contains("/guilds/") {
                "-# **Showing user's per-server banner**. To view user's main banner, set `force-main-banner` to `true`.\n"
            } else {
                ""
            },
        ))
        .add_file(File::new(
            format!("image.{}", if banner_url.contains("/a_") { "gif" } else { "png" }),
            REQWEST.get(banner_url).send().await?.bytes().await?,
        )),
        false,
    )
    .await
}
