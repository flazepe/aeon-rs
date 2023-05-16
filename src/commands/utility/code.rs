use crate::{
    statics::CACHE,
    structs::{
        api::tio::{statics::TIO_PROGRAMMING_LANGUAGES, Tio},
        interaction::Interaction,
    },
    traits::ArgGetters,
};
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder, Modal},
    structs::{
        components::{Components, TextInput, TextInputStyle},
        interactions::InteractionOptionType,
    },
};

pub fn get_command() -> Command {
    #[command(
        name = "code",
        description = "Runs a code.",
        options = [
            {
                name = "programming-language",
                description = "The programming language",
                option_type = InteractionOptionType::STRING,
                autocomplete = true,
            },
        ],
    )]
    async fn code(input: CommandInput, res: CommandResponder) {
        let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

        if input.is_autocomplete() {
            return interaction.hashmap_autocomplete(TIO_PROGRAMMING_LANGUAGES.iter()).await?;
        }

        // This had to be defined first
        let programming_language = input.get_string_arg("programming-language").ok().or(CACHE
            .last_tio_programming_languages
            .read()?
            .get(&input.user.id)
            .map(|programming_language| programming_language.clone()));

        let programming_language = match programming_language {
            Some(programming_language) => {
                // Cache user's last programming language
                CACHE.last_tio_programming_languages.write()?.insert(input.user.id.clone(), programming_language.clone());
                programming_language
            },
            None => return Ok(interaction.respond_error("Please provide a programming language.", true).await?),
        };

        match input.is_modal_submit() {
            true => {
                res.defer(false).await?;

                match Tio::new(programming_language, input.get_string_arg("code")?).run().await {
                    Ok(tio) => interaction.respond(tio.format(), false).await?,
                    Err(error) => interaction.respond_error(error, true).await?,
                };
            },
            false => {
                res.open_modal(
                    Modal::new("code", "modal", "Enter Code").set_components(
                        Components::new()
                            .add_text_input(TextInput::new().set_style(TextInputStyle::PARAGRAPH).set_id("code").set_label("Code")),
                    ),
                )
                .await?
            },
        }
    }

    code
}
