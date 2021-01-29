use color_eyre::eyre::Result;
use financial_importer::app::{LOG_ENV_VAR, VALIDATION_LOG_LEVEL};
use financial_importer::transaction_matcher;
use financial_importer::transaction_matcher::TransactionMatcher;
use log::trace;
use std::path::PathBuf;
use structopt::StructOpt;

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

    let app: App = App::from_args();

    initialize_logging(&app);

    // Load the configuration
    let _transaction_matcher: TransactionMatcher =
        transaction_matcher::load_configuration(app.config_file)?;

    // Now, dispatch based on the command
    match app.command {
        Command::ValidateConfig => trace!("Configuration validation completed."),
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

fn initialize_logging(app: &App) {
    // `pretty_env_logger` is configured through an environment variable,
    // so manually set the value to the desired level if the requested command
    // `validate-config`, as `validate-config` is simply loading the configuration
    // with increased logging and then exiting.
    if let Command::ValidateConfig = app.command {
        if let Ok(level) = std::env::var(LOG_ENV_VAR) {
            eprintln!("{} already set to '{}', leaving.", LOG_ENV_VAR, level);
        } else {
            std::env::set_var(LOG_ENV_VAR, &VALIDATION_LOG_LEVEL);
        }
    }

    pretty_env_logger::init_custom_env(&LOG_ENV_VAR);
}
