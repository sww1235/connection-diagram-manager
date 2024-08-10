use std::fmt;
use std::path::PathBuf;

use cdm_macros::{Empty, Merge, PartialEmpty};
use cdm_traits::partial_empty::PartialEmpty;

use dimensioned::ucum;

//TODO: create physical location stuff
/// `LocationType` represents a type/model of location.
///
/// Examples of `LocationType` include junction boxes, racks, panels, etc.
/// It does not include places these are located.
#[derive(Debug, Default, PartialEq, Clone, Merge, PartialEmpty, Empty)]
pub struct LocationType {
    /// Internal ID of Location Type
    pub id: String,
    /// Manufacturer of Location Type
    pub manufacturer: Option<String>,
    /// Model of Location Type
    pub model: Option<String>,
    /// Part Number of Location Type
    pub part_number: Option<String>,
    /// Manufacturer's Part Number
    pub manufacturer_part_number: Option<String>,
    /// Supplier of Part Number
    pub supplier: Option<String>,
    /// Supplier's Part Number
    pub supplier_part_number: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// Main material of Location Type
    pub material: Option<String>,
    /// Width of Location Type
    pub width: ucum::Meter<f64>,
    /// Height of Location Type
    pub height: ucum::Meter<f64>,
    /// Depth of Location Type
    pub depth: ucum::Meter<f64>,
    /// Usable Width of Location Type
    pub usable_width: ucum::Meter<f64>,
    /// Usable Height of Location Type
    pub usable_height: ucum::Meter<f64>,
    /// Usable Depth of Location Type
    pub usable_depth: ucum::Meter<f64>,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}

impl LocationType {
    /// Creates an empty instance of `LocationType`
    #[allow(clippy::arithmetic_side_effects)]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl fmt::Display for LocationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Connector Type:")?;
        if let Some(manufacturer) = &self.manufacturer {
            writeln!(f, "Manufacturer: {manufacturer}")?;
        }
        if let Some(model) = &self.model {
            writeln!(f, "Model: {model}")?;
        }
        if let Some(part_number) = &self.part_number {
            writeln!(f, "Part Number: {part_number}")?;
        }
        if let Some(manufacturer_part_number) = &self.manufacturer_part_number {
            writeln!(f, "Manufacturer Part Number: {manufacturer_part_number}")?;
        }
        if let Some(supplier) = &self.supplier {
            writeln!(f, "Supplier: {supplier}")?;
        }
        if let Some(supplier_part_number) = &self.supplier_part_number {
            writeln!(f, "Supplier Part Number: {supplier_part_number}")?;
        }
        if let Some(description) = &self.description {
            writeln!(f, "Description: {description}")?;
        }
        if let Some(material) = &self.material {
            writeln!(f, "Material: {material}")?;
        }
        writeln!(f, "Width: {}", self.width)?;
        writeln!(f, "Height: {}", self.height)?;
        writeln!(f, "Depth: {}", self.depth)?;
        writeln!(f, "Usable Width: {}", self.usable_width)?;
        writeln!(f, "Usable Height: {}", self.usable_height)?;
        writeln!(f, "Usable Depth: {}", self.usable_depth)?;
        Ok(())
    }
}
