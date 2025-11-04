use crate::{
    statics::REDIS,
    structs::{
        api::piston::{Piston, statics::PISTON_RUNTIMES},
        command::AeonCommand,
        command_context::{AeonCommandContext, AeonCommandInput},
        database::redis::keys::RedisKey,
    },
};
use anyhow::{Context, bail};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder, Modal},
    structs::{
        components::{Components, Label, TextInput, TextInputStyle},
        interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
    },
};
use std::sync::{Arc, LazyLock};

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("code", &["exec", "execute", "run"]).set_main(|ctx: Arc<AeonCommandContext>| async move {
        match &ctx.command_input {
            AeonCommandInput::ApplicationCommand(input, res) => {
                if input.is_autocomplete() {
                    return ctx.autocomplete(PISTON_RUNTIMES.iter().map(|runtime| (&runtime.language, runtime.label()))).await;
                }

                let redis = REDIS.get().context("Could not get Redis.")?;
                let key = RedisKey::UserLastPistonProgrammingLanguage(ctx.get_user_id());
                let programming_language = ctx
                    .get_string_arg("programming-language", 0, true)
                    .or(redis.get::<String>(&key).await)
                    .context("Please provide a programming language.")?;
                redis.set(&key, &programming_language, Some(60 * 60)).await?;

                if input.is_modal_submit() {
                    ctx.defer(false).await?;

                    let piston = Piston::new(programming_language, ctx.get_string_arg("code", 0, true)?).run().await?;
                    ctx.respond(piston.format(), false).await
                } else {
                    let code_input = TextInput::new().set_style(TextInputStyle::PARAGRAPH).set_id("code");
                    let components = Components::new_label(Label::new("Code")).add_text_input(code_input);
                    let modal = Modal::new("code", "modal", format!("Enter Code ({})", programming_language.to_lowercase()))
                        .set_components(components);

                    Ok(res.open_modal(modal).await?)
                }
            },
            AeonCommandInput::MessageCommand(_, args, _) => {
                let codeblock = args.get_content();

                if !codeblock.starts_with("```") || !codeblock.ends_with("```") {
                    bail!("Please provide a valid codeblock.");
                }

                let (programming_language, code) =
                    codeblock.trim_matches('`').split_once('\n').context("Please provide a codeblock with a valid language.")?;

                ctx.defer(false).await?;

                let piston = Piston::new(programming_language.trim(), code.trim()).run().await?;
                ctx.respond(piston.format(), false).await
            },
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
        COMMAND.run(AeonCommandInput::ApplicationCommand(Box::new(input), res)).await?;
    }

    func
}
