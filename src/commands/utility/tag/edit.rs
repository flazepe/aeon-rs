use crate::structs::{
    command_context::{AeonCommandContext, AeonCommandInput},
    database::tags::Tags,
};
use anyhow::Result;
use slashook::{
    commands::Modal,
    structs::components::{Components, TextInput, TextInputStyle},
};
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, res) = &ctx.command_input else { return Ok(()) };

    if input.is_modal_submit() {
        let name = ctx.get_string_arg("tag", 0, true)?;
        let guild_id = input.guild_id.as_ref().unwrap();
        let new_name = ctx.get_string_arg("name", 0, true)?;
        let content = ctx.get_string_arg("content", 0, true)?;
        let modifier = input.member.as_ref().unwrap();

        let response = Tags::edit(name, guild_id, new_name, content, modifier).await?;
        ctx.respond_success(response, true).await
    } else {
        let name = ctx.get_string_arg("tag", 0, true)?;
        let guild_id = input.guild_id.as_ref().unwrap();
        let member = input.member.as_ref().unwrap();
        let tag = Tags::get(name, guild_id).await.and_then(|tag| Tags::validate_tag_modifier(tag, member))?;
        let tag_input = TextInput::new().set_id("tag").set_max_length(32).set_label("Tag").set_value(tag.name);
        let new_name_input = TextInput::new()
            .set_id("name")
            .set_max_length(32)
            .set_label("New Name")
            .set_placeholder("Leave empty if you want to keep the tag name as is")
            .set_required(false);
        let content_input = TextInput::new()
            .set_style(TextInputStyle::PARAGRAPH)
            .set_id("content")
            .set_max_length(1000)
            .set_label("Content")
            .set_value(tag.content);
        let components =
            Components::new().add_text_input(tag_input).add_row().add_text_input(new_name_input).add_row().add_text_input(content_input);
        let modal = Modal::new("tag", "edit", "Edit Tag").set_components(components);

        Ok(res.open_modal(modal).await?)
    }
}
