use chrono::{offset::TimeZone, DateTime, Local, NaiveDate};
use color_eyre::eyre::Result;
use std::path::PathBuf;
use structopt::StructOpt;

use pricesdb_updater;

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

#[tokio::main]
pub async fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::from_args();

    let commodity: String = cli.commodities.first().unwrap().clone();

    let price_history = pricesdb_updater::get_commodity_history(commodity, cli.start_date, cli.end_date).await?;

    for price in price_history.iter() {
        println!("{}", price);
    }

    Ok(())
}
