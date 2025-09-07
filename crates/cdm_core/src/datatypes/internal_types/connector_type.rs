use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uom::si::rational64::Length;

use super::svg::Svg;
use crate::datatypes::util_types::Catalog;

use cdm_macros::{Empty, Merge, PartialEmpty};
use cdm_traits::partial_empty::PartialEmpty;

//TODO: Make some of these fields enums
/// `ConnectorType` represents a particular type of connector.
///
/// Connector can represent more than just a metal or plastic blob on the end of a cable, it can
/// represent a screw terminal on a piece of equipment or a hole for wire to be entered in.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConnectorType {
    /// Internal ID of `ConnectorType`
    pub id: String,
    /// Catalog information
    pub catalog: Option<Catalog>,
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
    /// height of connector
    pub height: Length,
    height_unit: String,
    /// width of connector
    pub width: Length,
    width_unit: String,
    /// depth of connector
    pub depth: Length,
    depth_unit: String,
    /// diameter of circular connectors
    pub diameter: Option<Length>,
    diameter_unit: Option<String>,
    /// connector color
    pub color: Option<String>,
    /// component designator
    pub component_designator: Option<String>,
    /// Vector of schematic symbols that can represent this connector_type
    pub schematic_symbol: Option<Vec<String>>,
    /// Optional list of other connector types this one can mate with
    pub connector_type_mate: Option<Vec<String>>,
    /// pins inside connector.
    ///
    /// Pin index is not guaranteed to be the same. Use `ConnectorPin.id` for confirming equality.
    pub pins: HashMap<String, ConnectorPin>,
    /// overall diagram of connector TODO: figure out what angle this should be
    pub visual_rep: Svg,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}

//TODO: store pin cross sectional area or something equivalent, also store pin type
/// Represents an individual pin in a `ConnectorType`
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConnectorPin {
    /// Pin Designation
    pub designation: String,
    /// Pin label or name
    pub label: Option<String>,
    /// Pin signal type
    pub signal_type: Option<String>,
    /// Pin color
    pub color: Option<String>,
    /// visual representation of an individual pin
    pub visual_rep: Option<Svg>,
    /// gender of pin
    pub gender: Option<String>,
}

impl ConnectorType {
    /// Creates an empty instance of `ConnectorType`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
