use crate::structs::{
    api::google::{statics::GOOGLE_TRANSLATE_LANGUAGES, Google},
    command_context::CommandContext,
};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    if ctx.input.is_autocomplete() {
        return ctx.autocomplete(GOOGLE_TRANSLATE_LANGUAGES.iter()).await;
    }

    match Google::translate(
        ctx.get_string_arg("text")?,
        ctx.get_string_arg("origin-language").unwrap_or("auto".into()),
        ctx.get_string_arg("target-language").unwrap_or("en".into()),
    )
    .await
    {
        Ok(translation) => ctx.respond(translation.format(), false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
