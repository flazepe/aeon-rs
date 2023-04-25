macro_rules! plural {
    ($amount:expr, $subject:expr$(,)?) => {{
        let mut subject = $subject.to_string();

        if $amount != 1 {
            match subject.ends_with("ny") {
                true => subject = format!("{}ies", subject.chars().take(subject.len() - 1).collect::<String>()),
                false => subject = format!("{}s", subject),
            }
        }

        format!("{} {subject}", $amount)
    }};
}

macro_rules! yes_no {
    ($condition:expr $(, $yes:expr, $no:expr)?$(,)?) => {
        {
            let _yes = "Yes";
            $(let _yes = $yes;)?

            let _no = "No";
            $(let _no = $no;)?

            match $condition {
                true => _yes,
                false => _no,
            }
        }
    };
}

pub(crate) use plural;
pub(crate) use yes_no;
