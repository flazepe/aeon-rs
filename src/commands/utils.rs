use crate::{
    and_then_or,
    constants::*,
    kv_autocomplete,
    structs::{
        distrowatch::*, exchange_rate::*, google_dns::*, google_translate::*, ip_info::*,
        saucenao::SauceNAOSearch, stock::*, unicode::*,
    },
    traits::*,
};
use anyhow::Context;
use slashook::{command, commands::Command};
use slashook::{
    commands::{CommandInput, CommandResponder},
    structs::interactions::*,
};

pub fn get_commands() -> Vec<Command> {
    #[command(
        name = "convert-currency",
        description = "Converts a currency to another currency.",
        options = [
            {
                name = "amount",
                description = "The amount of currency",
                option_type = InteractionOptionType::NUMBER,
                required = true
            },
            {
                name = "from-currency",
                description = "The origin currency, e.g. GBP, NOK, USD",
                option_type = InteractionOptionType::STRING,
                autocomplete = true,
                required = true
            },
            {
                name = "to-currency",
                description = "The currency to convert the amount to, e.g. GBP, NOK, USD",
                option_type = InteractionOptionType::STRING,
                autocomplete = true,
                required = true
            },
        ],
    )]
    async fn convert_currency(input: CommandInput, res: CommandResponder) {
        if input.is_autocomplete() {
            kv_autocomplete!(input, res, CURRENCIES);
        }

        match ExchangeRateConversion::get(
            &input.get_f64_arg("amount")?,
            &input.get_string_arg("from-currency")?,
            &input.get_string_arg("to-currency")?,
        )
        .await
        {
            Ok(exchange_rate_conversion) => {
                res.send_message(exchange_rate_conversion.format()).await?;
            }
            Err(error) => {
                res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
            }
        };
    }

    #[command(
        name = "distro",
        description = "Fetches a distribution information.",
        options = [
            {
                name = "distro",
                description = "The distribution",
                option_type = InteractionOptionType::STRING,
                required = true
            },
        ],
    )]
    async fn distro(input: CommandInput, res: CommandResponder) {
        match Distro::get(&input.get_string_arg("distro")?).await {
            Ok(distro) => {
                res.send_message(distro.format()).await?;
            }
            Err(error) => {
                res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
            }
        };
    }

    #[command(
        name = "dns",
        description = "Fetches DNS records of a domain.",
        options = [
            {
                name = "type",
                description = "The record type, such as A, AAAA, MX, NS, PTR, etc.",
                option_type = InteractionOptionType::STRING,
                required = true
            },
            {
                name = "url",
                description = "The URL",
                option_type = InteractionOptionType::STRING,
                required = true
            },
        ],
    )]
    fn dns(input: CommandInput, res: CommandResponder) {
        match GoogleDNS::query(
            &input.get_string_arg("type")?,
            &input.get_string_arg("url")?,
        )
        .await
        {
            Ok(records) => {
                res.send_message(records.format()).await?;
            }
            Err(error) => {
                res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
            }
        };
    }

    #[command(
        name = "ip",
        description = "Fetches information based on the given IP address.",
        options = [
            {
                name = "ip",
                description = "The IP address",
                option_type = InteractionOptionType::STRING,
                required = true
            },
        ],
    )]
    async fn ip(input: CommandInput, res: CommandResponder) {
        match IPInfo::get(&input.get_string_arg("ip")?).await {
            Ok(ip_info) => {
                res.send_message(ip_info.format()).await?;
            }
            Err(error) => {
                res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
            }
        };
    }

    #[command(
        name = "sauce",
        description = "Fetches sauce from an image.",
        options = [
            {
                name = "image-url",
                description = "The image URL",
                option_type = InteractionOptionType::STRING
            },
            {
                name = "image-attachment",
                description = "The image attachment",
                option_type = InteractionOptionType::ATTACHMENT
            },
        ],
    )]
    async fn sauce(input: CommandInput, res: CommandResponder) {
        let url = input
            .get_string_arg("image-url")
            .ok()
            .unwrap_or(and_then_or!(
                input.get_attachment_arg("image-attachment"),
                |attachment| Ok(attachment.url.to_string()),
                "".into()
            ));

        if url.is_empty() {
            return res
                .send_message(format!(
                    "{ERROR_EMOJI} Please provide an image URL or attachment."
                ))
                .await?;
        }

        match SauceNAOSearch::query(&url).await {
            Ok(saucenao_search) => {
                res.send_message(saucenao_search.format()).await?;
            }
            Err(error) => {
                res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
            }
        };
    }

    #[command(
        name = "stock",
        description = "Fetches stock information.",
        options = [
            {
                name = "ticker",
                description = "The ticker",
                option_type = InteractionOptionType::STRING,
                required = true
            },
        ],
    )]
    fn stock(input: CommandInput, res: CommandResponder) {
        // We have to defer since scraping this takes a bit of time
        res.defer(false).await?;

        match Stock::get(&input.get_string_arg("ticker")?).await {
            Ok(stock) => {
                res.send_message(stock.format()).await?;
            }
            Err(error) => {
                res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
            }
        };
    }

    #[command(
        name = "translate",
        description = "Translate a text to any language.",
        options = [
            {
                name = "text",
                description = "The text to translate",
                option_type = InteractionOptionType::STRING,
                required = true
            },
            {
                name = "to-language",
                description = "The language to translate the text to",
                option_type = InteractionOptionType::STRING,
                autocomplete = true
            },
            {
                name = "from-language",
                description = "The text's origin language",
                option_type = InteractionOptionType::STRING,
                autocomplete = true
            },
        ],
    )]
    async fn translate(input: CommandInput, res: CommandResponder) {
        if input.is_autocomplete() {
            kv_autocomplete!(input, res, GOOGLE_TRANSLATE_LANGUAGES);
        }

        match GoogleTranslate::translate(
            &input.get_string_arg("text")?,
            &input
                .get_string_arg("from-language")
                .unwrap_or("auto".into()),
            &input.get_string_arg("to-language").unwrap_or("en".into()),
        )
        .await
        {
            Ok(translation) => {
                res.send_message(translation.format()).await?;
            }
            Err(error) => {
                res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
            }
        }
    }

    #[command(
        name = "Translate to English",
        command_type = ApplicationCommandType::MESSAGE,
    )]
    async fn translate_message(input: CommandInput, res: CommandResponder) {
        match GoogleTranslate::translate(
            &input
                .target_message
                .context("missing target message")?
                .content,
            "auto",
            "en",
        )
        .await
        {
            Ok(translation) => {
                res.send_message(translation.format()).await?;
            }
            Err(error) => {
                res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
            }
        }
    }

    #[command(
        name = "unicode",
        description = "Does operations with unicode.",
        subcommands = [
            {
                name = "search",
                description = "Searches for a unicode emoji via query.",
                options = [
                    {
                        name = "query",
                        description = "The query",
                        option_type = InteractionOptionType::STRING,
                        required = true
                    },
                ],
            },
            {
                name = "list",
                description = "Lists unicodes from a text.",
                options = [
                    {
                        name = "text",
                        description = "The text",
                        option_type = InteractionOptionType::STRING,
                        required = true
                    },
                ],
            },
        ]
    )]
    async fn unicode(input: CommandInput, res: CommandResponder) {
        match input.subcommand.as_deref() {
            Some("search") => match UnicodeCharacter::get(&input.get_string_arg("query")?).await {
                Ok(unicode_character) => {
                    res.send_message(unicode_character.format()).await?;
                }
                Err(error) => {
                    res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
                }
            },
            Some("list") => {
                res.send_message(UnicodeCharacters::get(&input.get_string_arg("text")?).format())
                    .await?
            }
            _ => {}
        }
    }

    #[command(
        name = "List Unicodes",
        command_type = ApplicationCommandType::MESSAGE,
    )]
    async fn unicode_message(input: CommandInput, res: CommandResponder) {
        res.send_message(
            UnicodeCharacters::get(
                &input
                    .target_message
                    .context("missing target message")?
                    .content,
            )
            .format(),
        )
        .await?;
    }

    vec![
        convert_currency,
        distro,
        dns,
        ip,
        sauce,
        stock,
        translate,
        translate_message,
        unicode,
        unicode_message,
    ]
}
