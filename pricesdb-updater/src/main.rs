use chrono::{DateTime, Local, TimeZone, Utc};
use std::time::{Duration, UNIX_EPOCH};
use std::{convert::TryFrom, fmt::Display};
use yahoo_finance_api;

struct HistoricPrice {
    timestamp: DateTime<Local>,
    symbol: String,
    value: f64,
}

impl Display for HistoricPrice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let timestamp_str = self.timestamp.format("%Y/%m/%d %H:%M:%S").to_string();
        write!(f, "P {} {} ${:.4}", timestamp_str, self.symbol, self.value)
    }
}

impl TryFrom<&yahoo_finance_api::Quote> for HistoricPrice {
    type Error = &'static str;

    fn try_from(quote: &yahoo_finance_api::Quote) -> Result<Self, Self::Error> {
        let timestamp: DateTime<Local> =
            DateTime::from(UNIX_EPOCH + Duration::from_secs(quote.timestamp));

        let price: HistoricPrice = HistoricPrice {
            timestamp,
            symbol: String::from("AMZN"),
            value: quote.close,
        };

        Ok(price)
    }
}

#[tokio::main]
pub async fn main() {
    let provider = yahoo_finance_api::YahooConnector::new();
    let start: DateTime<Local> = Local.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0);
    let end: DateTime<Local> = Local.ymd(2021, 1, 1).and_hms_milli(0, 0, 0, 0);

    let response =
        provider.get_quote_history("AMZN", start.with_timezone(&Utc), end.with_timezone(&Utc));

    let history = response.await.unwrap();

    let price_history: Vec<HistoricPrice> = history
        .quotes()
        .unwrap()
        .iter()
        .map(HistoricPrice::try_from)
        .filter_map(Result::ok)
        .collect();

    for price in price_history.iter() {
        println!("{}", price);
    }
}
