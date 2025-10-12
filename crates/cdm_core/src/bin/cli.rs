//! # Connection Diagram Manager Command Line Interface (`cdm_cli`)
//!
//! `cdm_cli` is a command line interface for Connection Diagram Manager
//! that allows for basic operations directly, as well as launching both
//! a TUI and a proper GUI.
//!
//! Config file can live in either root project directory or src directory under root directory.

//TODO: change datafile parsing to parse individual files, and keep track of which files, which
//values came from.

use std::{
    fs,
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};

use cdm_core::{
    config::ApplicationConfig,
    datatypes::{
        library_types::Library,
        project_types::{self, Project},
    },
    directory_navigator,
};
use clap::Parser;
use figment::{
    Figment,
    providers::{Format, Serialized, Toml},
};
use log::{LevelFilter, debug, info};
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;

//TODO: change some of the panics in main to printed error messages with a returned error code.
fn main() -> anyhow::Result<()> {
    // parse command line flags and config files

    // check for config file in various locations first

    // {NAME_SCREAMING_SNAKE_CASE}_CONFIG envitonment variable
    // ~/.config/{name}/config.toml
    // /etc/{name}/config.toml
    // /usr/local/etc/{name}/config.toml
    // ~/Library/Preferences/{name}/config.toml

    // Doesn't work on windows
    // Figment will silently ignore missing files
    // Once the fix in the below issue is released, re-evaluate
    // https://github.com/SergioBenitez/Figment/issues/110

    let home_dir = std::env::home_dir().ok_or(io::Error::new(ErrorKind::NotFound, "Home Directory not found"))?;
    let root = Path::new("/");
    let app_config_filename = "cdm_config.toml";
    let app_config: ApplicationConfig = Figment::new()
        .merge(Serialized::defaults(ApplicationConfig::default()))
        .merge(Toml::file(
            home_dir
                .join(".config")
                .join("ConnectionDiagramManager")
                .join(app_config_filename),
        ))
        .merge(Toml::file(
            home_dir
                .join("Library")
                .join("Preferences")
                .join("ConnectionDiagramManager")
                .join(app_config_filename),
        ))
        .merge(Toml::file(
            root.join("etc").join("ConnectionDiagramManager").join(app_config_filename),
        ))
        .merge(Toml::file(
            root.join("usr")
                .join("local")
                .join("etc")
                .join("ConnectionDiagramManager")
                .join(app_config_filename),
        ))
        .merge(Toml::file(app_config_filename))
        .merge(Serialized::globals(Cli::parse()))
        .extract()?;

    debug!("{app_config:#?}");
    //parse command line flags
    let cli = Cli::parse();
    // initialize logging
    let mut logger = SimpleLogger::new();
    // match on how many times verbose flag is present in commandline
    logger = match cli.verbose {
        0 => logger.with_level(LevelFilter::Info),
        1 => logger.with_level(LevelFilter::Debug),
        _ => logger.with_level(LevelFilter::Trace),
    };

    // match on how many times quiet flag is present in commandline
    logger = match cli.quiet {
        0 => logger, // do nothing
        1 => logger.with_level(LevelFilter::Error),
        _ => logger.with_level(LevelFilter::Off),
    };
    #[expect(clippy::unwrap_used)]
    //TODO: investigate to see if it is worth trying to handle these
    //errors manually
    logger.with_colors(true).init().unwrap();

    // check if project_directory was specified and even exists

    if !cli.project_directory.exists() {
        return Err(io::Error::new(
            ErrorKind::NotFound,
            format!(
                "Project directory specified: {} does not exist",
                cli.project_directory.display()
            ),
        )
        .into());
    }

    if !cli.project_directory.is_dir() {
        return Err(io::Error::new(
            ErrorKind::NotADirectory,
            format!(
                "Project directory specified: {} is not a directory",
                cli.project_directory.display()
            ),
        )
        .into());
    }

    let project_config_contents = fs::read_to_string(cli.project_directory.join("cdm_project.toml"))?;
    let project_config: project_types::Config = toml::from_str(&project_config_contents)?;
    debug!("{project_config:#?}");

    let library_files = directory_navigator::files_in_dir(cli.project_directory.join("lib"), Some(".toml"), false)?;
    let project_files = directory_navigator::files_in_dir(cli.project_directory.join("src"), Some(".toml"), false)?;

    let mut library_data = Library::default();
    let mut project_data = Project::default();
    //TODO: include default libraries. use include_str! macro
    for file in library_files {
        let library_file_contents = fs::read_to_string(&file)?;
        let library_file: Library = toml::from_str(&library_file_contents)?;
        library_data.merge(library_file, &file.display().to_string(), &file.display().to_string())?;
    }

    ////TODO: handle errors here better
    //library
    //    .from_datafiles(data_files.clone(), merge_prompt_fn)
    //    .unwrap();

    //project
    //    .from_datafiles(data_files, &library, merge_prompt_fn)
    //    .unwrap();

    //debug! {"{library:?}"};
    //debug! {"{project:?}"};

    if cli.export_pdf {}
    Ok(())
}

/// `Cli` holds the defintions for command line arguments used in this binary
#[derive(Parser, Debug, Serialize)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Directory that project lives in
    project_directory: PathBuf,
    /// Increase verbosity of program by adding more v
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    /// Enable PostgreSQL features
    #[arg(long)]
    enable_post_gres: bool,
    /// Postgres DSN (optional)
    #[arg(short, long)]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    post_gres_dsn: Option<String>,
    /// Only shows log messages with <Error> level. Use twice to completely eliminate output. Takes
    /// precidence over verbose
    #[arg(short, long, action = clap::ArgAction::Count)]
    quiet: u8,
    /// Do not use default libraries included with program
    #[arg(short, long)]
    no_default_libs: bool,
    /// Export complete PDF
    #[arg(short, long)]
    export_pdf: bool,
}
