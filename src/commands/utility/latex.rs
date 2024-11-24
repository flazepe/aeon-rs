use crate::structs::{command::Command, command_context::CommandContext};
use image::ImageOutputFormat;
use mathjax::MathJax;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::{
        interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
        utils::File,
    },
};
use std::{io::Cursor, sync::LazyLock};

static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        ctx.defer(false).await?;

        let Ok(mathjax) = MathJax::new() else {
            return ctx.respond_error("Could not instantiate renderer.", false).await;
        };

        let Ok(mut render) = mathjax.render(ctx.get_string_arg("expression")?) else {
            return ctx.respond_error("Could not render expression.", false).await;
        };

        render.set_color(ctx.get_string_arg("color").as_deref().unwrap_or("#fff"));

        let Ok(image) = render.into_image(10.) else {
            return ctx.respond_error("Could not convert render into image.", false).await;
        };

        let mut bytes = Vec::new();
        image.write_to(&mut Cursor::new(&mut bytes), ImageOutputFormat::Png)?;

        ctx.respond(File::new("image.png", bytes), false).await
    })
});

pub fn get_command() -> SlashookCommand {
    #[command(
        name = "latex",
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
                description = "The font color",
                option_type = InteractionOptionType::STRING,
            },
        ],
    )]
    async fn latex(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    latex
}
