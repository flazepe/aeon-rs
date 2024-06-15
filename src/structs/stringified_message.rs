use crate::functions::escape_markdown;
use slashook::structs::{
    embeds::{Embed as SlashookEmbed, EmbedField as SlashookEmbedField},
    messages::Message as SlashookMessage,
    stickers::StickerItem as SlashookStickerItem,
};
use std::fmt::{Display, Formatter, Result as FmtResult};
use twilight_model::channel::message::{
    embed::{Embed as TwilightEmbed, EmbedField as TwilightEmbedField},
    sticker::MessageSticker as TwilightStickerItem,
    Message as TwilightMessage,
};

pub struct StringifiedMessage {
    pub content: String,
    pub embeds: Vec<SimpleEmbed>,
    pub attachments: Vec<(String, String)>,
    pub stickers: Vec<SimpleSticker>,
    pub reply_text: Option<String>,
}

impl From<SlashookMessage> for StringifiedMessage {
    fn from(value: SlashookMessage) -> Self {
        Self {
            content: value.content,
            embeds: value.embeds.into_iter().map(|embed| embed.into()).collect(),
            attachments: value.attachments.into_iter().map(|attachment| (attachment.filename, attachment.url)).collect(),
            stickers: value.sticker_items.unwrap_or_default().into_iter().map(|sticker| sticker.into()).collect(),
            reply_text: value.message_reference.map(|_| match value.referenced_message {
                Some(referenced_message) => {
                    format!(
                        "[Replying to {} ({})](https://discord.com/channels/{}/{}/{})",
                        escape_markdown(referenced_message.author.username),
                        referenced_message.author.id,
                        referenced_message.guild_id.unwrap_or_else(|| "@me".into()),
                        referenced_message.channel_id,
                        referenced_message.id,
                    )
                },
                None => "Replying to a deleted message".into(),
            }),
        }
    }
}

impl From<TwilightMessage> for StringifiedMessage {
    fn from(value: TwilightMessage) -> Self {
        Self {
            content: value.content,
            embeds: value.embeds.into_iter().map(|embed| embed.into()).collect(),
            attachments: value.attachments.into_iter().map(|attachment| (attachment.filename, attachment.url)).collect(),
            stickers: value.sticker_items.into_iter().map(|sticker| sticker.into()).collect(),
            reply_text: value.reference.map(|_| match value.referenced_message {
                Some(referenced_message) => {
                    format!(
                        "[Replying to {} ({})](https://discord.com/channels/{}/{}/{})",
                        escape_markdown(referenced_message.author.name),
                        referenced_message.author.id,
                        referenced_message.guild_id.map_or_else(|| "@me".into(), |guild_id| guild_id.to_string()),
                        referenced_message.channel_id,
                        referenced_message.id,
                    )
                },
                None => "Replying to a deleted message".into(),
            }),
        }
    }
}

impl Display for StringifiedMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut text = String::new();

        if let Some(reply_text) = self.reply_text.as_ref() {
            text += &format!("> {reply_text}\n");
        }

        text += &self.content;

        if !self.stickers.is_empty() {
            text += &format!(
                "\n\n{}",
                self.stickers
                    .iter()
                    .map(|sticker| {
                        format!("[{}](https://cdn.discordapp.com/stickers/{}.{}?size=4096)", sticker.name, sticker.id, sticker.format)
                    })
                    .collect::<Vec<String>>()
                    .join("\n"),
            );
        }

        if !self.attachments.is_empty() {
            text += &format!(
                "\n\n{}",
                self.attachments.iter().map(|(name, url)| format!("[{name}]({url})")).collect::<Vec<String>>().join("\n"),
            );
        }

        for embed in &self.embeds {
            if let Some(author) = embed.author.as_ref() {
                text += &format!("\n**{}**", escape_markdown(author));
            }

            if let Some(title) = embed.title.as_ref() {
                text += &format!("\n**[{title}](<{}>)**", embed.url.as_deref().unwrap_or(""));
            }

            if let Some(description) = embed.description.as_ref() {
                text += &format!("\n{description}");
            }

            text += &embed
                .fields
                .iter()
                .map(|(name, value)| format!("\n**{}**\n{}", escape_markdown(name.trim()), value))
                .collect::<Vec<String>>()
                .join("");

            if let Some(footer) = embed.footer.as_ref() {
                text += &format!("\n**{}**", escape_markdown(footer));
            }
        }

        write!(f, "{}", text.trim())
    }
}

pub struct SimpleEmbed {
    title: Option<String>,
    description: Option<String>,
    url: Option<String>,
    footer: Option<String>,
    author: Option<String>,
    fields: Vec<(String, String)>,
}

macro_rules! impl_simple_embed {
    ($struct_name:ident, $field_struct_name:ident) => {
        impl From<$struct_name> for SimpleEmbed {
            fn from(value: $struct_name) -> Self {
                let fields: Option<Vec<$field_struct_name>> = value.fields.into();

                Self {
                    title: value.title,
                    description: value.description,
                    url: value.url,
                    footer: value.footer.map(|footer| match footer.icon_url {
                        Some(icon_url) => format!("[{}]({icon_url})", footer.text),
                        None => footer.text,
                    }),
                    author: value.author.map(|author| {
                        let icon_url = match author.icon_url {
                            Some(icon_url) => format!("[[Icon]]({icon_url}) "),
                            None => "".into(),
                        };

                        match author.url {
                            Some(url) => format!("{icon_url}[{}]({url})", author.name),
                            None => format!("{icon_url}{}", author.name),
                        }
                    }),
                    fields: fields.unwrap_or_default().into_iter().map(|field| (field.name, field.value)).collect(),
                }
            }
        }
    };
}

impl_simple_embed!(SlashookEmbed, SlashookEmbedField);
impl_simple_embed!(TwilightEmbed, TwilightEmbedField);

pub struct SimpleSticker {
    id: String,
    name: String,
    format: String,
}

macro_rules! impl_simple_sticker {
    ($s:ident) => {
        impl From<$s> for SimpleSticker {
            fn from(value: $s) -> Self {
                Self { id: value.id.to_string(), name: value.name, format: format!("{:?}", value.format_type).to_lowercase() }
            }
        }
    };
}

impl_simple_sticker!(SlashookStickerItem);
impl_simple_sticker!(TwilightStickerItem);
