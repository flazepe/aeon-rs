use crate::structs::{
    api::google::Google,
    command_context::{CommandContext, CommandInputExt, Input},
    select_menu::SelectMenu,
};
use anyhow::Result;
use slashook::{commands::MessageResponse, structs::utils::File};

pub async fn run(ctx: CommandContext) -> Result<()> {
    let Input::ApplicationCommand { input, res: _ } = &ctx.input else { return Ok(()) };
    let query = input.get_string_arg("query").unwrap_or_else(|_| input.values.as_ref().unwrap()[0].clone());

    ctx.defer(false).await?;

    match Google::assistant(query).await {
        Ok(google_assistant) => {
            let mut response = MessageResponse::from(File::new("image.png", google_assistant.card_image));

            if !google_assistant.suggestions.is_empty() {
                let select_menu = SelectMenu::new("google", "assistant", "Try saying…", None::<String>).add_options(
                    google_assistant.suggestions.iter().map(|suggestion| (suggestion.clone(), suggestion.clone(), None::<String>)),
                );

                response = response.set_components(select_menu.into());
            }

            ctx.respond(response, false).await
        },
        Err(error) => ctx.respond_error(error, false).await,
    }
}
