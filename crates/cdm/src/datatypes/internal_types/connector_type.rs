use std::collections::HashMap;
use std::fmt;

use super::{svg::Svg, Empty, Mergable, PartialEmpty};

use dimensioned::ucum;

//TODO: Make some of these fields enums
/// `ConnectorType` represents a particular type of connector.
///
/// Connector can represent more than just a metal or plastic blob on the end of a cable, it can
/// represent a screw terminal on a piece of equipment or a hole for wire to be entered in.
#[derive(Debug, Default, PartialEq)]
pub struct ConnectorType {
    /// Internal ID of ConnectorType
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
        Self {
            id: String::new(),
            manufacturer: None,
            model: None,
            part_number: None,
            manufacturer_part_number: None,
            supplier: None,
            supplier_part_number: None,
            description: None,
            mount_type: None,
            panel_cutout: None,
            gender: None,
            height: 0.0_f64 * ucum::M,
            width: 0.0_f64 * ucum::M,
            depth: 0.0_f64 * ucum::M,
            diameter: None,
            pins: Vec::new(),
            visual_rep: Svg::new(),
        }
    }
}
impl Mergable for ConnectorType {
    #[allow(clippy::too_many_lines)]
    // TODO: see if this can be split up
    fn merge_prompt(
        &mut self,
        other: &Self,
        prompt_fn: fn(HashMap<String, [String; 2]>) -> HashMap<String, bool>,
    ) {
        //TODO: maybe check for partial_empty/empty here on other
        let mut input_map: HashMap<String, [String; 2]> = HashMap::new();
        if self.id != other.id {
            panic! {"attempting to merge structs with different IDs. This shouldn't have happened."}
        }
        if self.manufacturer != other.manufacturer {
            input_map.insert(
                "Manufacturer".to_string(),
                [
                    {
                        if let Some(manufacturer) = self.manufacturer.clone() {
                            manufacturer
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(manufacturer) = other.manufacturer.clone() {
                            manufacturer
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.model != other.model {
            input_map.insert(
                "Model".to_string(),
                [
                    {
                        if let Some(model) = self.model.clone() {
                            model
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(model) = other.model.clone() {
                            model
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.part_number != other.part_number {
            input_map.insert(
                "Part Number".to_string(),
                [
                    {
                        if let Some(part_number) = self.part_number.clone() {
                            part_number
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(part_number) = other.part_number.clone() {
                            part_number
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.manufacturer_part_number != other.manufacturer_part_number {
            input_map.insert(
                "Manufacturer Part Number".to_string(),
                [
                    {
                        if let Some(manufacturer_part_number) =
                            self.manufacturer_part_number.clone()
                        {
                            manufacturer_part_number
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(manufacturer_part_number) =
                            other.manufacturer_part_number.clone()
                        {
                            manufacturer_part_number
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.supplier != other.supplier {
            input_map.insert(
                "Supplier".to_string(),
                [
                    {
                        if let Some(supplier) = self.supplier.clone() {
                            supplier
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(supplier) = other.supplier.clone() {
                            supplier
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.supplier_part_number != other.supplier_part_number {
            input_map.insert(
                "Supplier Part Number".to_string(),
                [
                    {
                        if let Some(supplier_part_number) = self.supplier_part_number.clone() {
                            supplier_part_number
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(supplier_part_number) = other.supplier_part_number.clone() {
                            supplier_part_number
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.description != other.description {
            input_map.insert(
                "Description".to_string(),
                [
                    {
                        if let Some(description) = self.description.clone() {
                            description
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(description) = other.description.clone() {
                            description
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.mount_type != other.mount_type {
            input_map.insert(
                "Mount Type".to_string(),
                [
                    {
                        if let Some(mount_type) = self.mount_type.clone() {
                            mount_type
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(mount_type) = other.mount_type.clone() {
                            mount_type
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.panel_cutout != other.panel_cutout {
            input_map.insert(
                "Panel Cutout".to_string(),
                [
                    {
                        if let Some(panel_cutout) = self.panel_cutout.clone() {
                            panel_cutout
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(panel_cutout) = other.panel_cutout.clone() {
                            panel_cutout
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.gender != other.gender {
            input_map.insert(
                "Gender".to_string(),
                [
                    {
                        if let Some(gender) = self.gender.clone() {
                            gender
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(gender) = other.gender.clone() {
                            gender
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.height != other.height {
            input_map.insert(
                "Height".to_string(),
                [self.height.to_string(), other.height.to_string()],
            );
        }
        if self.width != other.width {
            input_map.insert(
                "Width".to_string(),
                [self.width.to_string(), other.width.to_string()],
            );
        }
        if self.depth != other.depth {
            input_map.insert(
                "Depth".to_string(),
                [self.depth.to_string(), other.depth.to_string()],
            );
        }
        if self.diameter != other.diameter {
            input_map.insert(
                "Diameter".to_string(),
                [
                    {
                        if let Some(diameter) = self.diameter {
                            diameter.to_string()
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(diameter) = other.diameter {
                            diameter.to_string()
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.pins != other.pins {
            let mut self_string = String::new();
            let mut other_string = String::new();
            for pin in &self.pins {
                self_string.push('(');
                self_string.push_str(pin.id.as_str());
                self_string.push_str(", ");
                if let Some(label) = &pin.label {
                    self_string.push_str(label.as_str());
                }
                self_string.push_str(", ");
                if let Some(color) = &pin.color {
                    self_string.push_str(color.as_str());
                }
                self_string.push_str(")\t");
            }
            for pin in &other.pins {
                other_string.push('(');
                other_string.push_str(pin.id.as_str());
                other_string.push_str(", ");

                if let Some(label) = &pin.label {
                    other_string.push_str(label.as_str());
                }
                other_string.push_str(", ");
                if let Some(color) = &pin.color {
                    other_string.push_str(color.as_str());
                }
                other_string.push_str(")\t");
            }
            input_map.insert("Pins".to_string(), [self_string, other_string]);
        }

        let results = prompt_fn(input_map);
        // false means don't replace value in self struct
        if results["Manufacturer"] {
            self.manufacturer = other.manufacturer.clone();
        }
        if results["Model"] {
            self.model = other.model.clone();
        }
        if results["Part Number"] {
            self.part_number = other.part_number.clone();
        }
        if results["Manufacturer Part Number"] {
            self.manufacturer_part_number = other.manufacturer_part_number.clone();
        }
        if results["Supplier"] {
            self.supplier = other.supplier.clone();
        }
        if results["Supplier Part Number"] {
            self.supplier_part_number = other.supplier_part_number.clone();
        }
        if results["Description"] {
            self.description = other.description.clone();
        }
        if results["Mount Type"] {
            self.mount_type = other.mount_type.clone();
        }
        if results["Panel Cutout"] {
            self.panel_cutout = other.panel_cutout.clone();
        }
        if results["Gender"] {
            self.gender = other.gender.clone();
        }
        if results["Height"] {
            self.height = other.height;
        }
        if results["Width"] {
            self.width = other.width;
        }
        if results["Depth"] {
            self.depth = other.depth;
        }
        if results["Diameter"] {
            self.diameter = other.diameter;
        }
        if results["Pins"] {
            self.pins = other.pins.clone();
        }
    }
}

impl Empty for ConnectorType {
    fn is_empty(&self) -> bool {
        self == &Self::new()
    }
}

impl PartialEmpty for ConnectorType {
    fn is_partial_empty(&self) -> bool {
        let tester = Self::new();
        self.manufacturer == tester.manufacturer
            && self.model == tester.model
            && self.part_number == tester.part_number
            && self.manufacturer_part_number == tester.manufacturer_part_number
            && self.supplier == tester.supplier
            && self.supplier_part_number == tester.supplier_part_number
            && self.description == tester.description
            && self.mount_type == tester.mount_type
            && self.panel_cutout == tester.panel_cutout
            && self.gender == tester.gender
            && self.height == tester.height
            && self.width == tester.width
            && self.diameter == tester.diameter
            && self.pins == tester.pins
            && self.visual_rep == tester.visual_rep
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
