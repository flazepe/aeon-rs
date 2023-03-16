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
