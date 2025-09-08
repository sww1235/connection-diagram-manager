use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uom::si::rational64::{Area, ElectricPotential, Length, TemperatureInterval};

use crate::datatypes::{
    color::Color,
    util_types::{Catalog, LineStyle},
};

use cdm_macros::{Empty, Merge, PartialEmpty};
use cdm_traits::partial_empty::PartialEmpty;

/// `WireType` represents a particular type of wire
///
/// Not all fields have to be populated, and some are
/// mainly provided for logical reasons rather than
/// functional (model/part number/manufacturer part number
/// may all be equivalent in some cases)
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct WireType {
    /// Catalog information
    pub catalog: Option<Catalog>,
    /// The standard wire type code (THHN, XHHW, SIS, etc)
    pub wire_type_code: Option<String>,
    /// The material the conductor or central element
    /// of the wire is made out of
    pub material: String,
    /// If the wire is insulated
    pub insulated: bool,
    /// What material the wire is insulated with
    pub insulation_material: Option<String>,
    /// Thickness of outer insulation
    pub insulation_thickness: Option<Length>,
    insulation_thickness_unit: Option<String>,
    /// Conductor cross sectional area.
    pub conductor_cross_sect_area: Area,
    conductor_cross_sect_area_unit: String,
    /// Overall wire cross sectional area, incluidng insulation.
    pub overall_cross_sect_area: Option<Area>,
    overall_cross_sect_area_unit: Option<String>,
    /// If conductor is stranded
    pub stranded: bool,
    /// How many strands is conductor made of
    pub num_strands: u64,
    /// cross sectional area of individual strand.
    pub strand_cross_sect_area: Option<Area>,
    strand_cross_sect_area_unit: Option<String>,
    /// Insulation voltage rating.
    pub insulation_potential_rating: Option<ElectricPotential>,
    insulation_potential_rating_unit: Option<String>,
    /// Insulation temperature rating.
    pub insulation_temperature_rating: Option<TemperatureInterval>,
    insulation_temperature_rating_unit: String,
    /// Insulation Color
    pub insulation_color: Option<Color>,
    /// Secondary Insulation Color
    pub secondary_insulation_color: Option<Color>,
    /// appearance in schematics
    pub line_style: Option<LineStyle>,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}

impl WireType {
    /// Creates an empty instance of `WireType`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
