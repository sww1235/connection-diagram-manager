use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{
    datatypes::{
        color::Color,
        file_types,
        library_types::LibraryData,
        unit_helper::{
            cross_sectional_area::CrossSectionalArea,
            electric_potential::ElectricPotential,
            length::Length,
            nominal_core_size::NominalCoreSize,
            temperature_interval::TemperatureInterval,
        },
        util_types::{Catalog, CrossSection, Dimension, LineStyle},
    },
    traits::FromFile,
};

//TODO: come up with better name

//TODO: add validation to check that Figure8 cable cross sections only have 2 cores
//
//TODO: add optional parameters for ac/dc electric potential, min/max temperature rating to
//cableType itself, maybe?
//
//TODO: add optional min/max bend radius parameters
/// `CableType` contains defintions for any type of linear item such as wire, cable, fiber optic
/// cable, tube that would be used to interconnect `Equipment`. `Pathway`s are used to hold
/// instances of `CableType`s such as conduit, raceway, tray, etc. A cable with multiple cores in a
/// outer sheath would not be a `Pathway`, but multiple `Cable`s in a conduit would be `Cable`s
/// within a `Pathway`.
///
/// Wire and Cable are not separated out here, as they are handled in a similar fashion in the
/// code.
#[derive(Debug, PartialEq, Clone)]
#[expect(clippy::partial_pub_fields, reason = "contained_datafile_path is not part of public API")]
pub struct CableType {
    /// Catalog information.
    pub catalog: Option<Catalog>,
    /// Cable Type Code.
    ///
    /// SOOW, NM, USE, etc.
    pub type_code: Option<String>,
    /// Overall cross sectional area.
    pub cross_sect_area: CrossSectionalArea,
    /// Cross section shape.
    ///
    /// Oval, circular, siamese.
    pub cross_section: CrossSection,
    /// Outer dimensions of cable.
    pub dimensions: Option<Dimension>,
    /// appearance in schematics.
    pub line_style: LineStyle,
    /// Map of cores in cable.
    ///
    /// Key of map is identifier of core within cable, and is unique within each cable.
    pub cores: BTreeMap<String, CableCore>,
    /// vector of exterior insulation/shielding layers.
    pub layers: Vec<CableLayer>,
    /// datafile the struct instance was read in from.
    pub(crate) contained_datafile_path: PathBuf,
}

impl From<file_types::cable_type::CableType> for CableType {
    #[inline]
    fn from(value: file_types::cable_type::CableType) -> Self {
        Self {
            catalog: value.catalog,
            type_code: value.type_code,
            cross_sect_area: value.cross_sect_area,
            cross_section: value.cross_section,
            dimensions: value.dimensions,
            line_style: value.line_style,
            cores: value
                .cores
                .into_iter()
                .map(|(key, inner_value)| (key, CableCore::from(inner_value)))
                .collect(),
            layers: value.layers,
            contained_datafile_path: PathBuf::new(),
        }
    }
}

impl FromFile for CableType {
    #[inline]
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
    #[inline]
    fn set_datafile(&mut self, datafile_path: &Path) {
        self.contained_datafile_path = datafile_path.to_path_buf();
    }
}

//https://stackoverflow.com/questions/67594909/multiple-possible-types-for-a-serializable-structs-field

/// `CableCore` represents an individual conductor, strength member or optical fiber in a cable.
#[derive(Debug, PartialEq, Clone)]
#[expect(clippy::exhaustive_enums, reason = "only two options make sense")]
pub struct CableCore {
    /// ID of `CableType` that this core is made of.
    pub type_id: String,
    /// `LineStyle` of `CableType`. If `None`, then it will inherit from the parent `CableType`.
    pub line_style: Option<LineStyle>,
    /// The material the core is made out of.
    pub material: Option<String>,
    /// Cross sectional area.
    pub cross_sect_area: Option<CrossSectionalArea>,
    /// Nominal size of core.
    pub nominal_size: Option<NominalCoreSize>,
    /// If `CableCore` is stranded.
    pub stranded: Option<bool>,
    /// How many strands is conductor made of.
    pub num_strands: Option<u64>,
    /// Cross sectional area of individual strand.
    pub strand_cross_sect_area: Option<CrossSectionalArea>,
}

impl From<file_types::cable_type::CableCore> for CableCore {
    #[inline]
    fn from(value: file_types::cable_type::CableCore) -> Self {
        Self {
            type_id: value.type_id,
            line_style: value.line_style,
            material: value.material,
            cross_sect_area: value.cross_sect_area,
            nominal_size: value.nominal_size,
            stranded: value.stranded,
            num_strands: value.num_strands,
            strand_cross_sect_area: value.strand_cross_sect_area,
        }
    }
}

//TODO: add a way to link 2 cores as a pair within a cable, and specify twisted + parameters
//

//TODO: either need to validate that layer number is unique within a cable, or remove and rely on
//ordering within TOML file. Need to test
//
//TODO: add minimum temperature rating
/// `CableLayer` represents an insulation or shield layer of the entire cable.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CableLayer {
    /// Layer number, counted from inside to outside of cable, 1 indexed.
    pub layer_number: u64,
    /// Layer type.
    pub layer_type: LayerType,
    /// `Material of CableLayer`.
    pub material: Option<String>,
    /// AC Voltage rating for insuation layer.
    pub ac_electric_potential_rating: Option<ElectricPotential>,
    /// DC Voltage rating for insuation layer.
    pub dc_electric_potential_rating: Option<ElectricPotential>,
    /// Temperature rating for insulation layer.
    pub temperature_rating: Option<TemperatureInterval>,
    /// Other insulation properties such as
    /// fire spread resistance, smoke generation, etc.
    pub rating: Option<String>,
    /// Thickness of `CableLayer`.
    pub thickness: Option<Length>,
    /// Color of `CableLayer`.
    pub color: Option<Color>,
    /// Secondary color of `CableLayer`.
    pub secondary_color: Option<Color>,
}

/// `LayerType` represents different functions of a `CableLayer` `layer_type`.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum LayerType {
    /// `Insulation` is a normal insulation layer of a cable.
    Insulation,
    /// `Semiconductor` is a semiconducting layer in high voltage cables.
    Semiconductor,
    /// `Shield` is a solid metallic shield for EMI reduction.
    Shield,
    /// `Screen` is a mesh-like metallic shield for EMI reduction.
    Screen,
    /// `ConcentricNeutral` is the outer concetric metallic neutral in high voltage cables.
    ConcentricNeutral,
    /// `Armor` is a strength member that resists mechanical damage and is not designed as part of
    /// the electrical paths in the cable.
    Armor,
    /// `Jacket` is the outer-most insulation of a cable.
    Jacket,
    /// `WaterBlocking` is a material that helps prevent ingress of water into cable, in addition
    /// to outer jackets and insulation. Common materials are water swellable tape or gel.
    WaterBlocking,
}

impl LibraryData for CableType {}
