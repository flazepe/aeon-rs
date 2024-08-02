use crate::structs::{api::vndb::Vndb, command_context::CommandContext, select_menu::SelectMenu};
use anyhow::Result;
use slashook::commands::MessageResponse;

pub async fn run(ctx: CommandContext) -> Result<()> {
    if ctx.input.is_string_select() {
        return ctx.respond(Vndb::search_trait(&ctx.input.values.as_ref().unwrap()[0]).await?[0].format(), false).await;
    }

    let results = match Vndb::search_trait(ctx.get_string_arg("trait")?).await {
        Ok(results) => results,
        Err(error) => return ctx.respond_error(error, true).await,
    };

    let mut select_menu = SelectMenu::new("vndb", "trait", "View other results…", None::<String>);

    for result in &results {
        select_menu = select_menu.add_option(&result.name, &result.id, Some(&result.group_name));
    }

    ctx.respond(MessageResponse::from(select_menu).add_embed(results[0].format()), false).await
}
