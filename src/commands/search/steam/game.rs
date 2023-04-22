use crate::{
    macros::if_else,
    statics::emojis::ERROR_EMOJI,
    structs::{api::steam::Steam, component_interaction::ComponentInteraction, select_menu::SelectMenu},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::{
    commands::{CommandInput, CommandResponder, MessageResponse},
    structs::components::SelectOption,
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    if input.get_bool_arg("search").unwrap_or(false) {
        res.send_message(
            SelectMenu::new(
                "steam",
                "game",
                "Select a game…",
                match Steam::search_game(input.get_string_arg("game")?).await {
                    Ok(results) => results
                        .into_iter()
                        .map(|result| SelectOption::new(result.name, result.id)),
                    Err(error) => {
                        res.send_message(MessageResponse::from(format!("{ERROR_EMOJI} {error}")).set_ephemeral(true))
                            .await?;

                        return Ok(());
                    },
                }
                .collect::<Vec<SelectOption>>(),
                None::<String>,
            )
            .to_components(),
        )
        .await?;

        return Ok(());
    }

    let Ok(interaction) = ComponentInteraction::verify(&input, &res).await else { return Ok(()); };

    let (query, section): (String, String) = {
        if input.is_string_select() {
            let mut split = input.values.as_ref().unwrap()[0].split("/");
            (split.next().unwrap().into(), split.next().unwrap_or("").into())
        } else {
            (input.get_string_arg("game")?, "".into())
        }
    };

    let game = if_else!(
        input.is_string_select(),
        Steam::get_game(query).await?,
        match Steam::search_game(query).await {
            Ok(results) => Steam::get_game(&results[0].id).await?,
            Err(error) => return interaction.respond(format!("{ERROR_EMOJI} {error}")).await,
        },
    );

    interaction
        .respond(
            MessageResponse::from(
                SelectMenu::new(
                    "steam",
                    "game",
                    "Select a section…",
                    vec![
                        SelectOption::new("Overview", format!("{}", game.id)),
                        SelectOption::new("Developers", format!("{}/developers", game.id)),
                        SelectOption::new("Details", format!("{}/details", game.id)),
                        SelectOption::new("Featured Achievements", format!("{}/featured-achievements", game.id)),
                    ],
                    Some(&section),
                )
                .to_components(),
            )
            .add_embed(match section.as_str() {
                "developers" => game.format_developers(),
                "details" => game.format_details(),
                "featured-achievements" => game.format_featured_achievements(),
                _ => game.format(),
            }),
        )
        .await
}
