use crate::{
    statics::{REDIS, REQWEST},
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
        components::{Components, FileUpload, Label, TextInput, TextInputStyle},
        interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
    },
};
use std::sync::{Arc, LazyLock};

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("code", &["exec", "execute", "run"]).set_main(|ctx: Arc<AeonCommandContext>| async move {
        match &ctx.command_input {
            AeonCommandInput::ApplicationCommand(input, res) => {
                if input.is_autocomplete() {
                    return ctx.autocomplete(PISTON_RUNTIMES.iter().map(|(key, runtime)| (key.clone(), runtime.label()))).await;
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

                    let code = ctx.get_string_arg("code", 0, true).ok();
                    let mut piston = Piston::new(programming_language, code);

                    let attachments = input
                        .args
                        .get("files")
                        .and_then(|option_value| option_value.as_values())
                        .map(|option_values| option_values.iter().flat_map(|value| value.as_attachment()));

                    if let Some(attachments) = attachments {
                        for attachment in attachments {
                            if attachment.size <= 5000000 {
                                piston = piston.add_file(&attachment.filename, REQWEST.get(&attachment.url).send().await?.bytes().await?);
                            }
                        }
                    }

                    piston = piston.run().await?;
                    ctx.respond(piston.format_pagination("code").save().await?, false).await
                } else {
                    let code_input = TextInput::new().set_id("code").set_style(TextInputStyle::PARAGRAPH).set_required(false);
                    let file_upload = FileUpload::new().set_id("files").set_max_values(10).set_required(false);
                    let components = Components::new_label(
                        Label::new("Code").set_description(format!("Runtime selected: {}", programming_language.to_lowercase())),
                    )
                    .add_text_input(code_input)
                    .add_label(Label::new("Files"))
                    .add_file_upload(file_upload);

                    let modal = Modal::new("code", "modal", "Enter Code").set_components(components);
                    Ok(res.open_modal(modal).await?)
                }
            },
            AeonCommandInput::MessageCommand(message, args, _) => {
                let codeblock = args.get_content();

                if !codeblock.starts_with("```") || !codeblock.ends_with("```") {
                    bail!("Please provide a valid codeblock.");
                }

                let (programming_language, code) =
                    codeblock.trim_matches('`').split_once('\n').context("Please provide a codeblock with a valid language.")?;

                ctx.defer(false).await?;

                let code = code.trim();
                let mut piston = Piston::new(programming_language.trim(), Some(code));

                for attachment in &message.attachments {
                    if attachment.size <= 5000000 {
                        piston = piston.add_file(&attachment.filename, REQWEST.get(&attachment.url).send().await?.bytes().await?);
                    }
                }

                piston = piston.run().await?;
                ctx.respond(piston.format_pagination("code").save().await?, false).await
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
