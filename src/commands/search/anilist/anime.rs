use crate::structs::{api::anilist::AniList, command_context::CommandContext, select_menu::SelectMenu};
use anyhow::Result;
use slashook::{commands::MessageResponse, structs::channels::Channel};

pub async fn run(ctx: CommandContext) -> Result<()> {
    if ctx.get_bool_arg("search").unwrap_or(false) {
        let mut select_menu = SelectMenu::new("anilist", "anime", "Select an anime…", None::<String>);

        for result in match AniList::search_anime(ctx.get_string_arg("anime")?).await {
            Ok(results) => results,
            Err(error) => return ctx.respond_error(error, true).await,
        } {
            select_menu = select_menu.add_option(
                result.title.romaji,
                result.id,
                Some(format!("{} - {}", result.format.map_or("TBA".into(), |format| format.to_string()), result.status)),
            );
        }

        return ctx.respond(select_menu, false).await;
    }

    let (query, section): (String, String) = match ctx.input.is_string_select() {
        true => {
            let mut split = ctx.input.values.as_ref().unwrap()[0].split('/');
            (split.next().unwrap().into(), split.next().unwrap_or("").into())
        },
        false => (ctx.get_string_arg("anime")?, "".into()),
    };

    let anime = match ctx.input.is_string_select() {
        true => AniList::get_anime(query.parse::<u64>()?).await?,
        false => match AniList::search_anime(query).await {
            Ok(mut results) => results.remove(0),
            Err(error) => return ctx.respond_error(error, true).await,
        },
    };

    if anime.is_adult
        && !Channel::fetch(&ctx.input.rest, ctx.input.channel_id.as_ref().unwrap())
            .await
            .map_or(false, |channel| channel.nsfw.unwrap_or(false))
    {
        return ctx.respond_error("NSFW channels only.", true).await;
    }

    ctx.respond(
        MessageResponse::from(
            SelectMenu::new("anilist", "anime", "Select a section…", Some(&section))
                .add_option("Overview", format!("{}", anime.id), None::<String>)
                .add_option("Description", format!("{}/description", anime.id), None::<String>)
                .add_option("Characters", format!("{}/characters", anime.id), None::<String>)
                .add_option("Relations", format!("{}/relations", anime.id), None::<String>),
        )
        .add_embed(match section.as_str() {
            "description" => anime.format_description(),
            "characters" => anime.format_characters(),
            "relations" => anime.format_relations(),
            _ => anime.format(),
        }),
        false,
    )
    .await
}
