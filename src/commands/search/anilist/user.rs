use crate::{
    macros::if_else,
    statics::emojis::ERROR_EMOJI,
    structs::{api::anilist::AniList, interaction::Interaction, select_menu::SelectMenu},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::{
    commands::{CommandInput, CommandResponder, MessageResponse},
    structs::components::SelectOption,
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let (query, section): (String, String) = {
        if input.is_string_select() {
            let mut split = input.values.as_ref().unwrap()[0].split("/");
            (split.next().unwrap().into(), split.next().unwrap_or("").into())
        } else {
            (input.get_string_arg("user")?, "".into())
        }
    };

    let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

    let user = if_else!(
        input.is_string_select(),
        AniList::get_user(query).await?,
        match AniList::get_user(query).await {
            Ok(result) => result,
            Err(error) => return interaction.respond(format!("{ERROR_EMOJI} {error}"), true).await,
        },
    );

    interaction
        .respond(
            MessageResponse::from(
                SelectMenu::new(
                    "anilist",
                    "user",
                    "Select a section…",
                    vec![
                        SelectOption::new("Overview", format!("{}", user.name)),
                        SelectOption::new("About", format!("{}/about", user.name)),
                        SelectOption::new("Favorite Anime", format!("{}/favorite-anime", user.name)),
                        SelectOption::new("Favorite Manga", format!("{}/favorite-manga", user.name)),
                        SelectOption::new("Favorite Characters", format!("{}/favorite-characters", user.name)),
                        SelectOption::new("Favorite Staff", format!("{}/favorite-staff", user.name)),
                    ],
                    Some(&section),
                )
                .to_components(),
            )
            .add_embed(match section.as_str() {
                "about" => user.format_about(),
                "favorite-anime" => user.format_favorite_anime(),
                "favorite-manga" => user.format_favorite_manga(),
                "favorite-characters" => user.format_favorite_characters(),
                "favorite-staff" => user.format_favorite_staff(),
                _ => user.format(),
            }),
            false,
        )
        .await
}
