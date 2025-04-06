use crate::structs::{
    api::urban_dictionary::UrbanDictionary,
    command_context::{AeonCommandContext, AeonCommandInput},
};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let ephemeral = match &ctx.command_input {
        AeonCommandInput::ApplicationCommand(..) => !ctx.get_bool_arg("show").unwrap_or(false),
        AeonCommandInput::MessageCommand(..) => false,
    };

    let urban_dictionary = UrbanDictionary::search(ctx.get_string_arg("word")?).await?;
    ctx.respond(urban_dictionary.format(), ephemeral).await
}
