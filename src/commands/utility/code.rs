use crate::{
    functions::hashmap_autocomplete,
    statics::{emojis::ERROR_EMOJI, tio::TIO_PROGRAMMING_LANGUAGES, CACHE},
    structs::api::tio::Tio,
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
use std::collections::HashMap;

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
        if input.is_autocomplete() {
            return hashmap_autocomplete(
                input,
                res,
                (HashMap::from_iter(TIO_PROGRAMMING_LANGUAGES.iter().map(|entry| (entry.id, entry.name)))
                    as HashMap<&str, &str>)
                    .iter(),
            )
            .await?;
        }

        let programming_language = {
            input.get_string_arg("programming-language").unwrap_or(
                CACHE
                    .last_tio_programming_languages
                    .read()?
                    .get(&input.user.id)
                    .map_or("".into(), |programming_language| programming_language.to_string()),
            )
        };

        if programming_language.is_empty() {
            return res
                .send_message(format!("{ERROR_EMOJI} Please provide a programming language."))
                .await?;
        }

        // Set user's last programming language
        {
            CACHE
                .last_tio_programming_languages
                .write()?
                .insert(input.user.id.clone(), programming_language.clone());
        }

        if input.is_modal_submit() {
            res.defer(false).await?;

            match Tio::new(programming_language, input.get_string_arg("code")?)
                .run()
                .await
            {
                Ok(tio) => res.send_message(tio.format()).await?,
                Err(error) => res.send_message(format!("{ERROR_EMOJI} {error}")).await?,
            };
        } else {
            res.open_modal(
                Modal::new("code", "modal", "enter code").set_components(
                    Components::new().add_text_input(
                        TextInput::new()
                            .set_id("code")
                            .set_label("code")
                            .set_style(TextInputStyle::PARAGRAPH)
                            .set_required(true),
                    ),
                ),
            )
            .await?;
        }
    }

    code
}
