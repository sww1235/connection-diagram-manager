use super::svg;
use serde::{Deserialize, Serialize};
use std::fmt;
//TODO: Make some of these fields enums
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConnectorType {
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub part_number: Option<String>,
    pub manufacturer_part_number: Option<String>,
    pub supplier: Option<String>,
    pub supplier_part_number: Option<String>,
    pub description: Option<String>,
    pub mount_type: Option<String>,
    pub panel_cutout: Option<String>,
    pub gender: Option<String>,
    pub height: Option<f64>,
    pub width: Option<f64>,
    pub depth: Option<f64>,
    pub diameter: Option<f64>,
    pub pin_count: Option<u64>,
    pub pins: Option<Vec<String>>,
    pub pin_labels: Option<Vec<String>>,
    pub pin_colors: Option<Vec<String>>,
    pub pin_signal_type: Option<Vec<String>>,
    pub visual_rep: Option<svg::Svg>,
    pub pin_visual_rep: Option<svg::Svg>,
}
impl fmt::Display for ConnectorType {
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
        if let Some(foo) = &self.mount_type {
            write!(f, "Mount Type: {}", foo)?;
        }
        if let Some(foo) = &self.panel_cutout {
            write!(f, "Panel Cutout: {}", foo)?;
        }
        if let Some(foo) = &self.gender {
            write!(f, "Gender: {}", foo)?;
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
        if let Some(foo) = &self.pin_count {
            write!(f, "Pin Count: {}", foo)?;
        }
        //TODO: implement loop here to print all pins
        //if let Some(foo) = &self.pins {
        //    write!(f, "Panel Cutout: {}", foo)?;
        //}
        //TODO: implement loops here to print all layers of cable
        //if let Some(foo) = &self.model {
        //    write!(f, "Model: {}", foo)?;
        //}
        //if let Some(foo) = &self.model {
        //    write!(f, "Model: {}", foo)?;
        //}
        //TODO: implement svg validation rules here
        Ok(())
    }
}
