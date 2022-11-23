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
        if let Some(foo) = &self.cable_type_code {
            write!(f, "Cable Type: {}", foo)?;
        }
        if let Some(foo) = &self.cross_sect_area {
            if f.alternate() {
                //TODO: implement mm^2 to awg conversion function. include function for changing units
                write!(f, "Cross Sectional Area: {:.2} AWG", foo)?;
            } else {
                write!(f, "Cross Sectional Area: {:.2} mm^2", foo)?;
            }
        }
        if let Some(foo) = &self.cross_section {
            write!(f, "Cross Section: {}", foo)?;
        }
        if let Some(foo) = &self.height {
            write!(f, "Height: {:.2} mm", foo)?;
        }
        if let Some(foo) = &self.width {
            write!(f, "Width: {:.2} mm", foo)?;
        }
        if let Some(foo) = &self.diameter {
            write!(f, "Diameter: {:.2} mm", foo)?;
        }
        //TODO: implement loops here to print all layers of cable
        //if let Some(foo) = &self.model {
        //    write!(f, "Model: {}", foo)?;
        //}
        //if let Some(foo) = &self.model {
        //    write!(f, "Model: {}", foo)?;
        //}
        Ok(())
    }
}
