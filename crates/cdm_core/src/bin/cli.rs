//! # Connection Diagram Manager Command Line Interface (`cdm_cli`)
//!
//! `cdm_cli` is a command line interface for Connection Diagram Manager
//! that allows for basic operations directly, as well as launching both
//! a TUI and a proper GUI.
//!
//! Config file can live in either root project directory or src directory under root directory.

use cdm_core::{bin_logic, datatypes};
use log::{debug, info};

//TODO: change some of the panics in main to printed error messages with a returned error code.
//#[expect(clippy::too_many_lines, reason = "main function")]
fn main() -> anyhow::Result<()> {
    let (app_config, cli) = bin_logic::read_config_files_cli()?;

    debug!("{app_config:#?}");

    bin_logic::logger_configuration(&cli).init()?;

    if cli.print_units.is_some() {
        bin_logic::print_file_units(&cli);
        return Ok(());
    }
    let (project_config, library_data, project_data) = datatypes::parse_datafiles(&cli)?;

    if cli.export_pdf {}

    Ok(())
}
