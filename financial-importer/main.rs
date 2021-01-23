use serde::Deserialize;

use std::error::Error;
use std::io;
use std::process;

use chrono::{NaiveDate, Datelike};

#[derive(Debug, Deserialize)]
struct Record {
    date: NaiveDate,
    description: String,
    amount: f64,
}

fn example() -> Result<(), Box<Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        println!("Date: {}", record.date.month());
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
