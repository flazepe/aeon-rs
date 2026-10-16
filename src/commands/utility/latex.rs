use crate::structs::{
    command::AeonCommand,
    command_context::{AeonCommandContext, AeonCommandInput},
};
use anyhow::Context;
use image::{ImageFormat, load_from_memory_with_format};
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
use std::{
    io::Cursor,
    sync::{Arc, LazyLock},
};

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
        .replace(r#"style=""#, &format!(r#"style="font-size: 32px; color: {parsed_color};"#));

        let tree = Tree::from_str(&svg, &TreeOptions::default())?;
        let mut pixmap = Pixmap::new(tree.size().width() as u32, tree.size().height() as u32).unwrap();
        render(&tree, Transform::default(), &mut pixmap.as_mut());

        let image = load_from_memory_with_format(&pixmap.encode_png()?, ImageFormat::Png)?;
        let mut bytes = Vec::new();
        image.write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)?;

        ctx.respond(File::new("image.png", bytes), false).await
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
