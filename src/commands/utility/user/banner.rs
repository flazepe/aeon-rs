use crate::{
    statics::REQWEST,
    structs::command_context::{CommandContext, CommandInputExt, Input},
};
use anyhow::Result;
use serde::Deserialize;
use slashook::{commands::MessageResponse, structs::utils::File};

#[derive(Deserialize, Debug)]
struct UserBanner {
    banner: Option<String>,
}

pub async fn run(ctx: CommandContext) -> Result<()> {
    let Input::ApplicationCommand { input, res: _ } = &ctx.input else { return Ok(()) };

    ctx.defer(false).await?;

    let user = input.get_user_arg("user").unwrap_or(&input.user);
    let user_id = &user.id;

    let Some(banner_url) = input
        .rest
        .get::<UserBanner>(format!("users/{user_id}"))
        .await?
        .banner
        .map(|banner| format!("https://cdn.discordapp.com/banners/{user_id}/{banner}?size=4096"))
    else {
        return ctx.respond_error("User has no banner set.", true).await;
    };

    ctx.respond(
        MessageResponse::from(format!("<{banner_url}>")).add_file(File::new(
            format!("image.{}", if banner_url.contains("a_") { "gif" } else { "png" }),
            REQWEST.get(banner_url).send().await?.bytes().await?,
        )),
        false,
    )
    .await
}
