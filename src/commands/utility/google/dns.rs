use crate::structs::{
    api::google::Google,
    command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt},
};
use anyhow::{Result, bail};
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let (record_type, domain) = match &ctx.command_input {
        AeonCommandInput::ApplicationCommand(input, _) => (input.get_string_arg("type")?, input.get_string_arg("domain")?),
        AeonCommandInput::MessageCommand(_, args, _) => {
            let mut args = args.split_whitespace();
            let Some(record_type) = args.next().map(|arg| arg.to_uppercase()) else { bail!("Please provide a record type.") };
            let Some(domain) = args.next().map(|arg| arg.to_string()) else { bail!("Please provide a domain.") };
            (record_type, domain)
        },
    };

    let dns = Google::query_dns(record_type, domain).await?;
    ctx.respond(dns.format(), false).await
}
