#[macro_export]
macro_rules! kv_autocomplete {
    ($input:expr, $res:expr, $kv_array:expr) => {
        let value = $input
            .args
            .get(&$input.focused.context("Missing focused arg.")?)
            .context("Could not get focused arg.")?
            .as_string()
            .context("Could not convert focused arg to String.")?
            .to_lowercase();

        return Ok($res
            .autocomplete(
                $kv_array
                    .iter()
                    .filter(|[k, v]| k.to_lowercase().contains(&value) || v.to_lowercase().contains(&value))
                    .map(|[k, v]| ApplicationCommandOptionChoice::new(v, k.to_string()))
                    .take(25)
                    .collect(),
            )
            .await?);
    };
}

#[macro_export]
macro_rules! stringify_message {
    ($message:expr $(, $empty_vec:expr)?) => {{
        let mut text = String::from(&$message.content);

        for embed in &$message.embeds {
            if let Some(author) = embed.author.as_ref() {
                text += &format!("\n**{}**", crate::escape_markdown!(&author.name));
            }

            if let Some(title) = embed.title.as_ref() {
                text += &format!(
                    "**[{title}](<{}>)**",
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
                        crate::escape_markdown!(field.name.trim()),
                        field.value
                    )
                })
                .collect::<Vec<String>>()
                .join("");

            if let Some(footer) = embed.footer.as_ref() {
                text += &format!("\n**{}**", crate::escape_markdown!(&footer.text));
            }
        }

        &text.trim().to_string()
    }};
}

#[macro_export]
macro_rules! twilight_user_to_tag {
    ($user:expr) => {
        format!("{}#{}", $user.name, $user.discriminator)
    };
}

#[macro_export]
macro_rules! escape_markdown {
    ($text:expr) => {
        regex::Regex::new(r"/\\?[*_~`]/g")
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

#[macro_export]
macro_rules! format_timestamp {
    ($timestamp:expr $(, $format:expr)?) => {{
        let duration = format!("<t:{}:R>", $timestamp);
        let simple = format!("<t:{}:D>", $timestamp);
        let full = format!("{simple} ({duration})");

        let format = "full";
        $(format = $format;)?

        match format {
            "duration" => duration,
            "simple" => simple,
            "full" => full,
            _ => full,
        }
    }};
}

#[macro_export]
macro_rules! if_else {
    ($condition:expr, $true:expr, $false:expr) => {
        if $condition {
            $true
        } else {
            $false
        }
    };
}

#[macro_export]
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

#[macro_export]
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

#[macro_export]
macro_rules! and_then_or {
    ($expr:expr, $and_then:expr, $else:expr) => {
        $expr.and_then($and_then).unwrap_or($else)
    };
}
