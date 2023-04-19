use crate::{statics::emojis::ERROR_EMOJI, structs::tags::Tags, traits::ArgGetters};
use anyhow::Result;
use slashook::{
    commands::{CommandInput, CommandResponder},
    structs::channels::Channel,
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    match Tags::new()
        .get(input.get_string_arg("tag")?, input.guild_id.unwrap())
        .await
    {
        Ok(tag) => {
            if let Some(channel) = Channel::fetch(&input.rest, input.channel_id.unwrap()).await.ok() {
                if !channel.nsfw.unwrap_or(false) && tag.nsfw {
                    res.send_message(format!("{ERROR_EMOJI} Tag is for NSFW channels only."))
                        .await?;

                    return Ok(());
                }
            }

            res.send_message(tag.content).await?
        },
        Err(error) => res.send_message(format!("{ERROR_EMOJI} {error}")).await?,
    };

    Ok(())
}
