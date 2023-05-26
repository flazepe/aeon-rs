use crate::structs::{command_context::CommandContext, database::tags::Tags};
use anyhow::Result;
use slashook::{
    commands::Modal,
    structs::components::{Components, TextInput, TextInputStyle},
};

pub async fn run(ctx: CommandContext) -> Result<()> {
    match ctx.input.is_modal_submit() {
        true => match Tags::edit(
            ctx.get_string_arg("tag")?,
            ctx.input.guild_id.as_ref().unwrap(),
            ctx.get_string_arg("name")?,
            ctx.get_string_arg("content")?,
            ctx.input.member.as_ref().unwrap(),
        )
        .await
        {
            Ok(response) => ctx.respond_success(response, true).await,
            Err(error) => ctx.respond_error(error, true).await,
        },
        false => match Tags::get(ctx.get_string_arg("tag")?, ctx.input.guild_id.as_ref().unwrap())
            .await
            .and_then(|tag| Tags::validate_tag_modifier(tag, ctx.input.member.as_ref().unwrap()))
        {
            Ok(tag) => Ok(ctx
                .res
                .open_modal(
                    Modal::new("tag", "edit", "Edit Tag").set_components(
                        Components::new()
                            .add_text_input(TextInput::new().set_id("tag").set_max_length(32).set_label("Tag").set_value(tag.name))
                            .add_row()
                            .add_text_input(
                                TextInput::new()
                                    .set_id("name")
                                    .set_max_length(32)
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
            Err(error) => ctx.respond_error(error, true).await,
        },
    }
}
