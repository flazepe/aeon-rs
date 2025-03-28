use crate::{
    statics::CACHE,
    structs::{
        api::google::statics::GOOGLE_TRANSLATE_LANGUAGES,
        command::AeonCommand,
        command_context::{AeonCommandContext, AeonCommandInput},
        scraping::petitlyrics::PetitLyrics,
        select_menu::SelectMenu,
    },
};
use anyhow::bail;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder, MessageResponse},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::{Arc, LazyLock};

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("lyrics", &["l", "ly", "lyr", "lyric"]).main(|ctx: Arc<AeonCommandContext>| async move {
        if let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input {
            if input.is_autocomplete() {
                return ctx.autocomplete(GOOGLE_TRANSLATE_LANGUAGES.iter()).await;
            }

            if input.is_string_select() {
                let (artist, title) = input.values.as_ref().unwrap()[0].split_once('|').unwrap_or(("", ""));

                ctx.defer(false).await?;

                let embed = PetitLyrics::search_perfect(Some(artist), Some(title), None::<String>).await?[0]
                    .get_formatted_lyrics(None::<String>)
                    .await?;

                return ctx.respond(embed, false).await;
            }
        }

        let (user_id, mut artist, mut title, lyrics, translate_language) = match &ctx.command_input {
            AeonCommandInput::ApplicationCommand(input, _) => {
                let query = ctx.get_string_arg("song").unwrap_or_default();
                let (artist, title) = query.split_once('-').unwrap_or(("", &query));
                (
                    input.user.id.clone(),
                    if artist.trim().is_empty() { None } else { Some(artist.into()) },
                    if title.trim().is_empty() { None } else { Some(title.into()) },
                    ctx.get_string_arg("lyrics").ok(),
                    ctx.get_string_arg("translate").ok(),
                )
            },
            AeonCommandInput::MessageCommand(message, args, _) => {
                let (artist, title) = args.split_once('-').unwrap_or(("", args));
                (
                    message.author.id.to_string(),
                    if artist.trim().is_empty() { None } else { Some(artist.into()) },
                    if title.trim().is_empty() { None } else { Some(title.into()) },
                    None,
                    None,
                )
            },
        };

        if let (None, None, None) = (&artist, &title, &lyrics) {
            if let Some(song) = CACHE.song_activities.read().unwrap().get(&user_id) {
                artist = Some(song.artist.split(',').next().unwrap().to_string());
                title = Some(song.title.clone());
            }
        }

        if let (None, None, None) = (&artist, &title, &lyrics) {
            bail!("Please provide a song artist, title, or lyrics.");
        };

        ctx.defer(false).await?;

        let results = PetitLyrics::search_partial(artist, title, lyrics).await?;

        let select_menu =
            SelectMenu::new("lyrics", "search", "View other lyrics…", Some(format!("{}|{}", results[0].artist, results[0].title)))
                .add_options(
                    results.iter().map(|result| (&result.title, format!("{}|{}", result.artist, result.title), Some(&result.artist))),
                );

        let embed = results[0].get_formatted_lyrics(translate_language).await?;

        ctx.respond(MessageResponse::from(select_menu).add_embed(embed), false).await
    })
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
		description = "Fetches song lyrics based on query or user's Spotify status.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
		options = [
            {
				name = "song",
				description = r#"The song. Artist can be specified by using the "Artist - Title" format"#,
				option_type = InteractionOptionType::STRING,
			},
            {
				name = "lyrics",
				description = "The song lyrics",
				option_type = InteractionOptionType::STRING,
			},
            {
                name = "translate",
                description = "Translate the lyrics to a language",
                option_type = InteractionOptionType::STRING,
                autocomplete = true,
            },
		],
	)]
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(AeonCommandInput::ApplicationCommand(input, res)).await?;
    }

    func
}
