use crate::structs::{api::vndb::Vndb, command_context::CommandContext, select_menu::SelectMenu};
use anyhow::Result;
use slashook::commands::MessageResponse;

pub async fn run(ctx: CommandContext) -> Result<()> {
    if ctx.input.is_string_select() {
        return ctx.respond(Vndb::search_tag(&ctx.input.values.as_ref().unwrap()[0]).await?[0].format(), false).await;
    }

    let mut select_menu = SelectMenu::new("vndb", "tag", "View other results…", None::<String>);

    let tags = match Vndb::search_tag(ctx.get_string_arg("tag")?).await {
        Ok(tags) => tags,
        Err(error) => return ctx.respond_error(error, true).await,
    };

    for tag in &tags {
        select_menu = select_menu.add_option(&tag.name, &tag.id, Some(&tag.category))
    }

    ctx.respond(MessageResponse::from(select_menu).add_embed(tags[0].format()), false).await
}
