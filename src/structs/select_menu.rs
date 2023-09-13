use slashook::{
    commands::MessageResponse,
    structs::components::{Components, SelectMenu as SlashookSelectMenu, SelectMenuType, SelectOption},
};

pub struct SelectMenu {
    command: String,
    id: String,
    placeholder: String,
    options: Vec<SelectOption>,
    default: Option<String>,
}

impl SelectMenu {
    pub fn new<T: ToString, U: ToString, V: ToString, W: ToString>(command: T, id: U, placeholder: V, default: Option<W>) -> Self {
        Self {
            command: command.to_string(),
            id: id.to_string(),
            placeholder: placeholder.to_string(),
            options: vec![],
            default: default.map(|default| default.to_string()),
        }
    }

    pub fn add_option<T: ToString, U: ToString, V: ToString>(mut self, label: T, value: U, description: Option<V>) -> Self {
        let value = value.to_string();
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

impl Into<Components> for SelectMenu {
    fn into(self) -> Components {
        let mut select_menu = SlashookSelectMenu::new(SelectMenuType::STRING)
            .set_id(self.command.to_string(), self.id.to_string())
            .set_placeholder(self.placeholder);

        for option in self.options.into_iter().take(25) {
            select_menu = select_menu.add_option(option);
        }

        Components::new().add_select_menu(select_menu)
    }
}

impl From<SelectMenu> for MessageResponse {
    fn from(select_menu: SelectMenu) -> Self {
        Self::from(<SelectMenu as Into<Components>>::into(select_menu))
    }
}
