use crate::{
    structs::{database::reminders::Reminders, interaction::Interaction},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::{
    commands::{CommandInput, CommandResponder},
    structs::interactions::ApplicationCommandOptionChoice,
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };
    let reminders = Reminders::new();
    let entries = reminders.get_many(&input.user.id).await.unwrap_or(vec![]);

    if input.is_autocomplete() {
        return Ok(res
            .autocomplete(
                entries
                    .iter()
                    .enumerate()
                    .map(|(index, entry)| {
                        ApplicationCommandOptionChoice::new(
                            format!("{}. {}", index + 1, entry.reminder).chars().take(100).collect::<String>(),
                            (index + 1).to_string(),
                        )
                    })
                    .collect::<Vec<ApplicationCommandOptionChoice>>(),
            )
            .await?);
    }

    match entries.get(match input.get_string_arg("entry")?.parse::<usize>() {
        Ok(index) => index - 1,
        Err(_) => return interaction.respond_error("Please enter a valid number.", true).await,
    }) {
        Some(entry) => {
            reminders.delete(entry._id).await?;
            interaction.respond_success("Gone.", true).await
        },
        None => interaction.respond_error("Invalid entry.", true).await,
    }
}
