use crate::statics::EMOJIS;
use serde::{Deserialize, Serialize};
use slashook::{
    chrono::Utc,
    commands::{MessageResponse, Modal},
    structs::{
        components::{ActionRow, Button, ButtonStyle, Component, Components, Label, TextInput},
        embeds::Embed,
    },
};
use std::fmt::Display;

#[derive(Deserialize, Serialize, Debug)]
pub struct Pagination {
    pub id: String,
    pub command: String,
    pub pages: PaginationPages,
    pub current_page: usize,
}

impl Pagination {
    pub fn new<T: Display>(command: T, pages: PaginationPages) -> Self {
        Self { id: Utc::now().timestamp().to_string(), command: command.to_string(), pages, current_page: 0 }
    }

    fn get_total_pages(&self) -> usize {
        match &self.pages {
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

    fn generate_custom_id<T: Display>(&self, custom_id: T) -> String {
        format!("pagination/{}/{}", self.id, custom_id)
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
        let control_action_row = self.generate_control_action_row(self.current_page);

        match &self.pages {
            PaginationPages::Embeds(pages) => {
                let Some(page) = pages.get(self.current_page) else {
                    return MessageResponse::from("Page not found.").set_components(Components::empty()).clear_embeds().clear_attachments();
                };

                let components = if let Some(control_action_row) = control_action_row {
                    Components::empty().add_component(control_action_row)
                } else {
                    Components::empty()
                };

                MessageResponse::from(page.clone()).set_components(components).clear_attachments()
            },
            PaginationPages::ComponentsV2(pages) => {
                let Some(page) = pages.get(self.current_page) else {
                    return MessageResponse::from("Page not found.").set_components(Components::empty()).clear_embeds().clear_attachments();
                };

                let mut components = Components(page.clone());

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

#[derive(Deserialize, Serialize, Debug)]
pub enum PaginationPages {
    Embeds(Vec<Vec<Embed>>),
    ComponentsV2(Vec<Vec<Component>>),
}
