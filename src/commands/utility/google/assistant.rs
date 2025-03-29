use crate::structs::{
    api::google::Google,
    command_context::{AeonCommandContext, AeonCommandInput},
};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let query = match ctx.get_string_arg("query") {
        Ok(query) => query,
        Err(error) => {
            let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Err(error) };
            input.values.as_ref().unwrap()[0].clone()
        },
    };

    ctx.defer(false).await?;

    let google_assistant = Google::assistant(query).await?;
    ctx.respond(google_assistant.format(), false).await
}
