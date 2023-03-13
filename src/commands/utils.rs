use crate::{
    constants::{CONTROL_CHARACTERS, CURRENCIES},
    structs::ExchangeRateConversion,
};
use nipper::Document;
use reqwest::get;
use slashook::{command, Client};
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

pub struct Utils<'a> {
    client: &'a mut Client,
}

impl<'a> Utils<'a> {
    pub fn init(client: &'a mut Client) -> Self {
        Utils { client }
    }

    pub fn register(&mut self) {
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
                let search = input
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
                                currency[0].to_lowercase().contains(&search)
                                    || currency[1].to_lowercase().contains(&search)
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

            let converted = (get(format!(
                "https://api.exchangerate.host/convert?amount={amount}&from={from_currency}&to={to_currency}"
            ))
            .await?
            .json::<ExchangeRateConversion>()
            .await?).result;

            res.send_message(format!(
                "{amount} {from_currency} equals {converted:.3} {to_currency}."
            ))
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
        async fn list_unicodes(input: CommandInput, res: CommandResponder) {
            res.send_message(Utils::parse_unicodes(
                &input.target_message.unwrap().content,
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

                        let currency = document.select("#quote-header-info span").first().text();
                        let currency = currency.split(" ").last().unwrap();

                        let price = document
                            .select("#quote-header-info [data-field=\"regularMarketPrice\"]")
                            .first()
                            .text();

                        let diff = ["regularMarketChange", "regularMarketChangePercent"]
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
                            .join(" ");

                        format!("```diff\n{currency} {price}\n{diff}```")
                    }),
            )
            .await?;
        }

        self.client.register_command(convert_currency);
        self.client.register_command(unicode);
        self.client.register_command(list_unicodes);
        self.client.register_command(stock);
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
}
