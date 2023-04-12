use crate::{
    statics::emojis::ERROR_EMOJI,
    structs::{api::vndb::Vndb, restricted_interaction::RestrictedInteraction, select_menu::SelectMenu},
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
        return RestrictedInteraction::verify(&input, &res)
            .await?
            .respond(
                vndb.search_trait(&input.values.as_ref().unwrap()[0])
                    .await?
                    .remove(0)
                    .format(),
            )
            .await;
    }

    let mut results = match vndb.search_trait(input.get_string_arg("trait")?).await {
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
                    .map(|result| SelectOption::new(&result.name, &result.id).set_description(&result.group_name))
                    .collect::<Vec<SelectOption>>(),
                None::<String>,
            )
            .to_components(),
        )
        .add_embed(results.remove(0).format()),
    )
    .await?;

    Ok(())
}
