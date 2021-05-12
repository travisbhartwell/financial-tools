use chrono::{DateTime, Local, Utc};
use color_eyre::eyre::{Error, Report, WrapErr};
use std::io::Write;
use std::{convert::TryFrom, fmt::Display};
use std::{
    fs::File,
    path::PathBuf,
    time::{Duration, UNIX_EPOCH},
};

#[derive(Debug)]
pub struct HistoricPrice {
    timestamp: DateTime<Local>,
    symbol: String,
    value: f64,
}

impl Ord for HistoricPrice {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.timestamp, &self.symbol).cmp(&(other.timestamp, &other.symbol))
    }
}

impl PartialOrd for HistoricPrice {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HistoricPrice {
    fn eq(&self, other: &Self) -> bool {
        (self.timestamp, &self.symbol) == (other.timestamp, &other.symbol)
    }
}

impl Eq for HistoricPrice {}

impl Display for HistoricPrice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let timestamp_str = self.timestamp.format("%Y/%m/%d %H:%M:%S").to_string();
        write!(f, "P {} {} ${:.4}", timestamp_str, self.symbol, self.value)
    }
}

impl TryFrom<(&yahoo_finance_api::Quote, &str)> for HistoricPrice {
    type Error = Error;

    fn try_from(
        commodity_quote: (&yahoo_finance_api::Quote, &str),
    ) -> eyre::Result<Self, Self::Error> {
        let (quote, symbol) = commodity_quote;

        let timestamp: DateTime<Local> =
            DateTime::from(UNIX_EPOCH + Duration::from_secs(quote.timestamp));

        let price: HistoricPrice = HistoricPrice {
            timestamp,
            symbol: String::from(symbol),
            value: quote.close,
        };

        Ok(price)
    }
}

pub async fn get_commodity_history(
    commodity: String,
    start: DateTime<Local>,
    end: DateTime<Local>,
) -> Result<Vec<HistoricPrice>, Report> {
    let provider = yahoo_finance_api::YahooConnector::new();
    eprintln!("Fetching prices for {}.", commodity);

    let response = provider.get_quote_history(
        &commodity,
        start.with_timezone(&Utc),
        end.with_timezone(&Utc),
    );

    let commodity_str = commodity.as_str();

    let history = response
        .await
        .wrap_err_with(|| format!("Problems fetching price history for '{}'.", commodity_str))?;

    let price_history: Result<Vec<HistoricPrice>, Report> = history
        .quotes()
        .wrap_err_with(|| format!("Problems fetching price history for '{}'.", commodity_str))?
        .iter()
        .map(|quote| HistoricPrice::try_from((quote, commodity_str)))
        .collect();

    price_history
}

pub fn write_pricesdb_file(
    filename: PathBuf,
    prices_history: Vec<HistoricPrice>,
) -> Result<(), Report> {
    let mut output_file = File::create(filename.as_path())?;

    for price in prices_history.iter() {
        writeln!(output_file, "{}", price)?;
    }

    Ok(())
}
