use crate::structs::{
    api::vndb::Vndb,
    command_context::{CommandContext, CommandInputExt, Input},
    select_menu::SelectMenu,
};
use anyhow::Result;
use slashook::commands::MessageResponse;

pub async fn run(ctx: CommandContext) -> Result<()> {
    if let Input::ApplicationCommand(input,  _) = &ctx.input {
        if input.get_bool_arg("search").unwrap_or(false) {
            let characters = match Vndb::search_character(input.get_string_arg("character")?).await {
                Ok(characters) => characters,
                Err(error) => return ctx.respond_error(error, true).await,
            };

            let select_menu = SelectMenu::new("vndb", "character", "Select a character…", None::<String>)
                .add_options(characters.iter().map(|character| (&character.name, &character.id, Some(&character.vns[0].title))));

            return ctx.respond(select_menu, false).await;
        }
    }

    let (query, section) = ctx.get_query_and_section("character")?;

    if query.is_empty() {
        return ctx.respond_error("Please provide a query.", true).await;
    }

    let character = match Vndb::search_character(query).await {
        Ok(mut characters) => characters.remove(0),
        Err(error) => return ctx.respond_error(error, true).await,
    };

    let id = &character.id;

    let select_menu = SelectMenu::new("vndb", "character", "View other sections…", Some(&section))
        .add_option("Overview", id, None::<String>)
        .add_option("Traits", format!("{id}/traits"), None::<String>)
        .add_option("Visual Novels", format!("{id}/visual-novels"), None::<String>);

    let embed = match section.as_str() {
        "traits" => character.format_traits(),
        "visual-novels" => character.format_visual_novels(),
        _ => character.format(),
    };

    ctx.respond(MessageResponse::from(select_menu).add_embed(embed), false).await
}
