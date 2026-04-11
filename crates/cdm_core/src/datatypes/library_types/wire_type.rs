use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    datatypes::{
        color::Color,
        library_types::LibraryData,
        unit_helper::{
            cross_sectional_area::CrossSectionalArea,
            electric_potential::ElectricPotential,
            length::Length,
            nominal_wire_size::NominalWireSize,
            temperature_interval::TemperatureInterval,
        },
        util_types::{Catalog, LineStyle},
    },
    traits::FromFile,
};

/// `WireType` represents a particular type of wire.
///
/// Not all fields have to be populated, and some are
/// mainly provided for logical reasons rather than
/// functional (model/part number/manufacturer part number
/// may all be equivalent in some cases).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::partial_pub_fields, reason = "contained_datafile_path is not part of public API")]
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
    pub nominal_cross_section: Option<NominalWireSize>,
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
    /// Datafile the struct instance was read in from.
    #[serde(skip)]
    pub(crate) contained_datafile_path: PathBuf,
}
impl FromFile for WireType {
    #[inline]
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
    #[inline]
    fn set_datafile(&mut self, datafile_path: &Path) {
        self.contained_datafile_path = datafile_path.to_path_buf();
    }
}

impl WireType {
    //pub fn overall_cross_sectional_area(&self) -> Result<CrossSectionalArea,WireTypeError>  {
    //    if !self.insulated {return Ok(self.conductor_cross_sect_area)}
    //   if self.insulated && self.insulation_thickness.is_none() {
    //        return Err(WireTypeError::UnableToCalculateOverallCrossSectionalArea("Insulation thickness
    // not defined".to_string()));   }
    //   if let Some(insulation_thickness) = self.insulation_thickness && self.insulated{
    //       if insulation_thickness.value ==
    // uom::si::rational64::Length::new::<uom::si::length::millimeter>(Rational64::new(0,1)) {
    //
    //   Err(WireTypeError::UnableToCalculateOverallCrossSectionalArea("Insulation Thickness is
    // zero".to_string()));       }
    //       let pi =
    // Rational64::from_f64(std::f64::consts::PI).
    // ok_or(WireTypeError::UnableToCalculateOverallCrossSectionalArea("PI failed to fit inside
    // f64".to_string()))?;       let test =
    // uom::si::rational64::Area::new::<uom::si::area::square_millimeter>(Rational64::new(5,2));
    //       let test_sqrt = test.sqrt();
    //        let conductor_radius = self.conductor_cross_sect_area.value.sqrt() / pi;
    //        let overall_radius: uom::si::rational64::Area = conductor_radius +
    // insulation_thickness.value;
    //
    //        let overall_cross_sect_area = CrossSectionalArea {value: pi * overall_radius.powi(2),
    // original_unit: self.conductor_cross_sect_area.original_unit, stranded:
    // self.conductor_cross_sect_area.stranded, num_strands: self.conductor_cross_sect_area.num_strands,
    // strand_cross_sect_area: self.conductor_cross_sect_area.strand_cross_sect_area};
    //   return Ok(overall_cross_sect_area);
    //   }
    //   Err(WireTypeError::UnableToCalculateOverallCrossSectionalArea("Something weird
    // happened".to_string()))
    //
    //}
}
impl LibraryData for WireType {}
