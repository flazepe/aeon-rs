use crate::structs::{
    api::google::{Google, statics::GOOGLE_TRANSLATE_LANGUAGES},
    command_context::{AeonCommandContext, AeonCommandInput},
    simple_message::SimpleMessage,
};
use anyhow::{Context, Result};
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    if let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input {
        if input.is_autocomplete() {
            return ctx.autocomplete(GOOGLE_TRANSLATE_LANGUAGES.iter()).await;
        }
    }

    let (text, origin_language, target_language) = match &ctx.command_input {
        AeonCommandInput::ApplicationCommand(_, _) => (
            ctx.get_string_arg("text")?,
            ctx.get_string_arg("origin-language").as_deref().unwrap_or("auto").to_string(),
            ctx.get_string_arg("target-language").as_deref().unwrap_or("en").to_string(),
        ),
        AeonCommandInput::MessageCommand(message, args, _) => {
            let mut args = args.split_whitespace();
            let target_language = args.next().map(|arg| arg.to_string()).unwrap_or("en".into());
            let reference_text = message.referenced_message.as_ref().map(|reply| SimpleMessage::from(*reply.clone()).to_string());
            let text = args.next().map(|arg| arg.to_string()).or(reference_text).context("Please provide a text.")?;

            (text, "auto".into(), target_language)
        },
    };

    let translation = Google::translate(text, origin_language, target_language).await?;
    ctx.respond(translation.format(), false).await
}
