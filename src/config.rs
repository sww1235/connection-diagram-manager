use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use std::fs::{self, File};

use log::{debug, info};

use serde::{Deserialize, Serialize};

// debug allows easy printing
// TODO: implement better display functionality
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    library_files: Vec<PathBuf>,
    no_default_libraries: bool,
}

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
    pub fn parse_config(project_directory: &Path) -> Result<Self, serde_yaml::Error> {
        let config_file = match read_config_file(project_directory) {
            Some(file) => Some(file),
            None => {
                info!("No Config file detected and parsed");
                info!("Returning empty config struct");
                None
            }
        };
        match config_file {
            Some(config_file) => {
                let config: Config = serde_yaml::from_reader(config_file)?;
                Ok(config) // need to have Ok here to return when everything is fine
            }
            None => {
                let config = Config::default();
                Ok(config) // need to have Ok here to return when everything is fine
            }
        }
    }
}
