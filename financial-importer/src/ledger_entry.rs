use chrono::NaiveDate;
use color_eyre::eyre::{eyre, Result};

#[derive(Debug)]
pub enum EntryLine {
    Comment { comment: String },
    Posting { account: String, amount: f64 },
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
