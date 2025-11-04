use crate::structs::{
    command_context::{AeonCommandContext, AeonCommandInput},
    database::Database,
};
use anyhow::{Context, Result};
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };

    let mongodb = Database::get_mongodb()?;
    let reminders = mongodb.reminders.get_many(&input.user.id).await.unwrap_or_else(|_| vec![]);

    if input.is_autocomplete() {
        let options = reminders.iter().enumerate().map(|(index, reminder)| {
            ((index + 1).to_string(), format!("{}. {}", index + 1, reminder.reminder).chars().take(100).collect::<String>())
        });

        return ctx.autocomplete(options).await;
    }

    let index = ctx.get_string_arg("reminder", 0, true)?.parse::<usize>().context("Please enter a valid number.")? - 1;
    let reminder = reminders.get(index).context("Invalid reminder.")?;

    mongodb.reminders.delete(reminder._id).await?;
    ctx.respond_success("Gone.", true).await
}
