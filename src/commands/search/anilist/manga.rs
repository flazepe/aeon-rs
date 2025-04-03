use crate::structs::{
    api::anilist::AniList,
    command_context::{AeonCommandContext, AeonCommandInput},
    select_menu::SelectMenu,
};
use anyhow::Result;
use slashook::commands::MessageResponse;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    if let AeonCommandInput::ApplicationCommand(_, _) = &ctx.command_input {
        if ctx.get_bool_arg("search").unwrap_or(false) {
            let results = AniList::search_manga(ctx.get_string_arg("manga")?).await?;
            let options = results.iter().map(|result| {
                (
                    &result.title.romaji,
                    &result.id,
                    Some(format!(
                        "{} - {}",
                        result.format.as_ref().map(|format| format.to_string()).as_deref().unwrap_or("TBA"),
                        result.status,
                    )),
                )
            });
            let select_menu = SelectMenu::new("anilist", "manga", "Select a manga…", None::<String>).add_options(options);

            return ctx.respond(select_menu, false).await;
        }
    }

    let (query, section) = ctx.get_query_and_section("manga")?;

    let manga = if ctx.is_string_select() {
        AniList::get_manga(query.parse::<u64>()?).await?
    } else {
        AniList::search_manga(query).await?.remove(0)
    };

    if manga.is_adult {
        ctx.ensure_nsfw_channel().await?;
    }

    let id = manga.id;

    let select_menu = SelectMenu::new("anilist", "manga", "View other sections…", Some(&section))
        .add_option("Overview", id, None::<String>)
        .add_option("Description", format!("{id}/description"), None::<String>)
        .add_option("Characters", format!("{id}/characters"), None::<String>)
        .add_option("Relations", format!("{id}/relations"), None::<String>);

    let embed = match section.as_str() {
        "description" => manga.format_description(),
        "characters" => manga.format_characters(),
        "relations" => manga.format_relations(),
        _ => manga.format(),
    };

    ctx.respond(MessageResponse::from(select_menu).add_embed(embed), false).await
}
