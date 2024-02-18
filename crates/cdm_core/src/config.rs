use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use std::fs::{self, File};

use log::{debug, info};

use serde::{Deserialize, Serialize};

/// `Config` represents configuration options for the various cdm binary programs
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    /// `library_files` contains an optional list of additional files to read into the main
    /// [`Library`](crate::datatypes::internal_types::Library)
    pub library_files: Vec<PathBuf>,
    /// `no_default_libraries` prevents loading of default libraries provided with the application
    pub no_default_libraries: bool,
}

/// `read_config_file` reads an indivdiual configuration yaml file into a [`fs::File`]
fn read_config_file(project_directory: &Path) -> Option<fs::File> {
    let src_dir = project_directory.join("src");
    let src_config_filepath = src_dir.join("cdm_config.yaml");
    let mut src_config_file_exists = false;
    let root_config_filepath = project_directory.join("cdm_config.yaml");
    let mut root_config_file_exists = false;
    let src_config_file: Option<fs::File> = match File::open(src_config_filepath) {
        Ok(file) => {
            src_config_file_exists = true;
            Some(file)
        }
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => {
                    debug!(
                        "Config file was not found in src directory {}",
                        src_dir.display()
                    );
                }

                ErrorKind::PermissionDenied => {
                    debug!(
                        "Config file in src directory {} does not have correct permissions",
                        src_dir.display()
                    );
                }
                _ => panic!("Something unexpected happened"),
            }
            None
        }
    };
    let root_config_file: Option<fs::File> = match File::open(root_config_filepath) {
        Ok(file) => {
            root_config_file_exists = true;
            Some(file)
        }
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => {
                    debug!(
                        "Config file was not found in src directory {}",
                        project_directory.join("src").display()
                    );
                }
                ErrorKind::PermissionDenied => {
                    debug!(
                        "Config file in src directory {} does not have correct permissions",
                        project_directory.join("src").display()
                    );
                }
                _ => panic!("Something unexpected happened"),
            }
            None
        }
    };

    if src_config_file_exists {
        info!("Using src config file");
        src_config_file
    } else if root_config_file_exists {
        info!("Using root config file");
        root_config_file
    } else {
        None
    }
}
impl Config {
    /// `parse_config` reads and selects the correct config file within the project directory
    ///
    /// # Errors
    ///
    /// Will error if any parsed yaml file fails to parse
    pub fn parse_config(project_directory: &Path) -> Result<Self, serde_yaml::Error> {
        let config_file = if let Some(file) = read_config_file(project_directory) {
            Some(file)
        } else {
            info!("No Config file detected and parsed");
            info!("Returning empty config struct");
            None
        };
        if let Some(config_file) = config_file {
            let config: Config = serde_yaml::from_reader(config_file)?;
            Ok(config) // need to have Ok here to return when everything is fine
        } else {
            let config = Config::default();
            Ok(config) // need to have Ok here to return when everything is fine
        }
    }
}
