use crate::{functions::escape_markdown, traits::MessageExt};
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

#[derive(Debug)]
pub struct SimpleMessage {
    pub reply_text: Option<String>,
    pub content: String,
    pub embeds: Vec<SimpleEmbed>,
    pub attachments: Vec<(String, String)>,
    pub stickers: Vec<SimpleSticker>,
}

impl Display for SimpleMessage {
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

macro_rules! impl_simple_message {
    ($message_struct_name:ident, $sticker_items_struct_name:ident) => {
        impl From<$message_struct_name> for SimpleMessage {
            fn from(value: $message_struct_name) -> Self {
                let reply_text = value.reply_text();
                let sticker_items: Option<Vec<$sticker_items_struct_name>> = value.sticker_items.into();

                Self {
                    reply_text,
                    content: value.content,
                    embeds: value.embeds.into_iter().map(|embed| embed.into()).collect(),
                    attachments: value.attachments.into_iter().map(|attachment| (attachment.filename, attachment.url)).collect(),
                    stickers: sticker_items.unwrap_or_default().into_iter().map(|sticker_item| sticker_item.into()).collect(),
                }
            }
        }
    };
}

impl_simple_message!(SlashookMessage, SlashookStickerItem);
impl_simple_message!(TwilightMessage, TwilightStickerItem);

#[derive(Debug)]
pub struct SimpleEmbed {
    title: Option<String>,
    description: Option<String>,
    url: Option<String>,
    footer: Option<String>,
    author: Option<String>,
    fields: Vec<(String, String)>,
}

macro_rules! impl_simple_embed {
    ($embed_struct_name:ident, $embed_field_struct_name:ident) => {
        impl From<$embed_struct_name> for SimpleEmbed {
            fn from(value: $embed_struct_name) -> Self {
                let fields: Option<Vec<$embed_field_struct_name>> = value.fields.into();

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

#[derive(Debug)]
pub struct SimpleSticker {
    id: String,
    name: String,
    format: String,
}

macro_rules! impl_simple_sticker {
    ($sticker_item_struct_name:ident) => {
        impl From<$sticker_item_struct_name> for SimpleSticker {
            fn from(value: $sticker_item_struct_name) -> Self {
                Self { id: value.id.to_string(), name: value.name, format: format!("{:?}", value.format_type).to_lowercase() }
            }
        }
    };
}

impl_simple_sticker!(SlashookStickerItem);
impl_simple_sticker!(TwilightStickerItem);
