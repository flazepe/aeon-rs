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

pub struct CommandContext {
    pub input: Input,
    verified: bool,
}

pub enum Input {
    ApplicationCommand { input: CommandInput, res: CommandResponder },
    MessageCommand { message: TwilightMessage, sender: MessageSender, args: String },
}

impl CommandContext {
    pub fn new(input: Input) -> Self {
        Self { input, verified: false }
    }

    pub async fn verify(mut self) -> Result<Self> {
        self.verified = true;

        let user_id = match &self.input {
            Input::ApplicationCommand { input, res: _ } => {
                // Ignore verification for autocomplete
                if input.is_autocomplete() {
                    return Ok(self);
                }

                if let Some(interaction_metadata) = input.message.as_ref().and_then(|message| message.interaction_metadata.as_ref()) {
                    if input.user.id != interaction_metadata.user.id {
                        self.respond_error("This isn't your interaction.", true).await?;
                        bail!("User is not the interaction initiator.");
                    }
                }

                input.user.id.clone()
            },
            Input::MessageCommand { message, sender: _, args: _ } => message.author.id.to_string(),
        };

        if CACHE.cooldowns.read().unwrap().get(&user_id).unwrap_or(&0) > &now() {
            self.respond_error("You are under a cooldown. Try again later.", true).await?;
            bail!("User is under a cooldown.");
        }

        Ok(self)
    }

    pub async fn defer(&self, ephemeral: bool) -> Result<()> {
        let Input::ApplicationCommand { input, res } = &self.input else { return Ok(()) };

        if input.message.is_some() {
            res.defer_update().await?
        } else {
            res.defer(ephemeral).await?
        };

        Ok(())
    }

    pub async fn respond<T: Into<MessageResponse>>(&self, response: T, ephemeral: bool) -> Result<()> {
        if !self.verified {
            bail!("Interaction isn't verified.");
        }

        match &self.input {
            Input::ApplicationCommand { input, res: _ } => {
                // Only add cooldown to non-search commands
                if !input.get_bool_arg("search").unwrap_or(false) {
                    CACHE.cooldowns.write().unwrap().insert(input.user.id.clone(), now() + 3);
                }
            },
            Input::MessageCommand { message, sender: _, args: _ } => {
                CACHE.cooldowns.write().unwrap().insert(message.author.id.to_string(), now() + 3);
            },
        };

        let mut response = response.into().set_ephemeral(ephemeral);

        if response.content.is_some() {
            response = response.set_components(Components::empty());
        } else {
            response = response.set_content("");
        }

        match &self.input {
            Input::ApplicationCommand { input, res } => {
                if input.message.is_some() && !ephemeral {
                    res.update_message(response).await?;
                } else {
                    res.send_message(response).await?;
                }
            },
            Input::MessageCommand { message, sender: _, args: _ } => {
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
        let Input::ApplicationCommand { input, res } = &self.input else { return Ok(()) };

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
        match &self.input {
            Input::ApplicationCommand { input, res: _ } => {
                if input.is_string_select() {
                    let mut split = input.values.as_ref().unwrap()[0].split('/');
                    Ok((split.next().unwrap().into(), split.next().unwrap_or("").into()))
                } else {
                    Ok((input.get_string_arg(option_name)?, "".into()))
                }
            },
            Input::MessageCommand { message: _, sender: _, args } => Ok((args.into(), "".into())),
        }
    }

    pub fn is_string_select(&self) -> bool {
        if let Input::ApplicationCommand { input, res: _ } = &self.input { input.is_string_select() } else { false }
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
