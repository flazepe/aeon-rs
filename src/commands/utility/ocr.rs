use crate::structs::{
    api::google::statics::GOOGLE_TRANSLATE_LANGUAGES,
    command_context::CommandContext,
    ocr::{statics::OCR_LANGUAGES, Ocr},
};

use crate::structs::command::Command;
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        if ctx.input.is_autocomplete() {
            match ctx.input.focused.as_deref() == Some("origin-language") {
                true => return ctx.autocomplete(OCR_LANGUAGES.iter()).await, // Origin language must be OCR-supported language
                false => return ctx.autocomplete(GOOGLE_TRANSLATE_LANGUAGES.iter()).await,
            }
        }

        ctx.res.defer(false).await?;

        match Ocr::read(
            match ctx.get_string_arg("image-url").or(ctx.get_attachment_arg("image-file").map(|attachment| attachment.url.clone())) {
                Ok(url) => url,
                Err(_) => return ctx.respond_error("Please provide an image URL or file.", true).await,
            },
            ctx.get_string_arg("origin-language").unwrap_or("eng".into()),
            ctx.get_string_arg("target-language").unwrap_or("en".into()),
        )
        .await
        {
            Ok(ocr) => ctx.respond(ocr.format(), false).await,
            Err(error) => ctx.respond_error(error, true).await,
        }
    })
});

pub fn get_command() -> SlashookCommand {
    #[command(
		name = "ocr",
		description = "Reads the text from an image.",
		options = [
			{
				name = "image-url",
				description = "The image URL",
				option_type = InteractionOptionType::STRING,
			},
			{
				name = "image-file",
				description = "The image file",
				option_type = InteractionOptionType::ATTACHMENT,
			},
            {
                name = "target-language",
                description = "The language to translate the text to",
                option_type = InteractionOptionType::STRING,
                autocomplete = true,
            },
            {
                name = "origin-language",
                description = "The text's origin language",
                option_type = InteractionOptionType::STRING,
                autocomplete = true,
            },
        ]
	)]
    async fn ocr(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    ocr
}
