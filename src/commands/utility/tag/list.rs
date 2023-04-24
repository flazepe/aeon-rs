use crate::{
    functions::limit_string,
    statics::colors::PRIMARY_COLOR,
    structs::{interaction::Interaction, tags::Tags},
    traits::{ArgGetters, AvatarURL, Tag},
};
use anyhow::Result;
use slashook::{
    commands::{CommandInput, CommandResponder},
    structs::embeds::Embed,
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let interaction = Interaction::new(&input, &res);
    let author = input.get_user_arg("author").ok();

    match Tags::new()
        .search(input.guild_id.as_ref().unwrap(), author.map(|user| &user.id))
        .await
    {
        Ok(tags) => {
            interaction
                .respond(
                    Embed::new()
                        .set_color(PRIMARY_COLOR)?
                        .set_thumbnail(author.map_or("".into(), |author| author.display_avatar_url("png", 512)))
                        .set_title(author.map_or("All tags".into(), |author| format!("{}'s tags", author.tag())))
                        .set_description(limit_string(
                            tags.iter()
                                .filter(|tag| {
                                    format!("{}{}", tag.name, tag.content)
                                        .to_lowercase()
                                        .contains(&input.get_string_arg("query").unwrap_or("".into()).to_lowercase())
                                })
                                .map(|tag| format!("`{}`", tag.name))
                                .collect::<Vec<String>>()
                                .join(", "),
                            ", ",
                            4096,
                        )),
                    false,
                )
                .await
        },
        Err(error) => interaction.respond_error(error, true).await,
    }
}
