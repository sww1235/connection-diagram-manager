use std::{
    env,
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};

use clap::{Parser, ValueEnum};
use figment::{
    Figment,
    providers::{Format as _, Serialized, Toml},
};
use log::LevelFilter;
use serde::Serialize;
use simple_logger::SimpleLogger;

use crate::{
    config::ApplicationConfig,
    datatypes::unit_helper::{
        area::Area,
        cross_sectional_area::CrossSectionalArea,
        electric_potential::ElectricPotential,
        length::Length,
        nominal_wire_size::NominalWireSize,
        temperature_interval::TemperatureInterval,
    },
    error::Error,
};

/// Parse command line flags and config files.
///
/// # Errors
///
/// Will error if config file or command line parsing fails.
#[inline]
#[expect(clippy::result_large_err, reason = "Don't want to have to split up error::Error ")]
pub fn read_config_files_cli() -> Result<(ApplicationConfig, Cli), Error> {
    // parse command line flags and config files

    // check for config file in various locations first

    // {NAME_SCREAMING_SNAKE_CASE}_CONFIG envitonment variable
    // ~/.config/ConnectionDiagramManager/cdm_config.toml
    // /etc/ConnectionDiagramManager/cdm_config.toml
    // /usr/local/etc/ConnectionDiagramManager/cdm_config.toml
    // ~/Library/Preferences/ConnectionDiagramManager/cdm_config.toml

    // Doesn't work on windows
    // Figment will silently ignore missing files
    // Once the fix in the below issue is released, re-evaluate
    // https://github.com/SergioBenitez/Figment/issues/110

    let home_dir = env::home_dir().ok_or(io::Error::new(ErrorKind::NotFound, "Home Directory not found"))?;
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

    //parse command line flags
    let cli = Cli::parse();

    Ok((app_config, cli))
}
/// `Cli` holds the defintions for command line arguments used in this binary.
#[derive(Parser, Debug, Serialize)]
#[non_exhaustive]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Directory that project lives in.
    #[arg(required = true)]
    pub project_directory: Option<PathBuf>,
    /// Increase verbosity of program by adding more v.
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
    /// Enable PostgreSQL features.
    #[arg(long)]
    pub enable_post_gres: bool,
    /// Postgres DSN (optional).
    #[arg(long)]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub post_gres_dsn: Option<String>,
    /// Only shows log messages with `Error` level. Use twice to completely eliminate output. Takes
    /// precidence over verbose.
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub quiet: u8,
    /// Do not use default libraries included with program.
    #[arg(short, long)]
    pub no_default_libs: bool,
    /// Export complete PDF.
    #[arg(short, long)]
    pub export_pdf: bool,
    /// print units accepted in configuration files.
    #[arg(short, long, value_enum, exclusive = true)]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub print_units: Option<PrintUnitCmdOption>,
}

/// Unit types that can be printed for use in configuration files.
#[derive(Parser, Debug, Serialize, Clone, Copy, ValueEnum, Default)]
#[non_exhaustive]
pub enum PrintUnitCmdOption {
    /// Print all unit options.
    #[default]
    All,
    /// Print `Area` unit options.
    Area,
    /// Print `CrossSectionalArea` unit options.
    CrossSectionalArea,
    /// Print `ElectricPotential` unit options.
    ElectricPotential,
    /// Print `Length` unit options.
    Length,
    /// Print `NominalWiresize` unit options.
    NominalWireSize,
    /// Print `TemperatureInterval` unit options.
    TemperatureInterval,
}

/// Set up application logging.
#[inline]
#[must_use]
pub fn logger_configuration(cli: &Cli) -> SimpleLogger {
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
    //TODO: investigate to see if it is worth trying to handle these
    //errors manually
    logger.with_colors(true)
}
/// Prints units that can be used in configuration, project and library files.
#[inline]
pub fn print_file_units(cli: &Cli) {
    #[expect(clippy::print_stdout, reason = "this is intended to print to the terminal")]
    match cli.print_units {
        Some(PrintUnitCmdOption::All) => {
            println!("{:^43}", "Area Units");
            println!("{}", Area::output_units());
            println!("{:^43}", "Cross Sectional Area Units");
            println!("{}", CrossSectionalArea::output_units());
            println!("{:^43}", "Electric Potential Units");
            println!("{}", ElectricPotential::output_units());
            println!("{:^43}", "Length Units");
            println!("{}", Length::output_units());
            println!("{:^43}", "Temperature Units");
            println!("{}", TemperatureInterval::output_units());
        }
        Some(PrintUnitCmdOption::Area) => {
            println!("{:^43}", "Area Units");
            println!("{}", Area::output_units());
        }
        Some(PrintUnitCmdOption::CrossSectionalArea) => {
            println!("{:^43}", "Cross Sectional Area Units");
            println!("{}", CrossSectionalArea::output_units());
        }
        Some(PrintUnitCmdOption::ElectricPotential) => {
            println!("{:^43}", "Electric Potential Units");
            println!("{}", ElectricPotential::output_units());
        }
        Some(PrintUnitCmdOption::Length) => {
            println!("{:^43}", "Length Units");
            println!("{}", Length::output_units());
        }
        Some(PrintUnitCmdOption::NominalWireSize) => {
            println!("{:^43}", "Nominal Wire Size Units");
            println!("{}", NominalWireSize::output_units());
        }
        Some(PrintUnitCmdOption::TemperatureInterval) => {
            println!("{:^43}", "Temperature Units");
            println!("{}", TemperatureInterval::output_units());
        }
        None => {}
    }
}
