use crate::app::{APP_NAME, DEFAULT_CONFIG_FILE_NAME};
use crate::transaction_matcher::definitions::FinancialImporter;
use color_eyre::eyre::{eyre, Result, WrapErr};
use log::{info, trace};
use platform_dirs::AppDirs;
use std::path::{Path, PathBuf};

pub fn load_configuration(config_file: Option<PathBuf>) -> Result<FinancialImporter> {
    let config_pathbuf: PathBuf = if let Some(config_pathname) = config_file {
        config_pathname
    } else {
        default_config_filename().ok_or_else(|| eyre!("Problems getting app config file path"))?
    };
    let config_pathname: &Path = config_pathbuf.as_path();

    info!(
        "Using config file path: {}",
        config_pathname.to_str().unwrap()
    );

    let contents = std::fs::read_to_string(config_pathname).wrap_err_with(|| {
        format!(
            "Encountered errors reading config file '{}'.",
            config_pathname.to_str().unwrap()
        )
    })?;

    trace!("Starting configuration file validation.");

    let importer: FinancialImporter = toml::from_str(&contents)?;
    Ok(importer)
}

fn default_config_filename() -> Option<PathBuf> {
    let app_dirs = AppDirs::new(Some(APP_NAME), false)?;
    let config_file_path = app_dirs.config_dir.join(DEFAULT_CONFIG_FILE_NAME);
    Some(config_file_path)
}
