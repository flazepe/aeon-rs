use crate::structs::{command_context::CommandContext, database::reminders::Reminders};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let reminders = Reminders::new();
    let entries = reminders.get_many(&ctx.input.user.id).await.unwrap_or(vec![]);

    if ctx.input.is_autocomplete() {
        return ctx
            .autocomplete(entries.iter().enumerate().map(|(index, entry)| {
                ((index + 1).to_string(), format!("{}. {}", index + 1, entry.reminder).chars().take(100).collect::<String>())
            }))
            .await;
    }

    match entries.get(match ctx.get_string_arg("entry")?.parse::<usize>() {
        Ok(index) => index - 1,
        Err(_) => return ctx.respond_error("Please enter a valid number.", true).await,
    }) {
        Some(entry) => {
            reminders.delete(entry._id).await?;
            ctx.respond_success("Gone.", true).await
        },
        None => ctx.respond_error("Invalid entry.", true).await,
    }
}
