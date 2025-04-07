use crate::structs::{
    api::anilist::AniList,
    command_context::{AeonCommandContext, AeonCommandInput},
    select_menu::SelectMenu,
};
use anyhow::Result;
use slashook::commands::MessageResponse;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    if let AeonCommandInput::ApplicationCommand(..) = &ctx.command_input {
        if ctx.get_bool_arg("search").unwrap_or(false) {
            let results = AniList::search_anime(ctx.get_string_arg("anime", 0, true)?).await?;
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
            let select_menu = SelectMenu::new("anilist", "anime", "Select an anime…", None::<String>).add_options(options);

            return ctx.respond(select_menu, false).await;
        }
    }

    let (query, section) = ctx.get_query_and_section("anime")?;

    let anime = if ctx.is_string_select() {
        AniList::get_anime(query.parse::<u64>()?).await?
    } else {
        AniList::search_anime(query).await?.remove(0)
    };

    if anime.is_adult {
        ctx.ensure_nsfw_channel().await?;
    }

    let id = anime.id;

    let select_menu = SelectMenu::new("anilist", "anime", "View other sections…", Some(&section))
        .add_option("Overview", id, None::<String>)
        .add_option("Description", format!("{id}/description"), None::<String>)
        .add_option("Characters", format!("{id}/characters"), None::<String>)
        .add_option("Relations", format!("{id}/relations"), None::<String>);

    let embed = match section.as_str() {
        "description" => anime.format_description(),
        "characters" => anime.format_characters(),
        "relations" => anime.format_relations(),
        _ => anime.format(),
    };

    ctx.respond(MessageResponse::from(select_menu).add_embed(embed), false).await
}
