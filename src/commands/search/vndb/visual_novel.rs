use crate::structs::{
    api::vndb::Vndb,
    command_context::{AeonCommandContext, AeonCommandInput},
    select_menu::SelectMenu,
};
use anyhow::Result;
use slashook::commands::MessageResponse;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    if let AeonCommandInput::ApplicationCommand(..) = &ctx.command_input {
        if ctx.get_bool_arg("search").unwrap_or(false) {
            let visual_novels = Vndb::search_visual_novel(ctx.get_string_arg("visual-novel", 0, true)?).await?;
            let options = visual_novels.iter().map(|visual_novel| (&visual_novel.title, &visual_novel.id, Some(&visual_novel.dev_status)));
            let select_menu = SelectMenu::new("vndb", "visual-novel", "Select a visual novel…", None::<String>).add_options(options);

            return ctx.respond(select_menu, false).await;
        }
    }

    let (query, section) = ctx.get_query_and_section("visual-novel")?;
    let visual_novel = Vndb::search_visual_novel(query).await?.remove(0);
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
