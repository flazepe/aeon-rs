use crate::functions::escape_markdown;
use slashook::structs::channels::Message;
use std::fmt::{Display, Formatter, Result as FmtResult};
use twilight_model::channel::Message as TwilightMessage;

pub struct SimpleEmbed {
    title: Option<String>,
    description: Option<String>,
    url: Option<String>,
    footer_text: Option<String>,
    author_name: Option<String>,
    fields: Vec<(String, String)>,
}

pub struct StringifiedMessage {
    pub content: String,
    pub embeds: Vec<SimpleEmbed>,
    pub attachments: Vec<(String, String)>,
}

impl From<Message> for StringifiedMessage {
    fn from(message: Message) -> Self {
        Self {
            content: message.content,
            embeds: message
                .embeds
                .into_iter()
                .map(|embed| SimpleEmbed {
                    title: embed.title,
                    description: embed.description,
                    url: embed.url,
                    footer_text: embed.footer.map(|footer| footer.text),
                    author_name: embed.author.map(|author| author.name),
                    fields: embed
                        .fields
                        .unwrap_or(vec![])
                        .into_iter()
                        .map(|field| (field.name, field.value))
                        .collect::<Vec<(String, String)>>(),
                })
                .collect::<Vec<SimpleEmbed>>(),
            attachments: message.attachments.into_iter().map(|attachment| (attachment.filename, attachment.url)).collect(),
        }
    }
}

impl From<TwilightMessage> for StringifiedMessage {
    fn from(message: TwilightMessage) -> Self {
        Self {
            content: message.content,
            embeds: message
                .embeds
                .into_iter()
                .map(|embed| SimpleEmbed {
                    title: embed.title,
                    description: embed.description,
                    url: embed.url,
                    footer_text: embed.footer.map(|footer| footer.text),
                    author_name: embed.author.map(|author| author.name),
                    fields: embed.fields.into_iter().map(|field| (field.name, field.value)).collect::<Vec<(String, String)>>(),
                })
                .collect::<Vec<SimpleEmbed>>(),
            attachments: message.attachments.into_iter().map(|attachment| (attachment.filename, attachment.url)).collect(),
        }
    }
}

impl Display for StringifiedMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut text = self.attachments.iter().map(|(name, url)| format!("[{name}]({url})")).collect::<Vec<String>>().join("\n");

        text += &format!("\n\n{}", self.content);

        for embed in &self.embeds {
            if let Some(author_name) = embed.author_name.as_ref() {
                text += &format!("\n**{}**", escape_markdown(&author_name));
            }

            if let Some(title) = embed.title.as_ref() {
                text += &format!("\n**[{title}](<{}>)**", embed.url.as_ref().unwrap_or(&"".into()));
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

            if let Some(footer_text) = embed.footer_text.as_ref() {
                text += &format!("\n**{}**", escape_markdown(&footer_text));
            }
        }

        write!(f, "{}", text.trim())
    }
}
