use crate::structs::{
    command::Command,
    command_context::{CommandContext, CommandInputExt, Input},
};
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

pub static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new("latex", &[]).main(|ctx: CommandContext| async move {
        let (expression, color) = match &ctx.input {
            Input::ApplicationCommand(input,  _) => {
                (input.get_string_arg("expression")?, input.get_string_arg("color").unwrap_or("#fff".into()))
            },
            Input::MessageCommand(_, _, args)   => (args.into(), "#fff".into()),
        };

        if expression.is_empty() {
            return ctx.respond_error("Please provide an expression.", true).await;
        }

        ctx.defer(false).await?;

        let Ok(mathjax) = MathJax::new() else {
            return ctx.respond_error("Could not instantiate renderer.", false).await;
        };

        let Ok(mut render) = mathjax.render(expression) else {
            return ctx.respond_error("Could not render expression.", false).await;
        };

        render.set_color(&color);

        let Ok(image) = render.into_image(10.) else {
            return ctx.respond_error("Could not convert render into image.", false).await;
        };

        let mut bytes = Vec::new();
        image.write_to(&mut Cursor::new(&mut bytes), ImageOutputFormat::Png)?;

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
        COMMAND.run(Input::ApplicationCommand(input, res)).await?;
    }

    func
}
