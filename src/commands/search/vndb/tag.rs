use crate::{
    macros::respond_to_component_interaction,
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
        respond_to_component_interaction!(
            input,
            res,
            vndb.search_tag(&input.values.as_ref().unwrap()[0])
                .await?
                .remove(0)
                .format()
        );
    }

    let mut results = match vndb.search_tag(input.get_string_arg("tag")?).await {
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
                "trait",
                "View other results…",
                results
                    .iter()
                    .map(|result| {
                        SelectOption::new(result.name.to_string(), &result.id).set_description(&result.category)
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