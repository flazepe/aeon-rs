use crate::{
    statics::CACHE,
    structs::{command::Command, command_context::CommandContext, scraping::petitlyrics::PetitLyrics, select_menu::SelectMenu},
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
            let (artist, title) = ctx.input.values.as_ref().unwrap()[0].split_once('|').unwrap_or(("", ""));

            ctx.defer(false).await?;

            let embed = match PetitLyrics::search_perfect(Some(artist), Some(title), None::<String>).await {
                Ok(results) => results[0].get_formatted_lyrics().await,
                Err(error) => return ctx.respond_error(error, true).await,
            };

            return match embed {
                Ok(embed) => ctx.respond(embed, false).await,
                Err(error) => return ctx.respond_error(error, true).await,
            };
        }

        let mut artist = ctx.get_string_arg("artist").ok();
        let mut title = ctx.get_string_arg("title").ok();
        let lyrics = ctx.get_string_arg("lyrics").ok();

        if let (None, None, None) = (&artist, &title, &lyrics) {
            if let Some(song) = CACHE.song_activities.read().unwrap().get(&ctx.input.user.id) {
                artist = Some(song.artist.clone());
                title = Some(song.title.clone());
            }
        }

        if let (None, None, None) = (&artist, &title, &lyrics) {
            return ctx.respond_error("Please provide a song artist, title, or lyrics.", true).await;
        };

        ctx.defer(false).await?;

        let mut select_menu = SelectMenu::new("lyrics", "search", "View other results…", None::<String>);

        let results = match PetitLyrics::search_partial(artist, title, lyrics).await {
            Ok(results) => results,
            Err(error) => return ctx.respond_error(error, true).await,
        };

        for result in &results {
            select_menu = select_menu.add_option(&result.title, format!("{}|{}", result.artist, result.title), Some(&result.artist));
        }

        ctx.respond(MessageResponse::from(select_menu).add_embed(results[0].get_formatted_lyrics().await?), false).await
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
				name = "artist",
				description = "The song artist",
				option_type = InteractionOptionType::STRING,
			},
            {
				name = "title",
				description = "The song title",
				option_type = InteractionOptionType::STRING,
			},
            {
				name = "lyrics",
				description = "The song lyrics",
				option_type = InteractionOptionType::STRING,
			},
		],
	)]
    async fn lyrics(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    lyrics
}
