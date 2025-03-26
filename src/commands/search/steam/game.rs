use crate::structs::{
    api::steam::Steam,
    command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt},
    select_menu::SelectMenu,
};
use anyhow::Result;
use slashook::commands::MessageResponse;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };

    if input.get_bool_arg("search").unwrap_or(false) {
        let results = Steam::search_game(input.get_string_arg("game")?).await?;
        let select_menu = SelectMenu::new("steam", "game", "Select a game…", None::<String>)
            .add_options(results.iter().map(|result| (&result.name, &result.id, None::<String>)));

        return ctx.respond(select_menu, false).await;
    }

    let (query, section) = ctx.get_query_and_section("game")?;

    let game = if input.is_string_select() {
        Steam::get_game(query).await?
    } else {
        let results = Steam::search_game(query).await?;
        Steam::get_game(&results[0].id).await?
    };

    let id = game.id;

    let select_menu = SelectMenu::new("steam", "game", "View other sections…", Some(&section))
        .add_option("Overview", id, None::<String>)
        .add_option("Developers", format!("{id}/developers"), None::<String>)
        .add_option("Details", format!("{id}/details"), None::<String>)
        .add_option("Featured Achievements", format!("{id}/featured-achievements"), None::<String>);

    let embed = match section.as_str() {
        "developers" => game.format_developers(),
        "details" => game.format_details(),
        "featured-achievements" => game.format_featured_achievements(),
        _ => game.format(),
    };

    ctx.respond(MessageResponse::from(select_menu).add_embed(embed), false).await
}
