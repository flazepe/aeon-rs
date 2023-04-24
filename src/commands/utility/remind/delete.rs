use crate::{
    structs::{interaction::Interaction, reminders::Reminders},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::{
    commands::{CommandInput, CommandResponder},
    structs::interactions::ApplicationCommandOptionChoice,
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
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
                            format!("{}. {}", index + 1, entry.reminder)
                                .chars()
                                .take(100)
                                .collect::<String>(),
                            (index + 1).to_string(),
                        )
                    })
                    .collect::<Vec<ApplicationCommandOptionChoice>>(),
            )
            .await?);
    }

    let interaction = Interaction::new(&input, &res);

    match entries.get(input.get_string_arg("entry")?.parse::<usize>()? - 1) {
        Some(entry) => {
            reminders.delete(entry._id).await?;
            interaction.respond_success("Gone.", true).await
        },
        None => {
            interaction
                .respond_error("Invalid entry. Make sure it's a valid number.", true)
                .await
        },
    }
}
