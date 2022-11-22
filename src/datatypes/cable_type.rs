use serde::{Deserialize, Serialize};

use std::fmt;

use std::collections::HashMap;
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CableType {
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub part_number: Option<String>,
    pub manufacturer_part_number: Option<String>,
    pub supplier: Option<String>,
    pub supplier_part_number: Option<String>,
    pub cable_type_code: Option<String>,
    pub cross_sect_area: Option<f64>,
    pub cross_section: Option<String>,
    pub height: Option<f64>,
    pub width: Option<f64>,
    pub diameter: Option<f64>,
    pub cable_core: Option<HashMap<String, CableCore>>,
    pub layers: Option<Vec<CableLayer>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CableCore {
    pub core_type: Option<String>,
    pub color: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CableLayer {
    pub layer_number: Option<u64>,
    pub layer_type: Option<String>,
    pub material: Option<String>,
    pub volt_rating: Option<f64>,
    pub temp_rating: Option<f64>,
    pub color: Option<String>,
}

impl fmt::Display for CableType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cable Type:")?;
        if let Some(txt) = &self.manufacturer {
            write!(f, "Manufacturer: {}", txt)?;
        }
        if let Some(txt) = &self.model {
            write!(f, "Model: {}", txt)?;
        }
        if let Some(txt) = &self.part_number {
            write!(f, "Part Number: {}", txt)?;
        }
        if let Some(txt) = &self.manufacturer_part_number {
            write!(f, "Manufacturer Part Number: {}", txt)?;
        }
        if let Some(txt) = &self.supplier {
            write!(f, "Supplier: {}", txt)?;
        }
        if let Some(txt) = &self.supplier_part_number {
            write!(f, "Supplier Part Number: {}", txt)?;
        }
        if let Some(txt) = &self.cable_type_code {
            write!(f, "Cable Type: {}", txt)?;
        }
        if let Some(txt) = &self.cross_sect_area {
            write!(f, "Cross Sectional Area: {:.2} mm^2", txt)?;
        }
        if let Some(txt) = &self.cross_section {
            write!(f, "Cross Section: {}", txt)?;
        }
        if let Some(txt) = &self.height {
            write!(f, "Height: {} mm", txt)?;
        }
        if let Some(txt) = &self.width {
            write!(f, "Width: {} mm", txt)?;
        }
        if let Some(txt) = &self.diameter {
            write!(f, "Diameter: {} mm", txt)?;
        }
        //TODO: implement loops here to print all layers of cable
        //if let Some(txt) = &self.model {
        //    write!(f, "Model: {}", txt)?;
        //}
        //if let Some(txt) = &self.model {
        //    write!(f, "Model: {}", txt)?;
        //}
        Ok(())
    }
}
