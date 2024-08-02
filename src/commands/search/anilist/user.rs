use crate::structs::{api::anilist::AniList, command_context::CommandContext, select_menu::SelectMenu};
use anyhow::Result;
use slashook::commands::MessageResponse;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let (query, section): (String, String) = match ctx.input.is_string_select() {
        true => {
            let mut split = ctx.input.values.as_ref().unwrap()[0].split('/');
            (split.next().unwrap().into(), split.next().unwrap_or("").into())
        },
        false => (ctx.get_string_arg("user")?, "".into()),
    };

    let user = match ctx.input.is_string_select() {
        true => AniList::get_user(query).await?,
        false => match AniList::get_user(query).await {
            Ok(result) => result,
            Err(error) => return ctx.respond_error(error, true).await,
        },
    };

    let name = &user.name;

    let select_menu = SelectMenu::new("anilist", "user", "Select a section…", Some(&section))
        .add_option("Overview", name, None::<String>)
        .add_option("About", format!("{name}/about"), None::<String>)
        .add_option("Favorite Anime", format!("{name}/favorite-anime"), None::<String>)
        .add_option("Favorite Manga", format!("{name}/favorite-manga"), None::<String>)
        .add_option("Favorite Characters", format!("{name}/favorite-characters"), None::<String>)
        .add_option("Favorite Staff", format!("{name}/favorite-staff"), None::<String>);

    let embed = match section.as_str() {
        "about" => user.format_about(),
        "favorite-anime" => user.format_favorite_anime(),
        "favorite-manga" => user.format_favorite_manga(),
        "favorite-characters" => user.format_favorite_characters(),
        "favorite-staff" => user.format_favorite_staff(),
        _ => user.format(),
    };

    ctx.respond(MessageResponse::from(select_menu).add_embed(embed), false).await
}
