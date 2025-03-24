use crate::{
    statics::CACHE,
    structs::{
        api::tio::{Tio, statics::TIO_PROGRAMMING_LANGUAGES},
        command::AeonCommand,
        command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt},
    },
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder, Modal},
    structs::{
        components::{Components, TextInput, TextInputStyle},
        interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
    },
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("code", &["execute", "run"]).main(|ctx: AeonCommandContext| {
        async move {
            let AeonCommandInput::ApplicationCommand(input, res) = &ctx.command_input else { return Ok(()) };

            if input.is_autocomplete() {
                return ctx.autocomplete(TIO_PROGRAMMING_LANGUAGES.iter()).await;
            }

            // This had to be defined first
            let programming_language = input.get_string_arg("programming-language").ok().or(CACHE
                .last_tio_programming_languages
                .read()
                .unwrap()
                .get(&input.user.id)
                .cloned());

            let programming_language = match programming_language {
                Some(programming_language) => {
                    // Cache user's last programming language
                    CACHE.last_tio_programming_languages.write().unwrap().insert(input.user.id.clone(), programming_language.clone());
                    programming_language
                },
                None => return ctx.respond_error("Please provide a programming language.", true).await,
            };

            if input.is_modal_submit() {
                ctx.defer(false).await?;

                match Tio::new(programming_language, input.get_string_arg("code")?).run().await {
                    Ok(tio) => ctx.respond(tio.format(), false).await,
                    Err(error) => ctx.respond_error(error, true).await,
                }
            } else {
                let code_input = TextInput::new().set_style(TextInputStyle::PARAGRAPH).set_id("code").set_label("Code");
                let components = Components::new().add_text_input(code_input);
                let modal = Modal::new("code", "modal", "Enter Code").set_components(components);

                Ok(res.open_modal(modal).await?)
            }
        }
    })
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
        description = "Runs a code.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
        options = [
            {
                name = "programming-language",
                description = "The programming language",
                option_type = InteractionOptionType::STRING,
                autocomplete = true,
            },
        ],
    )]
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(AeonCommandInput::ApplicationCommand(input, res)).await?;
    }

    func
}
