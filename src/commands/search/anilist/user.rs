use crate::{
    structs::{api::anilist::AniList, interaction::Interaction, select_menu::SelectMenu},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder, MessageResponse};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

    let (query, section): (String, String) = match input.is_string_select() {
        true => {
            let mut split = input.values.as_ref().unwrap()[0].split("/");
            (split.next().unwrap().into(), split.next().unwrap_or("").into())
        },
        false => (input.get_string_arg("user")?, "".into()),
    };

    let user = match input.is_string_select() {
        true => AniList::get_user(query).await?,
        false => match AniList::get_user(query).await {
            Ok(result) => result,
            Err(error) => return interaction.respond_error(error, true).await,
        },
    };

    interaction
        .respond(
            MessageResponse::from(match section.as_str() {
                "about" => user.format_about(),
                "favorite-anime" => user.format_favorite_anime(),
                "favorite-manga" => user.format_favorite_manga(),
                "favorite-characters" => user.format_favorite_characters(),
                "favorite-staff" => user.format_favorite_staff(),
                _ => user.format(),
            })
            .set_components(
                SelectMenu::new("anilist", "user", "Select a section…", Some(&section))
                    .add_option("Overview", format!("{}", user.name), None::<String>)
                    .add_option("About", format!("{}/about", user.name), None::<String>)
                    .add_option("Favorite Anime", format!("{}/favorite-anime", user.name), None::<String>)
                    .add_option("Favorite Manga", format!("{}/favorite-manga", user.name), None::<String>)
                    .add_option("Favorite Characters", format!("{}/favorite-characters", user.name), None::<String>)
                    .add_option("Favorite Staff", format!("{}/favorite-staff", user.name), None::<String>)
                    .to_components(),
            ),
            false,
        )
        .await
}
