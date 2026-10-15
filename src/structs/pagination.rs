use crate::{
    statics::{EMOJIS, REDIS},
    structs::database::redis::keys::RedisKey,
};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use slashook::{
    chrono::Utc,
    commands::{MessageResponse, Modal},
    structs::{
        components::{ActionRow, Button, ButtonStyle, Component, Components, Label, SelectMenu, SelectMenuType, SelectOption, TextInput},
        embeds::Embed,
    },
};
use std::fmt::Display;

#[derive(Deserialize, Serialize, Debug)]
pub struct Pagination {
    id: String,
    command: String,
    sections: Vec<PaginationSection>,
    current_section: usize,
    current_page: usize,
}

impl Pagination {
    pub fn new<T: Display>(command: T) -> Self {
        Self { id: Utc::now().timestamp().to_string(), command: command.to_string(), sections: vec![], current_section: 0, current_page: 0 }
    }

    pub fn add_section<T: Display, U: Into<PaginationPages>>(mut self, name: T, pages: U) -> Self {
        self.sections.push(PaginationSection::new(name, pages));
        self
    }

    pub async fn save(self) -> Result<Self> {
        let redis = REDIS.get().context("Could not get Redis.")?;
        let key = RedisKey::Paginations(self.id.clone());
        redis.set(&key, &self, Some(60 * 5)).await?;
        Ok(self)
    }

    fn get_pages(&self) -> &PaginationPages {
        &self.sections[self.current_section].pages
    }

    fn get_total_pages(&self) -> usize {
        match self.get_pages() {
            PaginationPages::Embeds(pages) => pages.len(),
            PaginationPages::ComponentsV2(pages) => pages.len(),
        }
    }

    pub fn go_to_first_page(&mut self) {
        self.current_page = 0;
    }

    pub fn go_to_last_page(&mut self) {
        self.current_page = self.get_total_pages() - 1;
    }

    pub fn go_to_previous_page(&mut self) {
        if self.current_page >= 1 {
            self.current_page -= 1;
        }
    }

    pub fn go_to_next_page(&mut self) {
        if self.current_page + 1 < self.get_total_pages() {
            self.current_page += 1;
        }
    }

    pub fn go_to_page(&mut self, page: usize) {
        if page < self.get_total_pages() {
            self.current_page = page;
        }
    }

    pub fn go_to_section<T: Display>(&mut self, section_name: T) {
        let section_name = section_name.to_string();

        for (index, section) in self.sections.iter().enumerate() {
            if section.name == section_name {
                self.current_section = index;
                self.current_page = 0;
            }
        }
    }

    fn generate_custom_id<T: Display>(&self, custom_id: T) -> String {
        format!("pagination/{}/{}", self.id, custom_id)
    }

    fn generate_section_action_row(&self) -> Option<ActionRow> {
        if self.sections.len() < 2 {
            return None;
        }

        let mut select_menu = SelectMenu::new(SelectMenuType::STRING).set_id(&self.command, self.generate_custom_id("section"));

        for (index, section) in self.sections.iter().enumerate() {
            let select_option = SelectOption::new(&section.name, &section.name).set_default(index == self.current_section);
            select_menu = select_menu.add_option(select_option);
        }

        let action_row = ActionRow::new().add_component(select_menu);

        Some(action_row)
    }

    fn generate_control_action_row(&self, page: usize) -> Option<ActionRow> {
        let total_pages = self.get_total_pages();

        if total_pages < 2 {
            return None;
        }

        let mut action_row = ActionRow::new();

        let emojis = EMOJIS.get().unwrap();

        if total_pages > 5 {
            let first_button = Button::new()
                .set_id(&self.command, self.generate_custom_id("first"))
                .set_emoji(emojis.get("aeon_first", "⏪"))
                .set_disabled(page == 0);
            action_row = action_row.add_component(first_button);
        }

        let previous_button = Button::new()
            .set_id(&self.command, self.generate_custom_id("previous"))
            .set_emoji(emojis.get("aeon_previous", "◀️"))
            .set_disabled(page == 0);
        let counter = Button::new()
            .set_id(&self.command, self.generate_custom_id("counter"))
            .set_style(ButtonStyle::SECONDARY)
            .set_label(format!("{} / {}", page + 1, self.get_total_pages()));
        let next_button = Button::new()
            .set_id(&self.command, self.generate_custom_id("next"))
            .set_emoji(emojis.get("aeon_next", "▶️"))
            .set_disabled(page == total_pages - 1);
        action_row = action_row.add_component(previous_button).add_component(counter).add_component(next_button);

        if total_pages > 5 {
            let last_button = Button::new()
                .set_id(&self.command, self.generate_custom_id("last"))
                .set_emoji(emojis.get("aeon_last", "⏩"))
                .set_disabled(page == total_pages - 1);
            action_row = action_row.add_component(last_button);
        }

        Some(action_row)
    }

    pub fn get_message_response(&self) -> MessageResponse {
        let section_action_row = self.generate_section_action_row();
        let control_action_row = self.generate_control_action_row(self.current_page);

        match self.get_pages() {
            PaginationPages::Embeds(pages) => {
                let Some(page) = pages.get(self.current_page) else {
                    return MessageResponse::from("Page not found.").set_components(Components::empty()).clear_embeds().clear_attachments();
                };

                let mut components = Components::empty();

                if let Some(section_action_row) = section_action_row {
                    components = components.add_component(section_action_row);
                }

                if let Some(control_action_row) = control_action_row {
                    components = components.add_component(control_action_row);
                }

                MessageResponse::from(page.clone()).set_components(components).clear_attachments()
            },
            PaginationPages::ComponentsV2(pages) => {
                let Some(page) = pages.get(self.current_page) else {
                    return MessageResponse::from("Page not found.").set_components(Components::empty()).clear_embeds().clear_attachments();
                };

                let mut components = Components(page.clone());

                if let Some(section_action_row) = section_action_row {
                    components = components.add_component(section_action_row);
                }

                if let Some(control_action_row) = control_action_row {
                    components = components.add_component(control_action_row);
                }

                MessageResponse::from(components).set_components_v2(true).clear_embeds().clear_attachments()
            },
        }
    }

    pub fn generate_counter_modal(&self) -> Modal {
        let label = Label::new(format!("Type page number (1-{})", self.get_total_pages()));
        let text_input = TextInput::new().set_id("page").set_required(true);
        let components = Components::new_label(label).add_text_input(text_input);

        let custom_id = self.generate_custom_id("go-to");
        Modal::new(&self.command, custom_id, "Go to page").set_components(components)
    }
}

impl From<Pagination> for MessageResponse {
    fn from(value: Pagination) -> Self {
        value.get_message_response()
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct PaginationSection {
    name: String,
    pages: PaginationPages,
}

impl PaginationSection {
    fn new<T: Display, U: Into<PaginationPages>>(name: T, pages: U) -> Self {
        Self { name: name.to_string(), pages: pages.into() }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum PaginationPages {
    Embeds(Vec<Vec<Embed>>),
    ComponentsV2(Vec<Vec<Component>>),
}

impl From<Embed> for PaginationPages {
    fn from(value: Embed) -> Self {
        Self::Embeds(vec![vec![value]])
    }
}

impl From<Vec<Embed>> for PaginationPages {
    fn from(value: Vec<Embed>) -> Self {
        Self::Embeds(value.into_iter().map(|value| vec![value]).collect())
    }
}

impl From<Vec<Vec<Embed>>> for PaginationPages {
    fn from(value: Vec<Vec<Embed>>) -> Self {
        Self::Embeds(value)
    }
}

impl From<Component> for PaginationPages {
    fn from(value: Component) -> Self {
        Self::ComponentsV2(vec![vec![value]])
    }
}

impl From<Vec<Component>> for PaginationPages {
    fn from(value: Vec<Component>) -> Self {
        Self::ComponentsV2(value.into_iter().map(|value| vec![value]).collect())
    }
}

impl From<Vec<Vec<Component>>> for PaginationPages {
    fn from(value: Vec<Vec<Component>>) -> Self {
        Self::ComponentsV2(value)
    }
}
