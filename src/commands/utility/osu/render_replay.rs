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

    let url = input
        .get_string_arg("replay-url")
        .ok()
        .unwrap_or(input.get_attachment_arg("replay-file").map_or("".into(), |attachment| attachment.url.to_string()));

    if url.is_empty() {
        return interaction.respond_error("Please provide a replay URL or file.", true).await;
    }

    match OrdrRender::new(url, input.get_string_arg("skin").ok()).await {
        Ok(render) => {
            res.defer(false).await?;
            render.poll_result(&input, &res).await
        },
        Err(error) => interaction.respond_error(error, true).await,
    }
}
