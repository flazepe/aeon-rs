#[macro_export]
macro_rules! kv_autocomplete {
    ($input:expr, $res:expr, $kv_array:expr) => {
        let value = $input
            .args
            .get(&$input.focused.context("Missing focused arg")?)
            .context("Could not get focused arg")?
            .as_string()
            .context("Could not convert focused arg to String")?
            .to_lowercase();

        return Ok($res
            .autocomplete(
                $kv_array
                    .iter()
                    .filter(|[k, v]| {
                        k.to_lowercase().contains(&value) || v.to_lowercase().contains(&value)
                    })
                    .map(|[k, v]| ApplicationCommandOptionChoice::new(v, k.to_string()))
                    .take(25)
                    .collect(),
            )
            .await?);
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
