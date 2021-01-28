use color_eyre::eyre::Result;
use std::path::PathBuf;
use structopt::StructOpt;

use financial_importer::transaction_matcher;
use financial_importer::transaction_matcher::TransactionMatcher;

#[derive(Debug, StructOpt)]
#[structopt(name = "financial-importer")]
struct App {
    // Common arguments:
    #[structopt(long, short = "c", env = "IMPORTER_CONFIG_FILE", parse(from_os_str))]
    config_file: Option<PathBuf>,
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum Command {
    /// Check that the specified configuration file is valid.
    ValidateConfig,
    /// Test input against configuration.
    TestMatches {
        #[structopt(long, short = "i", parse(from_os_str))]
        input_file: PathBuf,
    },
    /// Process a CSV file to produce entries.
    ProcessCSV,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let app = App::from_args();

    // Not the most ideal, but for validate configuration, I want to
    // add logging. Loading the configuration files need to be done for
    // the other steps regardless.

    if let Command::ValidateConfig = app.command {
        eprintln!("Validating configuration.");
    }

    // Load the configuration
    let transaction_matcher: TransactionMatcher =
        transaction_matcher::load_configuration(app.config_file)?;

    // Now, dispatch based on the command
    match app.command {
        Command::ValidateConfig => validate_config(transaction_matcher),
        Command::TestMatches { input_file } => {
            println!(
                "Testing matches, with input_file: {}!",
                input_file.to_str().unwrap()
            );
        }
        Command::ProcessCSV => {
            println!("Processing CSV!");
        }
    }

    Ok(())
}

fn validate_config(transaction_matcher: TransactionMatcher) {
    let account_count = transaction_matcher.accounts.len();
    let rule_count = transaction_matcher.transaction_rules.len();

    eprintln!("Configuration file loaded:");
    eprintln!("\tNumber of accounts defined: {}", account_count);
    eprintln!("\tNumber of rules defined: {}", rule_count);
    eprintln!("Configuration validation completed.")
}
