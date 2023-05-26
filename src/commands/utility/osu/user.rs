use crate::structs::{api::osu::Osu, command_context::CommandContext, select_menu::SelectMenu};
use anyhow::Result;
use slashook::commands::MessageResponse;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let (query, section): (String, String) = match ctx.input.is_string_select() {
        true => {
            let mut split = ctx.input.values.as_ref().unwrap()[0].split('/');
            (split.next().unwrap().into(), split.next().unwrap_or("").into())
        },
        false => (format!("{}|{}", ctx.get_string_arg("user")?, ctx.get_string_arg("mode").unwrap_or("default".into())), "".into()),
    };

    let (user, mode) = query.split_once("|").unwrap();

    let user = match Osu::get_user(user, mode).await {
        Ok(user) => user,
        Err(error) => return ctx.respond_error(error, true).await,
    };

    ctx.respond(
        MessageResponse::from(
            SelectMenu::new("osu", "user", "Select a section…", Some(&section))
                .add_option("Overview", format!("{}|{mode}", user.id), None::<String>)
                .add_option("About", format!("{}|{mode}/about", user.id), None::<String>)
                .add_option("Statistics", format!("{}|{mode}/statistics", user.id), None::<String>)
                .add_option("Website Statistics", format!("{}|{mode}/website-statistics", user.id), None::<String>),
        )
        .add_embed(match section.as_str() {
            "about" => user.format_about(),
            "statistics" => user.format_statistics(),
            "website-statistics" => user.format_website_statistics(),
            _ => user.format(),
        }),
        false,
    )
    .await
}
