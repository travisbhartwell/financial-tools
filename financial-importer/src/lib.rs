use chrono::NaiveDate;
use color_eyre::eyre::{eyre, Error, Result};
use format_num::NumberFormat;
use lazy_static::lazy_static;
use regex::{Regex, RegexSet};
use serde::Deserialize;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct SourceRecord {
    pub date: NaiveDate,
    pub description: String,
    pub amount: f64,
}

impl fmt::Display for SourceRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Date: {}, Description: '{}', Amount: {}",
            self.date, self.description, self.amount
        )
    }
}

impl SourceRecord {
    fn format_amount(&self, negate: bool) -> String {
        lazy_static! {
            static ref NUMBER_FORMAT: NumberFormat = NumberFormat::new();
        }

        let amount = if negate { -self.amount } else { self.amount };

        let formatted_number: String = NUMBER_FORMAT.format(",.2f", amount);

        // Hard-code the dollar-sign for now.
        format!("${}", formatted_number)
    }

    #[must_use]
    pub fn formatted_amount(&self) -> String {
        self.format_amount(false)
    }

    #[must_use]
    pub fn formatted_negative_amount(&self) -> String {
        self.format_amount(true)
    }

    #[must_use]
    pub fn formatted_date(&self) -> String {
        // We want the date in the format YYYY/MM/DD
        format!("{}", self.date.format("%Y/%m/%d"))
    }
}

pub type AccountAlias = String;
pub type FullAccountName = String;
pub type AccountMap = HashMap<AccountAlias, FullAccountName>;
pub type Payee = String;
// Deserialization and Validation technique borrowed from
// https://github.com/serde-rs/serde/issues/642#issuecomment-683276351
#[derive(Deserialize)]
pub struct TransactionRuleConfiguration {
    pub name: Option<String>,
    pub pattern_string: String,
    pub account1: AccountAlias,
    pub account2: AccountAlias,
    pub payee: Payee,
    pub needs_finalized: Option<bool>,
}

impl TransactionRuleConfiguration {
    pub fn new(
        name: Option<String>,
        pattern_string: String,
        account1: AccountAlias,
        account2: AccountAlias,
        payee: Payee,
        needs_finalized: Option<bool>,
    ) -> Self {
        Self {
            name,
            pattern_string,
            account1,
            account2,
            payee,
            needs_finalized,
        }
    }
}

#[derive(Deserialize)]
#[serde(try_from = "TransactionRuleConfiguration")]
pub struct TransactionRule {
    pub name: String,
    pub pattern_string: String,
    pub account1: AccountAlias,
    pub account2: AccountAlias,
    pub payee: Payee,
    pub needs_finalized: bool,
    pattern: Option<Regex>,
    payee_is_template: bool,
}

impl TransactionRule {
    // Is it safe to assume if this is called that the Record is a match?
    fn posting_for_record(&self, account_map: &AccountMap, record: &SourceRecord) -> String {
        // First, just handle the regular and simple case.
        // if self.payee_is_template {
        //     let pattern = self.pattern.unwrap();

        //     let templates = pattern.captures(record.description);
        // }
        let payee = &self.payee;
        let formatted_date = record.formatted_date();

        let account1 = account_map.get(&self.account1).unwrap();
        let formatted_pos_amount = record.formatted_amount();

        let account2 = account_map.get(&self.account2).unwrap();
        let formatted_neg_amount = record.formatted_negative_amount();
        format!(
            r"{} {}
    {}                             {}
    {}                            {}
",
            formatted_date, payee, account1, formatted_pos_amount, account2, formatted_neg_amount
        )
    }
}

impl TryFrom<TransactionRuleConfiguration> for TransactionRule {
    type Error = Error;

    fn try_from(config: TransactionRuleConfiguration) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref PAYEE_TEMPLATE_RE: Regex = Regex::new(r"\{[^}]+\}").unwrap();
        }

        let TransactionRuleConfiguration {
            name,
            pattern_string,
            account1,
            account2,
            payee,
            needs_finalized,
        } = config;

        let name_string: String = match name {
            Some(name_string) => name_string,
            None => {
                format!("Payee: '{}' with pattern '{}'.", payee, pattern_string)
            }
        };

        let needs_finalized_bool: bool = needs_finalized.unwrap_or(false);

        let payee_is_template: bool = PAYEE_TEMPLATE_RE.is_match(payee.as_str());

        // We only need a separate Regex for the rule if the Payee is a template
        // and thus requiring captures, which are not available for RegexSet.
        let pattern: Option<Regex> = if payee_is_template {
            let pattern: Regex = Regex::new(pattern_string.as_str())?;
            Some(pattern)
        } else {
            None
        };

        Ok(TransactionRule {
            name: name_string,
            pattern_string,
            account1,
            account2,
            payee,
            pattern,
            payee_is_template,
            needs_finalized: needs_finalized_bool,
        })
    }
}

#[derive(Deserialize)]
pub struct ImporterConfiguration {
    pub accounts: AccountMap,
    pub transaction_rules: Vec<TransactionRule>,
}

impl ImporterConfiguration {
    pub fn new() -> Self {
        let accounts: AccountMap = HashMap::new();
        let transaction_rules: Vec<TransactionRule> = Vec::new();

        Self {
            accounts,
            transaction_rules,
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
        let patterns = transaction_rules.iter().map(|rule| &rule.pattern_string);

        let rule_patterns: RegexSet = RegexSet::new(patterns)?;

        Ok(FinancialImporter {
            accounts,
            transaction_rules,
            rule_patterns,
        })
    }
}

impl FinancialImporter {
    pub fn posting_for_record(&self, record: &SourceRecord) -> eyre::Result<Option<String>> {
        let rule_matches: Vec<_> = self
            .rule_patterns
            .matches(&record.description)
            .into_iter()
            .collect();

        match rule_matches.len() {
            // No matches, no errors.
            0 => Ok(None),
            1 => {
                let index = rule_matches[0];
                let rule: &TransactionRule = &self.transaction_rules[index];
                let posting = rule.posting_for_record(&self.accounts, record);
                Ok(Some(posting))
            }
            // We consider multiple matches an error. The importer
            // configuration should be updated to not have overlapping
            // patterns.
            // TODO Give more meaningful error.
            _ => {
                Err(eyre!("Too many matches!"))
            }
        }
    }

    pub fn match_rule(&self, input: &str) {
        let rule_matches: Vec<_> = self.rule_patterns.matches(input).into_iter().collect();

        match rule_matches.len() {
            0 => println!("No matches found for {}.", input),
            1 => {
                let index = rule_matches[0];
                let rule: &TransactionRule = &self.transaction_rules[index];
                println!(
                    "Rule named '{}' matched for input '{}' with pattern '{}'.",
                    rule.name, input, rule.pattern_string
                );
                if rule.payee_is_template {
                    println!("\t Payee is template with the following matched variables:");

                    if let Some(pattern) = &rule.pattern {
                        let templates = pattern.captures(input).unwrap();
                        // Borrowed from https://stackoverflow.com/a/54259908
                        let dict: HashMap<&str, &str> = pattern
                            .capture_names()
                            .flatten()
                            .filter_map(|n| Some((n, templates.name(n)?.as_str())))
                            .collect();
                        println!("\t{:#?}", dict);
                    }
                }
            }
            _ => {
                eprintln!("Multiple matches found for input '{}'.", input);
                for index in rule_matches {
                    eprintln!(
                        "\tMatched rule with pattern: '{}'",
                        self.transaction_rules[index].pattern_string
                    );
                }
            }
        }
    }
}
