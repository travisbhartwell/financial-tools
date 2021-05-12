use chrono::NaiveDate;
use color_eyre::eyre::Result;
use log::{info, trace};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceRecord {
    pub date: NaiveDate,
    pub description: String,
    pub amount: f64,
}

impl Ord for SourceRecord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.date, &self.description).cmp(&(other.date, &other.description))
    }
}

impl PartialOrd for SourceRecord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SourceRecord {
    fn eq(&self, other: &Self) -> bool {
        (self.date, &self.description) == (other.date, &other.description)
    }
}

impl Eq for SourceRecord {}

pub fn load_source_records(input_path: &Path) -> Result<Vec<SourceRecord>> {
    trace!(
        "Processing CSV using input file '{}'.",
        input_path.to_str().unwrap()
    );
    let mut reader = csv::Reader::from_path(input_path)?;
    let (rows, errors): (Vec<_>, Vec<_>) = reader
        .deserialize::<SourceRecord>()
        .into_iter()
        .partition(Result::is_ok);

    let rows: Vec<SourceRecord> = rows.into_iter().map(Result::unwrap).collect();

    trace!("Successfully loaded {} source records.", rows.len());

    if !errors.is_empty() {
        info!(
            "Encountered {} errors in loading source records",
            errors.len()
        );

        for error in errors {
            let error = error.unwrap_err();
            eprintln!("Got error: {:?}", error.kind());
            if let Some(position) = error.position() {
                eprintln!(
                    "\tError at Line: {}, Byte: {}",
                    position.line(),
                    position.byte()
                );
            }
        }
    }

    Ok(rows)
}

pub fn write_source_records(output_path: &Path, source_records: &[&SourceRecord]) -> Result<()> {
    trace!(
        "Writing unmatched source records to '{}'.",
        output_path.to_str().unwrap()
    );

    // TODO: Check to avoid overwriting existing file
    let mut writer = csv::Writer::from_path(output_path)?;

    for record in source_records {
        writer.serialize(*record)?;
    }

    trace!(
        "Successfully wrote {} records to the CSV file.",
        source_records.len()
    );

    Ok(())
}
