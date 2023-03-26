use crate::{
    statics::emojis::{ERROR_EMOJI, SUCCESS_EMOJI},
    traits::ArgGetters,
};
use anyhow::Result;
use serde_json::json;
use slashook::{
    commands::{CommandInput, CommandResponder},
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
            res.send_message(format!("{SUCCESS_EMOJI} Removed timeout for <@{}>.", user.id))
                .await?;
        },
        Err(error) => {
            res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
        },
    }

    Ok(())
}
