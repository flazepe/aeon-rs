use crate::{
    functions::now,
    statics::{
        CACHE, REST,
        emojis::{ERROR_EMOJI, SUCCESS_EMOJI},
    },
};
use anyhow::{Context, Result, bail};
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
    verified: bool,
}

pub enum AeonCommandInput {
    ApplicationCommand(CommandInput, CommandResponder),
    MessageCommand(TwilightMessage, String, MessageSender),
}

impl AeonCommandContext {
    pub fn new(command_input: AeonCommandInput) -> Self {
        Self { command_input, verified: false }
    }

    pub fn verify(&mut self) -> Result<()> {
        self.verified = true;

        if let AeonCommandInput::ApplicationCommand(input, _) = &self.command_input {
            // Ignore verification for autocomplete
            if input.is_autocomplete() {
                return Ok(());
            }

            if let Some(interaction_metadata) = input.message.as_ref().and_then(|message| message.interaction_metadata.as_ref()) {
                if input.user.id != interaction_metadata.user.id {
                    bail!("This isn't your interaction.");
                }
            }
        }

        Ok(())
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
        if !self.verified {
            bail!("Interaction isn't verified.");
        }

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
            .get(input.focused.as_ref().context("Missing focused arg.")?)
            .context("Could not get focused arg.")?
            .as_string()
            .context("Could not convert focused arg to String.")?
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
                    Ok((input.get_string_arg(option_name)?, "".into()))
                }
            },
            AeonCommandInput::MessageCommand(_, args, _) => Ok((args.into(), "".into())),
        }
    }

    pub fn is_string_select(&self) -> bool {
        if let AeonCommandInput::ApplicationCommand(input, _) = &self.command_input { input.is_string_select() } else { false }
    }
}

#[allow(dead_code)]
pub trait CommandInputExt {
    fn get_string_arg<T: Display>(&self, arg: T) -> Result<String>;
    fn get_i64_arg<T: Display>(&self, arg: T) -> Result<i64>;
    fn get_bool_arg<T: Display>(&self, arg: T) -> Result<bool>;
    fn get_user_arg<T: Display>(&self, arg: T) -> Result<&User>;
    fn get_channel_arg<T: Display>(&self, arg: T) -> Result<&Channel>;
    fn get_role_arg<T: Display>(&self, arg: T) -> Result<&Role>;
    fn get_f64_arg<T: Display>(&self, arg: T) -> Result<f64>;
    fn get_attachment_arg<T: Display>(&self, arg: T) -> Result<&Attachment>;
}

impl CommandInputExt for CommandInput {
    fn get_string_arg<T: Display>(&self, arg: T) -> Result<String> {
        self.args.get(&arg.to_string()).context("Could not get arg.")?.as_string().context("Could not convert arg to String.")
    }

    fn get_i64_arg<T: Display>(&self, arg: T) -> Result<i64> {
        self.args.get(&arg.to_string()).context("Could not get arg.")?.as_i64().context("Could not convert arg to i64.")
    }

    fn get_bool_arg<T: Display>(&self, arg: T) -> Result<bool> {
        self.args.get(&arg.to_string()).context("Could not get arg.")?.as_bool().context("Could not convert arg to bool.")
    }

    fn get_user_arg<T: Display>(&self, arg: T) -> Result<&User> {
        self.args.get(&arg.to_string()).context("Could not get arg.")?.as_user().context("Could not convert arg to User.")
    }

    fn get_channel_arg<T: Display>(&self, arg: T) -> Result<&Channel> {
        self.args.get(&arg.to_string()).context("Could not get arg.")?.as_channel().context("Could not convert arg to Channel.")
    }

    fn get_role_arg<T: Display>(&self, arg: T) -> Result<&Role> {
        self.args.get(&arg.to_string()).context("Could not get arg.")?.as_role().context("Could not convert arg to Role.")
    }

    fn get_f64_arg<T: Display>(&self, arg: T) -> Result<f64> {
        self.args.get(&arg.to_string()).context("Could not get arg.")?.as_f64().context("Could not convert arg to f64.")
    }

    fn get_attachment_arg<T: Display>(&self, arg: T) -> Result<&Attachment> {
        self.args.get(&arg.to_string()).context("Could not get arg.")?.as_attachment().context("Could not convert arg to Attachment.")
    }
}
