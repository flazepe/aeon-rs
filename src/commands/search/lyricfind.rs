use crate::{
    statics::CACHE,
    structs::{api::lyricfind::LyricFind, command::Command, command_context::CommandContext},
};
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        let mut query = ctx.get_string_arg("song");

        if query.is_err() {
            if let Some(song) = CACHE.spotify.read().unwrap().get(&ctx.input.user.id) {
                query = Ok(format!("{} - {}", song.artist, song.title));
            }
        }

        let Ok(query) = query else { return ctx.respond_error("Please provide a song.", true).await };

        match LyricFind::search(query).await {
            Ok(mut lyricfind) => ctx.respond(lyricfind.remove(0).format(), false).await,
            Err(error) => ctx.respond_error(error, true).await,
        }
    })
});

pub fn get_command() -> SlashookCommand {
    #[command(
		name = "lyricfind",
		description = "Fetches a song from LyricFind based on query or user's Spotify status.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
		options = [
			{
				name = "song",
				description = "The song",
				option_type = InteractionOptionType::STRING,
			},
		],
	)]
    async fn lyrics(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    lyrics
}
