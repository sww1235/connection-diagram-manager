use super::svg;
use serde::{Deserialize, Serialize};
use std::fmt;
//TODO: Make some of these fields enums
/// `ConnectorType` represents a particular type of connector.
///
/// Connector can represent more than just a metal or plastic blob on the end of a cable, it can
/// represent a screw terminal on a piece of equipment or a hole for wire to be entered in.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConnectorType {
    /// Manufacturer of Connector
    pub manufacturer: Option<String>,
    /// Model of Connector
    pub model: Option<String>,
    /// Part Number of Connector
    pub part_number: Option<String>,
    /// Manufacturer's Part Number
    pub manufacturer_part_number: Option<String>,
    /// Supplier of Connector
    pub supplier: Option<String>,
    /// Supplier's Part Number
    pub supplier_part_number: Option<String>,
    /// Optional text description
    pub description: Option<String>,
    /// Mounting method of connector
    ///
    /// Cable, PCB through hole, PCB surface mout, panel
    pub mount_type: Option<String>,
    /// Panel Cutout of connector if it is panel mounted
    ///
    /// D, A, etc
    pub panel_cutout: Option<String>,
    /// Gender of connector
    ///
    /// Male, Female, RPMale, RPFemale, Hermaphrodidic, unknown
    pub gender: Option<String>,
    /// height of connector in mm
    pub height: Option<f64>,
    /// width of connector in mm
    pub width: Option<f64>,
    /// depth of connector in mm
    pub depth: Option<f64>,
    /// diameter of circular connectors in mm
    pub diameter: Option<f64>,
    //TODO: rethink how pins are specified. Maybe have a pin/contact type, with pin number, label, signal
    //type, etc
    /// total number of pins, if omitted, will be set to length of list below
    pub pin_count: Option<u64>,
    /// list of pin numbers/names
    ///
    /// if omitted, is autofilled with [1,2,3, ..., pincount]
    pub pins: Option<Vec<String>>,
    /// if omitted, is autofilled with blanks
    pub pin_labels: Option<Vec<String>>,
    /// colors assigned to pins
    ///
    /// goes in order of pin count/pin list, if fewer colors are specified than pins, end of list
    /// will have no colors specified.
    pub pin_colors: Option<Vec<String>>,
    /// signal type of each pin.
    /// goes in order of pin count/pin list, if fewer colors are specified than pins, end of list
    /// will have no colors specified.
    pub pin_signal_type: Option<Vec<String>>,
    /// overall diagram of connector TODO: figure out what angle this should be
    pub visual_rep: Option<svg::Svg>,
    /// representation of pin
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
