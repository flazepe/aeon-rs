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
        let value = value.to_string().chars().take(100).collect::<String>();
        let mut option = SelectOption::new(label.to_string().chars().take(100).collect::<String>(), &value);

        if let Some(default) = self.default.as_ref() {
            option = option.set_default(
                (default.is_empty() && !value.contains('/')) || (!default.is_empty() && value.split('/').last().unwrap() == default),
            );
        }

        if let Some(description) = description {
            option = option.set_description(description.to_string().chars().take(100).collect::<String>());
        }

        self.options.push(option);
        self
    }
}

impl From<SelectMenu> for Components {
    fn from(value: SelectMenu) -> Self {
        let mut select_menu = SlashookSelectMenu::new(SelectMenuType::STRING)
            .set_id(value.command.to_string(), value.id.to_string())
            .set_placeholder(value.placeholder);

        for option in value.options.into_iter().take(25) {
            // Prevent duplicate values in the new select menu
            if let Some(options) = &select_menu.options {
                if options.iter().any(|select_option| select_option.value == option.value) {
                    continue;
                }
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
