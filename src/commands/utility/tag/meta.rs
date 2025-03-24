use crate::{
    functions::{TimestampFormat, format_timestamp},
    macros::yes_no,
    structs::{
        command_context::{CommandContext, CommandInputExt, Input},
        database::tags::Tags,
    },
};
use anyhow::Result;
use slashook::structs::users::User;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let Input::ApplicationCommand(input,  _) = &ctx.input else { return Ok(()) };

    match Tags::get(input.get_string_arg("tag")?, input.guild_id.as_ref().unwrap()).await {
        Ok(tag) => {
            let name = tag.name;
            let author_id = tag.author_id;
            let author =
                input.rest.get::<User>(format!("users/{author_id}")).await.map(|user| user.username).unwrap_or_else(|_| "N/A".into());
            let created_timestamp = format_timestamp(tag.created_timestamp, TimestampFormat::Full);
            let updated_timestamp = format_timestamp(tag.updated_timestamp, TimestampFormat::Full);
            let nsfw = yes_no!(tag.nsfw);
            let mut aliases = tag.aliases.iter().map(|alias| format!("`{alias}`")).collect::<Vec<String>>().join(", ");

            if aliases.is_empty() {
                aliases = "None".into()
            }

            ctx.respond(format!("Tag `{name}` was created by {author} ({author_id}) at {created_timestamp}.\nAliases: {aliases}\nNSFW: {nsfw}\n\nLast updated {updated_timestamp}."), true).await
        },
        Err(error) => ctx.respond_error(error, true).await,
    }
}
