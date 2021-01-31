use chrono::NaiveDate;
use color_eyre::eyre::{eyre, Result};
use format_num::NumberFormat;
use lazy_static::lazy_static;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
pub enum EntryLine {
    Comment { comment: String },
    Posting { account: String, amount: f64 },
}

static ENTRY_LINE_INDENT_AMOUNT: usize = 3;
static RIGHT_ALIGN_COLUMN: usize = 67;

impl fmt::Display for EntryLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        lazy_static! {
            static ref ENTRY_LINE_INDENT: String =
                String::from_utf8(vec![b' '; ENTRY_LINE_INDENT_AMOUNT]).unwrap();
            static ref NUMBER_FORMAT: NumberFormat = NumberFormat::new();
        }

        write!(f, "{}", *ENTRY_LINE_INDENT)?;

        match self {
            EntryLine::Comment { comment } => {
                write!(f, "; {}", comment)
            }
            EntryLine::Posting { account, amount } => {
                let formatted_number: String = NUMBER_FORMAT.format(",.2f", *amount);
                let formatted_amount: String = format!("${}", formatted_number);
                // To right align at RIGHT_ALIGN_COLUMN, need to calculate the remaining size
                let width: usize = RIGHT_ALIGN_COLUMN - ENTRY_LINE_INDENT_AMOUNT - account.len();
                write!(f, "{} {:>width$}", account, formatted_amount, width = width)
            }
        }
    }
}

pub struct LedgerEntryBuilder {
    pub date: NaiveDate,
    pub payee: String,
    pub lines: Vec<EntryLine>,
}

#[derive(Debug)]
pub struct LedgerEntry {
    pub date: NaiveDate,
    pub payee: String,
    pub lines: Vec<EntryLine>,
}

impl Ord for LedgerEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.date, &self.payee).cmp(&(other.date, &other.payee))
    }
}

impl PartialOrd for LedgerEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for LedgerEntry {
    fn eq(&self, other: &Self) -> bool {
        (self.date, &self.payee) == (other.date, &other.payee)
    }
}

impl Eq for LedgerEntry {}

impl LedgerEntryBuilder {
    #[must_use]
    pub fn new(date: NaiveDate, payee: String) -> Self {
        let lines = Vec::new();
        Self { date, payee, lines }
    }

    pub fn add_comment(&mut self, comment: String) {
        let entry_line = EntryLine::Comment { comment };
        self.lines.push(entry_line);
    }

    pub fn add_posting(&mut self, account: String, amount: f64) {
        let entry_line = EntryLine::Posting { account, amount };
        self.lines.push(entry_line);
    }

    pub fn build(self) -> Result<LedgerEntry> {
        self.validate()?;

        Ok(LedgerEntry {
            date: self.date,
            payee: self.payee,
            lines: self.lines,
        })
    }

    fn validate(&self) -> Result<()> {
        // Simple validation rules:
        // The Payee must be non-empty
        if self.payee.is_empty() {
            return Err(eyre!("Payee must be non-empty."));
        }

        let mut total: f64 = 0.0;
        let count = self
            .lines
            .iter()
            .filter(|line| matches!(line, EntryLine::Posting { .. }))
            .inspect(|line| {
                if let EntryLine::Posting { account: _, amount } = line {
                    total += *amount
                }
            })
            .count();

        // The lines must have at least 2 Postings
        if count < 2 {
            return Err(eyre!(
                "Ledger entry must have 2 or more postings. Found {}.",
                count
            ));
        }

        // The Postings must balance, or amounts must add up to 0.0
        if total != 0.0 {
            return Err(eyre!(
                "Ledger entry posting lines must balance, found total of {:.2}",
                total
            ));
        }

        Ok(())
    }
}

impl fmt::Display for LedgerEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} {}", self.date.format("%Y/%m/%d"), self.payee)?;

        for line in &self.lines {
            writeln!(f, "{}", line)?;
        }

        Ok(())
    }
}

pub fn write_ledger_entries_file(filename: &PathBuf, entries: Vec<LedgerEntry>) -> Result<()> {
    // TODO: Check to avoid overwriting existing file
    let mut output_file = File::create(filename.as_path())?;

    for entry in entries {
        writeln!(output_file, "{}", entry)?;
    }

    Ok(())
}
