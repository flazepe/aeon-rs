use crate::{
    structs::{api::anilist::AniList, command_context::CommandContext, select_menu::SelectMenu},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::{commands::MessageResponse, structs::channels::Channel};

pub async fn run(ctx: CommandContext) -> Result<()> {
    if ctx.input.get_bool_arg("search").unwrap_or(false) {
        let mut select_menu = SelectMenu::new("anilist", "manga", "Select a manga…", None::<String>);

        for result in match AniList::search_manga(ctx.input.get_string_arg("manga")?).await {
            Ok(results) => results,
            Err(error) => return ctx.respond_error(error, true).await,
        } {
            select_menu = select_menu.add_option(
                result.title.romaji,
                result.id,
                Some(format!(
                    "{} - {}",
                    result.format.map_or("TBA".into(), |format| AniList::format_enum_value(format)),
                    AniList::format_enum_value(result.status),
                )),
            );
        }

        return ctx.respond(select_menu, false).await;
    }

    let (query, section): (String, String) = match ctx.input.is_string_select() {
        true => {
            let mut split = ctx.input.values.as_ref().unwrap()[0].split("/");
            (split.next().unwrap().into(), split.next().unwrap_or("").into())
        },
        false => (ctx.input.get_string_arg("manga")?, "".into()),
    };

    let manga = match ctx.input.is_string_select() {
        true => AniList::get_manga(query.parse::<u64>()?).await?,
        false => match AniList::search_manga(query).await {
            Ok(mut results) => results.remove(0),
            Err(error) => return ctx.respond_error(error, true).await,
        },
    };

    if manga.is_adult {
        if !Channel::fetch(&ctx.input.rest, ctx.input.channel_id.as_ref().unwrap())
            .await
            .map_or(false, |channel| channel.nsfw.unwrap_or(false))
        {
            return ctx.respond_error("NSFW channels only.", true).await;
        }
    }

    ctx.respond(
        MessageResponse::from(
            SelectMenu::new("anilist", "manga", "Select a section…", Some(&section))
                .add_option("Overview", format!("{}", manga.id), None::<String>)
                .add_option("Description", format!("{}/description", manga.id), None::<String>)
                .add_option("Characters", format!("{}/characters", manga.id), None::<String>)
                .add_option("Relations", format!("{}/relations", manga.id), None::<String>),
        )
        .add_embed(match section.as_str() {
            "description" => manga.format_description(),
            "characters" => manga.format_characters(),
            "relations" => manga.format_relations(),
            _ => manga.format(),
        }),
        false,
    )
    .await
}
