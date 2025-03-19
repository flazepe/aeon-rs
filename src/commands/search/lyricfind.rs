use crate::{
    statics::CACHE,
    structs::{api::lyricfind::LyricFind, command::Command, command_context::CommandContext, select_menu::SelectMenu},
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder, MessageResponse},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        if ctx.input.is_string_select() {
            return ctx.respond(LyricFind::search(&ctx.input.values.as_ref().unwrap()[0]).await?[0].format(), false).await;
        }

        let Some(query) = ctx.get_string_arg("song").ok().or_else(|| {
            CACHE.song_activities.read().unwrap().get(&ctx.input.user.id).map(|song| format!("{} - {}", song.artist, song.title))
        }) else {
            return ctx.respond_error("Please provide a song.", true).await;
        };

        let tracks = match LyricFind::search(&query).await {
            Ok(tracks) => tracks,
            Err(error) => return ctx.respond_error(error, true).await,
        };

        let select_menu = SelectMenu::new("lyricfind", "search", "View other lyrics…", Some(format!("{} {query}", tracks[0].artist.name)))
            .add_options(
                tracks.iter().map(|track| (&track.title, format!("{} {query}", &track.artist.name), Some(track.artist.name.clone()))),
            );

        ctx.respond(MessageResponse::from(select_menu).add_embed(tracks[0].format()), false).await
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
