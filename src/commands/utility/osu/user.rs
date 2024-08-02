use crate::structs::{api::osu::Osu, command_context::CommandContext, select_menu::SelectMenu};
use anyhow::Result;
use slashook::commands::MessageResponse;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let (query, section): (String, String) = match ctx.input.is_string_select() {
        true => {
            let mut split = ctx.input.values.as_ref().unwrap()[0].split('/');
            (split.next().unwrap().into(), split.next().unwrap_or("").into())
        },
        false => (format!("{}|{}", ctx.get_string_arg("user")?, ctx.get_string_arg("mode").as_deref().unwrap_or("default")), "".into()),
    };

    let (user, mode) = query.split_once('|').unwrap();

    let user = match Osu::get_user(user, mode).await {
        Ok(user) => user,
        Err(error) => return ctx.respond_error(error, true).await,
    };

    let id = user.id;

    let select_menu = SelectMenu::new("osu", "user", "Select a section…", Some(&section))
        .add_option("Overview", format!("{id}|{mode}"), None::<String>)
        .add_option("About", format!("{id}|{mode}/about"), None::<String>)
        .add_option("Statistics", format!("{id}|{mode}/statistics"), None::<String>)
        .add_option("Website Statistics", format!("{id}|{mode}/website-statistics"), None::<String>);

    let embed = match section.as_str() {
        "about" => user.format_about(),
        "statistics" => user.format_statistics(),
        "website-statistics" => user.format_website_statistics(),
        _ => user.format(),
    };

    ctx.respond(MessageResponse::from(select_menu).add_embed(embed), false).await
}
