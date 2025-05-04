use slashook::{
    commands::MessageResponse,
    structs::components::{Components, SelectMenu as SlashookSelectMenu, SelectMenuType, SelectOption},
};
use std::fmt::Display;

#[derive(Debug)]
pub struct SelectMenu {
    command: String,
    id: String,
    placeholder: String,
    options: Vec<SelectOption>,
    default: Option<String>,
}

impl SelectMenu {
    pub fn new<T: Display, U: Display, V: Display, W: Display>(command: T, id: U, placeholder: V, default: Option<W>) -> Self {
        Self {
            command: command.to_string(),
            id: id.to_string(),
            placeholder: placeholder.to_string(),
            options: vec![],
            default: default.map(|default| default.to_string()),
        }
    }

    pub fn add_option<T: Display, U: Display, V: Display>(mut self, label: T, value: U, description: Option<V>) -> Self {
        let value = value.to_string().chars().take(100).collect::<String>().trim().to_string();

        if self.options.iter().any(|option| option.value == value) {
            return self;
        }

        let mut option = SelectOption::new(label.to_string().chars().take(100).collect::<String>(), &value);

        if let Some(default) = self.default.as_ref() {
            option = option.set_default(
                (default.is_empty() && !value.contains('/')) || (!default.is_empty() && value.split('/').next_back().unwrap() == default),
            );
        }

        if let Some(description) = description {
            option = option.set_description(description.to_string().chars().take(100).collect::<String>());
        }

        self.options.push(option);
        self
    }

    pub fn add_options<T: Iterator<Item = (U, V, Option<W>)>, U: Display, V: Display, W: Display>(mut self, options: T) -> Self {
        for (label, value, description) in options {
            self = self.add_option(label.to_string(), value.to_string(), description.map(|description| description.to_string()));
        }
        self
    }
}

impl From<SelectMenu> for Components {
    fn from(value: SelectMenu) -> Self {
        let mut select_menu = SlashookSelectMenu::new(SelectMenuType::STRING)
            .set_id(value.command.to_string(), value.id.to_string())
            .set_placeholder(value.placeholder);

        for option in value.options {
            if select_menu.options.as_ref().map_or(0, |options| options.len()) == 25 {
                break;
            }

            // Prevent duplicate values in the new select menu
            if select_menu.options.as_deref().is_some_and(|options| options.iter().any(|select_option| select_option.value == option.value))
            {
                continue;
            }

            select_menu = select_menu.add_option(option);
        }

        Self::new().add_select_menu(select_menu)
    }
}

impl From<SelectMenu> for MessageResponse {
    fn from(select_menu: SelectMenu) -> Self {
        Self::from(<SelectMenu as Into<Components>>::into(select_menu))
    }
}
