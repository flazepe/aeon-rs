use crate::structs::{
    command::AeonCommand,
    command_context::{AeonCommandContext, AeonCommandInput},
};
use anyhow::Context;
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
use std::{
    io::Cursor,
    sync::{Arc, LazyLock},
};

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("latex", &[]).main(|ctx: Arc<AeonCommandContext>| async move {
        let expression = ctx.get_string_arg("expression")?;
        let color = match &ctx.command_input {
            AeonCommandInput::ApplicationCommand(_, _) => ctx.get_string_arg("color").unwrap_or("#fff".into()),
            AeonCommandInput::MessageCommand(_, _, _) => "#fff".into(),
        };

        ctx.defer(false).await?;

        let mathjax = MathJax::new().context("Could not instantiate renderer.")?;
        let mut render = mathjax.render(expression).context("Could not render expression.")?;

        render.set_color(&color);

        let image = render.into_image(10.).context("Could not convert render into image.")?;
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
        COMMAND.run(AeonCommandInput::ApplicationCommand(input, res)).await?;
    }

    func
}
