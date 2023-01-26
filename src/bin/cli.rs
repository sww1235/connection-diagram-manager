//! # Connection Diagram Manager Command Line Interface (`cdm_cli`)
//!
//! `cdm_cli` is a command line interface for Connection Diagram Manager
//! that allows for basic operations directly, as well as launching both
//! a TUI and a proper GUI.

//TODO:
//- change datafile parsing to parse individual files, and keep track of which files, which
//values came from.
//- restrict library info and src info from being in the same file.
//- separate out file parsing data structures from actual data structures used in program
//- Use RC and refcell for recursive multiple owner references

#![warn(missing_docs)]
use std::path::PathBuf;

// These are used in the testing logic
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use cdm_core::datatypes::{
    internal_types::{
        cable_type::{CableCore, CableLayer, CableType},
        wire_type::WireType,
    },
    util_types::CrossSection,
};

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

    debug! {"{:#?}", config}

    //let mut datastore = match datatypes::parse_project_dir(cli.project_directory) {
    //    Ok(datastore) => datastore,
    //    Err(e) => {
    //        //TODO: better handle errors here
    //        error! {"Failure to read in project directory. Error: {}", e}
    //        return;
    //    }
    //};

    //println! {"{:#?}", datastore}

    let mut lib2 = Library::new();

    let mut project = Project::new();

    // will be vector of DataFiles
    let data_files = match file_types::parse_project_dir(cli.project_directory) {
        Ok(datastore) => datastore,
        Err(e) => {
            //TODO: better handle errors here
            error! {"Failure to read in project directory. Error: {}", e}
            return;
        }
    };

    let SWTHHN12BK = Rc::new(RefCell::new(WireType {
        id: "SWTHHN12BK".to_string(),
        manufacturer: Some("Southwire".to_string()),
        model: Some("test1".to_string()),
        part_number: Some("12324".to_string()),
        manufacturer_part_number: Some("2o2sdfasd".to_string()),
        supplier: Some("Digikey".to_string()),
        supplier_part_number: Some("23974732adf".to_string()),
        conductor_material: Some("copper".to_string()),
        insulated: true,
        insulation_material: Some("PVC".to_string()),
        wire_type_code: Some("THHN".to_string()),
        conductor_cross_sect_area: Some(10.0),
        overall_cross_sect_area: Some(12.0),
        stranded: true,
        num_strands: Some(7),
        strand_cross_sect_area: Some(0.5),
        insul_volt_rating: Some(600),
        insul_temp_rating: Some(90),
        insul_color: Some("Black".to_string()),
    }));

    let SWTHHN12WT = Rc::new(RefCell::new(WireType {
        id: "SWTHHN12WT".to_string(),
        manufacturer: Some("Southwire".to_string()),
        model: Some("test1".to_string()),
        part_number: Some("12324".to_string()),
        manufacturer_part_number: Some("2o2sdfasd".to_string()),
        supplier: Some("Digikey".to_string()),
        supplier_part_number: Some("23974732adf".to_string()),
        conductor_material: Some("copper".to_string()),
        insulated: true,
        insulation_material: Some("PVC".to_string()),
        wire_type_code: Some("THHN".to_string()),
        conductor_cross_sect_area: Some(10.0),
        overall_cross_sect_area: Some(12.0),
        stranded: true,
        num_strands: Some(7),
        strand_cross_sect_area: Some(0.5),
        insul_volt_rating: Some(600),
        insul_temp_rating: Some(90),
        insul_color: Some("White".to_string()),
    }));

    let SWTHHN12GN = Rc::new(RefCell::new(WireType {
        id: "SWTHHN12GN".to_string(),
        manufacturer: Some("Southwire".to_string()),
        model: Some("test1".to_string()),
        part_number: Some("12324".to_string()),
        manufacturer_part_number: Some("2o2sdfasd".to_string()),
        supplier: Some("Digikey".to_string()),
        supplier_part_number: Some("23974732adf".to_string()),
        conductor_material: Some("copper".to_string()),
        insulated: true,
        insulation_material: Some("PVC".to_string()),
        wire_type_code: Some("THHN".to_string()),
        conductor_cross_sect_area: Some(10.0),
        overall_cross_sect_area: Some(12.0),
        stranded: true,
        num_strands: Some(7),
        strand_cross_sect_area: Some(0.5),
        insul_volt_rating: Some(600),
        insul_temp_rating: Some(90),
        insul_color: Some("Green".to_string()),
    }));

    let SOOWINT12BK = Rc::new(RefCell::new(WireType {
        id: "SOOWINT12BK".to_string(),
        manufacturer: None,
        model: None,
        part_number: None,
        manufacturer_part_number: None,
        supplier: None,
        supplier_part_number: None,
        conductor_material: Some("copper".to_string()),
        insulated: true,
        insulation_material: Some("Rubber".to_string()),
        wire_type_code: None,
        conductor_cross_sect_area: Some(10.0),
        overall_cross_sect_area: Some(12.0),
        stranded: true,
        num_strands: Some(7),
        strand_cross_sect_area: Some(0.5),
        insul_volt_rating: Some(600),
        insul_temp_rating: Some(90),
        insul_color: Some("Black".to_string()),
    }));

    let SOOWINT12WT = Rc::new(RefCell::new(WireType {
        id: "SOOWINT12WT".to_string(),
        manufacturer: None,
        model: None,
        part_number: None,
        manufacturer_part_number: None,
        supplier: None,
        supplier_part_number: None,
        conductor_material: Some("copper".to_string()),
        insulated: true,
        insulation_material: Some("Rubber".to_string()),
        wire_type_code: None,
        conductor_cross_sect_area: Some(10.0),
        overall_cross_sect_area: Some(12.0),
        stranded: true,
        num_strands: Some(7),
        strand_cross_sect_area: Some(0.5),
        insul_volt_rating: Some(600),
        insul_temp_rating: Some(90),
        insul_color: Some("White".to_string()),
    }));

    let SOOWINT12GN = Rc::new(RefCell::new(WireType {
        id: "SOOWINT12GN".to_string(),
        manufacturer: None,
        model: None,
        part_number: None,
        manufacturer_part_number: None,
        supplier: None,
        supplier_part_number: None,
        conductor_material: Some("copper".to_string()),
        insulated: true,
        insulation_material: Some("Rubber".to_string()),
        wire_type_code: None,
        conductor_cross_sect_area: Some(10.0),
        overall_cross_sect_area: Some(12.0),
        stranded: true,
        num_strands: Some(7),
        strand_cross_sect_area: Some(0.5),
        insul_volt_rating: Some(600),
        insul_temp_rating: Some(90),
        insul_color: Some("Green".to_string()),
    }));

    let PVCINT18BK = Rc::new(RefCell::new(WireType {
        id: "PVCINT18BK".to_string(),
        manufacturer: None,
        model: None,
        part_number: None,
        manufacturer_part_number: None,
        supplier: None,
        supplier_part_number: None,
        conductor_material: Some("copper".to_string()),
        insulated: true,
        insulation_material: Some("PVC".to_string()),
        wire_type_code: None,
        conductor_cross_sect_area: Some(1.0),
        overall_cross_sect_area: Some(1.5),
        stranded: true,
        num_strands: Some(7),
        strand_cross_sect_area: Some(0.01),
        insul_volt_rating: Some(600),
        insul_temp_rating: Some(60),
        insul_color: Some("Black".to_string()),
    }));

    let SWSOOW123 = Rc::new(RefCell::new(CableType {
        id: "SWSOOW123".to_string(),
        manufacturer: Some("Southwire".to_string()),
        model: Some("test2".to_string()),
        part_number: Some("1sadfas2324".to_string()),
        manufacturer_part_number: Some("2o2sd7u979232fasd".to_string()),
        supplier: Some("Digikey".to_string()),
        supplier_part_number: Some("23974732aaadf".to_string()),
        cable_type_code: Some("SOOW".to_string()),
        cross_sect_area: 40.0,
        cross_section: CrossSection::Circular,
        height: 12.0,
        width: 12.0,
        diameter: Some(12.0),
        cable_core: {
            let mut cable_cores = HashMap::new();
            cable_cores.insert(
                "SOOWINT12BK".to_string(),
                CableCore::WireType(SOOWINT12BK.clone()),
            );
            cable_cores.insert(
                "SOOWINT12WT".to_string(),
                CableCore::WireType(SOOWINT12WT.clone()),
            );
            cable_cores.insert(
                "SOOWINT12GN".to_string(),
                CableCore::WireType(SOOWINT12GN.clone()),
            );
            cable_cores
        },
        insul_layers: vec![CableLayer {
            layer_number: Some(1),
            layer_type: Some("Insulation".to_string()),
            material: Some("Rubber".to_string()),
            volt_rating: Some(600),
            temp_rating: Some(90),
            color: Some("Black".to_string()),
        }],
    }));

    lib2.wire_types
        .insert("SWTHHN12BK".to_string(), SWTHHN12BK.clone());
    lib2.wire_types
        .insert("SWTHHN12WT".to_string(), SWTHHN12WT.clone());
    lib2.wire_types
        .insert("SWTHHN12GN".to_string(), SWTHHN12GN.clone());
    lib2.wire_types
        .insert("SOOWINT12BK".to_string(), SOOWINT12BK.clone());
    lib2.wire_types
        .insert("SOOWINT12WT".to_string(), SOOWINT12WT.clone());
    lib2.wire_types
        .insert("SOOWINT12GN".to_string(), SOOWINT12GN.clone());
    lib2.wire_types
        .insert("PVCINT18Bk".to_string(), PVCINT18BK.clone());

    lib2.cable_types
        .insert("SWSOOW123".to_string(), SWSOOW123.clone());
    //cable_type
    //term_cable_type
    //location_type
    //connector_type
    //equipment_type
    //pathway_type
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
