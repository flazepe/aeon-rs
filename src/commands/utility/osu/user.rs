use crate::structs::{
    api::osu::Osu,
    command_context::{AeonCommandContext, AeonCommandInput},
    select_menu::SelectMenu,
};
use anyhow::Result;
use slashook::commands::MessageResponse;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(..) = &ctx.command_input else { return Ok(()) };

    let mode = ctx.get_string_arg("mode", 0, true);

    let (query, section) = ctx.get_query_and_section("user")?;
    let (user, mode) = query.split_once('|').unwrap_or((&query, mode.as_deref().unwrap_or("default")));

    let user = Osu::get_user(user, mode).await?;
    let id = user.id;

    let select_menu = SelectMenu::new("osu", "user", "View other sections…", Some(&section))
        .add_option("Overview", format!("{id}|{mode}"), None::<String>)
        .add_option("About", format!("{id}|{mode}/about"), None::<String>)
        .add_option("Statistics", format!("{id}|{mode}/statistics"), None::<String>)
        .add_option("Website Statistics", format!("{id}|{mode}/website-statistics"), None::<String>);

    let embed = match section.as_str() {
        "about" => user.format_about(),
        "statistics" => user.format_statistics(),
        "website-statistics" => user.format_website_statistics(),
        _ => user.format(),
    };

    ctx.respond(MessageResponse::from(select_menu).add_embed(embed), false).await
}
