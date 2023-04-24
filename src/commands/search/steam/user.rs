use crate::{
    structs::{api::steam::Steam, interaction::Interaction},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let interaction = Interaction::new(&input, &res);

    match Steam::get_user(input.get_string_arg("user")?).await {
        Ok(user) => interaction.respond(user.format(), false).await,
        Err(error) => interaction.respond_error(error, true).await,
    }
}
