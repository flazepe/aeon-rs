use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
};

pub fn get_command() -> Command {
    #[command(name = "source", description = "Sends my source.")]
    async fn source(_: CommandInput, res: CommandResponder) {
        res.send_message("<https://github.com/flazepe/aeon-rs>").await?;
    }

    source
}
