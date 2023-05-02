use crate::{
    structs::{database::tags::Tags, interaction::Interaction},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::{
    commands::{CommandInput, CommandResponder, MessageResponse},
    structs::channels::{AllowedMentions, Channel},
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

    match Tags::new().get(input.get_string_arg("tag")?, input.guild_id.as_ref().unwrap()).await {
        Ok(tag) => {
            if tag.nsfw {
                if !Channel::fetch(&input.rest, input.channel_id.as_ref().unwrap())
                    .await
                    .map_or(false, |channel| channel.nsfw.unwrap_or(false))
                {
                    return interaction.respond_error("NSFW channels only.", true).await;
                }
            }

            interaction.respond(MessageResponse::from(tag.content).set_allowed_mentions(AllowedMentions::new()), false).await
        },
        Err(error) => interaction.respond_error(error, true).await,
    }
}
