use crate::{
    statics::emojis::ERROR_EMOJI,
    structs::{api::anilist::AniList, select_menu::SelectMenu},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::{
    commands::{CommandInput, CommandResponder, MessageResponse},
    structs::components::SelectOption,
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    if input.is_string_select() {
        res.update_message(
            AniList::get_manga(input.values.unwrap()[0].parse::<u64>()?)
                .await?
                .format(),
        )
        .await?;

        return Ok(());
    }

    let mut results = match AniList::search_manga(input.get_string_arg("manga")?).await {
        Ok(results) => results,
        Err(error) => {
            res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
            return Ok(());
        },
    };

    res.send_message(
        MessageResponse::from(
            SelectMenu::new(
                "anilist",
                "manga",
                "Select a manga",
                results
                    .iter()
                    .map(|manga| {
                        SelectOption::new(&manga.title.romaji, &manga.id).set_description(format!(
                            "{} - {}",
                            AniList::prettify_enum_value(&manga.format),
                            AniList::prettify_enum_value(&manga.status)
                        ))
                    })
                    .collect::<Vec<SelectOption>>(),
            )
            .to_components(),
        )
        .add_embed(results.remove(0).format()),
    )
    .await?;

    Ok(())
}
