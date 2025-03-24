use crate::structs::{
    api::spotify::Spotify,
    command_context::{CommandContext, CommandInputExt, Input},
    select_menu::SelectMenu,
};
use anyhow::Result;
use slashook::commands::MessageResponse;

pub async fn run(ctx: CommandContext) -> Result<()> {
    if let Input::ApplicationCommand { input, res: _ } = &ctx.input {
        if input.get_bool_arg("search").unwrap_or(false) {
            let results = match Spotify::search_simple_album(input.get_string_arg("album")?).await {
                Ok(results) => results,
                Err(error) => return ctx.respond_error(error, true).await,
            };

            let select_menu = SelectMenu::new("spotify", "album", "Select an album…", None::<String>)
                .add_options(results.iter().map(|result| (&result.name, &result.id, Some(&result.artists[0].name))));

            return ctx.respond(select_menu, false).await;
        }
    }

    let (query, section) = ctx.get_query_and_section("album")?;

    if query.is_empty() {
        return ctx.respond_error("Please provide an album.", true).await;
    }

    let album = match ctx.is_string_select() {
        true => Spotify::get_album(query).await?,
        false => match Spotify::search_simple_album(query).await {
            Ok(result) => Spotify::get_album(&result[0].id).await?, // Get full album
            Err(error) => return ctx.respond_error(error, true).await,
        },
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
