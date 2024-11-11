use crate::structs::{command_context::CommandContext, database::tags::Tags};
use anyhow::Result;
use slashook::{
    commands::Modal,
    structs::components::{Components, TextInput, TextInputStyle},
};

pub async fn run(ctx: CommandContext) -> Result<()> {
    if ctx.input.is_modal_submit() {
        let name = ctx.get_string_arg("tag")?;
        let guild_id = ctx.input.guild_id.as_ref().unwrap();
        let new_name = ctx.get_string_arg("name")?;
        let content = ctx.get_string_arg("content")?;
        let modifier = ctx.input.member.as_ref().unwrap();

        match Tags::edit(name, guild_id, new_name, content, modifier).await {
            Ok(response) => ctx.respond_success(response, true).await,
            Err(error) => ctx.respond_error(error, true).await,
        }
    } else {
        let name = ctx.get_string_arg("tag")?;
        let guild_id = ctx.input.guild_id.as_ref().unwrap();
        let member = ctx.input.member.as_ref().unwrap();

        match Tags::get(name, guild_id).await.and_then(|tag| Tags::validate_tag_modifier(tag, member)) {
            Ok(tag) => {
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
                let components = Components::new()
                    .add_text_input(tag_input)
                    .add_row()
                    .add_text_input(new_name_input)
                    .add_row()
                    .add_text_input(content_input);
                let modal = Modal::new("tag", "edit", "Edit Tag").set_components(components);

                Ok(ctx.res.open_modal(modal).await?)
            },
            Err(error) => ctx.respond_error(error, true).await,
        }
    }
}
