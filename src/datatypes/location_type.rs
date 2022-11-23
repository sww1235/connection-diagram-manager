use serde::{Deserialize, Serialize};

use std::fmt;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LocationType {
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub part_number: Option<String>,
    pub manufacturer_part_number: Option<String>,
    pub supplier: Option<String>,
    pub supplier_part_number: Option<String>,
    pub description: Option<String>,
    pub material: Option<String>,
}
impl fmt::Display for LocationType {
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
        if let Some(foo) = &self.material {
            write!(f, "Material: {}", foo)?;
        }
        Ok(())
    }
}
