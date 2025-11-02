use crate::{
    statics::MONGODB,
    structs::command_context::{AeonCommandContext, AeonCommandInput},
};
use anyhow::Result;
use slashook::{
    commands::Modal,
    structs::components::{Components, Label, TextInput, TextInputStyle},
};
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, res) = &ctx.command_input else { return Ok(()) };

    if input.is_modal_submit() {
        let name = ctx.get_string_arg("tag", 0, true)?;
        let guild_id = input.guild_id.as_ref().unwrap();
        let author_id = &input.user.id;
        let content = ctx.get_string_arg("content", 0, true)?;
        let modifier = input.member.as_ref().unwrap();

        let mongodb = MONGODB.get().unwrap();
        let response = mongodb.tags.create(name, guild_id, author_id, content, modifier).await?;

        ctx.respond_success(response, true).await
    } else {
        let tag_input = TextInput::new().set_id("tag").set_max_length(32);
        let content_input = TextInput::new().set_style(TextInputStyle::PARAGRAPH).set_id("content").set_max_length(1000);
        let components = Components::new_label(Label::new("Tag"))
            .add_text_input(tag_input)
            .add_label(Label::new("Content"))
            .add_text_input(content_input);
        let modal = Modal::new("tag", "create", "Create Tag").set_components(components);

        Ok(res.open_modal(modal).await?)
    }
}
