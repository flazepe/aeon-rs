use crate::structs::{api::google::Google, command_context::CommandContext};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    match Google::query_dns(ctx.get_string_arg("type")?, ctx.get_string_arg("domain")?).await {
        Ok(records) => ctx.respond(records.format(), false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
