use crate::structs::{api::steam::Steam, command_context::CommandContext, select_menu::SelectMenu};
use anyhow::Result;
use slashook::commands::MessageResponse;

pub async fn run(ctx: CommandContext) -> Result<()> {
    if ctx.get_bool_arg("search").unwrap_or(false) {
        let mut select_menu = SelectMenu::new("steam", "game", "Select a game…", None::<String>);

        for result in match Steam::search_game(ctx.get_string_arg("game")?).await {
            Ok(results) => results,
            Err(error) => return ctx.respond_error(error, true).await,
        } {
            select_menu = select_menu.add_option(result.name, result.id, None::<String>);
        }

        return ctx.respond(select_menu, false).await;
    }

    let (query, section): (String, String) = match ctx.input.is_string_select() {
        true => {
            let mut split = ctx.input.values.as_ref().unwrap()[0].split('/');
            (split.next().unwrap().into(), split.next().unwrap_or("").into())
        },
        false => (ctx.get_string_arg("game")?, "".into()),
    };

    let game = match ctx.input.is_string_select() {
        true => Steam::get_game(query).await?,
        false => match Steam::search_game(query).await {
            Ok(results) => Steam::get_game(&results[0].id).await?,
            Err(error) => return ctx.respond_error(error, true).await,
        },
    };

    let id = game.id;

    let select_menu = SelectMenu::new("steam", "game", "Select a section…", Some(&section))
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
