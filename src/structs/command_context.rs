use crate::statics::{
    CACHE, REST,
    emojis::{ERROR_EMOJI, SUCCESS_EMOJI},
};
use anyhow::{Context, Error, Result};
use slashook::{
    commands::{CommandInput, CommandResponder, MessageResponse},
    structs::{
        channels::Channel,
        components::Components,
        guilds::Role,
        interactions::ApplicationCommandOptionChoice,
        messages::{Attachment, Message},
        users::User,
    },
};
use std::fmt::{Debug, Display};
use twilight_gateway::MessageSender;
use twilight_model::channel::Message as TwilightMessage;

pub struct AeonCommandContext {
    pub command_input: AeonCommandInput,
}

pub enum AeonCommandInput {
    ApplicationCommand(CommandInput, CommandResponder),
    MessageCommand(TwilightMessage, String, MessageSender),
}

impl AeonCommandContext {
    pub fn new(command_input: AeonCommandInput) -> Self {
        Self { command_input }
    }

    pub fn get_user_id(&self) -> String {
        match &self.command_input {
            AeonCommandInput::ApplicationCommand(input, _) => input.user.id.clone(),
            AeonCommandInput::MessageCommand(message, _, _) => message.author.id.to_string(),
        }
    }

    pub fn get_channel_id(&self) -> String {
        match &self.command_input {
            AeonCommandInput::ApplicationCommand(input, _) => input.channel_id.clone().unwrap(),
            AeonCommandInput::MessageCommand(message, _, _) => message.channel_id.to_string(),
        }
    }

    pub fn get_guild_id(&self) -> Option<String> {
        match &self.command_input {
            AeonCommandInput::ApplicationCommand(input, _) => input.guild_id.clone(),
            AeonCommandInput::MessageCommand(message, _, _) => message.guild_id.map(|guild_id| guild_id.to_string()),
        }
    }

    pub async fn defer(&self, ephemeral: bool) -> Result<()> {
        let AeonCommandInput::ApplicationCommand(input, res) = &self.command_input else { return Ok(()) };

        if input.message.is_some() {
            res.defer_update().await?
        } else {
            res.defer(ephemeral).await?
        }

        Ok(())
    }

    pub async fn respond<T: Into<MessageResponse>>(&self, response: T, ephemeral: bool) -> Result<()> {
        let mut response = response.into().set_ephemeral(ephemeral);

        if response.content.is_some() {
            response = response.set_components(Components::empty());
        } else {
            response = response.set_content("");
        }

        match &self.command_input {
            AeonCommandInput::ApplicationCommand(input, res) => {
                if input.message.is_some() && !ephemeral {
                    res.update_message(response).await?;
                } else {
                    res.send_message(response).await?;
                }
            },
            AeonCommandInput::MessageCommand(message, _, _) => {
                let command_response = CACHE.command_responses.read().unwrap().get(message.id.to_string().as_str()).cloned();

                if let Some(command_response) = command_response {
                    let _ = command_response.edit(&REST, response).await;
                    return Ok(());
                }

                if let Ok(command_response) = Message::create(&REST, message.channel_id, response).await {
                    CACHE.command_responses.write().unwrap().insert(message.id.to_string(), command_response);
                }
            },
        }

        Ok(())
    }

    pub async fn respond_error<T: Debug>(&self, response: T, ephemeral: bool) -> Result<()> {
        self.respond(
            MessageResponse::from(format!("{ERROR_EMOJI} {}", format!("{response:?}").trim_matches('"')))
                .set_components(Components::empty())
                .clear_embeds()
                .clear_attachments(),
            ephemeral,
        )
        .await
    }

    pub async fn respond_success<T: Display>(&self, response: T, ephemeral: bool) -> Result<()> {
        self.respond(
            MessageResponse::from(format!("{SUCCESS_EMOJI} {response}"))
                .set_components(Components::empty())
                .clear_embeds()
                .clear_attachments(),
            ephemeral,
        )
        .await
    }

    pub async fn autocomplete<T: Iterator<Item = (K, V)>, K: Display, V: Display>(&self, iter: T) -> Result<()> {
        let AeonCommandInput::ApplicationCommand(input, res) = &self.command_input else { return Ok(()) };

        let value = input
            .args
            .get(input.focused.as_ref().context("Missing focused argument.")?)
            .context("Could not get focused argument.")?
            .as_string()
            .context("Could not convert focused argument to String.")?
            .to_lowercase();

        Ok(res
            .autocomplete(
                iter.filter(|(k, v)| k.to_string().to_lowercase().contains(&value) || v.to_string().to_lowercase().contains(&value))
                    .map(|(k, v)| ApplicationCommandOptionChoice::new(v.to_string(), k.to_string()))
                    .take(25)
                    .collect(),
            )
            .await?)
    }

    pub fn get_query_and_section<T: Display>(&self, option_name: T) -> Result<(String, String)> {
        match &self.command_input {
            AeonCommandInput::ApplicationCommand(input, _) => {
                if input.is_string_select() {
                    let mut split = input.values.as_ref().unwrap()[0].split('/');
                    Ok((split.next().unwrap().into(), split.next().unwrap_or("").into()))
                } else {
                    Ok((self.get_string_arg(option_name)?, "".into()))
                }
            },
            AeonCommandInput::MessageCommand(_, _, _) => Ok((self.get_string_arg(option_name)?, "".into())),
        }
    }

    pub fn is_string_select(&self) -> bool {
        if let AeonCommandInput::ApplicationCommand(input, _) = &self.command_input { input.is_string_select() } else { false }
    }

    pub fn get_string_arg<T: Display>(&self, arg: T) -> Result<String> {
        match &self.command_input {
            AeonCommandInput::ApplicationCommand(input, _) => input
                .args
                .get(&arg.to_string())
                .context(format!("Please provide the `{arg}` argument."))?
                .as_string()
                .context(format!("Could not convert the `{arg}` argument to `String`.")),
            AeonCommandInput::MessageCommand(_, args, _) => {
                let args = args.trim().to_string();
                if args.is_empty() { Err(Error::msg(format!("Please provide the `{arg}` argument."))) } else { Ok(args) }
            },
        }
    }

    pub fn get_i64_arg<T: Display>(&self, arg: T) -> Result<i64> {
        match &self.command_input {
            AeonCommandInput::ApplicationCommand(input, _) => input
                .args
                .get(&arg.to_string())
                .context(format!("Please provide the `{arg}` argument."))?
                .as_i64()
                .context(format!("Could not convert the `{arg}` argument to `i64`.")),
            AeonCommandInput::MessageCommand(_, args, _) => {
                args.parse::<i64>().context(format!("Could not convert the `{arg}` argument to `i64`."))
            },
        }
    }

    pub fn get_f64_arg<T: Display>(&self, arg: T) -> Result<f64> {
        match &self.command_input {
            AeonCommandInput::ApplicationCommand(input, _) => input
                .args
                .get(&arg.to_string())
                .context(format!("Please provide the `{arg}` argument."))?
                .as_f64()
                .context(format!("Could not convert the `{arg}` argument to `f64`.")),
            AeonCommandInput::MessageCommand(_, args, _) => {
                args.parse::<f64>().context(format!("Could not convert the `{arg}` argument to `f64`."))
            },
        }
    }

    pub fn get_bool_arg<T: Display>(&self, arg: T) -> Result<bool> {
        let AeonCommandInput::ApplicationCommand(input, _) = &self.command_input else {
            return Err(Error::msg("Command input is not an application command."));
        };
        input
            .args
            .get(&arg.to_string())
            .context(format!("Please provide the `{arg}` argument."))?
            .as_bool()
            .context(format!("Could not convert the `{arg}` argument to `bool`."))
    }

    pub fn get_user_arg<T: Display>(&self, arg: T) -> Result<&User> {
        let AeonCommandInput::ApplicationCommand(input, _) = &self.command_input else {
            return Err(Error::msg("Command input is not an application command."));
        };
        input
            .args
            .get(&arg.to_string())
            .context(format!("Please provide the `{arg}` argument."))?
            .as_user()
            .context(format!("Could not convert the `{arg}` argument to `User`."))
    }

    pub fn get_channel_arg<T: Display>(&self, arg: T) -> Result<&Channel> {
        let AeonCommandInput::ApplicationCommand(input, _) = &self.command_input else {
            return Err(Error::msg("Command input is not an application command."));
        };
        input
            .args
            .get(&arg.to_string())
            .context(format!("Please provide the `{arg}` argument."))?
            .as_channel()
            .context(format!("Could not convert the `{arg}` argument to `Channel`."))
    }

    #[allow(dead_code)]
    pub fn get_role_arg<T: Display>(&self, arg: T) -> Result<&Role> {
        let AeonCommandInput::ApplicationCommand(input, _) = &self.command_input else {
            return Err(Error::msg("Command input is not an application command."));
        };
        input
            .args
            .get(&arg.to_string())
            .context(format!("Please provide the `{arg}` argument."))?
            .as_role()
            .context(format!("Could not convert the `{arg}` argument to `Role`."))
    }

    pub fn get_attachment_arg<T: Display>(&self, arg: T) -> Result<&Attachment> {
        let AeonCommandInput::ApplicationCommand(input, _) = &self.command_input else {
            return Err(Error::msg("Command input is not an application command."));
        };
        input
            .args
            .get(&arg.to_string())
            .context(format!("Please provide the `{arg}` argument."))?
            .as_attachment()
            .context(format!("Could not convert the `{arg}` argument to `Attachment`."))
    }
}
