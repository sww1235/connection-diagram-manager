use serde::{Deserialize, Serialize};

use std::collections::HashMap;

/// `CableType` represents a type of cable that consists of multiple cores. If something only has one
/// core, then it is a wire, not a cable.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CableType {
    /// Manufacturer of Cable
    pub manufacturer: Option<String>,
    /// Model of Cable
    pub model: Option<String>,
    /// Part number of Cable
    pub part_number: Option<String>,
    /// Manufacturer's Part Number
    pub manufacturer_part_number: Option<String>,
    /// Supplier of cable
    pub supplier: Option<String>,
    /// Supplier's part number
    pub supplier_part_number: Option<String>,
    /// Cable Type Code
    ///
    /// SOOW, NM, USE, etc
    pub cable_type_code: Option<String>,
    /// Cable cross sectional area, in mm^2
    pub cross_sect_area: f64,
    /// Cable cross section shape
    ///
    /// Oval, circular, siamese
    pub cross_section: String,
    /// height of cable in mm
    pub height: f64,
    /// width of cable in mm
    pub width: f64,
    /// diameter of cable in mm
    pub diameter: Option<f64>,
    /// map of cores in cable
    pub cable_cores: HashMap<String, CableCore>,
    /// vector of exterior insulation/shielding layers
    pub insul_layers: Vec<CableLayer>,
}

/// `CableCore` represents a core of a cable
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CableCore {
    /// `type_str` contains the string ID of the wire/cable that is represented by cablecore
    pub type_str: String,
    /// `is_wire` indicates if this cableCore is a wire or cable
    pub is_wire: bool,
}

/// `CableLayer` represents an insulation or shield layer of the entire cable
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CableLayer {
    /// layer number, counted from inside to outside of cable, 1 indexed
    pub layer_number: u64,
    /// Insulation, Semiconductor, shield, screen, concentric neutral
    pub layer_type: String,
    /// `Material of CableLayer`
    pub material: Option<String>,
    /// Voltage rating for insuation layer
    pub volt_rating: Option<f64>,
    /// Temperature rating for insulation layer, specified in C TODO: fix this with proper unicode
    pub temp_rating: Option<f64>,
    /// color of `CableLayer`
    pub color: Option<String>,
}
