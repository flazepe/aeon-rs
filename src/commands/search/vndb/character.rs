use crate::{macros::if_else, statics::emojis::ERROR_EMOJI, structs::api::vndb::Vndb, traits::ArgGetters};
use anyhow::Result;
use slashook::{
    commands::{CommandInput, CommandResponder, MessageResponse},
    structs::components::{Components, SelectMenu, SelectMenuType, SelectOption},
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let vndb = Vndb::new();

    if input.get_bool_arg("search")? {
        let mut select_menu = SelectMenu::new(SelectMenuType::STRING)
            .set_id("vndb", "character")
            .set_placeholder("Select a character");

        for character in match vndb.search_character(input.get_string_arg("character")?).await {
            Ok(results) => results,
            Err(error) => {
                res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
                return Ok(());
            },
        }
        .into_iter()
        .take(25)
        {
            select_menu = select_menu.add_option(
                SelectOption::new(character.name.chars().take(100).collect::<String>(), character.id)
                    .set_description(character.vns[0].title.chars().take(100).collect::<String>()),
            );
        }

        res.send_message(MessageResponse::from(Components::new().add_select_menu(select_menu)).set_ephemeral(true))
            .await?;

        return Ok(());
    }

    res.send_message(
        if_else!(
            input.is_string_select(),
            vndb.get_character(&input.values.as_ref().unwrap()[0]).await?,
            match vndb.search_character(input.get_string_arg("character")?).await {
                Ok(mut characters) => characters.remove(0),
                Err(error) => {
                    res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
                    return Ok(());
                },
            }
        )
        .format(),
    )
    .await?;

    Ok(())
}
