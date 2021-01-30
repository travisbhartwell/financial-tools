use color_eyre::{
    eyre::{eyre, Error},
    Result,
};
use lazy_static::lazy_static;
use log::trace;
use regex::{Regex, RegexSet};
use serde::Deserialize;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::iter::once;

// Deserialization and Validation technique borrowed from
// https://github.com/serde-rs/serde/issues/642#issuecomment-683276351
pub type AccountMap = HashMap<String, String>;
pub type ImportFileDefinitionMap = HashMap<String, TransactionMatcher>;

#[derive(Deserialize)]
#[serde(try_from = "FinancialImporterConfiguration")]
pub struct FinancialImporter {
    pub accounts: AccountMap,
    pub import_file_definitions: ImportFileDefinitionMap,
}

#[derive(Deserialize)]
pub struct FinancialImporterConfiguration {
    pub accounts: AccountMap,
    pub import_file_definitions: ImportFileDefinitionMap,
}

impl TryFrom<FinancialImporterConfiguration> for FinancialImporter {
    type Error = Error;

    fn try_from(
        FinancialImporterConfiguration {
            accounts,
            import_file_definitions,
        }: FinancialImporterConfiguration,
    ) -> Result<Self, Self::Error> {
        trace!("Loaded {} account alias definitions.", accounts.len());

        let validation_errors: Vec<_> = import_file_definitions
            .values()
            .flat_map(|matcher| matcher.validate_rule_account_aliases(&accounts))
            .filter(Result::is_err)
            .collect();

        if validation_errors.is_empty() {
            Ok(FinancialImporter {
                accounts,
                import_file_definitions,
            })
        } else {
            validation_errors.into_iter().map(Result::unwrap_err).fold(
                Err(eyre!(
                    "One or more account alias validation errors were found:"
                )),
                color_eyre::Help::section,
            )
        }
    }
}

#[derive(Deserialize)]
#[serde(try_from = "TransactionMatcherConfiguration")]
pub struct TransactionMatcher {
    // TODO pub file_format_name: String,
    pub transaction_rules: Vec<TransactionRule>,
    pub fallback_rule: TransactionRule,
    pub rule_patterns: RegexSet,
}

impl TransactionMatcher {
    fn validate_rule_account_aliases(&self, accounts: &AccountMap) -> Vec<Result<()>> {
        self.transaction_rules
            .iter()
            .flat_map(|rule| {
                // Ugly, but: https://users.rust-lang.org/t/flattening-a-vector-of-tuples/11409/4
                let validations = rule.validate_account_aliases(accounts);
                once(validations.0).chain(once(validations.1))
            })
            .collect()
    }
}

#[derive(Deserialize)]
pub struct TransactionMatcherConfiguration {
    pub fallback_rule: FallbackRuleConfiguration,
    pub transaction_rules: Vec<TransactionRule>,
}

impl TryFrom<TransactionMatcherConfiguration> for TransactionMatcher {
    type Error = Error;

    fn try_from(
        TransactionMatcherConfiguration {
            fallback_rule,
            transaction_rules,
        }: TransactionMatcherConfiguration,
    ) -> Result<Self, Self::Error> {
        let patterns = transaction_rules.iter().map(|rule| &rule.pattern_string);
        let rule_patterns: RegexSet = RegexSet::new(patterns)?;
        trace!(
            "Loaded matcher with {} compiled patterns.",
            rule_patterns.len()
        );

        let fallback_rule: TransactionRule = TransactionRule::try_from(fallback_rule)?;

        let matcher = TransactionMatcher {
            transaction_rules,
            fallback_rule,
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

impl TransactionRule {
    fn validate_account_aliases(&self, accounts: &AccountMap) -> (Result<()>, Result<()>) {
        (
            self.validate_alias(&self.account1, accounts),
            self.validate_alias(&self.account2, accounts),
        )
    }

    fn validate_alias(&self, account_alias: &str, accounts: &AccountMap) -> Result<()> {
        if accounts.contains_key(account_alias) {
            Ok(())
        } else {
            Err(eyre!(format!(
                "Account Alias '{}' from Transaction Rule '{}' is not defined.",
                account_alias, self.name
            )))
        }
    }
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct FallbackRuleConfiguration {
    pub account1: String,
    pub account2: String,
    pub payee: String,
    pub negate_first_amount: Option<bool>,
}

static FALLBACK_RULE_NAME: &str = "Fallback Transaction Rule";
static FALLBACK_PATTERN_STRING: &str = ".*";

impl TryFrom<FallbackRuleConfiguration> for TransactionRule {
    type Error = Error;

    fn try_from(
        FallbackRuleConfiguration {
            account1,
            account2,
            payee,
            negate_first_amount,
        }: FallbackRuleConfiguration,
    ) -> Result<Self, Self::Error> {
        let negate_first_amount_bool: bool = negate_first_amount.unwrap_or(false);

        let rule = TransactionRule {
            name: String::from(FALLBACK_RULE_NAME),
            pattern_string: String::from(FALLBACK_PATTERN_STRING),
            account1,
            account2,
            payee,
            pattern: None, // Really don't need a pattern
            payee_is_template: false,
            needs_finalized: true, // Fallbacks always need finalized
            negate_first_amount: negate_first_amount_bool,
        };

        trace!("Loaded fallback transaction rule");

        Ok(rule)
    }
}
