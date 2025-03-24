use crate::structs::{
    command_context::{CommandContext, CommandInputExt, Input},
    database::reminders::Reminders,
};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let Input::ApplicationCommand { input, res: _ } = &ctx.input else { return Ok(()) };
    let reminders = Reminders::get_many(&input.user.id).await.unwrap_or_else(|_| vec![]);

    if input.is_autocomplete() {
        let options = reminders.iter().enumerate().map(|(index, reminder)| {
            ((index + 1).to_string(), format!("{}. {}", index + 1, reminder.reminder).chars().take(100).collect::<String>())
        });

        return ctx.autocomplete(options).await;
    }

    let index = match input.get_string_arg("reminder")?.parse::<usize>() {
        Ok(index) => index - 1,
        Err(_) => return ctx.respond_error("Please enter a valid number.", true).await,
    };

    match reminders.get(index) {
        Some(reminder) => {
            Reminders::delete(reminder._id).await?;
            ctx.respond_success("Gone.", true).await
        },
        None => ctx.respond_error("Invalid reminder.", true).await,
    }
}
