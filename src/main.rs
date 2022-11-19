use std::path::PathBuf;

use clap::Parser;

use log::LevelFilter;
use simple_logger::SimpleLogger;

fn main() {
    //parse command line flags
    let cli = Cli::parse();
    // initialize logging
    // TODO: change logging level based on command line arguements
    // TODO: Panic if both quiet and verbose flags are set
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .with_colors(true)
        .init()
        .unwrap();

    //TODO: check if projectDirectory was specified and even exists

    //TODO: read config

    //TODO: Parse project directory
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Directory that project lives in
    project_directory: Option<PathBuf>,
    /// Increase verbosity of program by adding more v
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    /// Enable PostGresSql features
    #[arg(short, long)]
    enable_post_gres: bool,
    /// PostGres DSN (optional)
    #[arg(short, long)]
    post_gres_dsn: Option<String>,
    /// Shut program up. Program will still happen
    #[arg(short, long)]
    quiet: bool,
    /// Do not use default libraries included with program
    #[arg(short, long)]
    no_default_libs: bool,
}
