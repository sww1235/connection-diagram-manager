//! # Connection Diagram Manager Command Line Interface (`cdm_cli`)
//!
//! `cdm_cli` is a command line interface for Connection Diagram Manager
//! that allows for basic operations directly, as well as launching both
//! a TUI and a proper GUI.

//TODO:
//- change datafile parsing to parse individual files, and keep track of which files, which
//values came from.

#![warn(missing_docs)]
#![warn(clippy::correctness)]
#![warn(clippy::suspicious)]
#![warn(clippy::perf)]
#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
#![warn(clippy::style)]
// restriction lints
#![warn(clippy::arithmetic_side_effects)]
#![warn(clippy::unnecessary_cast)]
#![warn(clippy::cast_lossless)]
#![warn(clippy::cast_possible_truncation)]
#![warn(clippy::cast_possible_wrap)]
#![warn(clippy::cast_precision_loss)]
#![warn(clippy::cast_sign_loss)]
#![warn(clippy::fn_to_numeric_cast)]
#![warn(clippy::fn_to_numeric_cast_with_truncation)]
#![warn(clippy::char_lit_as_u8)]
#![warn(clippy::ptr_as_ptr)]
#![warn(clippy::as_underscore)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::create_dir)]
#![warn(clippy::default_numeric_fallback)]
#![warn(clippy::empty_drop)]
#![warn(clippy::empty_structs_with_brackets)]
#![warn(clippy::exhaustive_enums)]
#![warn(clippy::expect_used)]
#![warn(clippy::filetype_is_file)]
#![warn(clippy::float_cmp_const)]
#![warn(clippy::fn_to_numeric_cast_any)]
#![warn(clippy::format_push_string)]
#![warn(clippy::if_then_some_else_none)]
#![warn(clippy::large_include_file)]
#![warn(clippy::lossy_float_literal)]
#![warn(clippy::map_err_ignore)]
#![warn(clippy::mem_forget)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::mixed_read_write_in_expression)]
#![warn(clippy::mod_module_files)]
#![warn(clippy::multiple_inherent_impl)]
#![warn(clippy::panic_in_result_fn)]
#![warn(clippy::partial_pub_fields)]
#![warn(clippy::print_stderr)]
#![warn(clippy::rc_mutex)]
#![warn(clippy::rest_pat_in_fully_bound_structs)]
#![warn(clippy::same_name_method)]
#![warn(clippy::shadow_unrelated)]
#![warn(clippy::string_add)]
#![warn(clippy::string_to_string)]
#![warn(clippy::todo)]
#![warn(clippy::try_err)]
#![warn(clippy::unimplemented)]
#![warn(clippy::unnecessary_self_imports)]
#![warn(clippy::unseparated_literal_suffix)]
#![warn(clippy::unwrap_in_result)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::verbose_file_reads)]

use std::path::PathBuf;

// These are used in the testing logic
//use std::cell::RefCell;
//use std::collections::HashMap;
//use std::rc::Rc;
//
//use cdm_core::datatypes::{
//    internal_types::{
//        cable_type::{CableCore, CableLayer, CableType},
//        wire_type::WireType,
//    },
//    util_types::CrossSection,
//};

// These are used in the main program logic
use clap::Parser;

use log::{debug, error, LevelFilter};

use simple_logger::SimpleLogger;

use cdm_core::{
    config::Config,
    datatypes::{
        file_types,
        internal_types::{Library, Project},
    },
};

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

    assert! {cli.project_directory.exists(),
    "Project directory specified: {} does not exist", cli.project_directory.display()}

    assert! {cli.project_directory.is_dir(),
    "Project directory specified: {} is not a directory", cli.project_directory.display()}

    let config = match Config::parse_config(cli.project_directory.as_path()) {
        Ok(config) => config,
        Err(e) => {
            panic! {"Failure to parse config yaml file. Error: {e}"}
        }
    };

    debug! {"{:#?}", config}

    // will be vector of DataFiles
    let data_files = match file_types::parse_project_dir(cli.project_directory) {
        Ok(datastore) => datastore,
        Err(e) => {
            //TODO: better handle errors here
            error! {"Failure to read in project directory. Error: {e}"}
            return;
        }
    };

    let mut lib2 = Library::new();
    let mut proj2 = Project::new();
    for datafile in &data_files {
        println! {"{datafile}"};
        lib2.from_datafile(datafile.clone());
    }

    for datafile in &data_files {
        proj2.from_datafile(datafile.clone(), &lib2);
    }

    println! {"{lib2:#?}"};
    println! {"{proj2:#?}"};

    //   let mut lib = Library::new();
    //
    //   let mut project = Project::new();
    //
    //   let SWTHHN12BK = Rc::new(RefCell::new(WireType {
    //       id: "SWTHHN12BK".to_string(),
    //       manufacturer: Some("Southwire".to_string()),
    //       model: Some("test1".to_string()),
    //       part_number: Some("12324".to_string()),
    //       manufacturer_part_number: Some("2o2sdfasd".to_string()),
    //       supplier: Some("Digikey".to_string()),
    //       supplier_part_number: Some("23974732adf".to_string()),
    //       conductor_material: Some("copper".to_string()),
    //       insulated: true,
    //       insulation_material: Some("PVC".to_string()),
    //       wire_type_code: Some("THHN".to_string()),
    //       conductor_cross_sect_area: Some(10.0),
    //       overall_cross_sect_area: Some(12.0),
    //       stranded: true,
    //       num_strands: Some(7),
    //       strand_cross_sect_area: Some(0.5),
    //       insul_volt_rating: Some(600),
    //       insul_temp_rating: Some(90),
    //       insul_color: Some("Black".to_string()),
    //   }));
    //
    //   let SWTHHN12WT = Rc::new(RefCell::new(WireType {
    //       id: "SWTHHN12WT".to_string(),
    //       manufacturer: Some("Southwire".to_string()),
    //       model: Some("test1".to_string()),
    //       part_number: Some("12324".to_string()),
    //       manufacturer_part_number: Some("2o2sdfasd".to_string()),
    //       supplier: Some("Digikey".to_string()),
    //       supplier_part_number: Some("23974732adf".to_string()),
    //       conductor_material: Some("copper".to_string()),
    //       insulated: true,
    //       insulation_material: Some("PVC".to_string()),
    //       wire_type_code: Some("THHN".to_string()),
    //       conductor_cross_sect_area: Some(10.0),
    //       overall_cross_sect_area: Some(12.0),
    //       stranded: true,
    //       num_strands: Some(7),
    //       strand_cross_sect_area: Some(0.5),
    //       insul_volt_rating: Some(600),
    //       insul_temp_rating: Some(90),
    //       insul_color: Some("White".to_string()),
    //   }));
    //
    //   let SWTHHN12GN = Rc::new(RefCell::new(WireType {
    //       id: "SWTHHN12GN".to_string(),
    //       manufacturer: Some("Southwire".to_string()),
    //       model: Some("test1".to_string()),
    //       part_number: Some("12324".to_string()),
    //       manufacturer_part_number: Some("2o2sdfasd".to_string()),
    //       supplier: Some("Digikey".to_string()),
    //       supplier_part_number: Some("23974732adf".to_string()),
    //       conductor_material: Some("copper".to_string()),
    //       insulated: true,
    //       insulation_material: Some("PVC".to_string()),
    //       wire_type_code: Some("THHN".to_string()),
    //       conductor_cross_sect_area: Some(10.0),
    //       overall_cross_sect_area: Some(12.0),
    //       stranded: true,
    //       num_strands: Some(7),
    //       strand_cross_sect_area: Some(0.5),
    //       insul_volt_rating: Some(600),
    //       insul_temp_rating: Some(90),
    //       insul_color: Some("Green".to_string()),
    //   }));
    //
    //   let SOOWINT12BK = Rc::new(RefCell::new(WireType {
    //       id: "SOOWINT12BK".to_string(),
    //       manufacturer: None,
    //       model: None,
    //       part_number: None,
    //       manufacturer_part_number: None,
    //       supplier: None,
    //       supplier_part_number: None,
    //       conductor_material: Some("copper".to_string()),
    //       insulated: true,
    //       insulation_material: Some("Rubber".to_string()),
    //       wire_type_code: None,
    //       conductor_cross_sect_area: Some(10.0),
    //       overall_cross_sect_area: Some(12.0),
    //       stranded: true,
    //       num_strands: Some(7),
    //       strand_cross_sect_area: Some(0.5),
    //       insul_volt_rating: Some(600),
    //       insul_temp_rating: Some(90),
    //       insul_color: Some("Black".to_string()),
    //   }));
    //
    //   let SOOWINT12WT = Rc::new(RefCell::new(WireType {
    //       id: "SOOWINT12WT".to_string(),
    //       manufacturer: None,
    //       model: None,
    //       part_number: None,
    //       manufacturer_part_number: None,
    //       supplier: None,
    //       supplier_part_number: None,
    //       conductor_material: Some("copper".to_string()),
    //       insulated: true,
    //       insulation_material: Some("Rubber".to_string()),
    //       wire_type_code: None,
    //       conductor_cross_sect_area: Some(10.0),
    //       overall_cross_sect_area: Some(12.0),
    //       stranded: true,
    //       num_strands: Some(7),
    //       strand_cross_sect_area: Some(0.5),
    //       insul_volt_rating: Some(600),
    //       insul_temp_rating: Some(90),
    //       insul_color: Some("White".to_string()),
    //   }));
    //
    //   let SOOWINT12GN = Rc::new(RefCell::new(WireType {
    //       id: "SOOWINT12GN".to_string(),
    //       manufacturer: None,
    //       model: None,
    //       part_number: None,
    //       manufacturer_part_number: None,
    //       supplier: None,
    //       supplier_part_number: None,
    //       conductor_material: Some("copper".to_string()),
    //       insulated: true,
    //       insulation_material: Some("Rubber".to_string()),
    //       wire_type_code: None,
    //       conductor_cross_sect_area: Some(10.0),
    //       overall_cross_sect_area: Some(12.0),
    //       stranded: true,
    //       num_strands: Some(7),
    //       strand_cross_sect_area: Some(0.5),
    //       insul_volt_rating: Some(600),
    //       insul_temp_rating: Some(90),
    //       insul_color: Some("Green".to_string()),
    //   }));
    //
    //   let PVCINT18BK = Rc::new(RefCell::new(WireType {
    //       id: "PVCINT18BK".to_string(),
    //       manufacturer: None,
    //       model: None,
    //       part_number: None,
    //       manufacturer_part_number: None,
    //       supplier: None,
    //       supplier_part_number: None,
    //       conductor_material: Some("copper".to_string()),
    //       insulated: true,
    //       insulation_material: Some("PVC".to_string()),
    //       wire_type_code: None,
    //       conductor_cross_sect_area: Some(1.0),
    //       overall_cross_sect_area: Some(1.5),
    //       stranded: true,
    //       num_strands: Some(7),
    //       strand_cross_sect_area: Some(0.01),
    //       insul_volt_rating: Some(600),
    //       insul_temp_rating: Some(60),
    //       insul_color: Some("Black".to_string()),
    //   }));
    //
    //   let SWSOOW123 = Rc::new(RefCell::new(CableType {
    //       id: "SWSOOW123".to_string(),
    //       manufacturer: Some("Southwire".to_string()),
    //       model: Some("test2".to_string()),
    //       part_number: Some("1sadfas2324".to_string()),
    //       manufacturer_part_number: Some("2o2sd7u979232fasd".to_string()),
    //       supplier: Some("Digikey".to_string()),
    //       supplier_part_number: Some("23974732aaadf".to_string()),
    //       cable_type_code: Some("SOOW".to_string()),
    //       cross_sect_area: 40.0,
    //       cross_section: CrossSection::Circular,
    //       height: 12.0,
    //       width: 12.0,
    //       diameter: Some(12.0),
    //       cable_core: {
    //           let mut cable_cores = HashMap::new();
    //           cable_cores.insert(
    //               "SOOWINT12BK".to_string(),
    //               CableCore::WireType(SOOWINT12BK.clone()),
    //           );
    //           cable_cores.insert(
    //               "SOOWINT12WT".to_string(),
    //               CableCore::WireType(SOOWINT12WT.clone()),
    //           );
    //           cable_cores.insert(
    //               "SOOWINT12GN".to_string(),
    //               CableCore::WireType(SOOWINT12GN.clone()),
    //           );
    //           cable_cores
    //       },
    //       insul_layers: vec![CableLayer {
    //           layer_number: Some(1),
    //           layer_type: Some("Insulation".to_string()),
    //           material: Some("Rubber".to_string()),
    //           volt_rating: Some(600),
    //           temp_rating: Some(90),
    //           color: Some("Black".to_string()),
    //       }],
    //   }));
    //
    //   lib.wire_types
    //       .insert("SWTHHN12BK".to_string(), SWTHHN12BK.clone());
    //   lib.wire_types
    //       .insert("SWTHHN12WT".to_string(), SWTHHN12WT.clone());
    //   lib.wire_types
    //       .insert("SWTHHN12GN".to_string(), SWTHHN12GN.clone());
    //   lib.wire_types
    //       .insert("SOOWINT12BK".to_string(), SOOWINT12BK.clone());
    //   lib.wire_types
    //       .insert("SOOWINT12WT".to_string(), SOOWINT12WT.clone());
    //   lib.wire_types
    //       .insert("SOOWINT12GN".to_string(), SOOWINT12GN.clone());
    //   lib.wire_types
    //       .insert("PVCINT18Bk".to_string(), PVCINT18BK.clone());
    //
    //   lib.cable_types
    //       .insert("SWSOOW123".to_string(), SWSOOW123.clone());
    //cable_type
    //term_cable_type
    //location_type
    //connector_type
    //equipment_type
    //pathway_type
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
