use crate::{
    functions::limit_strings,
    statics::colors::PRIMARY_EMBED_COLOR,
    structs::{
        command_context::{AeonCommandContext, AeonCommandInput},
        database::tags::Tags,
    },
    traits::UserExt,
};
use anyhow::Result;
use slashook::structs::embeds::Embed;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let (query, author, guild_id) = match &ctx.command_input {
        AeonCommandInput::ApplicationCommand(input, _) => {
            (ctx.get_string_arg("query").as_deref().unwrap_or("").to_lowercase(), ctx.get_user_arg("author").ok(), input.guild_id.clone())
        },
        AeonCommandInput::MessageCommand(message, args, _) => (args.into(), None, message.guild_id.map(|guild_id| guild_id.to_string())),
    };

    let Some(guild_id) = guild_id else { return Ok(()) };
    let tags = Tags::search(guild_id, author.map(|user| &user.id)).await?;

    let thumbnail = author.map(|author| author.display_avatar_url("png", 512));
    let title = author.map(|author| format!("{}'s tags", author.username));
    let description = limit_strings(
        tags.iter()
            .filter(|tag| format!("{}{}", tag.name, tag.content).to_lowercase().contains(&query))
            .map(|tag| format!("`{}`", tag.name)),
        ", ",
        4096,
    );
    let embed = Embed::new()
        .set_color(PRIMARY_EMBED_COLOR)?
        .set_thumbnail(thumbnail.as_deref().unwrap_or(""))
        .set_title(title.as_deref().unwrap_or("All tags"))
        .set_description(description);

    ctx.respond(embed, true).await
}
