use std::path::PathBuf;

use clap::Parser;

use log::{debug, LevelFilter};

use simple_logger::SimpleLogger;

use std::fs::File;

use cdm_core::datatypes;

mod config;

use config::Config;
//https://stackoverflow.com/questions/66799905/how-to-make-some-structs-fields-mandatory-to-fill-and-others-optional-in-rust
fn main() {
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

    logger.with_colors(true).init().unwrap();

    // check if project_directory was specified and even exists

    if !cli.project_directory.exists() {
        panic! {"Project directory specified: {} does not exist", cli.project_directory.display()}
    }

    if !cli.project_directory.is_dir() {
        panic! {"Project directory specified: {} is not a directory", cli.project_directory.display()}
    }

    let config = match Config::parse_config(cli.project_directory.as_path()) {
        Ok(config) => config,
        Err(e) => {
            panic! {"Failure to parse config yaml file. Error: {}", e}
        }
    };

    debug! {"{:?}", config}

    //TODO: Parse project directory

    let data_file = File::open(
        cli.project_directory
            .join("src")
            .join("library.yaml")
            .as_path(),
    )
    .expect("failed to open library file");

    let data: datatypes::Data = match datatypes::data_parser(data_file) {
        Ok(data) => data,
        Err(e) => {
            panic! {"failure to parse datafile. Error: {}", e}
        }
    };

    println! {"{}", data}
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Directory that project lives in
    project_directory: PathBuf,
    /// Increase verbosity of program by adding more v
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    /// Enable PostGresSql features
    #[arg(short, long)]
    enable_post_gres: bool,
    /// PostGres DSN (optional)
    #[arg(short, long)]
    post_gres_dsn: Option<String>,
    /// Only shows log messages with <Error> level. Use twice to completely eliminate output. Takes precidence over verbose
    #[arg(short, long, action = clap::ArgAction::Count)]
    quiet: u8,
    /// Do not use default libraries included with program
    #[arg(short, long)]
    no_default_libs: bool,
}
