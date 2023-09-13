use crate::structs::{api::spotify::Spotify, command_context::CommandContext, select_menu::SelectMenu};
use anyhow::Result;
use slashook::commands::MessageResponse;

pub async fn run(ctx: CommandContext) -> Result<()> {
    if ctx.get_bool_arg("search").unwrap_or(false) {
        let mut select_menu = SelectMenu::new("spotify", "album", "Select an album…", None::<String>);

        for result in match Spotify::search_simple_album(ctx.get_string_arg("album")?).await {
            Ok(results) => results,
            Err(error) => return ctx.respond_error(error, true).await,
        } {
            select_menu = select_menu.add_option(result.name, result.id, Some(&result.artists[0].name))
        }

        return ctx.respond(select_menu, false).await;
    }

    let (query, section): (String, String) = match ctx.input.is_string_select() {
        true => {
            let mut split = ctx.input.values.as_ref().unwrap()[0].split('/');
            (split.next().unwrap().into(), split.next().unwrap_or("").into())
        },
        false => (ctx.get_string_arg("album")?, "".into()),
    };

    let album = match ctx.input.is_string_select() {
        true => Spotify::get_album(query).await?,
        false => match Spotify::search_simple_album(query).await {
            Ok(result) => Spotify::get_album(&result[0].id).await?, // Get full album
            Err(error) => return ctx.respond_error(error, true).await,
        },
    };

    ctx.respond(
        MessageResponse::from(
            SelectMenu::new("spotify", "album", "Select a section…", Some(&section))
                .add_option("Overview", &album.id, None::<String>)
                .add_option("Songs", format!("{}/songs", album.id), None::<String>)
                .add_option("Available Countries", format!("{}/available-countries", album.id), None::<String>),
        )
        .add_embed(match section.as_str() {
            "songs" => album.format_tracks(),
            "available-countries" => album.format_available_countries(),
            _ => album.format(),
        }),
        false,
    )
    .await
}
