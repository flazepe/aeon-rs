use crate::{
    macros::{format_timestamp, plural},
    statics::{colors::PRIMARY_COLOR, emojis::ERROR_EMOJI, CACHE},
};
use anyhow::Context;
use slashook::commands::{CommandInput, CommandResponder};
use slashook::structs::embeds::Embed;
use slashook::{command, commands::Command};
use sysinfo::{get_current_pid, ProcessExt, System, SystemExt};

pub fn get_command() -> Command {
    #[command(name = "status", description = "Sends the process status.")]
    async fn status(_: CommandInput, res: CommandResponder) {
        match get_current_pid() {
            Ok(pid) => {
                let mut system = System::new();
                system.refresh_process(pid);

                let process = system.process(pid).context("Could not get process.")?;

                res.send_message(
                    Embed::new()
                        .set_color(PRIMARY_COLOR)?
                        .add_field("Uptime", format_timestamp!(process.start_time()), false)
                        .add_field("Memory", format!("{} MB", process.memory() / 1024 / 1024), false)
                        .add_field(
                            "Virtual Memory",
                            format!("{} MB", process.virtual_memory() / 1024 / 1024),
                            false,
                        )
                        .add_field(
                            "Cache",
                            {
                                let channels = CACHE.channels.read()?;

                                [
                                    plural!(channels.len(), "channel"),
                                    plural!(
                                        channels
                                            .iter()
                                            .map(|(_, messages)| messages.len())
                                            .reduce(|acc, cur| acc + cur)
                                            .unwrap_or(0),
                                        "message"
                                    ),
                                    plural!(
                                        CACHE
                                            .snipes
                                            .read()?
                                            .iter()
                                            .map(|(_, messages)| messages.len())
                                            .reduce(|acc, cur| acc + cur)
                                            .unwrap_or(0),
                                        "snipe"
                                    ),
                                    plural!(
                                        CACHE
                                            .edit_snipes
                                            .read()?
                                            .iter()
                                            .map(|(_, messages)| messages.len())
                                            .reduce(|acc, cur| acc + cur)
                                            .unwrap_or(0),
                                        "edit snipe"
                                    ),
                                    plural!(
                                        CACHE
                                            .reaction_snipes
                                            .read()?
                                            .iter()
                                            .map(|(_, messages)| messages.len())
                                            .reduce(|acc, cur| acc + cur)
                                            .unwrap_or(0),
                                        "reaction snipe"
                                    ),
                                ]
                                .join("\n")
                            },
                            false,
                        ),
                )
                .await?;
            },
            Err(error) => {
                res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
            },
        }
    }

    status
}
