use crate::{
    statics::emojis::{ERROR_EMOJI, SUCCESS_EMOJI},
    structs::tags::Tags,
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::{
    commands::{CommandInput, CommandResponder, MessageResponse, Modal},
    structs::components::{Components, TextInput, TextInputStyle},
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    Ok(if input.is_modal_submit() {
        res.send_message(
            MessageResponse::from(
                match Tags::new()
                    .create(
                        input.get_string_arg("tag")?,
                        input.guild_id.as_ref().unwrap(),
                        &input.user.id,
                        input.get_string_arg("content")?,
                        input.member.unwrap(),
                    )
                    .await
                {
                    Ok(response) => format!("{SUCCESS_EMOJI} {response}"),
                    Err(error) => format!("{ERROR_EMOJI} {error}"),
                },
            )
            .set_ephemeral(true),
        )
        .await?;
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
        .await?;
    })
}