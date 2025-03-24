use crate::{
    functions::limit_strings,
    statics::colors::PRIMARY_COLOR,
    structs::{
        command_context::{CommandContext, CommandInputExt, Input},
        database::tags::Tags,
    },
    traits::UserExt,
};
use anyhow::Result;
use slashook::structs::embeds::Embed;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let Input::ApplicationCommand(input,  _) = &ctx.input else { return Ok(()) };
    let guild_id = input.guild_id.as_ref().unwrap();
    let author = input.get_user_arg("author").ok();

    match Tags::search(guild_id, author.map(|user| &user.id)).await {
        Ok(tags) => {
            let thumbnail = author.map(|author| author.display_avatar_url("png", 512));
            let title = author.map(|author| format!("{}'s tags", author.username));
            let tags = limit_strings(
                tags.iter()
                    .filter(|tag| {
                        format!("{}{}", tag.name, tag.content)
                            .to_lowercase()
                            .contains(&input.get_string_arg("query").as_deref().unwrap_or("").to_lowercase())
                    })
                    .map(|tag| format!("`{}`", tag.name)),
                ", ",
                4096,
            );
            let embed = Embed::new()
                .set_color(PRIMARY_COLOR)?
                .set_thumbnail(thumbnail.as_deref().unwrap_or(""))
                .set_title(title.as_deref().unwrap_or("All tags"))
                .set_description(tags);

            ctx.respond(embed, true).await
        },
        Err(error) => ctx.respond_error(error, true).await,
    }
}
