use crate::{
    functions::{format_timestamp, TimestampFormat},
    macros::yes_no,
    structs::{command_context::CommandContext, database::tags::Tags},
    traits::Tag,
};
use anyhow::Result;
use slashook::structs::users::User;

pub async fn run(ctx: CommandContext) -> Result<()> {
    match Tags::new().get(ctx.get_string_arg("tag")?, ctx.input.guild_id.as_ref().unwrap()).await {
        Ok(tag) => {
            let aliases = tag.aliases.iter().map(|alias| format!("`{alias}`")).collect::<Vec<String>>().join(", ");

            ctx.respond(
                format!(
                    "Tag `{}` was created by {} ({}) at {}.\nAliases: {}\nNSFW: {}\n\nLast updated {}.",
                    tag.name,
                    ctx.input.rest.get::<User>(format!("users/{}", tag.author_id)).await.ok().map_or("N/A".into(), |user| user.tag()),
                    tag.author_id,
                    format_timestamp(tag.created_timestamp, TimestampFormat::Full),
                    match aliases.is_empty() {
                        true => "None".into(),
                        false => aliases,
                    },
                    yes_no!(tag.nsfw),
                    format_timestamp(tag.updated_timestamp, TimestampFormat::Full),
                ),
                true,
            )
            .await
        },
        Err(error) => ctx.respond_error(error, true).await,
    }
}
