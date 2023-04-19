use crate::{
    statics::emojis::{ERROR_EMOJI, SUCCESS_EMOJI},
    structs::tags::Tags,
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder, MessageResponse};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    res.send_message(
        MessageResponse::from(
            match Tags::new()
                .edit(
                    input.get_string_arg("tag")?,
                    input.guild_id.as_ref().unwrap(),
                    input.get_string_arg("name").ok(),
                    input.get_string_arg("content").ok(),
                    input.get_bool_arg("nsfw").ok(),
                    input.member.unwrap(),
                )
                .await
            {
                Ok(response) => format!("{SUCCESS_EMOJI} {response}"),
                Err(error) => format!("{ERROR_EMOJI} {error}"),
            },
        )
        .set_ephemeral(true),
    )
    .await?;

    Ok(())
}
