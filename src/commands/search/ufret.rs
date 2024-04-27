use crate::{
    statics::CACHE,
    structs::{
        command::Command,
        command_context::CommandContext,
        scraping::ufret::{Ufret, UfretSong},
        select_menu::SelectMenu,
    },
};
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::{
        components::Components,
        interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
    },
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        if let Some(value) = ctx.input.values.as_ref().and_then(|values| values.first()) {
            ctx.res.defer_update().await?;

            return match UfretSong::from(value).screenshot().await {
                Ok(song) => ctx.respond(song.format(), false).await,
                Err(error) => ctx.respond_error(error, true).await,
            };
        }

        ctx.res.defer(false).await?;

        let mut query = ctx.get_string_arg("song");

        if query.is_err() {
            if let Some(song) = CACHE.spotify.read().unwrap().get(&ctx.input.user.id) {
                query = Ok(song.title.clone());
            }
        }

        let Ok(query) = query else { return ctx.respond_error("Please provide a song.", true).await };

        let mut results = match Ufret::search(query).await {
            Ok(results) => results,
            Err(error) => return ctx.respond_error(error, true).await,
        };

        let mut select_menu = SelectMenu::new("ufret", "search", "Select a song…", None::<String>);

        for result in &results {
            select_menu = select_menu.add_option(&result.name, format!("{}|{}", result.id, result.name), None::<String>);
        }

        ctx.respond(results.remove(0).screenshot().await?.format().set_components(Components::from(select_menu)), false).await
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
