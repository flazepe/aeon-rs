use crate::structs::{
    command::AeonCommand,
    command_context::{AeonCommandContext, AeonCommandInput},
};
use anyhow::{Context, Result};
use csscolorparser::{Color, parse as parse_color};
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

        let parse_color_arg = |arg| -> Result<Option<Color>> {
            let color = match &ctx.command_input {
                AeonCommandInput::ApplicationCommand(..) => ctx.get_string_arg(arg, 0, true).ok(),
                AeonCommandInput::MessageCommand(..) => None,
            };
            if let Some(color) = color { Ok(Some(parse_color(&color).context("Invalid color.")?)) } else { Ok(None) }
        };
        let text_color = parse_color_arg("text-color")?.unwrap_or(Color::from_rgba8(255, 255, 255, 255));
        let background_color = parse_color_arg("background-color")?.unwrap_or(Color::from_rgba8(0, 0, 0, 0));

        ctx.defer(false).await?;

        let svg = match render_tex(&expression, &MathjaxOptions::default()) {
            Ok(render) => render.replace(r#"style=""#, &format!(r#"style="color: {text_color}; background-color: {background_color};"#)),
            Err(error) => return ctx.respond_error(error, false).await,
        };
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
                name = "text-color",
                description = "The text color",
                option_type = InteractionOptionType::STRING,
            },
            {
                name = "background-color",
                description = "The background color",
                option_type = InteractionOptionType::STRING,
            },
        ],
    )]
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(AeonCommandInput::ApplicationCommand(Box::new(input), res)).await?;
    }

    func
}
