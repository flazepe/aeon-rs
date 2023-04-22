use crate::{
    statics::emojis::{ERROR_EMOJI, SUCCESS_EMOJI},
    traits::ArgGetters,
};
use anyhow::Result;
use serde_json::json;
use slashook::{
    commands::{CommandInput, CommandResponder, MessageResponse},
    structs::guilds::GuildMember,
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let user = input.get_user_arg("member")?;

    match input
        .rest
        .patch::<GuildMember, _>(
            format!("guilds/{}/members/{}", input.guild_id.as_ref().unwrap(), &user.id),
            json!({ "communication_disabled_until": null }),
        )
        .await
    {
        Ok(_) => {
            res.send_message(format!("{SUCCESS_EMOJI} Removed timeout for {}.", user.mention()))
                .await?
        },
        Err(error) => {
            res.send_message(MessageResponse::from(format!("{ERROR_EMOJI} {error}")).set_ephemeral(true))
                .await?
        },
    };

    Ok(())
}
