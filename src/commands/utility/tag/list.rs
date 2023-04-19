use crate::{
    functions::limit_string,
    statics::{colors::PRIMARY_COLOR, emojis::ERROR_EMOJI},
    structs::tags::Tags,
    traits::{ArgGetters, AvatarURL, Tag},
};
use anyhow::Result;
use slashook::{
    commands::{CommandInput, CommandResponder, MessageResponse},
    structs::embeds::Embed,
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let response = MessageResponse::from("").set_ephemeral(true);
    let author = input.get_user_arg("author").ok();

    res.send_message(
        match Tags::new()
            .search(input.guild_id.as_ref().unwrap(), author.map(|user| &user.id))
            .await
        {
            Ok(tags) => response.add_embed(
                Embed::new()
                    .set_color(PRIMARY_COLOR)?
                    .set_thumbnail(author.map_or("".into(), |author| author.display_avatar_url("png", 512)))
                    .set_title(author.map_or("All tags".into(), |author| format!("{}'s tags", author.tag())))
                    .set_description(limit_string(
                        tags.iter()
                            .filter(|tag| {
                                format!("{}{}", tag.name, tag.content)
                                    .contains(&input.get_string_arg("query").unwrap_or("".into()))
                            })
                            .map(|tag| format!("`{}`", tag.name))
                            .collect::<Vec<String>>()
                            .join(", "),
                        ", ",
                        4096,
                    )),
            ),
            Err(error) => response.set_content(format!("{ERROR_EMOJI} {error}")),
        },
    )
    .await?;

    Ok(())
}
