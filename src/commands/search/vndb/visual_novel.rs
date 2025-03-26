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
        if input.get_bool_arg("search").unwrap_or(false) {
            let visual_novels = Vndb::search_visual_novel(input.get_string_arg("visual-novel")?).await?;

            let select_menu = SelectMenu::new("vndb", "visual-novel", "Select a visual novel…", None::<String>).add_options(
                visual_novels.iter().map(|visual_novel| (&visual_novel.title, &visual_novel.id, Some(&visual_novel.dev_status))),
            );

            return ctx.respond(select_menu, false).await;
        }
    }

    let (query, section) = ctx.get_query_and_section("visual-novel")?;

    if query.is_empty() {
        bail!("Please provide a query.");
    }

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
