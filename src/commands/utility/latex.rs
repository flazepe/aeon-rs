use crate::structs::{
    command::AeonCommand,
    command_context::{AeonCommandContext, AeonCommandInput},
};
use anyhow::Context;
use mathjax_svg_rs::{Options as MathjaxOptions, render_tex};
use resvg::{
    render,
    tiny_skia::Pixmap,
    usvg::{Options as TreeOptions, Transform, Tree},
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::{
        interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
        utils::File,
    },
};
use std::sync::{Arc, LazyLock};

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("latex", &[]).set_main(|ctx: Arc<AeonCommandContext>| async move {
        let expression = ctx.get_string_arg("expression", 0, true)?;
        let color = match &ctx.command_input {
            AeonCommandInput::ApplicationCommand(..) => ctx.get_string_arg("color", 0, true).unwrap_or("#ffffff".into()),
            AeonCommandInput::MessageCommand(..) => "#ffffff".into(),
        };
        let parsed_color = csscolorparser::parse(&color).context("Invalid color.")?;

        ctx.defer(false).await?;

        let svg = match render_tex(&expression, &MathjaxOptions::default()) {
            Ok(render) => render,
            Err(error) => return ctx.respond_error(error, false).await,
        }
        .replace(r#"style=""#, &format!(r#"style="color: {parsed_color};"#));

        let tree = Tree::from_str(&svg, &TreeOptions { font_size: 32., ..Default::default() })?;
        let mut pixmap = Pixmap::new(tree.size().width() as u32, tree.size().height() as u32).context("Could not create Pixmap.")?;
        render(&tree, Transform::default(), &mut pixmap.as_mut());

        ctx.respond(File::new("image.png", pixmap.encode_png()?), false).await
    })
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
        description = "Renders a LaTeX expression.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
        options = [
            {
                name = "expression",
                description = "The expression",
                option_type = InteractionOptionType::STRING,
                required = true,
            },
            {
                name = "color",
                description = "The text color",
                option_type = InteractionOptionType::STRING,
            },
        ],
    )]
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(AeonCommandInput::ApplicationCommand(Box::new(input), res)).await?;
    }

    func
}
