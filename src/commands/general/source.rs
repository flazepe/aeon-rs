use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
};

use crate::structs::interaction::Interaction;

pub fn get_command() -> Command {
    #[command(name = "source", description = "Sends my source.")]
    async fn source(input: CommandInput, res: CommandResponder) {
        Interaction::new(&input, &res)
            .respond("<https://github.com/flazepe/aeon-rs>", false)
            .await?;
    }

    source
}
