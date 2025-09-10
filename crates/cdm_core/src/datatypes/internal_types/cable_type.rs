use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use uom::si::rational64::{Area, ElectricPotential, Length, TemperatureInterval};

use cdm_macros::{Empty, Merge, PartialEmpty};
use cdm_traits::partial_empty::PartialEmpty;

use crate::datatypes::{
    color::Color,
    internal_types::wire_type::WireType,
    util_types::{Catalog, CrossSection, Dimension, LineStyle},
};

/// `CableType` represents a type of cable that consists of multiple cores. If something only has one
/// core, then it is a wire, not a cable.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CableType {
    /// Catalog information
    pub catalog: Option<Catalog>,
    /// Cable Type Code
    ///
    /// SOOW, NM, USE, etc
    pub cable_type_code: Option<String>,
    /// Cable cross sectional area
    pub cross_sect_area: Area,
    cross_sect_area_unit: String,
    /// Cable cross section shape
    ///
    /// Oval, circular, siamese
    pub cross_section: CrossSection,
    /// Dimensions of cable
    pub dimensions: Option<Dimension>,
    /// appearance in schematics
    pub line_style: Option<LineStyle>,
    /// map of cores in cable
    pub cores: HashMap<String, CableCore>,
    /// vector of exterior insulation/shielding layers
    pub insul_layers: Vec<CableLayer>,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}

//https://stackoverflow.com/questions/67594909/multiple-possible-types-for-a-serializable-structs-field

/// `CableCore` represents an individual conductor, strength member or optical fiber in a cable.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::exhaustive_enums)]
pub enum CableCore {
    /// `WireType`
    WireType(Rc<RefCell<WireType>>),
    /// `CableType`
    CableType(Rc<RefCell<CableType>>),
}

/// `CableLayer` represents an insulation or shield layer of the entire cable
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CableLayer {
    /// layer number, counted from inside to outside of cable, 1 indexed
    pub layer_number: u64,
    /// layer type
    pub layer_type: LayerType,
    /// `Material of CableLayer`
    pub material: Option<String>,
    /// AC Voltage rating for insuation layer
    pub ac_electric_potential_rating: Option<ElectricPotential>,
    ac_electric_potential_rating_unit: Option<String>,
    /// DC Voltage rating for insuation layer
    pub dc_electric_potential_rating: Option<ElectricPotential>,
    dc_electric_potential_rating_unit: Option<String>,
    /// Temperature rating for insulation layer
    pub temperature_rating: Option<TemperatureInterval>,
    temperature_rating_unit: Option<String>,
    /// Other insulation properties such as
    /// fire spread resistance, smoke generation, etc
    pub rating: Option<String>,
    /// Thickness of `CableLayer`
    pub thickness: Option<Length>,
    thickness_unit: Option<String>,
    /// color of `CableLayer`
    pub color: Option<Color>,
}

/// `LayerType` represents different functions of a `CableLayer` `layer_type`
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum LayerType {
    /// `Insulation` is a normal insulation layer of a cable
    #[default]
    Insulation,
    /// `Semiconductor` is a semiconducting layer in high voltage cables
    Semiconductor,
    /// `Shield` is a solid metallic shield for EMI reduction
    Shield,
    /// `Screen` is a mesh-like metallic shield for EMI reduction
    Screen,
    /// `ConcentricNeutral` is the outer concetric metallic neutral in high voltage cables
    ConcentricNeutral,
    /// `Armor` is a strength member that resists mechanical damage and is not designed as part of
    /// the electrical paths in the cable
    Armor,
    /// `Jacket` is the outer-most insulation of a cable.
    Jacket,
}

impl CableType {
    /// Creates an empty instance of `CableType`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
