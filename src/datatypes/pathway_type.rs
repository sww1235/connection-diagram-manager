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
        writeln!(f, "Connector Type:")?;
        if let Some(foo) = &self.manufacturer {
            writeln!(f, "Manufacturer: {}", foo)?;
        }
        if let Some(foo) = &self.model {
            writeln!(f, "Model: {}", foo)?;
        }
        if let Some(foo) = &self.part_number {
            writeln!(f, "Part Number: {}", foo)?;
        }
        if let Some(foo) = &self.manufacturer_part_number {
            writeln!(f, "Manufacturer Part Number: {}", foo)?;
        }
        if let Some(foo) = &self.supplier {
            writeln!(f, "Supplier: {}", foo)?;
        }
        if let Some(foo) = &self.supplier_part_number {
            writeln!(f, "Supplier Part Number: {}", foo)?;
        }
        if let Some(foo) = &self.description {
            writeln!(f, "Description: {}", foo)?;
        }
        if let Some(foo) = &self.size {
            writeln!(f, "Size: {}", foo)?;
        }
        if let Some(foo) = &self.trade_size {
            writeln!(f, "Trade Size: {}", foo)?;
        }
        if let Some(foo) = &self.cross_sect_area {
            writeln!(f, "Cross Sectional Area: {:.2} mm^2", foo)?;
        }
        if let Some(foo) = &self.material {
            writeln!(f, "Material: {}", foo)?;
        }
        Ok(())
    }
}
