use crate::{
    structs::{duration::Duration, interaction::Interaction, reminders::Reminders},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

    res.defer(input.is_string_select()).await?;
    let original_message = res.get_original_message().await?;

    // Delete snoozed reminder
    if let Some(message) = input.message.as_ref() {
        if message.interaction.is_none() {
            input.rest.delete::<()>(format!("channels/{}/messages/{}", message.channel_id, message.id)).await.ok();
        }
    }

    let url = input.custom_id.as_ref().map_or_else(
        || format!("{}/{}/{}", input.guild_id.as_ref().unwrap_or(&"@me".into()), input.channel_id.as_ref().unwrap(), original_message.id),
        |custom_id| custom_id.to_string(),
    );

    let reminder = {
        let mut reminder = input.get_string_arg("reminder").unwrap_or("Do something".into());

        if input.is_string_select() {
            if let Some(parsed_reminder) = || -> Option<&String> { input.message.as_ref()?.embeds.get(0)?.description.as_ref() }() {
                reminder = parsed_reminder.to_string();
            };
        }

        reminder
    };

    let dm = input.message.as_ref().map_or(
        false,
        // DM if select menu's message was from an interaction
        |message| message.interaction.is_some(),
    ) || input.guild_id.is_none()
        || input.get_bool_arg("dm").unwrap_or(false);

    match Reminders::new()
        .set(
            &input.user.id,
            &url,
            Duration::new()
                .parse(
                    input.values.as_ref().map_or_else(|| input.get_string_arg("time").unwrap_or("".into()), |values| values[0].to_string()),
                )
                .unwrap_or(Duration::new()),
            Duration::new().parse(input.get_string_arg("interval").unwrap_or("".into())).unwrap_or(Duration::new()),
            &reminder,
            dm,
        )
        .await
    {
        Ok(response) => interaction.respond_success(response, false).await,
        Err(error) => interaction.respond_error(error, true).await,
    }
}
