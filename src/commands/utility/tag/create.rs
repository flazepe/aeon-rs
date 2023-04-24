use crate::{
    structs::{interaction::Interaction, tags::Tags},
    traits::ArgGetters,
};
use anyhow::{Error, Result};
use slashook::{
    commands::{CommandInput, CommandResponder, Modal},
    structs::components::{Components, TextInput, TextInputStyle},
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    if input.is_modal_submit() {
        let interaction = Interaction::new(&input, &res);

        match Tags::new()
            .create(
                input.get_string_arg("tag")?,
                input.guild_id.as_ref().unwrap(),
                &input.user.id,
                input.get_string_arg("content")?,
                input.member.as_ref().unwrap(),
            )
            .await
        {
            Ok(response) => interaction.respond_success(response, true).await,
            Err(error) => interaction.respond_error(error, true).await,
        }
    } else {
        res.open_modal(
            Modal::new("tag", "create", "Create Tag").set_components(
                Components::new()
                    .add_text_input(TextInput::new().set_id("tag").set_max_length(30).set_label("Tag"))
                    .add_row()
                    .add_text_input(
                        TextInput::new()
                            .set_style(TextInputStyle::PARAGRAPH)
                            .set_id("content")
                            .set_max_length(1000)
                            .set_label("Content"),
                    ),
            ),
        )
        .await
        .map_err(|error| Error::from(error))
    }
}
