use crate::{
    functions::limit_strings,
    statics::colors::PRIMARY_COLOR,
    structs::{command_context::CommandContext, database::tags::Tags},
    traits::UserExt,
};
use anyhow::Result;
use slashook::structs::embeds::Embed;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn run(ctx: CommandContext) -> Result<()> {
    let author = ctx.get_user_arg("author").ok();

    println!("Searching...");
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    match Tags::search(ctx.input.guild_id.as_ref().unwrap(), author.map(|user| &user.id)).await {
        Ok(tags) => {
            println!("Finished searching! Took {}s.", SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() - timestamp);

            ctx.respond(
                Embed::new()
                    .set_color(PRIMARY_COLOR)?
                    .set_thumbnail(author.map(|author| author.display_avatar_url("png", 512)).as_deref().unwrap_or(""))
                    .set_title(author.map(|author| format!("{}'s tags", author.username)).as_deref().unwrap_or("All tags"))
                    .set_description(limit_strings(
                        tags.iter()
                            .filter(|tag| {
                                format!("{}{}", tag.name, tag.content)
                                    .to_lowercase()
                                    .contains(&ctx.get_string_arg("query").as_deref().unwrap_or("").to_lowercase())
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
