use std::fmt;

use cdm_macros::Merge;
use cdm_traits::{empty::Empty, partial_empty::PartialEmpty};

use dimensioned::ucum;

/// `PathwayType` represents a route for wires and cables to take from one
/// [`LocationType`](super::location_type::LocationType) to another.
///
/// Examples of Pathways include, conduit, cable tray, free air
#[derive(Debug, Default, PartialEq, Merge)]
pub struct PathwayType {
    /// Internal ID of `PathwayType`
    pub id: String,
    /// Manufacturer of PathwayType
    pub manufacturer: Option<String>,
    /// Model of PathwayType
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
    /// height of pathway in mm
    pub height: ucum::Meter<f64>,
    /// width of pathway in mm
    pub width: ucum::Meter<f64>,
    /// Inner cross sectional area of pathway
    pub cross_sect_area: ucum::Meter2<f64>,
    /// Main material of pathway
    pub material: Option<String>,
}
impl PathwayType {
    /// Creates an empty instance of `PathwayType`
    #[allow(clippy::arithmetic_side_effects)]
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: String::new(),
            manufacturer: None,
            model: None,
            part_number: None,
            manufacturer_part_number: None,
            supplier: None,
            supplier_part_number: None,
            description: None,
            size: None,
            trade_size: None,
            height: 0.0_f64 * ucum::M,
            width: 0.0_f64 * ucum::M,
            cross_sect_area: 0.0_f64 * ucum::M2,
            material: None,
        }
    }
}

impl Empty for PathwayType {
    fn is_empty(&self) -> bool {
        self == &Self::new()
    }
}

impl PartialEmpty for PathwayType {
    fn is_partial_empty(&self) -> bool {
        let tester = Self::new();
        self.manufacturer == tester.manufacturer
            && self.model == tester.model
            && self.part_number == tester.part_number
            && self.manufacturer_part_number == tester.manufacturer_part_number
            && self.supplier == tester.supplier
            && self.supplier_part_number == tester.supplier_part_number
            && self.description == tester.description
            && self.size == tester.size
            && self.trade_size == tester.trade_size
            && self.height == tester.height
            && self.width == tester.width
            && self.cross_sect_area == tester.cross_sect_area
            && self.material == tester.material
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
        writeln!(f, "Height: {:.2}", self.height)?;
        writeln!(f, "Width: {:.2}", self.width)?;
        //TODO: implement unit conversion function
        writeln!(f, "Cross Sectional Area: {:.2} mm^2", self.cross_sect_area)?;
        if let Some(material) = &self.material {
            writeln!(f, "Material: {material}")?;
        }
        Ok(())
    }
}
