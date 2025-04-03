use crate::structs::{
    api::vndb::Vndb,
    command_context::{AeonCommandContext, AeonCommandInput},
    select_menu::SelectMenu,
};
use anyhow::Result;
use slashook::commands::MessageResponse;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    if let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input {
        if input.is_string_select() {
            return ctx.respond(Vndb::search_tag(&input.values.as_ref().unwrap()[0]).await?[0].format(), false).await;
        }
    }

    let tags = Vndb::search_tag(ctx.get_string_arg("tag")?).await?;
    let options = tags.iter().map(|tag| (&tag.name, &tag.id, Some(&tag.category)));
    let select_menu = SelectMenu::new("vndb", "tag", "View other tags…", None::<String>).add_options(options);

    ctx.respond(MessageResponse::from(select_menu).add_embed(tags[0].format()), false).await
}
