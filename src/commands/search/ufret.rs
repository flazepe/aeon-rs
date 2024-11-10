use crate::{
    statics::CACHE,
    structs::{
        command::Command,
        command_context::CommandContext,
        scraping::ufret::{Ufret, UfretSong},
        select_menu::SelectMenu,
    },
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::{
        components::Components,
        interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
    },
};
use std::sync::LazyLock;

static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        if let Some(value) = ctx.input.values.as_ref().and_then(|values| values.first()) {
            ctx.defer(true).await?;

            return match UfretSong::from(value).screenshot().await {
                Ok(song) => ctx.respond(song.format(), false).await,
                Err(error) => ctx.respond_error(error, true).await,
            };
        }

        ctx.defer(false).await?;

        let Some(query) = ctx
            .get_string_arg("song")
            .ok()
            .or_else(|| CACHE.spotify.read().unwrap().get(&ctx.input.user.id).map(|song| format!("{} - {}", song.artist, song.title)))
        else {
            return ctx.respond_error("Please provide a song.", true).await;
        };

        let mut songs = match Ufret::search(query).await {
            Ok(songs) => songs,
            Err(error) => return ctx.respond_error(error, true).await,
        };

        let mut select_menu = SelectMenu::new("ufret", "search", "View other results…", None::<String>);

        for song in &songs {
            select_menu = select_menu.add_option(&song.name, format!("{}|{}", song.id, song.name), None::<String>);
        }

        ctx.respond(songs.remove(0).screenshot().await?.format().set_components(Components::from(select_menu)), false).await
    })
});

pub fn get_command() -> SlashookCommand {
    #[command(
		name = "ufret",
		description = "Fetches guitar chords based on query or user's Spotify status.",
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
    async fn ufret(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    ufret
}
