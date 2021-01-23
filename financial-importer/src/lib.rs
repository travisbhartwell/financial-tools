use color_eyre::eyre::{Error, Result};
use regex::RegexSet;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;

// Deserialization and Validation technique borrowed from
// https://github.com/serde-rs/serde/issues/642#issuecomment-683276351

pub type AccountAlias = String;
pub type FullAccountName = String;

pub type AccountMap = HashMap<AccountAlias, FullAccountName>;

pub type Payee = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionRule {
    pub pattern: String,
    pub account1: AccountAlias,
    pub account2: AccountAlias,
    pub payee: Payee,
}

impl TransactionRule {
    pub fn new(
        pattern_string: String,
        account1: AccountAlias,
        account2: AccountAlias,
        payee: Payee,
    ) -> Self {
        Self {
            pattern: pattern_string,
            account1: account1,
            account2: account2,
            payee: payee,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ImporterConfiguration {
    pub accounts: AccountMap,
    pub transaction_rules: Vec<TransactionRule>,
}

impl ImporterConfiguration {
    pub fn new() -> Self {
        let account_map: AccountMap = HashMap::new();
        let transaction_rules: Vec<TransactionRule> = Vec::new();

        Self {
            accounts: account_map,
            transaction_rules: transaction_rules,
        }
    }

    pub fn add_account_alias(&mut self, alias: AccountAlias, full_name: FullAccountName) {
        self.accounts.insert(alias, full_name);
    }

    pub fn add_transaction_rule(&mut self, transaction_rule: TransactionRule) {
        self.transaction_rules.push(transaction_rule);
    }
}

#[derive(Deserialize)]
#[serde(try_from = "ImporterConfiguration")]
pub struct FinancialImporter {
    pub accounts: AccountMap,
    pub transaction_rules: Vec<TransactionRule>,
    rule_patterns: RegexSet,
}

impl TryFrom<ImporterConfiguration> for FinancialImporter {
    type Error = Error;

    fn try_from(config: ImporterConfiguration) -> Result<Self, Self::Error> {
        let ImporterConfiguration {
            accounts,
            transaction_rules,
        } = config;

        // TODO: Accumulate and report errors for missing account aliases
        // transaction_rules.iter().map(|&rule| {
        //     accounts.contains_key(&rule.account1) && accounts.contains_key(&rule.account2)
        // });

        let patterns = transaction_rules.iter().map(|rule| rule.pattern.as_str());

        let rule_patterns: RegexSet = RegexSet::new(patterns)?;

        Ok(FinancialImporter {
            accounts,
            transaction_rules,
            rule_patterns,
        })
    }
}

impl FinancialImporter {
    pub fn match_rule(&self, input: &str) {
        let rule_matches: Vec<_> = self.rule_patterns.matches(input).into_iter().collect();

        match rule_matches.len() {
            0 => println!("No matches found for {}.", input),
            1 => {
                let index = rule_matches[0];
                let rule: &TransactionRule = &self.transaction_rules[index];
                println!(
                    "Rule matched for input '{}' with pattern '{}'.",
                    input, rule.pattern
                );
            }
            _ => {
                eprintln!("Multiple matches found for input '{}'.", input);
                for index in rule_matches.into_iter() {
                    eprintln!(
                        "\tMatched rule with pattern: '{}'",
                        self.transaction_rules[index].pattern
                    );
                }
            }
        }
    }
}
