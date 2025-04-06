use crate::structs::{
    api::google::Google,
    command_context::{AeonCommandContext, AeonCommandInput},
};
use anyhow::{Context, Result};
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let (record_type, domain) = match &ctx.command_input {
        AeonCommandInput::ApplicationCommand(..) => (ctx.get_string_arg("type")?, ctx.get_string_arg("domain")?),
        AeonCommandInput::MessageCommand(_, args, _) => {
            let mut args = args.split_whitespace();
            let record_type = args.next().map(|arg| arg.to_uppercase()).context("Please provide a record type.")?;
            let domain = args.next().map(|arg| arg.to_string()).context("Please provide a domain.")?;
            (record_type, domain)
        },
    };

    let dns = Google::query_dns(record_type, domain).await?;
    ctx.respond(dns.format(), false).await
}
