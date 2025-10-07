use crate::{
    functions::limit_strings,
    structs::{
        api::vndb::Vndb,
        command_context::{AeonCommandContext, AeonCommandInput},
        components_v2::ComponentsV2Embed,
        select_menu::SelectMenu,
    },
};
use anyhow::Result;
use slashook::structs::components::{ActionRow, Button, Components, Section, Separator, TextDisplay, Thumbnail};
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    if let AeonCommandInput::ApplicationCommand(_, res) = &ctx.command_input
        && ctx.get_bool_arg("search").unwrap_or(false)
    {
        res.defer(false).await?;

        let visual_novels = Vndb::search_visual_novel(ctx.get_string_arg("visual-novel", 0, true)?).await?;
        let mut embed = ComponentsV2Embed::new().set_title("Select a visual novel");
        let mut components = Components::empty();

        let iter = visual_novels.into_iter().take(7).enumerate();
        let total = iter.len();

        for (i, visual_novel) in iter {
            let mut aliases = vec![];
            if let Some(alt_title) = &visual_novel.alt_title {
                aliases.push(format!("_{alt_title}_"));
            }
            for alias in &visual_novel.aliases {
                aliases.push(format!("_{alias}_"));
            }
            let text_display = TextDisplay::new(format!(
                "### [{} ({})](https://vndb.org/{}){}",
                limit_strings(visual_novel.title.split(""), "", 300),
                visual_novel.dev_status,
                visual_novel.id,
                if aliases.is_empty() { "".into() } else { format!("\n{}", aliases.join("\n")) },
            ));

            if let Some(image) = &visual_novel.image
                && image.sexual == 0.
                && image.violence == 0.
            {
                let thumbnail = Thumbnail::new(&image.url);
                let section = Section::new().add_component(text_display).set_accessory(thumbnail);
                components = components.add_component(section);
            } else {
                components = components.add_component(text_display);
            }

            let button = Button::new().set_id("vndb", format!("visual-novel/{}", visual_novel.id)).set_label("Select");
            let action_row = ActionRow::new().add_component(button);
            components = components.add_component(action_row);

            if i != total - 1 {
                let separator: Separator = Separator::new();
                components = components.add_component(separator);
            }
        }

        embed = embed.set_components(components);

        return ctx.respond(embed, false).await;
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

    ctx.respond(embed.set_select_menu(select_menu), false).await
}
