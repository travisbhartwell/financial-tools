use color_eyre::eyre::Error;
use lazy_static::lazy_static;
use log::trace;
use regex::{Regex, RegexSet};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;

// Deserialization and Validation technique borrowed from
// https://github.com/serde-rs/serde/issues/642#issuecomment-683276351
pub type AccountMap = HashMap<String, String>;
pub type ImportFileDefinitionMap = HashMap<String, TransactionMatcher>;

#[derive(Deserialize)]
pub struct FinancialImporter {
    pub accounts: AccountMap,
    pub import_file_definitions: ImportFileDefinitionMap,
}

// trace!("Loaded {} account alias definitions.", accounts.len());

#[derive(Deserialize)]
#[serde(try_from = "TransactionMatcherConfiguration")]
pub struct TransactionMatcher {
    // TODO pub file_format_name: String,
    pub transaction_rules: Vec<TransactionRule>,
    // pub fallback_rule: TransactionRule,
    pub rule_patterns: RegexSet,
}

#[derive(Deserialize)]
pub struct TransactionMatcherConfiguration {
    // pub fallback_rule: TransactionRule,
    pub transaction_rules: Vec<TransactionRule>,
}

impl TryFrom<TransactionMatcherConfiguration> for TransactionMatcher {
    type Error = Error;

    fn try_from(
        TransactionMatcherConfiguration { transaction_rules }: TransactionMatcherConfiguration,
    ) -> Result<Self, Self::Error> {
        // TODO: Accumulate and report errors for missing account aliases
        // transaction_rules.iter().map(|&rule| {
        //     accounts.contains_key(&rule.account1) && accounts.contains_key(&rule.account2)
        // });
        let patterns = transaction_rules.iter().map(|rule| &rule.pattern_string);
        let rule_patterns: RegexSet = RegexSet::new(patterns)?;
        trace!(
            "Loaded matcher with {} compiled patterns.",
            rule_patterns.len()
        );

        let matcher = TransactionMatcher {
            transaction_rules,
            rule_patterns,
        };
        Ok(matcher)
    }
}

#[derive(Deserialize)]
#[serde(try_from = "TransactionRuleConfiguration")]
pub struct TransactionRule {
    pub name: String,
    pub pattern_string: String,
    pub account1: String,
    pub account2: String,
    pub payee: String,
    pub needs_finalized: bool,
    pub negate_first_amount: bool,
    pub pattern: Option<Regex>,
    pub payee_is_template: bool,
}

#[derive(Deserialize, Serialize)]
pub struct TransactionRuleConfiguration {
    pub name: Option<String>,
    pub pattern_string: String,
    pub account1: String,
    pub account2: String,
    pub payee: String,
    pub needs_finalized: Option<bool>,
    pub negate_first_amount: Option<bool>,
}

impl TryFrom<TransactionRuleConfiguration> for TransactionRule {
    type Error = Error;

    fn try_from(
        TransactionRuleConfiguration {
            name,
            pattern_string,
            account1,
            account2,
            payee,
            needs_finalized,
            negate_first_amount,
        }: TransactionRuleConfiguration,
    ) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref PAYEE_TEMPLATE_RE: Regex = Regex::new(r"\{[^}]+\}").unwrap();
        }
        let name_string: String = match name {
            Some(name_string) => name_string,
            None => {
                format!("Payee: '{}' with pattern '{}'.", payee, pattern_string)
            }
        };
        let needs_finalized_bool: bool = needs_finalized.unwrap_or(false);
        let negate_first_amount_bool: bool = negate_first_amount.unwrap_or(false);
        let payee_is_template: bool = PAYEE_TEMPLATE_RE.is_match(payee.as_str());

        // First compile the regex here to make sure it's valid
        let pattern_re: Regex = Regex::new(pattern_string.as_str())?;
        // We only need to keep a separate Regex for the rule if the Payee is a template
        // and thus requiring captures, which are not available for RegexSet.
        let pattern: Option<Regex> = if payee_is_template {
            Some(pattern_re)
        } else {
            None
        };

        let rule = TransactionRule {
            name: name_string,
            pattern_string,
            account1,
            account2,
            payee,
            pattern,
            payee_is_template,
            needs_finalized: needs_finalized_bool,
            negate_first_amount: negate_first_amount_bool,
        };

        trace!("Loaded Transaction Rule: '{}'", &rule.name);

        Ok(rule)
    }
}
