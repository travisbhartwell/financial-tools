use color_eyre::eyre::Result;
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

// TODO:
// - Set up error handling
// - Define subcommands and common arguments
//   - validate-config <- start here
//   - test-matches
//   - process-csv
fn main() -> Result<()> {
    color_eyre::install()?;

    let app = App::from_args();

    match app.command {
        Command::ValidateConfig => {
            println!("Validating configuration!");
        }
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
