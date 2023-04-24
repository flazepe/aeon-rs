use crate::{
    statics::MONGODB,
    structs::{interaction::Interaction, reminders::Reminder},
    traits::ArgGetters,
};
use anyhow::Result;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use slashook::{
    commands::{CommandInput, CommandResponder},
    structs::interactions::ApplicationCommandOptionChoice,
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let reminders = MONGODB.get().unwrap().collection::<Reminder>("reminders");

    let entries = reminders
        .find(doc! { "user_id": input.user.id.to_string() }, None)
        .await?
        .try_collect::<Vec<Reminder>>()
        .await?;

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
            reminders.delete_one(doc! { "_id": entry._id }, None).await?;
            interaction.respond_success("Gone.", true).await
        },
        None => {
            interaction
                .respond_error("Invalid entry. Make sure it's a valid number.", true)
                .await
        },
    }
}
