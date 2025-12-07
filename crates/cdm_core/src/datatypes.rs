/// `color` is used to define common colors, as well as allow custom colors to be defined
pub mod color;
/// `library_types` contains the types that are contained in a library
pub mod library_types;
/// `project_types` contains the types that are contained in a project
pub mod project_types;
/// `svg` represents a complete SVG image
pub mod svg;
/// `unit_helper` contains wrapper types around UOM units, to allow tracking what unit was defined
/// in the data file easier.
///
/// Also contains conversion helper functions
pub mod unit_helper;
///`util_types` contains utility types that are used in multiple other different types and files,
///including generic enums.
pub mod util_types;

use std::{
    fs,
    io::{self, ErrorKind},
};

use log::debug;

use crate::{
    bin_logic::Cli,
    datatypes::{
        library_types::Library,
        project_types::{Config as ProjectConfig, Project},
    },
    directory_navigator,
    error::Error,
};

//TODO: investigate local structs instead of tuples
//https://stackoverflow.com/questions/39008880/is-it-possible-to-declare-local-anonymous-structs-in-rust
/// Parse datafiles and return data structs
///
/// # Errors
///
/// Will error on IO errors or parsing errors
#[expect(clippy::result_large_err, reason = "Don't want to have to split up error::Error ")]
#[inline]
pub fn parse_datafiles(cli: &Cli) -> Result<(ProjectConfig, Library, Project), Error> {
    // check if project_directory was specified and even exists
    if let Some(project_directory) = &cli.project_directory {
        if !project_directory.exists() {
            return Err(io::Error::new(
                ErrorKind::NotFound,
                format!("Project directory specified: {} does not exist", project_directory.display()),
            )
            .into());
        }

        if !project_directory.is_dir() {
            return Err(io::Error::new(
                ErrorKind::NotADirectory,
                format!(
                    "Project directory specified: {} is not a directory",
                    project_directory.display()
                ),
            )
            .into());
        }

        let project_config_contents = fs::read_to_string(project_directory.join("cdm_project.toml"))?;
        let project_config: ProjectConfig = toml::from_str(&project_config_contents)?;
        debug!("{project_config:#?}");

        let library_files = directory_navigator::files_in_dir(project_directory.join("lib"), Some(".toml"), false)?;
        let project_files = directory_navigator::files_in_dir(project_directory.join("src"), Some(".toml"), false)?;

        let mut library_data = Library::default();
        let mut project_data = Project::default();
        //TODO: include default libraries. use include_str! macro
        //
        //TODO: add prefix to all string keys read in from file (maybe file_name or something) to
        //avoid unintended duplicate keys in multiple files
        for file in library_files {
            let library_file_contents = fs::read_to_string(&file)?;
            let library_file: Library = toml::from_str(&library_file_contents)?;
            library_data.merge(library_file, &file)?;
        }
        for file in project_files {
            let project_file_contents = fs::read_to_string(&file)?;
            let project_file: Project = toml::from_str(&project_file_contents)?;
            project_data.merge(project_file, &file)?;
        }
        debug! {"{library_data:?}"};
        debug! {"{project_data:?}"};

        Ok((project_config, library_data, project_data))
    } else {
        Err(io::Error::other(
            "Project Directory not specified when it should have been. This should be impossible if Clap's logic is working \
             correctly",
        )
        .into())
    }
}
