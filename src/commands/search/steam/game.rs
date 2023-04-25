use crate::{
    macros::if_else,
    structs::{api::steam::Steam, interaction::Interaction, select_menu::SelectMenu},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder, MessageResponse};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

    if input.get_bool_arg("search").unwrap_or(false) {
        let mut select_menu = SelectMenu::new("steam", "game", "Select a game…", None::<String>);

        for result in match Steam::search_game(input.get_string_arg("game")?).await {
            Ok(results) => results,
            Err(error) => return interaction.respond_error(error, true).await,
        } {
            select_menu = select_menu.add_option(result.name, result.id, None::<String>);
        }

        return interaction.respond(select_menu.to_components(), false).await;
    }

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
            Err(error) => return interaction.respond_error(error, true).await,
        },
    );

    interaction
        .respond(
            MessageResponse::from(
                SelectMenu::new("steam", "game", "Select a section…", Some(&section))
                    .add_option("Overview", format!("{}", game.id), None::<String>)
                    .add_option("Developers", format!("{}/developers", game.id), None::<String>)
                    .add_option("Details", format!("{}/details", game.id), None::<String>)
                    .add_option("Featured Achievements", format!("{}/featured-achievements", game.id), None::<String>)
                    .to_components(),
            )
            .add_embed(match section.as_str() {
                "developers" => game.format_developers(),
                "details" => game.format_details(),
                "featured-achievements" => game.format_featured_achievements(),
                _ => game.format(),
            }),
            false,
        )
        .await
}
