use crate::structs::{api::vndb::Vndb, command_context::CommandContext, select_menu::SelectMenu};
use anyhow::Result;
use slashook::commands::MessageResponse;

pub async fn run(ctx: CommandContext) -> Result<()> {
    if ctx.input.is_string_select() {
        return ctx.respond(Vndb::search_tag(&ctx.input.values.as_ref().unwrap()[0]).await?[0].format(), false).await;
    }

    let tags = match Vndb::search_tag(ctx.get_string_arg("tag")?).await {
        Ok(tags) => tags,
        Err(error) => return ctx.respond_error(error, true).await,
    };

    let select_menu = SelectMenu::new("vndb", "tag", "View other tags…", Some(&tags[0].id))
        .add_options(tags.iter().map(|tag| (&tag.name, &tag.id, Some(&tag.category))));

    ctx.respond(MessageResponse::from(select_menu).add_embed(tags[0].format()), false).await
}
