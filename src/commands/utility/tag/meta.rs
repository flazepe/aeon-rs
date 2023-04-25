use crate::{
    functions::{format_timestamp, TimestampFormat},
    macros::yes_no,
    structs::{database::tags::Tags, interaction::Interaction},
    traits::{ArgGetters, Tag},
};
use anyhow::Result;
use slashook::{
    commands::{CommandInput, CommandResponder},
    structs::users::User,
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

    match Tags::new().get(input.get_string_arg("tag")?, input.guild_id.as_ref().unwrap()).await {
        Ok(tag) => {
            let aliases = tag.aliases.iter().map(|alias| format!("`{alias}`")).collect::<Vec<String>>().join(", ");

            interaction
                .respond(
                    format!(
                        "Tag `{}` was created by {} ({}) at {}.\nAliases: {}\nNSFW: {}\n\nLast updated {}.",
                        tag.name,
                        input.rest.get::<User>(format!("users/{}", tag.author_id)).await.ok().map_or("N/A".into(), |user| user.tag()),
                        tag.author_id,
                        format_timestamp(tag.created_timestamp, TimestampFormat::Full),
                        match aliases.is_empty() {
                            true => "None".into(),
                            false => aliases,
                        },
                        yes_no!(tag.nsfw),
                        format_timestamp(tag.updated_timestamp, TimestampFormat::Full)
                    ),
                    true,
                )
                .await
        },
        Err(error) => interaction.respond_error(error, true).await,
    }
}
