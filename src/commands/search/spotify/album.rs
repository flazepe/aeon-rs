use crate::structs::{
    api::spotify::Spotify,
    command_context::{AeonCommandContext, AeonCommandInput},
    select_menu::SelectMenu,
};
use anyhow::Result;
use slashook::commands::MessageResponse;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    if let AeonCommandInput::ApplicationCommand(_, _) = &ctx.command_input {
        if ctx.get_bool_arg("search").unwrap_or(false) {
            let results = Spotify::search_simple_album(ctx.get_string_arg("album")?).await?;
            let options = results.iter().map(|result| (&result.name, &result.id, Some(&result.artists[0].name)));
            let select_menu = SelectMenu::new("spotify", "album", "Select an album…", None::<String>).add_options(options);

            return ctx.respond(select_menu, false).await;
        }
    }

    let (query, section) = ctx.get_query_and_section("album")?;

    let album = if ctx.is_string_select() {
        Spotify::get_album(query).await?
    } else {
        let id = Spotify::search_simple_album(query).await?.remove(0).id;
        Spotify::get_album(id).await?
    };

    let id = &album.id;

    let select_menu = SelectMenu::new("spotify", "album", "View other sections…", Some(&section))
        .add_option("Overview", id, None::<String>)
        .add_option("Songs", format!("{id}/songs"), None::<String>)
        .add_option("Available Countries", format!("{id}/available-countries"), None::<String>);

    let embed = match section.as_str() {
        "songs" => album.format_tracks(),
        "available-countries" => album.format_available_countries(),
        _ => album.format(),
    };

    ctx.respond(MessageResponse::from(select_menu).add_embed(embed), false).await
}
