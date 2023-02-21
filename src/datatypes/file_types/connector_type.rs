use super::svg::Svg;
use serde::{Deserialize, Serialize};
use std::fmt;
//TODO: Make some of these fields enums
/// `ConnectorType` represents a particular type of connector.
///
/// Connector can represent more than just a metal or plastic blob on the end of a cable, it can
/// represent a screw terminal on a piece of equipment or a hole for wire to be entered in.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
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
    pub height: f64,
    /// width of connector in mm
    pub width: f64,
    /// depth of connector in mm
    pub depth: f64,
    /// diameter of circular connectors in mm
    pub diameter: Option<f64>,
    /// pins inside connector.
    ///
    /// Pin index is not guaranteed to be the same. Use `ConnectorPin.id` for confirming equality.
    pub pins: Vec<ConnectorPin>,
    /// Visual representation of connector
    pub visual_rep: Svg,
}

/// Represents an individual pin in a `ConnectorType`
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ConnectorPin {
    /// Pin number or identifier in connector
    pub id: Option<String>,
    /// Pin label or name
    pub label: Option<String>,
    /// Pin signal type
    pub signal_type: Option<String>,
    /// Pin color
    pub color: Option<String>,
    /// visual representation of an individual pin
    pub visual_rep: Option<Svg>,
    /// gender of pin
    pub gender: Option<String>,
}

impl fmt::Display for ConnectorPin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Pin:")?;
        if let Some(id) = &self.id {
            writeln!(f, "\tPin ID: {id}")?;
        }
        if let Some(label) = &self.label {
            writeln!(f, "\tPin Label: {label}")?;
        }
        if let Some(signal_type) = &self.signal_type {
            writeln!(f, "\tPin Signal Type: {signal_type}")?;
        }
        if let Some(color) = &self.color {
            writeln!(f, "\tPin Color: {color}")?;
        }
        if let Some(gender) = &self.gender {
            writeln!(f, "\tPin Gender: {gender}")?;
        }
        //TODO: provide a way of showing visual representation
        Ok(())
    }
}
impl fmt::Display for ConnectorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Connector Type:")?;
        if let Some(manufacturer) = &self.manufacturer {
            writeln!(f, "\tManufacturer: {manufacturer}")?;
        }
        if let Some(model) = &self.model {
            writeln!(f, "\tModel: {model}")?;
        }
        if let Some(part_number) = &self.part_number {
            writeln!(f, "\tPart Number: {part_number}")?;
        }
        if let Some(manufacturer_part_number) = &self.manufacturer_part_number {
            writeln!(f, "\tManufacturer Part Number: {manufacturer_part_number}")?;
        }
        if let Some(supplier) = &self.supplier {
            writeln!(f, "\tSupplier: {supplier}")?;
        }
        if let Some(supplier_part_number) = &self.supplier_part_number {
            writeln!(f, "\tSupplier Part Number: {supplier_part_number}")?;
        }
        if let Some(description) = &self.description {
            writeln!(f, "\tDescription: {description}")?;
        }
        if let Some(mount_type) = &self.mount_type {
            writeln!(f, "\tMount Type: {mount_type}")?;
        }
        if let Some(panel_cutout) = &self.panel_cutout {
            writeln!(f, "\tPanel Cutout: {panel_cutout}")?;
        }
        if let Some(gender) = &self.gender {
            writeln!(f, "\tGender: {gender}")?;
        }
        writeln!(f, "\tHeight: {:.2} mm", self.height)?;
        writeln!(f, "\tWidth: {:.2} mm", self.width)?;
        writeln!(f, "\tDepth: {:.2} mm", self.depth)?;
        if let Some(diameter) = &self.diameter {
            writeln!(f, "\tDiameter: {diameter:.2} mm")?;
        }
        writeln!(f, "\tPin Count: {}", self.pins.len())?;
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
