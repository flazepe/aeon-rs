use crate::{statics::REQWEST, structs::command_context::CommandContext};
use anyhow::Result;
use serde::Deserialize;
use slashook::structs::utils::File;

#[derive(Deserialize)]
struct UserBanner {
    banner: Option<String>,
}

pub async fn run(ctx: CommandContext) -> Result<()> {
    let user = ctx.get_user_arg("user").unwrap_or(&ctx.input.user);

    match ctx.input.rest.get::<UserBanner>(format!("users/{}", user.id)).await?.banner {
        Some(user_banner) => {
            ctx.respond(
                File::new(
                    format!(
                        "image.{}",
                        match user_banner.starts_with("a_") {
                            true => "gif",
                            false => "png",
                        },
                    ),
                    REQWEST
                        .get(format!("https://cdn.discordapp.com/banners/{}/{}?size=4096", user.id, user_banner))
                        .send()
                        .await?
                        .bytes()
                        .await?,
                ),
                false,
            )
            .await
        },
        None => ctx.respond_error("User has no banner set.", true).await,
    }
}
