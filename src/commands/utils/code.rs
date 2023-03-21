use crate::{and_then_or, constants::*, kv_autocomplete, structs::api::tio::*, traits::*};
use anyhow::Context;
use slashook::{
    command,
    commands::*,
    structs::{components::*, interactions::*},
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
    fn code(input: CommandInput, res: CommandResponder) {
        if input.is_autocomplete() {
            kv_autocomplete!(
                input,
                res,
                TIO_PROGRAMMING_LANGUAGES
                    .iter()
                    .map(|entry| [entry.id, entry.name])
                    .collect::<Vec<[&str; 2]>>()
            );
        }

        let programming_language = {
            let cache = CACHE.lock()?;

            input
                .get_string_arg("programming-language")
                .unwrap_or(and_then_or!(
                    cache.last_tio_programming_languages.get(&input.user.id),
                    |programming_language| Some(programming_language.to_string()),
                    "".into()
                ))
        };

        if programming_language.is_empty() {
            return res
                .send_message(format!(
                    "{ERROR_EMOJI} please provide a valid programming language"
                ))
                .await?;
        }

        // Set user's last programming language
        {
            let mut cache = CACHE.lock()?;

            cache
                .last_tio_programming_languages
                .insert(input.user.id.clone(), programming_language.clone());
        }

        if input.is_modal_submit() {
            match Tio::new(&programming_language, &input.get_string_arg("code")?)
                .run()
                .await
            {
                Ok(tio) => {
                    res.send_message(tio.format()).await?;
                }
                Err(error) => {
                    res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
                }
            }
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
