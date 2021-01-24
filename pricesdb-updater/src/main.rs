use chrono::{offset::TimeZone, DateTime, Local, NaiveDate};
use color_eyre::eyre::Result;
use futures::stream::futures_unordered::FuturesUnordered;
use futures::stream::StreamExt;
use std::path::PathBuf;

use structopt::StructOpt;

use pricesdb_updater::{get_commodity_history, write_pricesdb_file, HistoricPrice};

fn parse_local_datetime(src: &str) -> Result<DateTime<Local>> {
    // First Get a NaiveDate
    let naive_date: NaiveDate = src.parse()?;

    // Then convert to a Local DateTime at midnight
    let local_date_time: DateTime<Local> = Local
        .from_local_date(&naive_date)
        .unwrap()
        .and_hms_milli(0, 0, 0, 0);

    Ok(local_date_time)
}

#[derive(Debug, StructOpt)]
#[structopt(name = "pricesdb-updater")]
struct Cli {
    // Output File
    #[structopt(short = "o", long = "output-file", parse(from_os_str))]
    output_file: PathBuf,

    #[structopt(short = "s", long = "start-date", parse(try_from_str = parse_local_datetime))]
    start_date: DateTime<Local>,

    #[structopt(short = "e", long = "end-date", parse(try_from_str = parse_local_datetime))]
    end_date: DateTime<Local>,

    #[structopt(short = "c", long = "commodity")]
    commodities: Vec<String>,
}

async fn update_prices_db(cli: Cli) -> Result<()> {
    let (price_histories, errors): (Vec<_>, Vec<_>) = cli
        .commodities
        .iter()
        .map(|commodity| get_commodity_history(commodity.clone(), cli.start_date, cli.end_date))
        .collect::<FuturesUnordered<_>>()
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .partition(Result::is_ok);

    let mut price_histories: Vec<HistoricPrice> = price_histories
        .into_iter()
        .flat_map(Result::unwrap)
        .collect();

    price_histories.sort();

    let errors: Vec<_> = errors.into_iter().flat_map(Result::err).collect();

    write_pricesdb_file(cli.output_file, price_histories)?;
  
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[tokio::main]
pub async fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::from_args();

    update_prices_db(cli).await?;

    Ok(())
}
