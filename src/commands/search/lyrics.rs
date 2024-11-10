use crate::{
    statics::CACHE,
    structs::{api::spotify::Spotify, command::Command, command_context::CommandContext},
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        let Some(query) = ctx
            .get_string_arg("song")
            .ok()
            .or_else(|| CACHE.spotify.read().unwrap().get(&ctx.input.user.id).map(|song| format!("{} - {}", song.artist, song.title)))
        else {
            return ctx.respond_error("Please provide a song.", true).await;
        };

        ctx.defer(false).await?;

        let Ok(mut track) = Spotify::search_track(query).await else { return ctx.respond_error("Song not found.", true).await };

        match Spotify::get_lyrics(track.remove(0)).await {
            Ok(lyrics) => ctx.respond(lyrics.format(), false).await,
            Err(error) => ctx.respond_error(error, true).await,
        }
    })
});

pub fn get_command() -> SlashookCommand {
    #[command(
		name = "lyrics",
		description = "Fetches song lyrics based on query or user's Spotify status.",
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
