use crate::structs::{
    api::vndb::Vndb,
    command_context::{CommandContext, CommandInputExt, Input},
    select_menu::SelectMenu,
};
use anyhow::Result;
use slashook::commands::MessageResponse;

pub async fn run(ctx: CommandContext) -> Result<()> {
    if let Input::ApplicationCommand { input, res: _ } = &ctx.input {
        if input.is_string_select() {
            return ctx.respond(Vndb::search_tag(&input.values.as_ref().unwrap()[0]).await?[0].format(), false).await;
        }
    }

    let tag_query = match &ctx.input {
        Input::ApplicationCommand { input, res: _ } => input.get_string_arg("tag")?,
        Input::MessageCommand { message: _, sender: _, args } => args.into(),
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
