use crate::{statics::emojis::ERROR_EMOJI, structs::api::google::Google, traits::ArgGetters};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    match Google::query_dns(input.get_string_arg("type")?, input.get_string_arg("domain")?).await {
        Ok(records) => res.send_message(records.format()).await?,
        Err(error) => res.send_message(format!("{ERROR_EMOJI} {error}")).await?,
    };

    Ok(())
}