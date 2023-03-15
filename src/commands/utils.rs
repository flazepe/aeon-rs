use crate::{
    constants::{CONTROL_CHARACTERS, CURRENCIES, DNS_CODES, GOOGLE_TRANSLATE_LANGUAGES},
    structs::{DNSResponse, ExchangeRateConversion, GoogleTranslateResponse, IPInfo},
};
use anyhow::Result;
use nipper::Document;
use reqwest::get;
use slashook::{command, commands::Command};
use slashook::{
    commands::{CommandInput, CommandResponder},
    structs::{
        embeds::Embed,
        interactions::{
            ApplicationCommandOptionChoice, ApplicationCommandType, InteractionOptionType,
        },
    },
};
use unicode_names2::name as get_unicode_name;

pub struct Utils {}

impl Utils {
    pub fn init() -> Self {
        Self {}
    }

    pub fn get_commands(self) -> Vec<Command> {
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
                }
            ]
        )]
        async fn convert_currency(input: CommandInput, res: CommandResponder) {
            if input.is_autocomplete() {
                let value = input
                    .args
                    .get(&input.focused.unwrap())
                    .unwrap()
                    .as_string()
                    .unwrap()
                    .to_lowercase();

                return res
                    .autocomplete(
                        CURRENCIES
                            .iter()
                            .filter(|currency| {
                                currency[0].to_lowercase().contains(&value)
                                    || currency[1].to_lowercase().contains(&value)
                            })
                            .map(|currency| {
                                ApplicationCommandOptionChoice::new(currency[0], currency[1])
                            })
                            .take(25)
                            .collect(),
                    )
                    .await?;
            }

            let amount = input.args.get("amount").unwrap().as_f64().unwrap();

            let from_currency = input
                .args
                .get("from-currency")
                .unwrap()
                .as_string()
                .unwrap()
                .to_uppercase();

            let to_currency = input
                .args
                .get("to-currency")
                .unwrap()
                .as_string()
                .unwrap()
                .to_uppercase();

            if !CURRENCIES
                .iter()
                .any(|[_, currency]| currency == &from_currency)
                || !CURRENCIES
                    .iter()
                    .any(|[_, currency]| currency == &to_currency)
            {
                return res.send_message("Invalid currency.").await?;
            }

            res.send_message(
                format!(
                    "{amount} {from_currency} equals {:.3} {to_currency}.",
                    (
                        get(format!("https://api.exchangerate.host/convert?amount={amount}&from={from_currency}&to={to_currency}"))
                        .await?
                        .json::<ExchangeRateConversion>()
                        .await?
                    ).result
                )
            )
            .await?;
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
                }
            ]
        )]
        fn distro(input: CommandInput, res: CommandResponder) {
            let fields: Vec<[String; 2]> = {
                let document = Document::from(
                    &get(format!(
                        "https://distrowatch.com/table.php?distribution={}",
                        input.args.get("distro").unwrap().as_string().unwrap(),
                    ))
                    .await?
                    .text()
                    .await?,
                );

                let name = document.select("td.TablesTitle h1").text();

                if name.is_empty() {
                    vec![]
                } else {
                    let get_table_nth_child = |n: u8| {
                        document
                            .select(&format!("td.TablesTitle li:nth-child({n})"))
                            .text()
                            .split(":")
                            .last()
                            .unwrap()
                            .to_string()
                    };

                    vec![
                        ["Name".into(), name.to_string()],
                        ["Type".into(), get_table_nth_child(1)],
                        ["Architecture".into(), get_table_nth_child(4)],
                        ["Based on".into(), get_table_nth_child(2)],
                        ["Origin".into(), get_table_nth_child(3)],
                        ["Status".into(), get_table_nth_child(7)],
                        ["Category".into(), get_table_nth_child(6)],
                        ["Desktop".into(), get_table_nth_child(5)],
                        ["Popularity".into(), get_table_nth_child(8)],
                    ]
                }
            };

            if fields.is_empty() {
                res.send_message("Distribution not found.").await?;
            } else {
                let mut embed = Embed::new();

                for [name, value] in fields {
                    embed = embed.add_field(name, value, true);
                }

                res.send_message(embed).await?;
            }
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
                }
            ]
        )]
        fn dns(input: CommandInput, res: CommandResponder) {
            let response = get(format!(
                "https://dns.google/resolve?type={}&name={}",
                input.args.get("type").unwrap().as_string().unwrap(),
                input
                    .args
                    .get("url")
                    .unwrap()
                    .as_string()
                    .unwrap()
                    .to_lowercase()
                    .replace("http://", "")
                    .replace("https://", "")
            ))
            .await?;

            if response.status() != 200 {
                return res.send_message("Invalid record type.").await?;
            }

            let json = response.json::<DNSResponse>().await?;

            if json.status != 0 {
                let status = DNS_CODES
                    .iter()
                    .enumerate()
                    .find(|(index, _)| index == &(json.status as usize));

                return res
                    .send_message(if let Some(status) = status {
                        format!("{}: {}", status.1[0], status.1[1])
                    } else {
                        "An unknown error occurred.".into()
                    })
                    .await?;
            }

            let records = json.answer.unwrap_or(json.authority.unwrap_or(vec![]));

            if records.is_empty() {
                return res.send_message("No records found.").await?;
            }

            res.send_message(format!(
                "{}```diff\n{}```",
                json.comment.unwrap_or("".into()),
                records
                    .iter()
                    .map(|record| format!("+ {} (TTL {})", record.data.trim(), record.ttl))
                    .collect::<Vec<String>>()
                    .join("\n")
            ))
            .await?;
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
                }
            ]
        )]
        async fn ip(input: CommandInput, res: CommandResponder) {
            let response = get(format!(
                "https://ipinfo.io/{}/json",
                input
                    .args
                    .get("ip")
                    .unwrap()
                    .as_string()
                    .unwrap()
                    .replace(['/', '?'], "")
            ))
            .await?;

            if response.status() != 200 {
                return res.send_message("IP address not found.").await?;
            }

            let json = response.json::<IPInfo>().await?;

            res.send_message(format!(
                "[{ip}](<https://whatismyipaddress.com/ip/{ip}>)\n{}",
                [
                    json.hostname.unwrap_or("".into()),
                    [
                        json.city.unwrap_or("".into()),
                        json.region.unwrap_or("".into()),
                        json.country.unwrap_or("".into()),
                    ]
                    .into_iter()
                    .filter(|entry| !entry.is_empty())
                    .collect::<Vec<String>>()
                    .join(", "),
                    json.loc
                        .and_then(|loc| Some(loc.replace(',', ", ")))
                        .unwrap_or("".into()),
                    json.org.unwrap_or("".into()),
                ]
                .into_iter()
                .filter(|entry| !entry.is_empty())
                .collect::<Vec<String>>()
                .join("\n"),
                ip = json.ip
            ))
            .await?;
        }

        #[command(
            name = "stock",
            description = "Fetches stock information.",
            options = [
                {
                    name = "stock",
                    description = "The stock name",
                    option_type = InteractionOptionType::STRING,
                    required = true
                }
            ]
        )]
        fn stock(input: CommandInput, res: CommandResponder) {
            res.defer(false).await?;

            let search = {
                let document = Document::from(
                    &get(format!(
                        "https://finance.yahoo.com/lookup/equity?s={}",
                        input.args.get("stock").unwrap().as_string().unwrap(),
                    ))
                    .await?
                    .text()
                    .await?,
                );

                let selection = &document.select("td a");

                if selection.nodes().is_empty() {
                    vec![]
                } else {
                    vec![
                        selection.attr("href").unwrap().to_string(),
                        selection.attr("title").unwrap().to_string(),
                        selection.attr("data-symbol").unwrap().to_string(),
                    ]
                }
            };

            if search.is_empty() {
                return res.send_message("Not found.").await?;
            }

            res.send_message(
                Embed::new()
                    .set_title(format!("{} ({})", search[1], search[2]))
                    .set_url(format!("https://finance.yahoo.com/quote/{}", search[2]))
                    .set_description({
                        let document = Document::from(
                            &get(format!("https://finance.yahoo.com{}", search[0]))
                                .await?
                                .text()
                                .await?,
                        );

                        format!(
                            "```diff\n{} {}\n{}```",
                            document
                                .select("#quote-header-info span")
                                .first()
                                .text()
                                .split(" ")
                                .last()
                                .unwrap(),
                            document
                                .select("#quote-header-info [data-field=\"regularMarketPrice\"]")
                                .first()
                                .text(),
                            ["regularMarketChange", "regularMarketChangePercent"]
                                .map(|field| {
                                    document
                                        .select(&format!(
                                            "#quote-header-info [data-field=\"{}\"]",
                                            field
                                        ))
                                        .first()
                                        .text()
                                        .to_string()
                                })
                                .join(" "),
                        )
                    }),
            )
            .await?;
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
                    autocomplete = true,
                    required = true
                },
                {
                    name = "from-language",
                    description = "The text's origin language",
                    option_type = InteractionOptionType::STRING,
                    autocomplete = true
                }
            ]
        )]
        async fn translate(input: CommandInput, res: CommandResponder) {
            if input.is_autocomplete() {
                let value = input
                    .args
                    .get(&input.focused.unwrap())
                    .unwrap()
                    .as_string()
                    .unwrap()
                    .to_lowercase();

                return res
                    .autocomplete(
                        GOOGLE_TRANSLATE_LANGUAGES
                            .iter()
                            .filter(|[language_code, language_name]| {
                                language_code == &value
                                    || language_name.to_lowercase().contains(&value)
                            })
                            .map(|[language_code, language_name]| {
                                ApplicationCommandOptionChoice::new(
                                    language_name,
                                    language_code.to_string(),
                                )
                            })
                            .take(25)
                            .collect(),
                    )
                    .await?;
            }

            res.send_message(
                Utils::translate(
                    &input.args.get("text").unwrap().as_string().unwrap(),
                    &input
                        .args
                        .get("from-language")
                        .and_then(|arg| arg.as_string())
                        .unwrap_or("auto".into()),
                    &input.args.get("to-language").unwrap().as_string().unwrap(),
                )
                .await?,
            )
            .await?;
        }

        #[command(
            name = "Translate to English",
            command_type = ApplicationCommandType::MESSAGE
        )]
        async fn translate_message(input: CommandInput, res: CommandResponder) {
            res.send_message(
                Utils::translate(&input.target_message.unwrap().content, "auto", "en").await?,
            )
            .await?;
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
                        }
                    ]
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
                        }
                    ]
                }
            ]
        )]
        async fn unicode(input: CommandInput, res: CommandResponder) {
            match input.sub_command.as_deref() {
                Some("search") => {
                    let result = {
                        let document = Document::from(
                            &get(format!(
                                "https://symbl.cc/en/search/?q={}",
                                input.args.get("query").unwrap().as_string().unwrap(),
                            ))
                            .await?
                            .text()
                            .await?,
                        );

                        let name = document.select("h2").first().text();
                        let character = document.select(".search-page__char").first().text();

                        format!(
                            "`U+{:04X}` - {} - {}",
                            character.trim().chars().next().unwrap() as u32,
                            name.trim(),
                            character.trim()
                        )
                    };

                    res.send_message(result).await?;
                }
                Some("list") => {
                    res.send_message(Utils::parse_unicodes(
                        &input.args.get("text").unwrap().as_string().unwrap(),
                    ))
                    .await?;
                }
                _ => {}
            }
        }

        #[command(
            name = "List Unicodes",
            command_type = ApplicationCommandType::MESSAGE
        )]
        async fn unicode_message(input: CommandInput, res: CommandResponder) {
            res.send_message(Utils::parse_unicodes(
                &input.target_message.unwrap().content,
            ))
            .await?;
        }

        vec![
            convert_currency,
            distro,
            dns,
            ip,
            stock,
            translate,
            translate_message,
            unicode,
            unicode_message,
        ]
    }

    fn parse_unicodes(string: &str) -> String {
        let characters = string;
        let mut unicodes: Vec<String> = vec![];

        for character in characters.chars() {
            let unicode = format!("U+{:04X}", character as u32);
            let mut name = String::from("UNKNOWN");

            if let Some(character_name) = CONTROL_CHARACTERS.iter().find(|[control_character, _]| {
                control_character == &format!("{:X}", character as u32)
            }) {
                name = character_name[1].to_string();
            }

            if let Some(character_name) = get_unicode_name(character) {
                name = character_name.to_string();
            }

            unicodes.push(format!("`{unicode}` - {name}"));
        }

        unicodes = unicodes.into_iter().take(20).collect::<Vec<String>>();

        format!(
            "Showing first {} character(s):\n\n{}",
            unicodes.len(),
            unicodes.join("\n")
        )
    }

    async fn translate(string: &str, from_language: &str, to_language: &str) -> Result<Embed> {
        let from_language = GOOGLE_TRANSLATE_LANGUAGES
            .iter()
            .find(|[language, _]| language == &from_language)
            .unwrap_or(&GOOGLE_TRANSLATE_LANGUAGES[0]); // Set auto as fallback

        let to_language = GOOGLE_TRANSLATE_LANGUAGES
            .iter()
            .find(|[language, _]| language == &to_language)
            .unwrap_or(&GOOGLE_TRANSLATE_LANGUAGES[22]); // Set english as fallback

        let json  = get(format!("https://translate.googleapis.com/translate_a/single?client=gtx&dj=1&dt=t&sl={}&tl={}&q={string}", from_language[0], to_language[0])).await?.json::<GoogleTranslateResponse>().await?;

        Ok(Embed::new()
            .set_title(format!(
                "{}{} to {}",
                // Get origin language from the response
                GOOGLE_TRANSLATE_LANGUAGES
                    .iter()
                    .find(|[language, _]| language == &json.src)
                    .unwrap()[1],
                if from_language[0] == "auto" {
                    " (detected)"
                } else {
                    ""
                },
                to_language[1]
            ))
            .set_description(
                json.sentences
                    .into_iter()
                    .map(|sentence| sentence.trans) // 🏳️‍⚧️
                    .collect::<Vec<String>>()
                    .join("")
                    .chars()
                    .take(4000)
                    .collect::<String>(),
            ))
    }
}
