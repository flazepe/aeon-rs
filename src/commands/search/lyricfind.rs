use crate::{
    statics::CACHE,
    structs::{api::lyricfind::LyricFind, command::Command, command_context::CommandContext, select_menu::SelectMenu},
};
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder, MessageResponse},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        if ctx.input.is_string_select() {
            return ctx.respond(LyricFind::search(&ctx.input.values.as_ref().unwrap()[0]).await?[0].format(), false).await;
        }

        let mut query = ctx.get_string_arg("song");

        if query.is_err() {
            if let Some(song) = CACHE.spotify.read().unwrap().get(&ctx.input.user.id) {
                query = Ok(format!("{} - {}", song.artist, song.title));
            }
        }

        let Ok(query) = query else { return ctx.respond_error("Please provide a song.", true).await };

        let tracks = match LyricFind::search(&query).await {
            Ok(tracks) => tracks,
            Err(error) => return ctx.respond_error(error, true).await,
        };

        let mut select_menu = SelectMenu::new("lyricfind", "search", "View other results…", None::<String>);

        for track in &tracks {
            select_menu = select_menu.add_option(&track.title, format!("{query} {}", &track.artist.name), Some(track.artist.name.clone()));
        }

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
