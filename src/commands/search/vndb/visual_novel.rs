use crate::structs::{api::vndb::Vndb, command_context::CommandContext, select_menu::SelectMenu};
use anyhow::Result;
use slashook::commands::MessageResponse;

pub async fn run(ctx: CommandContext) -> Result<()> {
    if ctx.get_bool_arg("search").unwrap_or(false) {
        let mut select_menu = SelectMenu::new("vndb", "visual-novel", "Select a visual novel…", None::<String>);

        for visual_novel in match Vndb::search_visual_novel(ctx.get_string_arg("visual-novel")?).await {
            Ok(visual_novels) => visual_novels,
            Err(error) => return ctx.respond_error(error, true).await,
        } {
            select_menu = select_menu.add_option(visual_novel.title, visual_novel.id, Some(visual_novel.dev_status));
        }

        return ctx.respond(select_menu, false).await;
    }

    let (query, section) = ctx.get_query_and_section("visual-novel")?;

    let visual_novel = match Vndb::search_visual_novel(query).await {
        Ok(mut visual_novels) => visual_novels.remove(0),
        Err(error) => return ctx.respond_error(error, true).await,
    };

    let id = &visual_novel.id;

    let select_menu = SelectMenu::new("vndb", "visual-novel", "Select a section…", Some(&section))
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
