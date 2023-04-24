use crate::{
    statics::google::GOOGLE_TRANSLATE_LANGUAGES,
    structs::{api::google::Google, interaction::Interaction},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let interaction = Interaction::new(&input, &res);

    if input.is_autocomplete() {
        return interaction
            .hashmap_autocomplete(GOOGLE_TRANSLATE_LANGUAGES.iter())
            .await;
    }

    match Google::translate(
        input.get_string_arg("text")?,
        input.get_string_arg("origin-language").unwrap_or("auto".into()),
        input.get_string_arg("target-language").unwrap_or("en".into()),
    )
    .await
    {
        Ok(translation) => interaction.respond(translation.format(), false).await,
        Err(error) => interaction.respond_error(error, true).await,
    }
}
