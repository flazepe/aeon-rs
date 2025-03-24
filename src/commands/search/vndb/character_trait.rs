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
            return ctx.respond(Vndb::search_trait(&input.values.as_ref().unwrap()[0]).await?[0].format(), false).await;
        }
    }

    let trait_query = match &ctx.input {
        Input::ApplicationCommand { input, res: _ } => input.get_string_arg("trait")?,
        Input::MessageCommand { message: _, sender: _, args } => args.into(),
    };

    if trait_query.is_empty() {
        return ctx.respond_error("Please provide a trait.", true).await;
    }

    let results = match Vndb::search_trait(trait_query).await {
        Ok(results) => results,
        Err(error) => return ctx.respond_error(error, true).await,
    };

    let select_menu = SelectMenu::new("vndb", "trait", "View other traits…", Some(&results[0].id))
        .add_options(results.iter().map(|result| (&result.name, &result.id, Some(&result.group_name))));

    ctx.respond(MessageResponse::from(select_menu).add_embed(results[0].format()), false).await
}
