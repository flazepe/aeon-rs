use crate::{
    statics::emojis::{ERROR_EMOJI, SUCCESS_EMOJI},
    structs::tags::Tags,
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder, MessageResponse};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let tag = input.get_string_arg("tag")?;
    let alias = input.get_string_arg("alias")?;

    res.send_message(
        MessageResponse::from(
            match Tags::new()
                .toggle_alias(&tag, input.guild_id.as_ref().unwrap(), &alias, input.member.unwrap())
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
