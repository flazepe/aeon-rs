use crate::{
    macros::if_else,
    statics::emojis::ERROR_EMOJI,
    structs::{api::anilist::AniList, select_menu::SelectMenu},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::{
    commands::{CommandInput, CommandResponder},
    structs::components::SelectOption,
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    if input.get_bool_arg("search")? {
        res.send_message(
            SelectMenu::new(
                "anilist",
                "manga",
                "Select a manga",
                match AniList::search_manga(input.get_string_arg("manga")?).await {
                    Ok(results) => results,
                    Err(error) => {
                        res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
                        return Ok(());
                    },
                }
                .into_iter()
                .map(|manga| {
                    SelectOption::new(manga.title.romaji, manga.id)
                        .set_description(AniList::prettify_enum_value(manga.status))
                })
                .collect::<Vec<SelectOption>>(),
            )
            .to_components(),
        )
        .await?;

        return Ok(());
    }

    if_else!(
        input.is_string_select(),
        res.update_message(
            AniList::get_manga(input.values.unwrap()[0].parse::<u64>()?)
                .await?
                .format()
        )
        .await?,
        res.send_message(match AniList::search_manga(input.get_string_arg("manga")?).await {
            Ok(mut results) => results.remove(0).format(),
            Err(error) => {
                res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
                return Ok(());
            },
        })
        .await?
    );

    Ok(())
}
