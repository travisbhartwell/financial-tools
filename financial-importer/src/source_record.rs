use chrono::NaiveDate;
use color_eyre::eyre::Result;
use log::{info, trace};
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

    let (rows, errors): (Vec<_>, Vec<_>) = reader
        .deserialize::<SourceRecord>()
        .into_iter()
        .partition(Result::is_ok);

    let rows: Vec<SourceRecord> = rows.into_iter().map(Result::unwrap).collect();

    trace!("Successfully loaded {} source records.", rows.len());
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

    Ok(rows)
}
