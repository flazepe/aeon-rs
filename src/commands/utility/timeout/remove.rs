use crate::{structs::interaction::Interaction, traits::ArgGetters};
use anyhow::Result;
use serde_json::json;
use slashook::{
    commands::{CommandInput, CommandResponder},
    structs::guilds::GuildMember,
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let interaction = Interaction::new(&input, &res);
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
            interaction
                .respond_success(format!("Removed timeout for {}.", user.mention()), false)
                .await?
        },
        Err(error) => interaction.respond_error(error, true).await?,
    };

    Ok(())
}
