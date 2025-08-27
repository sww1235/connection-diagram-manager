use std::fmt;
use std::path::PathBuf;

use uom::si::{
    area::square_millimeter,
    length::millimeter,
    rational64::{Area, Length},
};

use cdm_macros::{Empty, Merge, PartialEmpty};
use cdm_traits::partial_empty::PartialEmpty;

/// `PathwayType` represents a route for wires and cables to take from one
/// [`LocationType`](super::location_type::LocationType) to another.
///
/// Examples of Pathways include, conduit, cable tray, free air
#[derive(Debug, Default, PartialEq, Clone, Merge, PartialEmpty, Empty)]
pub struct PathwayType {
    /// Internal ID of Pathway Type
    pub id: String,
    /// Manufacturer of Pathway Type
    pub manufacturer: Option<String>,
    /// Model of Pathway Type
    pub model: Option<String>,
    /// Part Number of Pathway Type
    pub part_number: Option<String>,
    /// Manufacturer's Part Number of Pathway Type
    pub manufacturer_part_number: Option<String>,
    /// Supplier of Pathway Type
    pub supplier: Option<String>,
    /// Supplier's Part Number of Pathway Type
    pub supplier_part_number: Option<String>,
    /// Optional description text
    pub description: Option<String>,
    /// actual size of pathway
    pub size: Option<String>,
    /// Trade Size of pathway
    pub trade_size: Option<String>,
    /// height of pathway
    pub height: Length,
    /// width of pathway
    pub width: Length,
    /// Inner cross sectional area of pathway
    pub cross_sect_area: Area,
    /// Main material of pathway
    pub material: Option<String>,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}
impl PathwayType {
    /// Creates an empty instance of `PathwayType`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl fmt::Display for PathwayType {
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
        if let Some(size) = &self.size {
            writeln!(f, "Size: {size}")?;
        }
        if let Some(trade_size) = &self.trade_size {
            writeln!(f, "Trade Size: {trade_size}")?;
        }
        writeln!(f, "Height: {:.2}", self.height.get::<millimeter>())?;
        writeln!(f, "Width: {:.2}", self.width.get::<millimeter>())?;
        //TODO: implement unit conversion function
        writeln!(
            f,
            "Cross Sectional Area: {:.2} mm^2",
            self.cross_sect_area.get::<square_millimeter>()
        )?;
        if let Some(material) = &self.material {
            writeln!(f, "Material: {material}")?;
        }
        Ok(())
    }
}
