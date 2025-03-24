use crate::{
    statics::CACHE,
    structs::{
        api::lyricfind::LyricFind,
        command::Command,
        command_context::{CommandContext, CommandInputExt, Input},
        select_menu::SelectMenu,
    },
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder, MessageResponse},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new("kmslol", &[]).main(|ctx: CommandContext| async move {
        let Input::ApplicationCommand { input, res: _ } = &ctx.input else { return Ok(()) };

        if input.is_string_select() {
            return ctx.respond(LyricFind::search(&input.values.as_ref().unwrap()[0]).await?[0].format(), false).await;
        }

        let Some(query) = input
            .get_string_arg("song")
            .ok()
            .or_else(|| CACHE.song_activities.read().unwrap().get(&input.user.id).map(|song| format!("{} - {}", song.artist, song.title)))
        else {
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

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
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
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(Input::ApplicationCommand { input, res }).await?;
    }

    func
}
