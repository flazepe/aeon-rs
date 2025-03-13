use crate::{statics::REQWEST, structs::command_context::CommandContext};
use anyhow::Result;
use serde::Deserialize;
use slashook::{commands::MessageResponse, structs::utils::File};

#[derive(Deserialize, Debug)]
struct UserBanner {
    banner: Option<String>,
}

pub async fn run(ctx: CommandContext) -> Result<()> {
    ctx.defer(false).await?;

    let user = ctx.get_user_arg("user").unwrap_or(&ctx.input.user);

    let Some(banner_hash) = ctx.input.rest.get::<UserBanner>(format!("users/{}", user.id)).await?.banner else {
        return ctx.respond_error("User has no banner set.", true).await;
    };

    let banner = format!("https://cdn.discordapp.com/banners/{}/{banner_hash}?size=4096", user.id);

    ctx.respond(
        MessageResponse::from(format!("<{banner}>")).add_file(File::new(
            format!("image.{}", if banner_hash.starts_with("a_") { "gif" } else { "png" }),
            REQWEST.get(banner).send().await?.bytes().await?,
        )),
        false,
    )
    .await
}
