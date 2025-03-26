use crate::structs::{
    api::steam::Steam,
    command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt},
};
use anyhow::{Result, bail};
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let id = match &ctx.command_input {
        AeonCommandInput::ApplicationCommand(input, _) => input.get_string_arg("user")?,
        AeonCommandInput::MessageCommand(_, args, _) => args.into(),
    };

    if id.is_empty() {
        bail!("Please provide a user.");
    }

    let user = Steam::get_user(id).await?;
    ctx.respond(user.format(), false).await
}
