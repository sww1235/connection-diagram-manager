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
        if let Some(foo) = &self.mount_type {
            writeln!(f, "Mount Type: {}", foo)?;
        }
        if let Some(foo) = &self.panel_cutout {
            writeln!(f, "Panel Cutout: {}", foo)?;
        }
        if let Some(foo) = &self.gender {
            writeln!(f, "Gender: {}", foo)?;
        }
        if let Some(foo) = &self.height {
            writeln!(f, "Height: {:.2} mm", foo)?;
        }
        if let Some(foo) = &self.width {
            writeln!(f, "Width: {:.2} mm", foo)?;
        }
        if let Some(foo) = &self.diameter {
            writeln!(f, "Diameter: {:.2} mm", foo)?;
        }
        if let Some(foo) = &self.pin_count {
            writeln!(f, "Pin Count: {}", foo)?;
        }
        //TODO: implement loop here to print all pins
        //if let Some(foo) = &self.pins {
        //    writeln!(f, "Panel Cutout: {}", foo)?;
        //}
        //TODO: implement loops here to print all layers of cable
        //if let Some(foo) = &self.model {
        //    writeln!(f, "Model: {}", foo)?;
        //}
        //if let Some(foo) = &self.model {
        //    writeln!(f, "Model: {}", foo)?;
        //}
        //TODO: implement svg validation rules here
        Ok(())
    }
}
