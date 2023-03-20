use crate::{constants::ERROR_EMOJI, format_timestamp};
use anyhow::Context;
use slashook::commands::{CommandInput, CommandResponder};
use slashook::{command, commands::Command};
use sysinfo::{get_current_pid, ProcessExt, System, SystemExt};

pub fn get_commands() -> Vec<Command> {
    #[command(name = "source", description = "Sends my source.")]
    async fn source(_: CommandInput, res: CommandResponder) {
        res.send_message("<https://github.com/flazepe/aeon-rs>")
            .await?;
    }

    #[command(name = "status", description = "Sends the process status.")]
    async fn status(_: CommandInput, res: CommandResponder) {
        match get_current_pid() {
            Ok(pid) => {
                let mut system = System::new();

                system.refresh_process(pid);

                let process = system.process(pid).context("could not get process")?;

                res.send_message(
                    vec![
                        ["Uptime".into(), format_timestamp!(process.start_time())],
                        [
                            "Memory".into(),
                            format!("{} MB", process.memory() / 1024 / 1024),
                        ],
                        [
                            "Virtual Memory".into(),
                            format!("{} MB", process.virtual_memory() / 1024 / 1024),
                        ],
                    ]
                    .iter()
                    .map(|[k, v]| format!("{k}: {v}"))
                    .collect::<Vec<String>>()
                    .join("\n"),
                )
                .await?;
            }
            Err(error) => {
                res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
            }
        }
    }

    vec![source, status]
}
