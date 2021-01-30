use chrono::NaiveDate;
use color_eyre::eyre::Result;
use log::trace;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct SourceRecord {
    pub date: NaiveDate,
    pub description: String,
    pub amount: f64,
}

pub fn load_source_records(input_path: PathBuf) -> Result<Vec<SourceRecord>> {
    let mut reader = csv::Reader::from_path(input_path)?;

    // TODO: Figure out error handling
    // - Collect into Vec<Result<SourceRecord,csv::Error>>
    // - Partition in SourceRecord, csv::Error
    // - Show errors, but continue
    let rows: Vec<SourceRecord> = reader
        .deserialize::<SourceRecord>()
        .flat_map(|row| row)
        .collect();

    trace!("Successfully parsed {} source records.", rows.len());

    Ok(rows)
}
