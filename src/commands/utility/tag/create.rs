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
        let name = ctx.get_string_arg("tag")?;
        let guild_id = input.guild_id.as_ref().unwrap();
        let author_id = &input.user.id;
        let content = ctx.get_string_arg("content")?;
        let modifier = input.member.as_ref().unwrap();
        let response = Tags::create(name, guild_id, author_id, content, modifier).await?;

        ctx.respond_success(response, true).await
    } else {
        let tag_input = TextInput::new().set_id("tag").set_max_length(32).set_label("Tag");
        let content_input =
            TextInput::new().set_style(TextInputStyle::PARAGRAPH).set_id("content").set_max_length(1000).set_label("Content");
        let components = Components::new().add_text_input(tag_input).add_row().add_text_input(content_input);
        let modal = Modal::new("tag", "create", "Create Tag").set_components(components);

        Ok(res.open_modal(modal).await?)
    }
}
