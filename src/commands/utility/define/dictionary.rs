use crate::structs::{
    api::dictionary::Dictionary,
    command_context::{AeonCommandContext, AeonCommandInput},
};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let ephemeral = match &ctx.command_input {
        AeonCommandInput::ApplicationCommand(..) => !ctx.get_bool_arg("show").unwrap_or(false),
        AeonCommandInput::MessageCommand(..) => false,
    };

    let dictionary = Dictionary::search(ctx.get_string_arg("word", 0, true)?).await?;
    ctx.respond(dictionary.format(), ephemeral).await
}
