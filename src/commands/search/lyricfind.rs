use crate::{
    statics::CACHE,
    structs::{
        api::lyricfind::LyricFind,
        command::AeonCommand,
        command_context::{AeonCommandContext, AeonCommandInput},
        select_menu::SelectMenu,
    },
};
use anyhow::Context;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder, MessageResponse},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::{Arc, LazyLock};

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("lyricfind", &["lf"]).set_main(|ctx: Arc<AeonCommandContext>| async move {
        if let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input {
            if input.is_string_select() {
                return ctx.respond(LyricFind::search(&input.values.as_ref().unwrap()[0]).await?[0].format(), false).await;
            }
        }

        let query = ctx
            .get_string_arg("song", 0, true)
            .ok()
            .or_else(|| {
                CACHE.song_activities.read().unwrap().get(&ctx.get_user_id()).map(|song| format!("{} - {}", song.artist, song.title))
            })
            .context("Please provide a song.")?;

        let tracks = LyricFind::search(&query).await?;
        let options = tracks.iter().map(|track| (&track.title, format!("{} {query}", &track.artist.name), Some(track.artist.name.clone())));
        let select_menu = SelectMenu::new("lyricfind", "search", "View other lyrics…", None::<String>).add_options(options);

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
        COMMAND.run(AeonCommandInput::ApplicationCommand(input, res)).await?;
    }

    func
}
