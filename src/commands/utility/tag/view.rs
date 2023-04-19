use crate::{statics::emojis::ERROR_EMOJI, structs::tags::Tags, traits::ArgGetters};
use anyhow::Result;
use slashook::{
    commands::{CommandInput, CommandResponder, MessageResponse},
    structs::channels::Channel,
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    Ok(
        match Tags::new()
            .get(input.get_string_arg("tag")?, input.guild_id.as_ref().unwrap())
            .await
        {
            Ok(tag) => {
                if tag.nsfw {
                    if !Channel::fetch(&input.rest, input.channel_id.as_ref().unwrap())
                        .await
                        .map_or(false, |channel| channel.nsfw.unwrap_or(false))
                    {
                        res.send_message(format!("{ERROR_EMOJI} NSFW channels only.")).await?;

                        return Ok(());
                    }
                }

                res.send_message(tag.content).await?;
            },
            Err(error) => {
                res.send_message(MessageResponse::from(format!("{ERROR_EMOJI} {error}")).set_ephemeral(true))
                    .await?;
            },
        },
    )
}
