use crate::{
    functions::limit_strings,
    statics::colors::PRIMARY_COLOR,
    structs::{command_context::CommandContext, database::tags::Tags},
    traits::{AvatarUrl, Tag},
};
use anyhow::Result;
use slashook::structs::embeds::Embed;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let author = ctx.get_user_arg("author").ok();

    match Tags::search(ctx.input.guild_id.as_ref().unwrap(), author.map(|user| &user.id)).await {
        Ok(tags) => {
            ctx.respond(
                Embed::new()
                    .set_color(PRIMARY_COLOR)?
                    .set_thumbnail(author.map_or("".into(), |author| author.display_avatar_url("png", 512)))
                    .set_title(author.map_or("All tags".into(), |author| format!("{}'s tags", author.tag())))
                    .set_description(limit_strings(
                        tags.iter()
                            .filter(|tag| {
                                format!("{}{}", tag.name, tag.content)
                                    .to_lowercase()
                                    .contains(&ctx.get_string_arg("query").unwrap_or("".into()).to_lowercase())
                            })
                            .map(|tag| format!("`{}`", tag.name)),
                        ", ",
                        4096,
                    )),
                true,
            )
            .await
        },
        Err(error) => ctx.respond_error(error, true).await,
    }
}
