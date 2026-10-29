use crate::{
    functions::paginate_strings,
    statics::{MONGODB, colors::PRIMARY_EMBED_COLOR},
    structs::{command_context::AeonCommandContext, pagination::Pagination},
};
use anyhow::Result;
use slashook::structs::embeds::Embed;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let mongodb = MONGODB.get().unwrap();
    let code_snippets = mongodb.code_snippets.get_all(ctx.get_user_id()).await?;
    let pages = paginate_strings(
        code_snippets.iter().map(|entry| {
            format!("- `{}` - {}", entry.piston_state.runtime.as_deref().unwrap_or("unknown runtime"), entry.name.replace('`', "\\`"))
        }),
        "\n",
        4096,
    )
    .iter()
    .map(|entry| Embed::new().set_color(PRIMARY_EMBED_COLOR).unwrap_or_default().set_description(entry))
    .collect::<Vec<Embed>>();

    ctx.respond(Pagination::new("code").add_section("Main", pages).save().await?, true).await
}
