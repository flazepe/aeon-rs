use crate::{
    functions::{format_timestamp, TimestampFormat},
    macros::{if_else, yes_no},
    statics::emojis::ERROR_EMOJI,
    structs::tags::Tags,
    traits::{ArgGetters, Tag},
};
use anyhow::Result;
use slashook::{
    commands::{CommandInput, CommandResponder, MessageResponse},
    structs::users::User,
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    res.send_message(
        MessageResponse::from(
            match Tags::new()
                .get(input.get_string_arg("tag")?, input.guild_id.unwrap())
                .await
            {
                Ok(tag) => {
                    format!(
                        "Tag `{}` was created by {} ({}) at {}.\nAliases: {}\nNSFW: {}\n\nLast updated {}.",
                        tag.name,
                        input
                            .rest
                            .get::<User>(format!("users/{}", tag.author_id))
                            .await
                            .ok()
                            .map_or("N/A".into(), |user| user.tag()),
                        tag.author_id,
                        format_timestamp(tag.created_timestamp, TimestampFormat::Full),
                        {
                            let aliases = tag
                                .aliases
                                .iter()
                                .map(|alias| format!("`{alias}`"))
                                .collect::<Vec<String>>()
                                .join(", ");

                            if_else!(aliases.is_empty(), "None".into(), aliases)
                        },
                        yes_no!(tag.nsfw),
                        format_timestamp(tag.updated_timestamp, TimestampFormat::Full)
                    )
                },
                Err(error) => format!("{ERROR_EMOJI} {error}"),
            },
        )
        .set_ephemeral(true),
    )
    .await?;

    Ok(())
}
