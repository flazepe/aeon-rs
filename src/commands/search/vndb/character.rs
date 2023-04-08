use crate::{
    macros::if_else,
    statics::emojis::ERROR_EMOJI,
    structs::{api::vndb::Vndb, select_menu::SelectMenu},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::{
    commands::{CommandInput, CommandResponder},
    structs::components::SelectOption,
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let vndb = Vndb::new();

    if input.get_bool_arg("search")? {
        res.send_message(
            SelectMenu::new(
                "vndb",
                "character",
                "Select a character",
                match vndb.search_character(input.get_string_arg("character")?).await {
                    Ok(results) => results,
                    Err(error) => {
                        res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
                        return Ok(());
                    },
                }
                .into_iter()
                .map(|character| {
                    SelectOption::new(character.name, character.id).set_description(character.vns[0].title.clone())
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
            vndb.search_character(&input.values.as_ref().unwrap()[0])
                .await?
                .remove(0)
                .format()
        )
        .await?,
        res.send_message(match vndb.search_character(input.get_string_arg("character")?).await {
            Ok(mut characters) => characters.remove(0).format(),
            Err(error) => {
                res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
                return Ok(());
            },
        })
        .await?
    );

    Ok(())
}
