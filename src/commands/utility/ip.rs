use crate::{
    structs::{api::ip_info::IPInfo, interaction::Interaction},
    traits::ArgGetters,
};
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
        let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

        match IPInfo::get(input.get_string_arg("ip")?).await {
            Ok(ip_info) => interaction.respond(ip_info.format(), false).await?,
            Err(error) => interaction.respond_error(error, true).await?,
        };
    }

    ip
}
