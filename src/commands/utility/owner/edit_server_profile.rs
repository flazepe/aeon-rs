use crate::{
    statics::REQWEST,
    structs::command_context::{AeonCommandContext, AeonCommandInput},
};
use anyhow::Result;
use slashook::structs::{
    Permissions,
    members::{GuildMember, GuildMemberModifyOptions},
    utils::File,
};
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };

    let guild_id = ctx.get_string_arg("server-id", 0, false).unwrap_or_else(|_| ctx.get_guild_id().unwrap());
    let mut options = GuildMemberModifyOptions::new();
    let mut changed_properties = vec![];

    ctx.defer(true).await?;

    if ctx.get_bool_arg("reset-all").unwrap_or(false) {
        options = options.set_avatar(None::<String>).set_banner(None::<String>).set_bio(None::<String>);

        if input.app_permissions.contains(Permissions::CHANGE_NICKNAME) {
            options = options.set_nick(None::<String>);
        }

        GuildMember::modify_current_member(&input.rest, guild_id, options, None::<String>).await?;
        return ctx.respond_success("Successfuly reset bot's server profile.", true).await;
    }

    if let Ok(reset) = ctx.get_string_arg("reset", 0, false) {
        options = match reset.as_str() {
            "avatar" => options.set_avatar(None::<String>),
            "banner" => options.set_banner(None::<String>),
            "nickname" => options.set_nick(None::<String>),
            "bio" => options.set_bio(None::<String>),
            _ => options,
        };

        GuildMember::modify_current_member(&input.rest, guild_id, options, None::<String>).await?;
        return ctx.respond_success(format!("Successfuly reset bot's server {reset}."), true).await;
    }

    if let Ok(avatar) = ctx.get_attachment_arg("avatar") {
        let res = REQWEST.get(&avatar.url).send().await?;
        let bytes = res.bytes().await?;

        options = options.set_avatar(Some(File::new("", bytes)));
        changed_properties.push("avatar");
    }

    if let Ok(banner) = ctx.get_attachment_arg("banner") {
        let res = REQWEST.get(&banner.url).send().await?;
        let bytes = res.bytes().await?;

        options = options.set_banner(Some(File::new("", bytes)));
        changed_properties.push("banner");
    }

    if let Ok(nickname) = ctx.get_string_arg("nickname", 0, false) {
        options = options.set_nick(Some(nickname));
        changed_properties.push("nickname");
    }

    if let Ok(about_me) = ctx.get_string_arg("bio", 0, false) {
        options = options.set_bio(Some(about_me));
        changed_properties.push("about me");
    }

    GuildMember::modify_current_member(&input.rest, guild_id, options, None::<String>).await?;
    ctx.respond_success(format!("Successfuly set bot's server {}.", changed_properties.join(", ")), true).await
}
