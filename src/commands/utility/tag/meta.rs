use crate::{
    functions::format_timestamp,
    macros::yes_no,
    statics::REST,
    structs::{command_context::AeonCommandContext, database::tags::Tags},
};
use anyhow::Result;
use slashook::structs::users::User;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let Some(guild_id) = ctx.get_guild_id() else { return Ok(()) };

    let tag = Tags::get(ctx.get_string_arg("tag", 0, true)?, guild_id).await?;
    let name = tag.name;
    let author_id = tag.author_id;
    let author = REST.get::<User>(format!("users/{author_id}")).await.map(|user| user.username).unwrap_or_else(|_| "N/A".into());
    let created_timestamp = format_timestamp(tag.created_timestamp, true);
    let updated_timestamp = format_timestamp(tag.updated_timestamp, true);
    let nsfw = yes_no!(tag.nsfw);
    let mut aliases = tag.aliases.iter().map(|alias| format!("`{alias}`")).collect::<Vec<String>>().join(", ");

    if aliases.is_empty() {
        aliases = "None".into()
    }

    ctx.respond(format!("Tag `{name}` was created by {author} ({author_id}) at {created_timestamp}.\nAliases: {aliases}\nNSFW: {nsfw}\n\nLast updated {updated_timestamp}."), true).await
}
