use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{
    datatypes::{
        color::Color,
        library_types::LibraryData,
        unit_helper::{
            cross_sectional_area::CrossSectionalArea,
            electric_potential::ElectricPotential,
            length::Length,
            temperature_interval::TemperatureInterval,
        },
        util_types::{Catalog, CrossSection, Dimension, LineStyle},
    },
    traits::FromFile,
};

//TODO: add validation to check that Figure8 cable cross sections only have 2 cores
//
//TODO: add optional parameters for ac/dc electric potential, min/max temperature rating to
//cableType itself, maybe?
//
//TODO: add optional min/max bend radius parameters
/// `CableType` represents a type of cable that consists of multiple cores. If something only has
/// one core, then it is a wire, not a cable.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::partial_pub_fields, reason = "contained_datafile_path is not part of public API")]
pub struct CableType {
    /// Catalog information.
    pub catalog: Option<Catalog>,
    /// Cable Type Code.
    ///
    /// SOOW, NM, USE, etc.
    pub cable_type_code: Option<String>,
    /// Cable cross sectional area.
    pub cross_sect_area: CrossSectionalArea,
    /// Cable cross section shape.
    ///
    /// Oval, circular, siamese.
    pub cross_section: CrossSection,
    /// Dimensions of cable.
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
    #[serde(skip)]
    pub(crate) contained_datafile_path: PathBuf,
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
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::exhaustive_enums, reason = "only two options make sense")]
pub enum CableCore {
    /// `WireType`.
    ///
    /// If `line_style` is not specified, the `WireType`'s `line_style` will be inheritied.
    WireType {
        /// ID of `WireType` that this core is made of.
        type_id: String,
        /// `LineStyle` of `WireType`. If `None`, then it will inherit from the parent `CableType`.
        line_style: Option<LineStyle>,
    },
    /// `CableType`.
    ///
    /// If `line_style` is not specified, the `CableType`'s `line_style` will be inheritied.
    CableType {
        /// ID of `CableType` that this core is made of.
        type_id: String,
        /// `LineStyle` of `CableType`. If `None`, then it will inherit from the parent `CableType`.
        line_style: Option<LineStyle>,
    },
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
    /// layer number, counted from inside to outside of cable, 1 indexed.
    pub layer_number: u64,
    /// layer type.
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
    /// color of `CableLayer`.
    pub color: Option<Color>,
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
