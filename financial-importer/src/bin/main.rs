use color_eyre::{eyre::eyre, Result, Section};
use financial_importer::source_record;
use financial_importer::source_record::{write_source_records, SourceRecord};
use financial_importer::transaction_matcher;
use financial_importer::transaction_matcher::{FinancialImporter, GeneratedLedgerEntry};
use financial_importer::{
    app::{LOG_ENV_VAR, VALIDATION_LOG_LEVEL},
    ledger_entry::{write_ledger_entries_file, LedgerEntry},
};
use log::trace;
use std::iter::Map;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "financial-importer")]
struct App {
    // Common arguments:
    #[structopt(
        long,
        short = "c",
        env = "FINANCIAL_IMPORTER_CONFIG",
        parse(from_os_str)
    )]
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
    ProcessCSV {
        #[structopt(long, short = "f")]
        format_name: String,
        #[structopt(long, short = "i", parse(from_os_str))]
        input_file: PathBuf,
        #[structopt(long, short = "u", parse(from_os_str))]
        unmatched_records_file: Option<PathBuf>,
        #[structopt(
            long,
            short = "l",
            parse(from_os_str),
            default_value = "ledger-postings.dat"
        )]
        ledger_output_file: PathBuf,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let app: App = App::from_args();

    initialize_logging(&app);

    // Load the configuration
    let importer: FinancialImporter = transaction_matcher::load_configuration(app.config_file)?;

    // Now, dispatch based on the command
    match app.command {
        Command::ValidateConfig => trace!("Configuration validation completed."),
        Command::TestMatches { input_file } => {
            println!(
                "Testing matches, with input_file: {}!",
                input_file.to_str().unwrap()
            );
        }
        Command::ProcessCSV {
            format_name,
            input_file,
            unmatched_records_file,
            ledger_output_file,
        } => process_csv(
            &importer,
            format_name.as_str(),
            &input_file,
            unmatched_records_file,
            &ledger_output_file,
        )?,
    }

    Ok(())
}

fn process_csv(
    importer: &FinancialImporter,
    format_name: &str,
    input_file: &Path,
    unmatched_records_file: Option<PathBuf>,
    ledger_output_file: &Path,
) -> Result<()> {
    println!("Summary: ");
    println!("- Using the file format definition \"{}\".", &format_name);

    let unmatched_records_path = get_unmatched_file_path(unmatched_records_file, input_file);
    let records: Vec<SourceRecord> = source_record::load_source_records(input_file)?;

    println!(
        "- Loaded {} source records from file {}.\n",
        records.len(),
        input_file.to_str().unwrap()
    );

    let (entries, errors): (Vec<_>, Vec<_>) = records
        .iter()
        .map(|record| importer.ledger_entry_for_source_record(format_name, record))
        .partition(Result::is_ok);

    // This is going to be a little messy, but I need to do it this way to report everything
    let (matched_entries, unmatched_entries): (Vec<_>, Vec<_>) = entries
        .into_iter()
        .map(Result::unwrap)
        .partition(GeneratedLedgerEntry::is_from_matched_rule);

    let (unmatched_entries, mut unmatched_records): (Vec<_>, Vec<_>) = unmatched_entries
        .into_iter()
        .map(GeneratedLedgerEntry::unwrap)
        .unzip();

    unmatched_records.sort();
    write_source_records(&unmatched_records_path, &unmatched_records)?;

    let mut entries: Vec<LedgerEntry> = matched_entries
        .into_iter()
        .map(GeneratedLedgerEntry::unwrap_entry)
        .collect();

    let matched_count = entries.len();
    let unmatched_count = unmatched_entries.len();

    let mut unmatched_entries = unmatched_entries;
    entries.append(&mut unmatched_entries);
    entries.sort();

    let entries_count = entries.len();
    write_ledger_entries_file(ledger_output_file, entries)?;

    println!(
        "- Wrote {} Ledger entries to file {}.",
        entries_count,
        ledger_output_file.to_str().unwrap()
    );
    println!(
        "   - {} Ledger entries generated from matching transaction rules.",
        matched_count
    );
    println!(
        "   - {} Ledger entries generated using the fallback rule.\n",
        unmatched_count
    );
    println!(
        "- Wrote {} unmatched source records to the file {}.",
        unmatched_count,
        unmatched_records_path.to_str().unwrap()
    );

    if errors.is_empty() {
        Ok(())
    } else {
        Map::fold(
            errors.into_iter().map(Result::unwrap_err),
            Err(eyre!("One or more errors were reported!")),
            |report, e| report.section(e),
        )
    }
}

static UNMATCHED_RECORDS_FILE_SUFFIX: &str = "-unmatched";

fn get_unmatched_file_path(unmatched_records_file: Option<PathBuf>, input_file: &Path) -> PathBuf {
    unmatched_records_file.map_or_else(
        || {
            let mut unmatched_records_path = PathBuf::new();
            unmatched_records_path.push(input_file.parent().unwrap());
            let mut filename = input_file.file_stem().unwrap().to_os_string();
            filename.push(UNMATCHED_RECORDS_FILE_SUFFIX);
            unmatched_records_path.push(filename);
            unmatched_records_path.set_extension(input_file.extension().unwrap());
            unmatched_records_path
        },
        |unmatched_records_file| unmatched_records_file,
    )
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
            std::env::set_var(LOG_ENV_VAR, VALIDATION_LOG_LEVEL);
        }
    }

    pretty_env_logger::init_custom_env(LOG_ENV_VAR);
}
