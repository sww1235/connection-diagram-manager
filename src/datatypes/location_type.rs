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
        writeln!(f, "Connector Type:")?;
        if let Some(manufacturer) = &self.manufacturer {
            writeln!(f, "Manufacturer: {}", manufacturer)?;
        }
        if let Some(model) = &self.model {
            writeln!(f, "Model: {}", model)?;
        }
        if let Some(part_number) = &self.part_number {
            writeln!(f, "Part Number: {}", part_number)?;
        }
        if let Some(manufacturer_part_number) = &self.manufacturer_part_number {
            writeln!(f, "Manufacturer Part Number: {}", manufacturer_part_number)?;
        }
        if let Some(supplier) = &self.supplier {
            writeln!(f, "Supplier: {}", supplier)?;
        }
        if let Some(supplier_part_number) = &self.supplier_part_number {
            writeln!(f, "Supplier Part Number: {}", supplier_part_number)?;
        }
        if let Some(description) = &self.description {
            writeln!(f, "Description: {}", description)?;
        }
        if let Some(material) = &self.material {
            writeln!(f, "Material: {}", material)?;
        }
        Ok(())
    }
}
