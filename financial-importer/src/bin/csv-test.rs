use std::error::Error;
use std::io;
use std::process;
use financial_importer::SourceRecord;

fn example() -> Result<(), Box<Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: SourceRecord = result?;
        println!(
            "Date: '{}', Amount: '{}', Description: '{}'",
            record.formatted_date(),
            record.formatted_amount(),
            record.description
        );
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
