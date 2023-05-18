use crate::{
    structs::{api::google::Google, command_context::CommandContext},
    traits::ArgGetters,
};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    match Google::query_dns(ctx.input.get_string_arg("type")?, ctx.input.get_string_arg("domain")?).await {
        Ok(records) => ctx.respond(records.format(), false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
