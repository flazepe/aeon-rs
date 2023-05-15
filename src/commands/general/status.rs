use crate::{
    functions::{format_timestamp, TimestampFormat},
    macros::plural,
    statics::{colors::PRIMARY_COLOR, CACHE},
    structs::interaction::Interaction,
};
use anyhow::Context;
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::embeds::Embed,
};
use std::collections::hash_map::Iter;
use sysinfo::{get_current_pid, ProcessExt, System, SystemExt};

pub fn get_command() -> Command {
    #[command(name = "status", description = "Sends the process status.")]
    async fn status(input: CommandInput, res: CommandResponder) {
        let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

        match get_current_pid() {
            Ok(pid) => {
                let mut system = System::new();
                system.refresh_process(pid);

                let process = system.process(pid).context("Could not get process.")?;

                interaction
                    .respond(
                        Embed::new()
                            .set_color(PRIMARY_COLOR)?
                            .add_field("Process Started", format_timestamp(process.start_time(), TimestampFormat::Full), false)
                            .add_field("Memory", bytes_to_mb(process.memory()), false)
                            .add_field("Virtual Memory", bytes_to_mb(process.virtual_memory()), false)
                            .add_field(
                                "Cache",
                                [
                                    plural!(CACHE.channels.read()?.len(), "channel"),
                                    plural!(sum_cache_len(CACHE.channels.read()?.iter()), "message",),
                                    plural!(sum_cache_len(CACHE.snipes.read()?.iter()), "snipe"),
                                    plural!(sum_cache_len(CACHE.edit_snipes.read()?.iter()), "edit snipe"),
                                    plural!(sum_cache_len(CACHE.reaction_snipes.read()?.iter()), "reaction snipe"),
                                ]
                                .join("\n"),
                                false,
                            ),
                        false,
                    )
                    .await?
            },
            Err(error) => interaction.respond_error(error, true).await?,
        }
    }

    status
}

fn bytes_to_mb(bytes: u64) -> String {
    format!("{} MB", bytes / 1024 / 1024)
}

fn sum_cache_len<T: Clone>(iter: Iter<String, Vec<T>>) -> usize {
    iter.map(|(_, vec)| vec.len()).reduce(|acc, cur| acc + cur).unwrap_or(0)
}
