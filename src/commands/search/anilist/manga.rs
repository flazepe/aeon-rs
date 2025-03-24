use crate::structs::{
    api::anilist::AniList,
    command_context::{AeonCommandContext, CommandInputExt, AeonCommandInput},
    select_menu::SelectMenu,
};
use anyhow::Result;
use slashook::commands::MessageResponse;

pub async fn run(ctx: AeonCommandContext) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };

    if input.get_bool_arg("search").unwrap_or(false) {
        let results = match AniList::search_manga(input.get_string_arg("manga")?).await {
            Ok(results) => results,
            Err(error) => return ctx.respond_error(error, true).await,
        };

        let select_menu =
            SelectMenu::new("anilist", "manga", "Select a manga…", None::<String>).add_options(results.iter().map(|result| {
                (
                    &result.title.romaji,
                    &result.id,
                    Some(format!(
                        "{} - {}",
                        result.format.as_ref().map(|format| format.to_string()).as_deref().unwrap_or("TBA"),
                        result.status,
                    )),
                )
            }));

        return ctx.respond(select_menu, false).await;
    }

    let (query, section) = ctx.get_query_and_section("manga")?;

    let manga = match input.is_string_select() {
        true => AniList::get_manga(query.parse::<u64>()?).await?,
        false => match AniList::search_manga(query).await {
            Ok(mut results) => results.remove(0),
            Err(error) => return ctx.respond_error(error, true).await,
        },
    };

    if manga.is_adult && !input.channel.as_ref().and_then(|channel| channel.nsfw).unwrap_or(false) {
        return ctx.respond_error("NSFW channels only.", true).await;
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
