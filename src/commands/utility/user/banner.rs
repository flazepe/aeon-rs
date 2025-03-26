use crate::{
    statics::REQWEST,
    structs::command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt},
};
use anyhow::{Context, Result};
use serde::Deserialize;
use slashook::{commands::MessageResponse, structs::utils::File};
use std::sync::Arc;

#[derive(Deserialize, Debug)]
struct UserBanner {
    banner: Option<String>,
}

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };

    ctx.defer(false).await?;

    let user = input.get_user_arg("user").unwrap_or(&input.user);
    let user_id = &user.id;
    let banner_url = input
        .rest
        .get::<UserBanner>(format!("users/{user_id}"))
        .await?
        .banner
        .map(|banner| format!("https://cdn.discordapp.com/banners/{user_id}/{banner}?size=4096"))
        .context("User has no banner set.")?;

    ctx.respond(
        MessageResponse::from(format!("<{banner_url}>")).add_file(File::new(
            format!("image.{}", if banner_url.contains("a_") { "gif" } else { "png" }),
            REQWEST.get(banner_url).send().await?.bytes().await?,
        )),
        false,
    )
    .await
}
