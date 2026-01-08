//! # Connection Diagram Manager Graphical User Interface (`cdm_gui`)
//!
//! `cdm_gui` is a graphical user interface for Connection Diagram Manager

//#[cfg(feature = "gui")]
use cdm_core::{bin_logic, datatypes};
use itertools::Itertools as _;
use log::debug;
use miniquad::{self as mq, conf::Conf as mqConf};

/// Main GUI app
mod app;

fn main() -> anyhow::Result<()> {
    let (app_config, cli) = bin_logic::read_config_files_cli()?;

    debug!("{app_config:#?}");

    bin_logic::logger_configuration(&cli).init()?;

    if cli.print_units.is_some() {
        bin_logic::print_file_units(&cli);
        return Ok(());
    }
    let (project_config, library_data, project_data) = datatypes::parse_datafiles(&cli)?;
    debug!("{library_data:#?}");

    debug!("{project_data:#?}");

    #[expect(
        clippy::expect_used,
        reason = "This function should never error, no Err() return in function currently"
    )]
    #[expect(clippy::unwrap_in_result, reason = "The expect should never trigger currently")]
    let library_validation = library_data.validate().expect("This function should never error");

    if !library_validation.is_empty() {
        let library_validation_string: String = library_validation.iter().format("\n").to_string();
        anyhow::bail!(library_validation_string);
    }

    #[expect(
        clippy::expect_used,
        reason = "This function should never error, no Err() return in function currently"
    )]
    #[expect(clippy::unwrap_in_result, reason = "The expect should never trigger currently")]
    let project_validation = project_data
        .validate(&library_data)
        .expect("This function should never error");
    if !project_validation.is_empty() {
        let project_validation_string: String = project_validation.iter().format("\n").to_string();
        anyhow::bail!(project_validation_string);
    }

    let mut gui_conf: mqConf = app_config.clone().graphics_config.into();

    gui_conf.window_resizable = true;
    gui_conf.window_title = "Connection Diagram Manager".to_owned();
    mq::start(gui_conf, move || {
        Box::new(app::App::new(&app_config, project_config, project_data, library_data))
    });

    Ok(())
}
