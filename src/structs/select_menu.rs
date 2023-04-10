use slashook::structs::components::{Components, SelectMenu as SlashookSelectMenu, SelectMenuType, SelectOption};

pub struct SelectMenu {
    pub command: String,
    pub id: String,
    pub placeholder: String,
    pub options: Vec<SelectOption>,
    pub default: Option<String>,
}

impl SelectMenu {
    pub fn new<T: ToString, U: ToString, V: ToString, W: ToString>(
        command: T,
        id: U,
        placeholder: V,
        options: Vec<SelectOption>,
        default: Option<W>,
    ) -> Self {
        Self {
            command: command.to_string(),
            id: id.to_string(),
            placeholder: placeholder.to_string(),
            options,
            default: default.map(|default| default.to_string()),
        }
    }

    pub fn to_components(self) -> Components {
        let mut select_menu = SlashookSelectMenu::new(SelectMenuType::STRING)
            .set_id(self.command.to_string(), self.id.to_string())
            .set_placeholder(self.placeholder);

        for mut option in self.options.into_iter().take(25) {
            option.label = option.label.chars().take(100).collect::<String>();

            if let Some(description) = option.description {
                option.description = Some(description.chars().take(100).collect::<String>());
            }

            if let Some(default) = self.default.as_ref() {
                if !default.is_empty() && option.value.contains(default) {
                    option.default = Some(true);
                }
            }

            select_menu = select_menu.add_option(option);
        }

        Components::new().add_select_menu(select_menu)
    }
}
