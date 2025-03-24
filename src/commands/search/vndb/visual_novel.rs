use crate::structs::{
    api::vndb::Vndb,
    command_context::{CommandContext, CommandInputExt, Input},
    select_menu::SelectMenu,
};
use anyhow::Result;
use slashook::commands::MessageResponse;

pub async fn run(ctx: CommandContext) -> Result<()> {
    if let Input::ApplicationCommand(input,  _) = &ctx.input {
        if input.get_bool_arg("search").unwrap_or(false) {
            let visual_novels = match Vndb::search_visual_novel(input.get_string_arg("visual-novel")?).await {
                Ok(visual_novels) => visual_novels,
                Err(error) => return ctx.respond_error(error, true).await,
            };

            let select_menu = SelectMenu::new("vndb", "visual-novel", "Select a visual novel…", None::<String>).add_options(
                visual_novels.iter().map(|visual_novel| (&visual_novel.title, &visual_novel.id, Some(&visual_novel.dev_status))),
            );

            return ctx.respond(select_menu, false).await;
        }
    }

    let (query, section) = ctx.get_query_and_section("visual-novel")?;

    if query.is_empty() {
        return ctx.respond_error("Please provide a query.", true).await;
    }

    let visual_novel = match Vndb::search_visual_novel(query).await {
        Ok(mut visual_novels) => visual_novels.remove(0),
        Err(error) => return ctx.respond_error(error, true).await,
    };

    let id = &visual_novel.id;

    let select_menu = SelectMenu::new("vndb", "visual-novel", "View other sections…", Some(&section))
        .add_option("Overview", id, None::<String>)
        .add_option("Description", format!("{id}/description"), None::<String>)
        .add_option("Tags", format!("{id}/tags"), None::<String>);

    let embed = match section.as_str() {
        "description" => visual_novel.format_description(),
        "tags" => visual_novel.format_tags(),
        _ => visual_novel.format(),
    };

    ctx.respond(MessageResponse::from(select_menu).add_embed(embed), false).await
}
