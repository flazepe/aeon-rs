use crate::structs::{
    api::vndb::Vndb,
    command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt},
    select_menu::SelectMenu,
};
use anyhow::{Result, bail};
use slashook::commands::MessageResponse;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    if let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input {
        if input.is_string_select() {
            return ctx.respond(Vndb::search_tag(&input.values.as_ref().unwrap()[0]).await?[0].format(), false).await;
        }
    }

    let tag_query = match &ctx.command_input {
        AeonCommandInput::ApplicationCommand(input, _) => input.get_string_arg("tag")?,
        AeonCommandInput::MessageCommand(_, args, _) => args.into(),
    };

    if tag_query.is_empty() {
        bail!("Please provide a tag.");
    }

    let tags = Vndb::search_tag(tag_query).await?;
    let select_menu = SelectMenu::new("vndb", "tag", "View other tags…", Some(&tags[0].id))
        .add_options(tags.iter().map(|tag| (&tag.name, &tag.id, Some(&tag.category))));

    ctx.respond(MessageResponse::from(select_menu).add_embed(tags[0].format()), false).await
}
