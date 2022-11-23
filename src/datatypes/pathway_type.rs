use serde::{Deserialize, Serialize};

use std::fmt;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PathwayType {
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub part_number: Option<String>,
    pub manufacturer_part_number: Option<String>,
    pub supplier: Option<String>,
    pub supplier_part_number: Option<String>,
    pub description: Option<String>,
    pub size: Option<String>,
    pub trade_size: Option<String>,
    //TODO: add in height, width, etc
    pub cross_sect_area: Option<f64>,
    pub material: Option<String>,
}
impl fmt::Display for PathwayType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Connector Type:")?;
        if let Some(foo) = &self.manufacturer {
            write!(f, "Manufacturer: {}", foo)?;
        }
        if let Some(foo) = &self.model {
            write!(f, "Model: {}", foo)?;
        }
        if let Some(foo) = &self.part_number {
            write!(f, "Part Number: {}", foo)?;
        }
        if let Some(foo) = &self.manufacturer_part_number {
            write!(f, "Manufacturer Part Number: {}", foo)?;
        }
        if let Some(foo) = &self.supplier {
            write!(f, "Supplier: {}", foo)?;
        }
        if let Some(foo) = &self.supplier_part_number {
            write!(f, "Supplier Part Number: {}", foo)?;
        }
        if let Some(foo) = &self.description {
            write!(f, "Description: {}", foo)?;
        }
        if let Some(foo) = &self.size {
            write!(f, "Size: {}", foo)?;
        }
        if let Some(foo) = &self.trade_size {
            write!(f, "Trade Size: {}", foo)?;
        }
        if let Some(foo) = &self.cross_sect_area {
            write!(f, "Cross Sectional Area: {:.2} mm^2", foo)?;
        }
        if let Some(foo) = &self.material {
            write!(f, "Material: {}", foo)?;
        }
        Ok(())
    }
}
