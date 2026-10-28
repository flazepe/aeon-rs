use crate::{
    statics::{REDIS, REQWEST},
    structs::{
        api::piston::{Piston, PistonState},
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
        components::{Checkbox, Components, FileUpload, Label, TextInput, TextInputStyle},
        interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
    },
};
use std::sync::{Arc, LazyLock};

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("code", &["exec", "execute", "run"]).set_main(|ctx: Arc<AeonCommandContext>| async move {
        match &ctx.command_input {
            AeonCommandInput::ApplicationCommand(input, res) => {
                if input.is_autocomplete() {
                    res.autocomplete(Piston::autocomplete_runtime(input)).await?;
                    return Ok(());
                }

                let redis = REDIS.get().context("Could not get Redis.")?;
                let key = RedisKey::PistonStates(ctx.get_user_id());
                let mut piston_state = redis.get::<PistonState>(&key).await.unwrap_or_default();

                let programming_language = ctx
                    .get_string_arg("programming-language", 0, true)
                    .ok()
                    .or(piston_state.runtime)
                    .context("Please provide a programming language.")?;
                let runtime = Piston::get_runtime(&programming_language).context("Programming language not found.")?;
                piston_state.runtime = Some(runtime.to_string());
                redis.set(&key, &piston_state, Some(60 * 60)).await?;

                if input.is_modal_submit() {
                    ctx.defer(false).await?;

                    let code = ctx.get_string_arg("code", 0, true).ok();
                    let stdin = ctx.get_string_arg("stdin", 0, true).ok();
                    let args = ctx.get_string_arg("args", 0, true).ok();

                    let keep_state = ctx.get_bool_arg("keep-state").unwrap_or_default();
                    piston_state.keep_state = keep_state;

                    if keep_state {
                        piston_state.code = code.clone();
                        piston_state.stdin = stdin.clone();
                        piston_state.args = args.clone();
                    } else {
                        piston_state.code = None;
                        piston_state.stdin = None;
                        piston_state.args = None;
                    }

                    redis.set(&key, &piston_state, Some(60 * 60)).await?;

                    let mut piston = Piston::new(runtime, code, stdin, args);

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
                    let keep_state_checkbox = Checkbox::new().set_id("keep-state").set_default(piston_state.keep_state);
                    let code_input = TextInput::new()
                        .set_id("code")
                        .set_style(TextInputStyle::PARAGRAPH)
                        .set_required(false)
                        .set_value(piston_state.code.as_deref().unwrap_or_default());
                    let stdin_input =
                        TextInput::new().set_id("stdin").set_required(false).set_value(piston_state.stdin.as_deref().unwrap_or_default());
                    let args_input =
                        TextInput::new().set_id("args").set_required(false).set_value(piston_state.args.as_deref().unwrap_or_default());
                    let files_input = FileUpload::new().set_id("files").set_max_values(10).set_required(false);
                    let components = Components::new_label(Label::new("Keep state for next time"))
                        .add_checkbox(keep_state_checkbox)
                        .add_label(Label::new("Code").set_description(runtime.to_string()))
                        .add_text_input(code_input)
                        .add_label(Label::new("stdin").set_description("stdin text to provide to the program"))
                        .add_text_input(stdin_input)
                        .add_label(Label::new("args").set_description(r#"array of args to provide to the program, e.g., ["foo", "bar"]"#))
                        .add_text_input(args_input)
                        .add_label(Label::new("Files").set_description("upload files that the program can have access to"))
                        .add_file_upload(files_input);

                    let modal = Modal::new("code", "modal", "Enter Code").set_components(components);
                    Ok(res.open_modal(modal).await?)
                }
            },
            AeonCommandInput::MessageCommand(message, args, _) => {
                let codeblock = args.get_content();

                if !codeblock.starts_with("```") || !codeblock.ends_with("```") {
                    bail!("Please provide a valid codeblock.");
                }

                let (programming_language, code) = codeblock
                    .trim_matches('`')
                    .split_once('\n')
                    .context("Please provide a codeblock with a valid programming language.")?;

                let runtime = Piston::get_runtime(programming_language).context("Programming language not found.")?;
                ctx.defer(false).await?;

                let code = code.trim();
                let mut piston = Piston::new(runtime, Some(code), None::<String>, None::<String>);

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
