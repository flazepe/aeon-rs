use crate::structs::{api::google::Google, command_context::CommandContext, select_menu::SelectMenu};
use anyhow::Result;
use slashook::{commands::MessageResponse, structs::utils::File};

pub async fn run(ctx: CommandContext) -> Result<()> {
    ctx.defer(true).await?;

    let query = match ctx.input.is_string_select() {
        true => ctx.input.values.as_ref().unwrap()[0].clone(),
        false => ctx.get_string_arg("query")?,
    };

    match Google::assistant(query).await {
        Ok(google_assistant) => {
            let mut response = MessageResponse::from(File::new("image.png", google_assistant.card_image));

            if !google_assistant.suggestions.is_empty() {
                let mut select_menu = SelectMenu::new("google", "assistant", "Try saying…", None::<String>);

                for suggestion in google_assistant.suggestions {
                    select_menu = select_menu.add_option(&suggestion, &suggestion, None::<String>);
                }

                response = response.set_components(select_menu.into());
            }

            ctx.respond(response, false).await
        },
        Err(error) => ctx.respond_error(error, false).await,
    }
}
