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
        if let Some(mount_type) = &self.mount_type {
            writeln!(f, "Mount Type: {}", mount_type)?;
        }
        if let Some(panel_cutout) = &self.panel_cutout {
            writeln!(f, "Panel Cutout: {}", panel_cutout)?;
        }
        if let Some(gender) = &self.gender {
            writeln!(f, "Gender: {}", gender)?;
        }
        if let Some(height) = &self.height {
            writeln!(f, "Height: {:.2} mm", height)?;
        }
        if let Some(width) = &self.width {
            writeln!(f, "Width: {:.2} mm", width)?;
        }
        if let Some(diameter) = &self.diameter {
            writeln!(f, "Diameter: {:.2} mm", diameter)?;
        }
        if let Some(pin_count) = &self.pin_count {
            writeln!(f, "Pin Count: {}", pin_count)?;
        }
        //TODO: implement loop here to print all pins
        //if let Some() = &self.pins {
        //    writeln!(f, "Panel Cutout: {}", )?;
        //}
        //TODO: implement loops here to print all layers of cable
        //if let Some() = &self.model {
        //    writeln!(f, "Model: {}", )?;
        //}
        //if let Some() = &self.model {
        //    writeln!(f, "Model: {}", )?;
        //}
        //TODO: implement svg validation rules here
        Ok(())
    }
}
