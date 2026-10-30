use crate::{
    statics::{MONGODB, REDIS, REQWEST},
    structs::{
        api::piston::{Piston, PistonState},
        command_context::{AeonCommandContext, AeonCommandInput},
        database::{mongodb::code_snippets::CodeSnippet, redis::keys::RedisKey},
    },
};
use anyhow::{Context, Result, ensure};
use slashook::{
    chrono::Utc,
    commands::Modal,
    structs::components::{Checkbox, Components, FileUpload, Label, TextInput, TextInputStyle},
};
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    match &ctx.command_input {
        AeonCommandInput::ApplicationCommand(input, res) => {
            let redis = REDIS.get().context("Could not get Redis.")?;
            let key = RedisKey::PistonStates(ctx.get_user_id());
            let mut piston_state = redis.get::<PistonState>(&key).await.unwrap_or_default();

            let mongodb = MONGODB.get().unwrap();
            let name = ctx.get_string_arg("name", 0, true).ok().and_then(|name| {
                let normalized_name = name.trim().to_lowercase().split_whitespace().collect::<Vec<_>>().join(" ");
                if normalized_name.is_empty() { None } else { Some(normalized_name) }
            });
            let code_snippet = if let Some(name) = &name { mongodb.code_snippets.get(name, ctx.get_user_id()).await } else { None };

            let programming_language = ctx
                .get_string_arg("programming-language", 0, true)
                .ok()
                .or_else(|| code_snippet.as_ref().and_then(|code_snippet| code_snippet.piston_state.runtime.clone()))
                .or_else(|| piston_state.runtime.clone())
                .context("Please provide a programming language.")?;
            let runtime = Piston::get_runtime(&programming_language).context("Programming language not found.")?;

            piston_state.runtime = Some(runtime.to_string());
            redis.set(&key, &piston_state, Some(60 * 60)).await?;

            if input.is_modal_submit() {
                ctx.defer(false).await?;

                let code = ctx.get_string_arg("code", 0, true).ok();
                let stdin = ctx.get_string_arg("stdin", 0, true).ok();
                let args = ctx.get_string_arg("args", 0, true).ok();

                // This bool only exists if it was a non-code snippet modal submission
                if let Ok(keep_state) = ctx.get_bool_arg("keep-state") {
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
                }

                // Exising code snippets should be saved or upserted if a name was provided
                if let Some(name) = name {
                    let new_code_snippet = CodeSnippet {
                        name,
                        user_id: ctx.get_user_id(),
                        piston_state: PistonState {
                            runtime: Some(runtime.to_string()),
                            code: code.clone(),
                            stdin: stdin.clone(),
                            args: args.clone(),
                        },
                    };

                    mongodb.code_snippets.upsert(new_code_snippet).await?;
                }

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
                ctx.respond(piston.format_pagination("code").save().await?, false).await?;
            } else {
                let mut code = piston_state.code;
                let mut stdin = piston_state.stdin;
                let mut args = piston_state.args;

                // Code snippets should have either data from the existing code snippet or empty fields to start with
                if name.is_some() {
                    if let Some(code_snippet) = code_snippet {
                        code = code_snippet.piston_state.code;
                        stdin = code_snippet.piston_state.stdin;
                        args = code_snippet.piston_state.args;
                    } else {
                        code = None;
                        stdin = None;
                        args = None;
                    }
                }

                let code_input = TextInput::new()
                    .set_id("code")
                    .set_style(TextInputStyle::PARAGRAPH)
                    .set_required(false)
                    .set_value(code.as_deref().unwrap_or_default());
                let stdin_input = TextInput::new()
                    .set_id("stdin")
                    .set_style(TextInputStyle::PARAGRAPH)
                    .set_required(false)
                    .set_value(stdin.as_deref().unwrap_or_default());
                let args_input = TextInput::new()
                    .set_id("args")
                    .set_required(false)
                    .set_value(args.as_deref().unwrap_or_default())
                    .set_placeholder(r#"e.g., ["foo", "bar"]"#);
                let files_input = FileUpload::new().set_id("files").set_max_values(10).set_required(false);

                let mut components = Components::empty();

                if name.is_some() {
                    /*
                        Only show the name input if it was a code snippet. This allows the user
                        to either empty it to avoid saving or to save it under a different name.
                    */
                    let snippet_name_input = TextInput::new()
                        .set_id("name")
                        .set_required(false)
                        .set_value(name.as_deref().unwrap_or_default())
                        .set_placeholder("Case-insensitive snippet name");
                    components = components
                        .add_label(Label::new("Name").set_description("Empty this field to avoid saving"))
                        .add_text_input(snippet_name_input);
                } else {
                    // Otherwise, for non-code snippets, we give them an option to keep the state for next time (true by default)
                    let keep_state_checkbox = Checkbox::new().set_id("keep-state").set_default(true);
                    components = components.add_label(Label::new("Keep state for next time")).add_checkbox(keep_state_checkbox);
                }

                components = components
                    .add_label(Label::new("Code").set_description(format!("Runtime: {runtime}")))
                    .add_text_input(code_input)
                    .add_label(Label::new("Stdin").set_description("Stdin text to provide to the program"))
                    .add_text_input(stdin_input)
                    .add_label(Label::new("Args").set_description("Array of args to provide to the program"))
                    .add_text_input(args_input)
                    .add_label(Label::new("Files").set_description("Upload files that the program can have access to"))
                    .add_file_upload(files_input);

                /*
                    Modals can be glitchy (sending old states, fields being empty when they should not, etc.)
                    if I only provide the same custom IDs. The timestamp is added to force Discord to send a new
                    modal every time to prevent such weird behaviors.
                */
                let custom_id = format!("run/{}", Utc::now().timestamp());
                let modal = Modal::new("code", custom_id, "Enter Code").set_components(components);
                res.open_modal(modal).await?;
            }
        },
        AeonCommandInput::MessageCommand(message, args, _) => {
            let codeblock = args.get_content();
            ensure!(codeblock.starts_with("```") && codeblock.ends_with("```"), "Please provide a valid codeblock.");

            let (programming_language, code) =
                codeblock.trim_matches('`').split_once('\n').context("Please provide a codeblock with a valid programming language.")?;

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
            ctx.respond(piston.format_pagination("code").save().await?, false).await?;
        },
    }

    Ok(())
}
