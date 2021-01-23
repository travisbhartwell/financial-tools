use financial_importer::{FinancialImporter, SourceRecord};
use std::io;
use color_eyre::eyre::Result;


fn load_config() -> Result<FinancialImporter> {
    let contents = std::fs::read_to_string("test.toml")
        .unwrap_or_else(|_| panic!("Problems reading from file"));

    let importer: FinancialImporter = toml::from_str(&contents)?;

    Ok(importer)
}

fn process_csv_stdio(importer: FinancialImporter) -> Result<()> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: SourceRecord = result?;

        if let Some(posting) = importer.posting_for_record(&record)? {
            println!("Posting generated for input: {}", &record);
            println!("{}", posting);
        } else {
            println!("No match for input: {}", &record);
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let importer: FinancialImporter = load_config()?;
    process_csv_stdio(importer)?;

    Ok(())
}

