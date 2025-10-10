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
    path::{Path, PathBuf},
};

use cdm_core::{
    config::ApplicationConfig,
    datatypes::{
        library_types::Library,
        project_types::{self, Project},
    },
};
use clap::Parser;
use log::{LevelFilter, debug, info};
use simple_logger::SimpleLogger;

fn main() {
    //TODO: add config file parsing via figment
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

    assert! { cli.project_directory.exists(),
    "Project directory specified: {} does not exist", cli.project_directory.display()}

    assert! {cli.project_directory.is_dir(),
    "Project directory specified: {} is not a directory", cli.project_directory.display()}

    let home_dir = std::env::home_dir();
    let root = Path::new("/");
    let app_config_filename = Path::new("cdm_config.toml");
    let mut app_config_paths = Vec::new();
    let mut app_config_string = None;
    if let Some(home_dir) = home_dir {
        app_config_paths.push(
            home_dir
                .join(".config")
                .join("ConnectionDiagramManager")
                .join(app_config_filename),
        );
        app_config_paths.push(
            home_dir
                .join("Library")
                .join("Preferences")
                .join("ConnectionDiagramManager")
                .join(app_config_filename),
        );
    }
    app_config_paths.push(root.join("etc").join("ConnectionDiagramManager").join(app_config_filename));
    app_config_paths.push(
        root.join("usr")
            .join("local")
            .join("etc")
            .join("ConnectionDiagramManager")
            .join(app_config_filename),
    );
    for path in app_config_paths {
        match fs::read_to_string(&path) {
            Ok(data) => {
                info!("found application configuration file at {}", path.display());
                app_config_string = Some(data);
                break;
            }
            Err(err) => {
                debug!(
                    "tried searching for application configuration file at {}, but didn't find it. See {err} for details",
                    path.display()
                );
            }
        }
    }

    let config: ApplicationConfig = {
        if let Some(app_config_string) = app_config_string {
            match toml::from_str(&app_config_string) {
                Ok(c) => c,
                Err(e) => {
                    panic! {"Failure to parse config yaml file. Error: {e}"}
                }
            }
        } else {
            ApplicationConfig::default()
        }
    };

    debug! {"{config:#?}"}

    // will be vector of DataFiles
    //let data_files = match file_types::parse_project_dir(cli.project_directory) {
    //    Ok(datastore) => datastore,
    //    Err(e) => {
    //        //TODO: better handle errors here
    //        error! {"Failure to read in project directory. Error: {e}"}
    //        return;
    //    }
    //};

    //let mut library = Library::new();
    //let mut project = Project::new();
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
}

/// `Cli` holds the defintions for command line arguments used in this binary
#[derive(Parser)]
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
