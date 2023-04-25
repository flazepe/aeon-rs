use crate::{
    structs::{interaction::Interaction, tags::Tags},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::{
    commands::{CommandInput, CommandResponder, Modal},
    structs::components::{Components, TextInput, TextInputStyle},
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };
    let tags = Tags::new();

    if input.is_modal_submit() {
        match tags
            .edit(
                input.get_string_arg("tag")?,
                input.guild_id.as_ref().unwrap(),
                input.get_string_arg("name")?,
                input.get_string_arg("content")?,
                input.member.as_ref().unwrap(),
            )
            .await
        {
            Ok(response) => interaction.respond_success(response, true).await,
            Err(error) => interaction.respond_error(error, true).await,
        }
    } else {
        match tags
            .get(input.get_string_arg("tag")?, input.guild_id.as_ref().unwrap())
            .await
            .and_then(|tag| Tags::validate_tag_modifier(tag, input.member.as_ref().unwrap()))
        {
            Ok(tag) => Ok(res
                .open_modal(
                    Modal::new("tag", "edit", "Edit Tag").set_components(
                        Components::new()
                            .add_text_input(TextInput::new().set_id("tag").set_max_length(30).set_label("Tag").set_value(tag.name))
                            .add_row()
                            .add_text_input(
                                TextInput::new()
                                    .set_id("name")
                                    .set_max_length(30)
                                    .set_label("New Name")
                                    .set_placeholder("Leave empty if you want to keep the tag name as is")
                                    .set_required(false),
                            )
                            .add_row()
                            .add_text_input(
                                TextInput::new()
                                    .set_style(TextInputStyle::PARAGRAPH)
                                    .set_id("content")
                                    .set_max_length(1000)
                                    .set_label("Content")
                                    .set_value(tag.content),
                            ),
                    ),
                )
                .await?),
            Err(error) => interaction.respond_error(error, true).await,
        }
    }
}
