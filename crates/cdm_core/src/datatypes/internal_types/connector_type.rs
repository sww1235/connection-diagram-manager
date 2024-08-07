use std::fmt;
use std::path::PathBuf;

use super::svg::Svg;

use dimensioned::ucum;

use cdm_macros::{Empty, Merge, PartialEmpty};

//TODO: Make some of these fields enums
/// `ConnectorType` represents a particular type of connector.
///
/// Connector can represent more than just a metal or plastic blob on the end of a cable, it can
/// represent a screw terminal on a piece of equipment or a hole for wire to be entered in.
#[derive(Debug, Default, PartialEq, Merge, PartialEmpty, Empty)]
pub struct ConnectorType {
    /// Internal ID of `ConnectorType`
    pub id: String,
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
    pub height: ucum::Meter<f64>,
    /// width of connector in mm
    pub width: ucum::Meter<f64>,
    /// depth of connector in mm
    pub depth: ucum::Meter<f64>,
    /// diameter of circular connectors in mm
    pub diameter: Option<ucum::Meter<f64>>,
    /// pins inside connector.
    ///
    /// Pin index is not guaranteed to be the same. Use `ConnectorPin.id` for confirming equality.
    pub pins: Vec<ConnectorPin>,
    /// overall diagram of connector TODO: figure out what angle this should be
    pub visual_rep: Svg,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}

/// Represents an individual pin in a `ConnectorType`
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ConnectorPin {
    /// Pin number or identifier in connector
    pub id: String,
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

impl ConnectorType {
    /// Creates an empty instance of `ConnectorType`
    #[allow(clippy::arithmetic_side_effects)]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl fmt::Display for ConnectorPin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Pin:")?;
        writeln!(f, "Pin ID: {}", self.id)?;
        if let Some(label) = &self.label {
            writeln!(f, "Pin Label: {label}")?;
        }
        if let Some(signal_type) = &self.signal_type {
            writeln!(f, "Pin Signal Type: {signal_type}")?;
        }
        if let Some(color) = &self.color {
            writeln!(f, "Pin Color: {color}")?;
        }
        if let Some(gender) = &self.gender {
            writeln!(f, "Pin Gender: {gender}")?;
        }
        //TODO: provide a way of showing visual representation
        Ok(())
    }
}

impl fmt::Display for ConnectorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Connector Type:")?;
        if let Some(manufacturer) = &self.manufacturer {
            writeln!(f, "Manufacturer: {manufacturer}")?;
        }
        if let Some(model) = &self.model {
            writeln!(f, "Model: {model}")?;
        }
        if let Some(part_number) = &self.part_number {
            writeln!(f, "Part Number: {part_number}")?;
        }
        if let Some(manufacturer_part_number) = &self.manufacturer_part_number {
            writeln!(f, "Manufacturer Part Number: {manufacturer_part_number}")?;
        }
        if let Some(supplier) = &self.supplier {
            writeln!(f, "Supplier: {supplier}")?;
        }
        if let Some(supplier_part_number) = &self.supplier_part_number {
            writeln!(f, "Supplier Part Number: {supplier_part_number}")?;
        }
        if let Some(description) = &self.description {
            writeln!(f, "Description: {description}")?;
        }
        if let Some(mount_type) = &self.mount_type {
            writeln!(f, "Mount Type: {mount_type}")?;
        }
        if let Some(panel_cutout) = &self.panel_cutout {
            writeln!(f, "Panel Cutout: {panel_cutout}")?;
        }
        if let Some(gender) = &self.gender {
            writeln!(f, "Gender: {gender}")?;
        }
        writeln!(f, "Height: {:.2}", self.height)?;
        writeln!(f, "Width: {:.2}", self.width)?;
        if let Some(diameter) = &self.diameter {
            writeln!(f, "Diameter: {diameter:.2} mm")?;
        }
        for pin in &self.pins {
            writeln!(f, "{pin}")?;
        }
        //TODO: implement loop here to print all pins
        //if let Some() = &self.pins {
        //    writeln!(f, "Panel Cutout: {}", )?;
        //}
        //TODO: implement svg validation rules here
        Ok(())
    }
}
