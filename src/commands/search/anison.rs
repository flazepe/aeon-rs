use crate::structs::{
    command::AeonCommand,
    command_context::{AeonCommandContext, AeonCommandInput},
    scraping::anime_song_lyrics::AnimeSongLyrics,
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::{Arc, LazyLock};

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("anison", &["anime-song", "anisong"]).main(|ctx: Arc<AeonCommandContext>| async move {
        let anime_song_lyrics = AnimeSongLyrics::query(ctx.get_string_arg("song")?).await?;
        ctx.respond(anime_song_lyrics.format(), false).await
    })
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
		description = "Fetches the anime from a song title or lyrics.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
		options = [
            {
                name = "song",
                description = "The song title or partial lyrics",
                option_type = InteractionOptionType::STRING,
                required = true,
            },
        ],
	)]
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(AeonCommandInput::ApplicationCommand(input, res)).await?;
    }

    func
}
