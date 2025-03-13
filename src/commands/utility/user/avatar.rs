use crate::{statics::REQWEST, structs::command_context::CommandContext, traits::UserExt};
use anyhow::Result;
use slashook::{
    commands::MessageResponse,
    structs::{guilds::GuildMember, utils::File},
};

pub async fn run(ctx: CommandContext) -> Result<()> {
    ctx.defer(false).await?;

    let user = ctx.get_user_arg("user").unwrap_or(&ctx.input.user);

    let guild_avatar = match ctx.input.guild_id.as_ref() {
        Some(guild_id) => match ctx.input.rest.get::<GuildMember>(format!("guilds/{guild_id}/members/{}", user.id)).await {
            Ok(member) => member
                .avatar
                .map(|avatar| format!("https://cdn.discordapp.com/guilds/{}/users/{}/avatars/{}?size=4096", guild_id, user.id, avatar)),
            Err(_) => None,
        },
        None => None,
    };

    let avatar = if ctx.get_bool_arg("force-user-avatar").unwrap_or(false) {
        user.display_avatar_url("gif", 4096)
    } else {
        guild_avatar.unwrap_or_else(|| user.display_avatar_url("gif", 4096))
    };

    ctx.respond(
        MessageResponse::from(format!(
            "{}<{avatar}>",
            if avatar.contains("guild") {
                "**Showing member's server avatar**. To view member's user avatar, set `force-user-avatar` to `true`.\n"
            } else {
                ""
            },
        ))
        .add_file(File::new(
            format!("image.{}", if avatar.starts_with("a_") { "gif" } else { "png" }),
            REQWEST.get(avatar).send().await?.bytes().await?,
        )),
        false,
    )
    .await
}
