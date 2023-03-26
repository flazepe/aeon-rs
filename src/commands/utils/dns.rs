use crate::{statics::emojis::*, structs::api::google_dns::*, traits::*};
use slashook::{command, commands::*, structs::interactions::*};

pub fn get_command() -> Command {
    #[command(
        name = "dns",
        description = "Fetches DNS records of a domain.",
        options = [
            {
                name = "type",
                description = "The record type, such as A, AAAA, MX, NS, PTR, etc.",
                option_type = InteractionOptionType::STRING,
                required = true,
            },
            {
                name = "url",
                description = "The URL",
                option_type = InteractionOptionType::STRING,
                required = true,
            },
        ],
    )]
    fn dns(input: CommandInput, res: CommandResponder) {
        match GoogleDNS::query(input.get_string_arg("type")?, input.get_string_arg("url")?).await {
            Ok(records) => {
                res.send_message(records.format()).await?;
            },
            Err(error) => {
                res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
            },
        };
    }

    dns
}
