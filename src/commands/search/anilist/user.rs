use crate::structs::{
    api::anilist::AniList,
    command_context::{AeonCommandContext, AeonCommandInput},
    select_menu::SelectMenu,
};
use anyhow::Result;
use slashook::commands::MessageResponse;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(_, _) = &ctx.command_input else { return Ok(()) };

    let (query, section) = ctx.get_query_and_section("user")?;
    let user = AniList::get_user(query).await?;
    let name = &user.name;

    let select_menu = SelectMenu::new("anilist", "user", "View other sections…", Some(&section))
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
