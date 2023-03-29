use crate::macros::add_reminder_select_options;
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder, MessageResponse},
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
    async fn remind_message(input: CommandInput, res: CommandResponder) {
        res.send_message(MessageResponse::from(
            Components::new().add_select_menu(
                add_reminder_select_options!(SelectMenu::new(SelectMenuType::STRING))
                    .set_id(
                        "remind",
                        format!(
                            "{}/{}/{}",
                            input.guild_id.as_ref().unwrap_or(&"@me".into()),
                            input.channel_id.as_ref().unwrap(),
                            input.target_message.unwrap().id
                        ),
                    )
                    .set_placeholder("Select time to remind about message"),
            ),
        ))
        .await?;
    }

    remind_message
}
