use crate::structs::{
    command::AeonCommand, command_context::CommandContext, stringified_message::StringifiedMessage, unicode::UnicodeCharacters,
};
use anyhow::Result;
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::ApplicationCommandType,
};

pub fn get_command() -> Command {
    #[command(
        name = "List Unicodes",
        command_type = ApplicationCommandType::MESSAGE,
    )]
    async fn unicode_message(input: CommandInput, res: CommandResponder) {
        AeonCommand::new(input, res).main(run).run().await?;
    }

    unicode_message
}

async fn run(ctx: CommandContext) -> Result<()> {
    ctx.respond(UnicodeCharacters::get(StringifiedMessage::from(ctx.input.target_message.as_ref().unwrap().clone())).format(), false).await
}
