use crate::structs::{
    api::vndb::Vndb,
    command_context::{AeonCommandContext, AeonCommandInput},
    select_menu::SelectMenu,
};
use anyhow::Result;
use slashook::commands::MessageResponse;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    if let AeonCommandInput::ApplicationCommand(_, _) = &ctx.command_input {
        if ctx.get_bool_arg("search").unwrap_or(false) {
            let characters = Vndb::search_character(ctx.get_string_arg("character")?).await?;
            let select_menu = SelectMenu::new("vndb", "character", "Select a character…", None::<String>)
                .add_options(characters.iter().map(|character| (&character.name, &character.id, Some(&character.vns[0].title))));

            return ctx.respond(select_menu, false).await;
        }
    }

    let (query, section) = ctx.get_query_and_section("character")?;
    let character = Vndb::search_character(query).await?.remove(0);
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
