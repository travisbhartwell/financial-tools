use color_eyre::eyre::{eyre, Result};
use log::trace;
use source_record::SourceRecord;

use crate::ledger_entry::{LedgerEntry, LedgerEntryBuilder};
use crate::source_record;

use super::definitions::{AccountMap, FinancialImporter, TransactionMatcher, TransactionRule};

pub enum GeneratedLedgerEntry<'a> {
    ByMatchedRule {
        ledger_entry: LedgerEntry,
        source_record: &'a SourceRecord,
    },
    ByFallback {
        ledger_entry: LedgerEntry,
        source_record: &'a SourceRecord,
    },
}

impl<'a> GeneratedLedgerEntry<'a> {
    #[must_use]
    pub fn unwrap_entry(self) -> LedgerEntry {
        match self {
            GeneratedLedgerEntry::ByMatchedRule {
                ledger_entry,
                source_record: _,
            }
            | GeneratedLedgerEntry::ByFallback {
                ledger_entry,
                source_record: _,
            } => ledger_entry,
        }
    }

    #[must_use]
    pub fn unwrap_source_record(self) -> &'a SourceRecord {
        match self {
            GeneratedLedgerEntry::ByMatchedRule {
                ledger_entry: _,
                source_record,
            }
            | GeneratedLedgerEntry::ByFallback {
                ledger_entry: _,
                source_record,
            } => source_record,
        }
    }

    #[must_use]
    pub fn unwrap(self) -> (LedgerEntry, &'a SourceRecord) {
        match self {
            GeneratedLedgerEntry::ByMatchedRule {
                ledger_entry,
                source_record,
            }
            | GeneratedLedgerEntry::ByFallback {
                ledger_entry,
                source_record,
            } => (ledger_entry, source_record),
        }
    }

    #[must_use]
    pub fn from_matched_rule(&self) -> bool {
        matches!(*self, GeneratedLedgerEntry::ByMatchedRule { .. })
    }
}

impl FinancialImporter {
    pub fn ledger_entry_for_source_record<'a>(
        &self,
        file_format: &str,
        record: &'a SourceRecord,
    ) -> Result<GeneratedLedgerEntry<'a>> {
        let matcher: &TransactionMatcher = self
            .import_file_definitions
            .get(file_format)
            .ok_or_else(|| {
                eyre!(format!(
                    "File format definition '{}' not found.",
                    file_format
                ))
            })?;

        matcher.ledger_entry_for_source_record(&self.accounts, record)
    }
}

impl TransactionMatcher {
    pub fn ledger_entry_for_source_record<'a>(
        &self,
        accounts: &AccountMap,
        record: &'a SourceRecord,
    ) -> Result<GeneratedLedgerEntry<'a>> {
        let rule_matches: Vec<_> = self
            .rule_patterns
            .matches(&record.description)
            .into_iter()
            .collect();

        match rule_matches.len() {
            0 => {
                trace!(
                    "No match found for record with description '{}', using fallback rule.",
                    record.description
                );

                match self
                    .fallback_rule
                    .ledger_entry_for_source_record(accounts, record)
                {
                    Ok(ledger_entry) => Ok(GeneratedLedgerEntry::ByFallback {
                        ledger_entry,
                        source_record: record,
                    }),
                    Err(e) => Err(e),
                }
            }
            1 => {
                let rule_index = rule_matches[0];
                let rule: &TransactionRule = &self.transaction_rules[rule_index];
                trace!(
                    "Rule named '{}' matched for record with description '{}' by pattern '{}'",
                    rule.name,
                    record.description,
                    rule.pattern_string
                );

                match rule.ledger_entry_for_source_record(accounts, record) {
                    Ok(ledger_entry) => Ok(GeneratedLedgerEntry::ByMatchedRule {
                        ledger_entry,
                        source_record: record,
                    }),
                    Err(e) => Err(e),
                }
            }
            _ => {
                let mut error_str: String = String::from("Found multiple matches: ");
                for rule_index in rule_matches {
                    error_str.push_str(
                        format!(", {}", self.transaction_rules[rule_index].name).as_str(),
                    );
                }
                Err(eyre!(error_str))
            }
        }
    }
}

static SOURCE_COMMENT: &str = "SOURCE";
static NEEDS_FINALIZED_COMMENT: &str = "NEEDS FINALIZED";

impl TransactionRule {
    pub fn ledger_entry_for_source_record(
        &self,
        account_map: &AccountMap,
        record: &SourceRecord,
    ) -> Result<LedgerEntry> {
        let payee = if self.payee_is_template {
            if let Some(pattern) = &self.pattern {
                let mut payee = String::new();
                let templates = pattern.captures(&record.description).unwrap();
                templates.expand(&self.payee, &mut payee);
                payee
            } else {
                panic!("Regex missing for template pattern!")
            }
        } else {
            self.payee.clone()
        };

        let mut entry_builder: LedgerEntryBuilder = LedgerEntryBuilder::new(record.date, payee);

        // Add the source record description as a comment:
        entry_builder.add_comment(format!("{}: {}", SOURCE_COMMENT, record.description));

        if self.needs_finalized {
            entry_builder.add_comment(NEEDS_FINALIZED_COMMENT.to_string());
        }

        let account1 = account_map.get(&self.account1).unwrap();
        let account2 = account_map.get(&self.account2).unwrap();

        if self.negate_first_amount {
            entry_builder.add_posting(account1.clone(), -record.amount);
            entry_builder.add_posting(account2.clone(), record.amount);
        } else {
            entry_builder.add_posting(account1.clone(), record.amount);
            entry_builder.add_posting(account2.clone(), -record.amount);
        }

        entry_builder.build()
    }
}
