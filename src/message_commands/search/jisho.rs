use crate::{
    structs::{api::jisho::JishoSearch, select_menu::SelectMenu},
    traits::CommandsExt,
};
use anyhow::Result;
use slashook::commands::MessageResponse;
use std::fmt::Display;
use twilight_gateway::MessageSender;
use twilight_model::channel::Message;

pub async fn run<T: Display>(message: &Message, _sender: &MessageSender, args: T) -> Result<()> {
    let query = args.to_string();

    if query.is_empty() {
        return message.send_error("Plese provide a query.").await;
    };

    let results = match JishoSearch::search(query).await {
        Ok(results) => results,
        Err(error) => return message.send_error(error).await,
    };

    let select_menu = SelectMenu::new("jisho", "search", "View other results…", Some(&results[0].slug))
        .add_options(results.iter().map(|result| (result.format_title(), result.slug.clone(), None::<String>)));

    message.send(MessageResponse::from(select_menu).add_embed(results[0].format())).await
}
