use crate::{
    structs::{api::vndb::Vndb, command_context::CommandContext, select_menu::SelectMenu},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::commands::MessageResponse;

pub async fn run(ctx: CommandContext) -> Result<()> {
    if ctx.input.is_string_select() {
        return ctx.respond(Vndb::search_tag(&ctx.input.values.as_ref().unwrap()[0]).await?.remove(0).format(), false).await;
    }

    let results = match Vndb::search_tag(ctx.input.get_string_arg("tag")?).await {
        Ok(results) => results,
        Err(error) => return ctx.respond_error(error, true).await,
    };

    let mut select_menu = SelectMenu::new("vndb", "tag", "View other results…", None::<String>);

    for result in &results {
        select_menu = select_menu.add_option(&result.name, &result.id, Some(&result.category))
    }

    ctx.respond(MessageResponse::from(select_menu).add_embed(results[0].format()), false).await
}
