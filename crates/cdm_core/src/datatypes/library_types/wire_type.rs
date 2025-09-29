use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::datatypes::{
    color::Color,
    unit_helper::{CrossSectionalArea, ElectricPotential, Length, TemperatureInterval},
    util_types::{Catalog, LineStyle},
};

/// `WireType` represents a particular type of wire
///
/// Not all fields have to be populated, and some are
/// mainly provided for logical reasons rather than
/// functional (model/part number/manufacturer part number
/// may all be equivalent in some cases)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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
    /// Conductor cross sectional area.
    pub conductor_cross_sect_area: CrossSectionalArea,
    /// Overall wire cross sectional area, incluidng insulation.
    pub overall_cross_sect_area: Option<CrossSectionalArea>,
    /// If conductor is stranded
    pub stranded: bool,
    /// How many strands is conductor made of
    pub num_strands: u64,
    /// cross sectional area of individual strand.
    pub strand_cross_sect_area: Option<CrossSectionalArea>,
    /// AC Insulation voltage rating.
    pub ac_insulation_potential_rating: Option<ElectricPotential>,
    /// DC Insulation voltage rating.
    pub dc_insulation_potential_rating: Option<ElectricPotential>,
    /// Insulation temperature rating.
    pub insulation_temperature_rating: Option<TemperatureInterval>,
    /// Other insulation properties such as
    /// Flamability or smoke generation
    pub insulation_rating: Option<String>,
    /// Insulation Color
    pub insulation_color: Option<Color>,
    /// Secondary Insulation Color
    pub secondary_insulation_color: Option<Color>,
    /// appearance in schematics
    pub line_style: Option<LineStyle>,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}
