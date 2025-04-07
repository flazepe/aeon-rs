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

    let origin_language = if let AeonCommandInput::MessageCommand(..) = &ctx.command_input {
        "auto".into()
    } else {
        ctx.get_string_arg("origin-language", 0, true).as_deref().unwrap_or("auto").to_string()
    };
    let target_language = ctx.get_string_arg("target-language", 0, false).as_deref().unwrap_or("en").to_string();

    let reference_text = if let AeonCommandInput::MessageCommand(message, ..) = &ctx.command_input {
        message.referenced_message.as_ref().map(|reply| SimpleMessage::from(*reply.clone()).to_string())
    } else {
        None
    };
    let text = match ctx.get_string_arg("text", 1, true) {
        Ok(text) => Ok(text),
        Err(error) => reference_text.context(error),
    }?;

    let translation = Google::translate(text, origin_language, target_language).await?;
    ctx.respond(translation.format(), false).await
}
