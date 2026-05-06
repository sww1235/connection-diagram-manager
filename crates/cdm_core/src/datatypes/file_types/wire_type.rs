use serde::{Deserialize, Serialize};

use crate::datatypes::{
    color::Color,
    unit_helper::{
        cross_sectional_area::CrossSectionalArea,
        electric_potential::ElectricPotential,
        length::Length,
        nominal_wire_size::NominalWireSize,
        temperature_interval::TemperatureInterval,
    },
    util_types::{Catalog, LineStyle},
};

/// `CableType` is the source code representation of the on-disk file format for an in-memory
/// [`CableType`](crate::datatypes::library_types::cable_type::CableType).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WireType {
    /// Catalog information.
    pub catalog: Option<Catalog>,
    /// The standard wire type code (THHN, XHHW, SIS, etc).
    pub wire_type_code: Option<String>,
    /// The material the conductor or central element
    /// of the wire is made out of.
    pub material: String,
    /// If the wire is insulated.
    pub insulated: bool,
    /// What material the wire is insulated with.
    pub insulation_material: Option<String>,
    /// Thickness of outer insulation.
    pub insulation_thickness: Option<Length>,
    /// Conductor cross sectional area.
    pub conductor_cross_sect_area: Option<CrossSectionalArea>,
    /// Nominal cross sectional area.
    pub nominal_cross_sect_area: Option<NominalWireSize>,
    /// AC Insulation voltage rating.
    pub ac_insulation_potential_rating: Option<ElectricPotential>,
    /// DC Insulation voltage rating.
    pub dc_insulation_potential_rating: Option<ElectricPotential>,
    /// Insulation temperature rating.
    pub insulation_temperature_rating: Option<TemperatureInterval>,
    /// Other insulation properties such as
    /// Flamability or smoke generation.
    pub insulation_rating: Option<String>,
    /// Insulation Color.
    pub insulation_color: Option<Color>,
    /// Secondary Insulation Color.
    pub secondary_insulation_color: Option<Color>,
    /// Appearance in schematics.
    #[serde(default)]
    pub line_style: LineStyle,
    /// If `WireType` is stranded.
    pub stranded: bool,
    /// How many strands is conductor made of.
    pub num_strands: u64,
    /// Cross sectional area of individual strand.
    pub strand_cross_sect_area: Option<CrossSectionalArea>,
}
