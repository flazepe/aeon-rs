use crate::structs::{api::vndb::Vndb, command_context::CommandContext, select_menu::SelectMenu};
use anyhow::Result;
use slashook::commands::MessageResponse;

pub async fn run(ctx: CommandContext) -> Result<()> {
    if ctx.get_bool_arg("search").unwrap_or(false) {
        let mut select_menu = SelectMenu::new("vndb", "character", "Select a character…", None::<String>);

        for character in match Vndb::search_character(ctx.get_string_arg("character")?).await {
            Ok(characters) => characters,
            Err(error) => return ctx.respond_error(error, true).await,
        } {
            select_menu = select_menu.add_option(character.name, character.id, Some(&character.vns[0].title));
        }

        return ctx.respond(select_menu, false).await;
    }

    let (query, section): (String, String) = match ctx.input.is_string_select() {
        true => {
            let mut split = ctx.input.values.as_ref().unwrap()[0].split('/');
            (split.next().unwrap().into(), split.next().unwrap_or("").into())
        },
        false => (ctx.get_string_arg("character")?, "".into()),
    };

    let character = match Vndb::search_character(query).await {
        Ok(mut characters) => characters.remove(0),
        Err(error) => return ctx.respond_error(error, true).await,
    };

    let id = &character.id;

    let select_menu = SelectMenu::new("vndb", "character", "Select a section…", Some(&section))
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
