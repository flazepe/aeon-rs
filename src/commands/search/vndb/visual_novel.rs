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
                "visual-novel",
                "Select a visual novel",
                match vndb.search_visual_novel(input.get_string_arg("visual-novel")?).await {
                    Ok(results) => results,
                    Err(error) => {
                        res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
                        return Ok(());
                    },
                }
                .into_iter()
                .map(|visual_novel| {
                    SelectOption::new(visual_novel.title, visual_novel.id).set_description(visual_novel.dev_status)
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
            vndb.search_visual_novel(&input.values.as_ref().unwrap()[0])
                .await?
                .remove(0)
                .format()
        )
        .await?,
        res.send_message(
            match vndb.search_visual_novel(input.get_string_arg("visual-novel")?).await {
                Ok(mut visual_novels) => visual_novels.remove(0).format(),
                Err(error) => {
                    res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
                    return Ok(());
                },
            }
        )
        .await?
    );

    Ok(())
}
