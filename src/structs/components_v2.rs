use crate::statics::colors::PRIMARY_EMBED_COLOR;
use serde_json::{Value, json};
use slashook::{
    commands::MessageResponse,
    structs::{
        components::{ActionRow, Button, Components, Container, Section, Separator, TextDisplay, Thumbnail},
        messages::MessageFlags,
    },
};
use std::fmt::Display;

pub struct ComponentsV2Embed {
    color: Option<String>,
    title: Option<String>,
    url: Option<String>,
    thumbnail: Option<String>,
    description: Option<String>,
    components: Components,
    footer: Option<String>,
    buttons: Vec<Button>,
    ephemeral: bool,
}

impl ComponentsV2Embed {
    pub fn new() -> Self {
        Self {
            color: None,
            title: None,
            url: None,
            thumbnail: None,
            description: None,
            components: Components::empty(),
            footer: None,
            buttons: vec![],
            ephemeral: false,
        }
    }

    pub fn set_color<T: Display>(mut self, color: T) -> Self {
        self.color = Some(color.to_string());
        self
    }

    pub fn set_title<T: Display>(mut self, title: T) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn set_url<T: Display>(mut self, url: T) -> Self {
        self.url = Some(url.to_string());
        self
    }

    pub fn set_thumbnail<T: Display>(mut self, thumbnail: T) -> Self {
        self.thumbnail = Some(thumbnail.to_string());
        self
    }

    pub fn set_description<T: Display>(mut self, description: T) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn set_components(mut self, components: Components) -> Self {
        self.components = components;
        self
    }

    pub fn set_footer<T: Display>(mut self, footer: T) -> Self {
        self.footer = Some(footer.to_string());
        self
    }

    pub fn set_buttons(mut self, buttons: Vec<Button>) -> Self {
        self.buttons = buttons;
        self
    }

    pub fn set_ephemeral(mut self, ephemeral: bool) -> Self {
        self.ephemeral = ephemeral;
        self
    }
}

impl Default for ComponentsV2Embed {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ComponentsV2Embed> for Container {
    fn from(value: ComponentsV2Embed) -> Self {
        let mut container = Container::new();

        let title = value.title.map(|title| format!("### {}", value.url.map(|url| format!("[{title}]({url})")).unwrap_or(title)));
        let text = [title.unwrap_or_default(), value.description.unwrap_or_default()]
            .into_iter()
            .filter(|entry| !entry.is_empty())
            .collect::<Vec<String>>()
            .join("\n");

        if let Some(color) = value.color {
            container = container.set_accent_color(color).unwrap_or_default();
        } else {
            container = container.set_accent_color(PRIMARY_EMBED_COLOR).unwrap_or_default();
        }

        if let Some(url) = value.thumbnail {
            let text_display = TextDisplay::new(text);
            let thumbnail = Thumbnail::new(url);
            let section = Section::new().add_component(text_display).set_accessory(thumbnail);
            container = container.add_component(section);
        } else {
            let text_display = TextDisplay::new(text);
            container = container.add_component(text_display);
        }

        if !value.components.0.is_empty() {
            let separator = Separator::new();
            container = container.add_component(separator);

            for component in value.components.0 {
                container = container.add_component(component);
            }
        }

        if !value.buttons.is_empty() {
            let separator = Separator::new();
            container = container.add_component(separator);

            let mut action_row = ActionRow::new();

            for button in value.buttons {
                action_row = action_row.add_component(button);
            }

            container = container.add_component(action_row);
        }

        if let Some(footer) = value.footer {
            let separator = Separator::new();
            container = container.add_component(separator);

            let text_display = TextDisplay::new(format!("-# {footer}"));
            container = container.add_component(text_display);
        }

        container
    }
}

impl From<ComponentsV2Embed> for MessageResponse {
    fn from(value: ComponentsV2Embed) -> Self {
        let ephemeral = value.ephemeral;
        let components = Components::empty().add_component(Container::from(value));
        Self::from(components).set_components_v2(true).set_ephemeral(ephemeral)
    }
}

impl From<ComponentsV2Embed> for Value {
    fn from(value: ComponentsV2Embed) -> Self {
        let mut flags = MessageFlags::IS_COMPONENTS_V2;

        if value.ephemeral {
            flags |= MessageFlags::EPHEMERAL;
        }

        json!({
            "components": [Container::from(value)],
            "flags": flags,
            "allowed_mentions": {
                "parse": [],
            },
        })
    }
}
