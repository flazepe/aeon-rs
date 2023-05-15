use crate::{functions::add_reminder_select_options, structs::interaction::Interaction};
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::{
        components::{Components, SelectMenu, SelectMenuType},
        interactions::ApplicationCommandType,
    },
};

pub fn get_command() -> Command {
    #[command(
		name = "Remind me",
		command_type = ApplicationCommandType::MESSAGE,
	)]
    async fn reminder_set_message(input: CommandInput, res: CommandResponder) {
        let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

        interaction
            .respond(
                Components::new().add_select_menu(
                    add_reminder_select_options(SelectMenu::new(SelectMenuType::STRING))
                        .set_id(
                            "reminder",
                            format!(
                                "{}/{}/{}",
                                input.guild_id.as_ref().unwrap_or(&"@me".into()),
                                input.channel_id.as_ref().unwrap(),
                                input.target_message.as_ref().unwrap().id,
                            ),
                        )
                        .set_placeholder("Select time to remind about message"),
                ),
                true,
            )
            .await?;
    }

    reminder_set_message
}
