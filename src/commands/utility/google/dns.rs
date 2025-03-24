use crate::structs::{
    api::google::Google,
    command_context::{CommandContext, CommandInputExt, Input},
};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let (record_type, domain) = match &ctx.input {
        Input::ApplicationCommand { input, res: _ } => (input.get_string_arg("type")?, input.get_string_arg("domain")?),
        Input::MessageCommand { message: _, sender: _, args } => {
            let mut args = args.split(' ').filter(|entry| !entry.is_empty());

            let Some(record_type) = args.next().map(|arg| arg.to_uppercase()) else {
                return ctx.respond_error("Please provide a record type.", true).await;
            };

            let Some(domain) = args.next().map(|arg| arg.to_string()) else {
                return ctx.respond_error("Please provide a domain.", true).await;
            };

            (record_type, domain)
        },
    };

    match Google::query_dns(record_type, domain).await {
        Ok(records) => ctx.respond(records.format(), false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
