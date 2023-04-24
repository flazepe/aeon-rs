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
                            .add_field(
                                "Uptime",
                                format_timestamp(process.start_time(), TimestampFormat::Full),
                                false,
                            )
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
                                            "message",
                                        ),
                                        plural!(
                                            CACHE
                                                .snipes
                                                .read()?
                                                .iter()
                                                .map(|(_, messages)| messages.len())
                                                .reduce(|acc, cur| acc + cur)
                                                .unwrap_or(0),
                                            "snipe",
                                        ),
                                        plural!(
                                            CACHE
                                                .edit_snipes
                                                .read()?
                                                .iter()
                                                .map(|(_, messages)| messages.len())
                                                .reduce(|acc, cur| acc + cur)
                                                .unwrap_or(0),
                                            "edit snipe",
                                        ),
                                        plural!(
                                            CACHE
                                                .reaction_snipes
                                                .read()?
                                                .iter()
                                                .map(|(_, messages)| messages.len())
                                                .reduce(|acc, cur| acc + cur)
                                                .unwrap_or(0),
                                            "reaction snipe",
                                        ),
                                    ]
                                    .join("\n")
                                },
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
