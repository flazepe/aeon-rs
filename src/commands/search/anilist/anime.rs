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
            AniList::get_anime(input.values.unwrap()[0].parse::<u64>()?)
                .await?
                .format(),
        )
        .await?;

        return Ok(());
    }

    let mut results = match AniList::search_anime(input.get_string_arg("anime")?).await {
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
                "anime",
                "View other results…",
                results
                    .iter()
                    .map(|result| {
                        SelectOption::new(&result.title.romaji, &result.id).set_description(format!(
                            "{} - {}",
                            AniList::prettify_enum_value(&result.format),
                            AniList::prettify_enum_value(&result.status)
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
