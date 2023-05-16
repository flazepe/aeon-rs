use crate::{
    statics::CACHE,
    structs::{
        api::ordr::{statics::ORDR_SKINS, OrdrRender},
        interaction::Interaction,
    },
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

    if input.is_autocomplete() {
        return interaction.hashmap_autocomplete(ORDR_SKINS.iter()).await;
    }

    if CACHE.ordr_rendering_users.read().unwrap().contains_key(&input.user.id) {
        return interaction.respond_error("You already have an ongoing replay rendering.", true).await;
    }

    res.defer(false).await?;

    match OrdrRender::new(
        match input.get_string_arg("replay-url").or(input.get_attachment_arg("replay-file").map(|attachment| attachment.url.clone())) {
            Ok(url) => url,
            Err(_) => return interaction.respond_error("Please provide an image URL or file.", true).await,
        },
        input.get_string_arg("skin").ok(),
    )
    .await
    {
        Ok(render) => render.poll_progress(&input, &res).await,
        Err(error) => interaction.respond_error(error, false).await,
    }
}
