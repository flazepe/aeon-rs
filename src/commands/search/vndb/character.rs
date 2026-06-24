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
    if let AeonCommandInput::ApplicationCommand(..) = &ctx.command_input
        && ctx.get_bool_arg("search").unwrap_or(false)
    {
        let characters = Vndb::search_character(ctx.get_string_arg("character", 0, true)?).await?;
        let mut embed = ComponentsV2Embed::new().set_title("Select a character");
        let mut components = Components::empty();

        let iter = characters.into_iter().take(6).enumerate();
        let total = iter.len();

        for (i, character) in iter {
            let aliases = character.aliases.iter().map(|alias| format!("_{alias}_")).collect::<Vec<String>>();
            let text_display = TextDisplay::new(format!(
                "### [{}](https://vndb.org/{}){}\n-# {}",
                limit_strings(character.name.split(""), "", 300),
                character.id,
                if aliases.is_empty() { "".into() } else { format!("\n{}", aliases.join("\n")) },
                character.vns[0].title,
            ));

            if let Some(image) = &character.image
                && image.sexual == 0.
                && image.violence == 0.
            {
                let thumbnail = Thumbnail::new(&image.url);
                let section = Section::new().add_component(text_display).set_accessory(thumbnail);
                components = components.add_component(section);
            } else {
                components = components.add_component(text_display);
            }

            let button = Button::new().set_id("vndb", format!("character/{}", character.id)).set_label("Select");
            let action_row = ActionRow::new().add_component(button);
            components = components.add_component(action_row);

            if i != total - 1 {
                let separator = Separator::new();
                components = components.add_component(separator);
            }
        }

        embed = embed.set_components(components);

        return ctx.respond(embed, false).await;
    }

    let (query, section) = ctx.get_query_and_section("character")?;
    let character = Vndb::search_character(query).await?.remove(0);
    let id = &character.id;
    let select_menu = SelectMenu::new("vndb", "character", "View other sections…", Some(&section))
        .add_option("Overview", id, None::<String>)
        .add_option("Traits", format!("{id}/traits"), None::<String>)
        .add_option("Visual Novels", format!("{id}/visual-novels"), None::<String>);
    let embed = match section.as_str() {
        "traits" => character.format_traits(),
        "visual-novels" => character.format_visual_novels(),
        _ => character.format(),
    };

    ctx.respond(embed.set_select_menu(select_menu), false).await
}
