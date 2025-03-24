use std::cmp::Reverse;

use crate::structs::{
    command_context::{CommandContext, CommandInputExt, Input},
    database::guilds::Guilds,
};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let Input::ApplicationCommand { input, res: _ } = &ctx.input else { return Ok(()) };
    let mut guild = Guilds::get(input.guild_id.as_ref().unwrap()).await?;

    if input.is_autocomplete() {
        return ctx.autocomplete(guild.prefixes.iter().map(|prefix| (prefix, prefix))).await;
    }

    let prefix = input.get_string_arg("prefix")?.to_lowercase();
    let remove_prefix = guild.prefixes.contains(&prefix);

    if !remove_prefix && guild.prefixes.len() >= 10 {
        return ctx.respond_error("A server can only have up to 10 prefixes.", true).await;
    }

    let message = if remove_prefix {
        format!("Removed **{}** from prefixes.", prefix.replace("*", "\\*"))
    } else {
        format!("Added **{}** to prefixes.", prefix.replace("*", "\\*"))
    };

    if remove_prefix {
        guild.prefixes = guild.prefixes.into_iter().filter(|entry| entry != &prefix).collect::<Vec<String>>();
    } else {
        guild.prefixes.push(prefix);

        // Sort alphabetically
        guild.prefixes.sort();

        // Sort by longest to shortest for accuracy
        guild.prefixes.sort_by_key(|entry| Reverse(entry.len()));
    }

    if let Err(error) = Guilds::update(guild).await {
        ctx.respond_error(error, true).await
    } else {
        ctx.respond_success(message, true).await
    }
}
