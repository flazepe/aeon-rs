use crate::{
    functions::limit_strings,
    statics::colors::PRIMARY_COLOR,
    structs::{command_context::CommandContext, database::tags::Tags},
    traits::UserExt,
};
use anyhow::Result;
use slashook::structs::embeds::Embed;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let guild_id = ctx.input.guild_id.as_ref().unwrap();
    let author = ctx.get_user_arg("author").ok();

    match Tags::search(guild_id, author.map(|user| &user.id)).await {
        Ok(tags) => {
            let thumbnail = author.map(|author| author.display_avatar_url("png", 512)).unwrap_or_else(|| "".into());
            let title = author.map(|author| format!("{}'s tags", author.username)).unwrap_or_else(|| "All tags".into());
            let tags = limit_strings(
                tags.iter()
                    .filter(|tag| {
                        format!("{}{}", tag.name, tag.content)
                            .to_lowercase()
                            .contains(&ctx.get_string_arg("query").as_deref().unwrap_or("").to_lowercase())
                    })
                    .map(|tag| format!("`{}`", tag.name)),
                ", ",
                4096,
            );
            let embed = Embed::new().set_color(PRIMARY_COLOR)?.set_thumbnail(thumbnail).set_title(title).set_description(tags);

            ctx.respond(embed, true).await
        },
        Err(error) => ctx.respond_error(error, true).await,
    }
}
