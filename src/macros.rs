macro_rules! and_then_or {
    ($expr:expr, $and_then:expr, $else:expr) => {
        $expr.and_then($and_then).unwrap_or($else)
    };
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

pub(crate) use and_then_or;
pub(crate) use if_else;
pub(crate) use kv_autocomplete;
pub(crate) use plural;
pub(crate) use yes_no;
