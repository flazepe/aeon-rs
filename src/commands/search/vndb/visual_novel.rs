use crate::{
    statics::emojis::ERROR_EMOJI,
    structs::{api::vndb::Vndb, select_menu::SelectMenu},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::{
    commands::{CommandInput, CommandResponder, MessageResponse},
    structs::components::SelectOption,
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let vndb = Vndb::new();

    if input.is_string_select() {
        res.update_message(
            vndb.search_visual_novel(&input.values.as_ref().unwrap()[0])
                .await?
                .remove(0)
                .format(),
        )
        .await?;

        return Ok(());
    }

    let mut results = match vndb.search_visual_novel(input.get_string_arg("visual-novel")?).await {
        Ok(results) => results,
        Err(error) => {
            res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
            return Ok(());
        },
    };

    res.send_message(
        MessageResponse::from(
            SelectMenu::new(
                "vndb",
                "visual-novel",
                "View other results…",
                results
                    .iter()
                    .map(|visual_novel| {
                        SelectOption::new(&visual_novel.title, &visual_novel.id)
                            .set_description(&visual_novel.dev_status)
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
