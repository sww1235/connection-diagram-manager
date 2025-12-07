//! # Connection Diagram Manager Graphical User Interface (`cdm_gui`)
//!
//! `cdm_gui` is a graphical user interface for Connection Diagram Manager

//#[cfg(feature = "gui")]
use cdm_core::{bin_logic, datatypes};
use log::debug;
use miniquad::{self as mq, conf::Conf as mqConf};

/// Main GUI app
mod app;

//https://stackoverflow.com/questions/66799905/how-to-make-some-structs-fields-mandatory-to-fill-and-others-optional-in-rust
fn main() -> anyhow::Result<()> {
    let (app_config, cli) = bin_logic::read_config_files_cli()?;

    debug!("{app_config:#?}");

    bin_logic::logger_configuration(&cli).init()?;

    if cli.print_units.is_some() {
        bin_logic::print_file_units(&cli);
        return Ok(());
    }
    let (project_config, library_data, project_data) = datatypes::parse_datafiles(&cli)?;

    let mut gui_conf: mqConf = app_config.graphics_config.unwrap_or_default().into();

    gui_conf.window_resizable = true;
    gui_conf.window_title = "Connection Diagram Manager".to_owned();
    mq::start(gui_conf, || Box::new(app::Cdm::new()));

    Ok(())
}
