use crate::{statics::emojis::ERROR_EMOJI, structs::api::ip_info::IPInfo, traits::ArgGetters};
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

pub fn get_command() -> Command {
    #[command(
        name = "ip",
        description = "Fetches information based on the given IP address.",
        options = [
            {
                name = "ip",
                description = "The IP address",
                option_type = InteractionOptionType::STRING,
                required = true,
            },
        ],
    )]
    async fn ip(input: CommandInput, res: CommandResponder) {
        match IPInfo::get(input.get_string_arg("ip")?).await {
            Ok(ip_info) => res.send_message(ip_info.format()).await?,
            Err(error) => res.send_message(format!("{ERROR_EMOJI} {error}")).await?,
        };
    }

    ip
}
