macro_rules! add_reminder_select_options {
    ($select_menu:expr) => {
        $select_menu
            .add_option(slashook::structs::components::SelectOption::new("5 minutes", "5m"))
            .add_option(slashook::structs::components::SelectOption::new("15 minutes", "15m"))
            .add_option(slashook::structs::components::SelectOption::new("30 minutes", "30m"))
            .add_option(slashook::structs::components::SelectOption::new("1 hour", "1h"))
            .add_option(slashook::structs::components::SelectOption::new("3 hours", "3h"))
            .add_option(slashook::structs::components::SelectOption::new("6 hours", "6h"))
            .add_option(slashook::structs::components::SelectOption::new("12 hours", "12h"))
            .add_option(slashook::structs::components::SelectOption::new("24 hours", "24h"))
    };
}

macro_rules! and_then_or {
    ($expr:expr, $and_then:expr, $else:expr) => {
        $expr.and_then($and_then).unwrap_or($else)
    };
}

macro_rules! escape_markdown {
    ($text:expr) => {
        regex::Regex::new(r"\\?[*_~`]")
            .unwrap()
            .replace_all($text, |caps: &regex::Captures| {
                if caps[0].starts_with("\\") {
                    caps[0].to_string()
                } else {
                    format!("\\{}", caps[0].to_string())
                }
            })
    };
}

macro_rules! format_timestamp {
    ($timestamp:expr $(, $format:expr)?) => {{
        let duration = format!("<t:{}:R>", $timestamp);
        let simple = format!("<t:{}:D>", $timestamp);
        let full = format!("{simple} ({duration})");

        let mut _format = "full";
        $(_format = $format;)?

        match _format {
            "duration" => duration,
            "simple" => simple,
            "full" => full,
            _ => full,
        }
    }};
}

macro_rules! hastebin {
    ($text:expr) => {{
        let domain = "https://paste.pythondiscord.com";

        let json = reqwest::Client::new()
            .post(format!("{domain}/documents"))
            .body($text.to_string())
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        format!("{domain}/raw/{}", json["key"].as_str().unwrap_or(""))
    }};
}

macro_rules! if_else {
    ($condition:expr, $true:expr, $false:expr) => {
        if $condition {
            $true
        } else {
            $false
        }
    };
}

macro_rules! kv_autocomplete {
    ($input:expr, $res:expr, $hashmap:expr) => {
        let value = $input
            .args
            .get(&$input.focused.context("Missing focused arg.")?)
            .context("Could not get focused arg.")?
            .as_string()
            .context("Could not convert focused arg to String.")?
            .to_lowercase();

        return Ok($res
            .autocomplete(
                $hashmap
                    .iter()
                    .filter(|(k, v)| k.to_lowercase().contains(&value) || v.to_lowercase().contains(&value))
                    .map(|(k, v)| {
                        slashook::structs::interactions::ApplicationCommandOptionChoice::new(v, k.to_string())
                    })
                    .take(25)
                    .collect(),
            )
            .await?);
    };
}

macro_rules! plural {
    ($amount:expr, $subject:expr) => {{
        let mut subject = $subject.to_string();

        if $amount != 1 {
            if subject.ends_with("ny") {
                subject = format!("{}ies", subject.chars().take(subject.len() - 1).collect::<String>());
            } else {
                subject = format!("{}s", subject);
            }
        }

        format!("{} {subject}", $amount)
    }};
}

macro_rules! respond_to_component_interaction {
    ($input:expr,$res:expr,$response:expr) => {
        if let Some(message) = $input.message.as_ref() {
            if let Some(interaction) = message.interaction.as_ref() {
                if $input.user.id == interaction.user.id {
                    $res.update_message($response).await?;
                } else {
                    $res.send_message(
                        slashook::commands::MessageResponse::from(format!(
                            "{} This isn't your interaction.",
                            crate::statics::emojis::ERROR_EMOJI
                        ))
                        .set_ephemeral(true),
                    )
                    .await?;
                }

                return Ok(());
            }
        }
    };
}

macro_rules! stringify_message {
    ($message:expr $(, $empty_vec:expr)?) => {{
        let mut text = String::from(&$message.content);

        for embed in &$message.embeds {
            if let Some(author) = embed.author.as_ref() {
                text += &format!("\n**{}**", crate::macros::escape_markdown!(&author.name));
            }

            if let Some(title) = embed.title.as_ref() {
                text += &format!(
                    "\n**[{title}](<{}>)**",
                    embed.url.as_ref().unwrap_or(&"".into())
                );
            }

            if let Some(description) = embed.description.as_ref() {
                text += &format!("\n{description}");
            }

            text += &embed
                .fields
                $(.as_ref().unwrap_or(&$empty_vec))?
                .iter()
                .map(|field| {
                    format!(
                        "\n**{}**\n{}",
                        crate::macros::escape_markdown!(field.name.trim()),
                        field.value
                    )
                })
                .collect::<Vec<String>>()
                .join("");

            if let Some(footer) = embed.footer.as_ref() {
                text += &format!("\n**{}**", crate::macros::escape_markdown!(&footer.text));
            }
        }

        &text.trim().to_string()
    }};
}

macro_rules! twilight_user_to_tag {
    ($user:expr) => {
        format!("{}#{}", $user.name, $user.discriminator())
    };
}

macro_rules! yes_no {
    ($condition:expr $(, $yes:expr, $no:expr)?) => {
        {
            let _yes = "Yes";
            $(let _yes = $yes;)?

            let _no = "No";
            $(let _no = $no;)?

            if $condition { _yes } else { _no }
        }
    };
}

pub(crate) use add_reminder_select_options;
pub(crate) use and_then_or;
pub(crate) use escape_markdown;
pub(crate) use format_timestamp;
pub(crate) use hastebin;
pub(crate) use if_else;
pub(crate) use kv_autocomplete;
pub(crate) use plural;
pub(crate) use respond_to_component_interaction;
pub(crate) use stringify_message;
pub(crate) use twilight_user_to_tag;
pub(crate) use yes_no;
