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
            .set_id("vndb", "visual-novel")
            .set_placeholder("Select a visual novel");

        for visual_novel in match vndb.search_visual_novel(input.get_string_arg("visual-novel")?).await {
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
                SelectOption::new(
                    visual_novel.title.chars().take(100).collect::<String>(),
                    visual_novel.id,
                )
                .set_description(visual_novel.dev_status),
            );
        }

        res.send_message(MessageResponse::from(Components::new().add_select_menu(select_menu)).set_ephemeral(true))
            .await?;

        return Ok(());
    }

    res.send_message(
        if_else!(
            input.is_string_select(),
            vndb.get_visual_novel(&input.values.as_ref().unwrap()[0]).await?,
            match vndb.search_visual_novel(input.get_string_arg("visual-novel")?).await {
                Ok(mut visual_novels) => visual_novels.remove(0),
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
