use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::datatypes::{
    color::Color,
    svg::Svg,
    util_types::{Catalog, Dimension},
};

//TODO: Make some of these fields enums
/// `ConnectorType` represents a particular type of connector.
///
/// Connector can represent more than just a metal or plastic blob on the end of a cable, it can
/// represent a screw terminal on a piece of equipment or a hole for wire to be entered in.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConnectorType {
    /// Catalog information
    pub catalog: Option<Catalog>,
    /// Dimensional information of connector
    pub dimensions: Option<Dimension>,
    /// Mounting method of connector
    ///
    /// Cable, PCB through hole, PCB surface mount, panel
    pub mount_type: Option<String>,
    /// Panel Cutout of connector if it is panel mounted
    ///
    /// D, A, etc
    pub panel_cutout: Option<String>,
    /// Gender of connector
    ///
    /// Male, Female, RPMale, RPFemale, Hermaphrodidic, unknown
    pub gender: Option<String>,
    /// connector color
    pub color: Option<Color>,
    /// component designator
    pub component_designator: Option<String>,
    /// Vector of schematic symbols that can represent this `connector_type`
    pub schematic_symbol: Option<Vec<String>>,
    /// Optional list of other connector types this one can mate with
    pub connector_type_mate: Option<Vec<String>>,
    /// pins inside connector.
    ///
    /// Pin index is not guaranteed to be the same. Use `ConnectorPin.id` for confirming equality.
    pub pins: HashMap<String, ConnectorPin>,
    /// overall diagram of connector TODO: figure out what angle this should be
    pub visual_representation: Option<Svg>,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}

//TODO: store pin cross sectional area or something equivalent, also store pin type
/// Represents an individual pin in a `ConnectorType`
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConnectorPin {
    /// Pin Designation
    pub designation: String,
    /// Pin label or name
    pub label: Option<String>,
    /// Pin signal type
    pub signal_type: Option<String>,
    /// Pin color
    pub color: Option<Color>,
    /// visual representation of an individual pin
    pub visual_rep: Option<Svg>,
    /// gender of pin
    pub gender: Option<String>,
    /// Rating information of pin, not parsed.
    pub rating: Option<String>,
}
