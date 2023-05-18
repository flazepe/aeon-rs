use crate::{
    structs::{api::vndb::Vndb, command_context::CommandContext, select_menu::SelectMenu},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::commands::MessageResponse;

pub async fn run(ctx: CommandContext) -> Result<()> {
    if ctx.input.get_bool_arg("search").unwrap_or(false) {
        let mut select_menu = SelectMenu::new("vndb", "visual-novel", "Select a visual novel…", None::<String>);

        for result in match Vndb::search_visual_novel(ctx.input.get_string_arg("visual-novel")?).await {
            Ok(results) => results,
            Err(error) => return ctx.respond_error(error, true).await,
        } {
            select_menu = select_menu.add_option(result.title, result.id, Some(result.dev_status));
        }

        return ctx.respond(select_menu, false).await;
    }

    let (query, section): (String, String) = match ctx.input.is_string_select() {
        true => {
            let mut split = ctx.input.values.as_ref().unwrap()[0].split("/");
            (split.next().unwrap().into(), split.next().unwrap_or("").into())
        },
        false => (ctx.input.get_string_arg("visual-novel")?, "".into()),
    };

    let visual_novel = match Vndb::search_visual_novel(query).await {
        Ok(mut results) => results.remove(0),
        Err(error) => return ctx.respond_error(error, true).await,
    };

    ctx.respond(
        MessageResponse::from(
            SelectMenu::new("vndb", "visual-novel", "Select a section…", Some(&section))
                .add_option("Overview", format!("{}", visual_novel.id), None::<String>)
                .add_option("Description", format!("{}/description", visual_novel.id), None::<String>)
                .add_option("Tags", format!("{}/tags", visual_novel.id), None::<String>),
        )
        .add_embed(match section.as_str() {
            "description" => visual_novel.format_description(),
            "tags" => visual_novel.format_tags(),
            _ => visual_novel.format(),
        }),
        false,
    )
    .await
}
