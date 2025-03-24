use crate::structs::{
    api::vndb::Vndb,
    command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt},
    select_menu::SelectMenu,
};
use anyhow::Result;
use slashook::commands::MessageResponse;

pub async fn run(ctx: AeonCommandContext) -> Result<()> {
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
        return ctx.respond_error("Please provide a tag.", true).await;
    }

    let tags = match Vndb::search_tag(tag_query).await {
        Ok(tags) => tags,
        Err(error) => return ctx.respond_error(error, true).await,
    };

    let select_menu = SelectMenu::new("vndb", "tag", "View other tags…", Some(&tags[0].id))
        .add_options(tags.iter().map(|tag| (&tag.name, &tag.id, Some(&tag.category))));

    ctx.respond(MessageResponse::from(select_menu).add_embed(tags[0].format()), false).await
}
