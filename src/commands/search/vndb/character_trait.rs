use crate::structs::{
    api::vndb::Vndb,
    command_context::{AeonCommandContext, AeonCommandInput},
    select_menu::SelectMenu,
};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    if let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input
        && input.is_string_select()
    {
        return ctx
            .respond(Vndb::search_trait(&input.values.as_ref().unwrap()[0]).await?[0].format().set_select_menu_from_input(input), false)
            .await;
    }

    let results = Vndb::search_trait(ctx.get_string_arg("trait", 0, true)?).await?;
    let options = results.iter().map(|result| (&result.name, &result.id, result.group_name.as_ref()));
    let select_menu = SelectMenu::new("vndb", "trait", "View other traits…", None::<String>).add_options(options);

    ctx.respond(results[0].format().set_select_menu(select_menu), false).await
}
