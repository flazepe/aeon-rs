use crate::{statics::emojis::ERROR_EMOJI, structs::api::vndb::Vndb, traits::ArgGetters};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    match Vndb::new()
        .search_visual_novel(input.get_string_arg("visual-novel")?)
        .await
    {
        Ok(mut visual_novel) => {
            res.send_message(visual_novel.remove(0).format()).await?;
        },
        Err(error) => {
            res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
        },
    }

    Ok(())
}
