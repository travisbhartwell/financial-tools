use color_eyre::eyre::{eyre, Result};
use log::trace;
use source_record::SourceRecord;

use crate::ledger_entry::{LedgerEntry, LedgerEntryBuilder};
use crate::source_record;

use super::definitions::{AccountMap, FinancialImporter, TransactionMatcher, TransactionRule};

impl FinancialImporter {
    pub fn ledger_entry_for_source_record(
        &self,
        file_format: &str,
        record: &SourceRecord,
    ) -> Result<Option<LedgerEntry>> {
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
    pub fn ledger_entry_for_source_record(
        &self,
        accounts: &AccountMap,
        record: &SourceRecord,
    ) -> Result<Option<LedgerEntry>> {
        let rule_matches: Vec<_> = self
            .rule_patterns
            .matches(&record.description)
            .into_iter()
            .collect();

        match rule_matches.len() {
            0 => {
                trace!(
                    "No match found for record with description '{}'",
                    record.description
                );
                Ok(None)
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
                    Ok(posting) => Ok(Some(posting)),
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
        // TODO Handle template case for the Payee
        let mut entry_builder: LedgerEntryBuilder =
            LedgerEntryBuilder::new(record.date, self.payee.clone());

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
