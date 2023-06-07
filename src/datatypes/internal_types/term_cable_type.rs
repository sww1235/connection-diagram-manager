use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use dimensioned::ucum;

use super::{
    cable_type::CableType, connector_type::ConnectorType, wire_type::WireType, Empty, Mergable,
    PartialEmpty,
};
/// `TermCableType` represents a terminated cable with 2 ends and a connector on at least 1 end.
#[derive(Debug, Default, PartialEq)]
pub struct TermCableType {
    /// Internal ID of `TermCableType`
    pub id: String,
    /// Manufacturer of Terminated cable
    pub manufacturer: Option<String>,
    /// Model of Terminated Cable
    pub model: Option<String>,
    /// Part Number of Terminated Cable
    pub part_number: Option<String>,
    /// Manufacturers part number of Terminated Cable
    pub manufacturer_part_number: Option<String>,
    /// Supplier of Terminated Cable
    pub supplier: Option<String>,
    /// Supplier part number of Terminated Cable
    pub supplier_part_number: Option<String>,
    /// Optional text description of Terminated Cable
    pub description: Option<String>,
    /// Underlying wire or cable type of Terminated Cable
    pub wire_cable: WireCable,
    /// Nominal Length of Terminated Cable
    pub nominal_length: Option<ucum::Meter<f64>>, //TODO: decide if one of these should be optional or not
    /// Actual Length of Terminated Cable
    pub actual_length: Option<ucum::Meter<f64>>,
    /// One end of Terminated Cable.
    pub end1: Vec<TermCableConnector>,
    /// The other end of Terminated Cable
    pub end2: Vec<TermCableConnector>,
}

/// `WireCable` allows either a `WireType` or `CableType` to be the root of a `TermCableType`
#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::exhaustive_enums)]
pub enum WireCable {
    /// CableType
    CableType(Rc<RefCell<CableType>>),
    /// WireType
    WireType(Rc<RefCell<WireType>>),
}
// have to implement default for this for some weird reason
impl Default for WireCable {
    fn default() -> Self {
        WireCable::WireType(Rc::new(RefCell::new(WireType::new())))
    }
}

/// `TermCableConnectorTermination` represents the connections between a pin of an individual
/// `TermCableConnector` and the individual core of the cable.
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TermCableConnectorTermination {
    /// `Core` represents which individual wire inside a cable this pin is connected to
    pub core: Option<u64>,
    /// `Pin` represents which pin in the associated connector the core is connected to
    pub pin: Option<u64>,
}

/// `TermCableConnector` represents a connector on one end of a `TermCable`
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TermCableConnector {
    /// `connector_type` represents the connector type that is on the end of a `TermCable`
    pub connector_type: Rc<RefCell<ConnectorType>>,
    /// `terminations` represents the pin/core mapping for this connector
    pub terminations: Option<Vec<TermCableConnectorTermination>>,
}
impl TermCableType {
    /// Creates an empty instance of `TermCableType`
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
            wire_cable: WireCable::default(),
            nominal_length: None,
            actual_length: None,
            end1: Vec::new(),
            end2: Vec::new(),
        }
    }
}

impl Mergable for TermCableType {
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
        if self.wire_cable != other.wire_cable {
            let self_string = match &self.wire_cable {
                WireCable::CableType(cable_type) => cable_type.borrow().id.clone(),
                WireCable::WireType(wire_type) => wire_type.borrow().id.clone(),
            };

            let other_string = match &other.wire_cable {
                WireCable::CableType(cable_type) => cable_type.borrow().id.clone(),
                WireCable::WireType(wire_type) => wire_type.borrow().id.clone(),
            };

            input_map.insert("Wire Cable".to_string(), [self_string, other_string]);
        }
        if self.nominal_length != other.nominal_length {
            input_map.insert(
                "Nominal Length".to_string(),
                [
                    {
                        if let Some(nominal_length) = self.nominal_length {
                            nominal_length.to_string()
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(nominal_length) = other.nominal_length {
                            nominal_length.to_string()
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.actual_length != other.actual_length {
            input_map.insert(
                "Actual Length".to_string(),
                [
                    {
                        if let Some(actual_length) = self.actual_length {
                            actual_length.to_string()
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(actual_length) = other.actual_length {
                            actual_length.to_string()
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.end1 != other.end1 {
            let mut self_string = String::new();
            let mut other_string = String::new();
            for connector in &self.end1 {
                self_string.push('(');
                self_string.push_str(connector.connector_type.borrow().id.as_str());
                self_string.push('\t');
            }
            for connector in &other.end1 {
                other_string.push('(');
                other_string.push_str(connector.connector_type.borrow().id.as_str());
                other_string.push('\t');
            }
            input_map.insert("End1".to_string(), [self_string, other_string]);
        }
        if self.end2 != other.end2 {
            let mut self_string = String::new();
            let mut other_string = String::new();
            for connector in &self.end2 {
                self_string.push('(');
                self_string.push_str(connector.connector_type.borrow().id.as_str());
                self_string.push('\t');
            }
            for connector in &other.end2 {
                other_string.push('(');
                other_string.push_str(connector.connector_type.borrow().id.as_str());
                other_string.push('\t');
            }
            input_map.insert("End2".to_string(), [self_string, other_string]);
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
        if results["Wire Cable"] {
            self.wire_cable = other.wire_cable.clone();
        }
        if results["Nominal Length"] {
            self.nominal_length = other.nominal_length;
        }
        if results["Actual Length"] {
            self.actual_length = other.actual_length;
        }
        if results["End1"] {
            self.end1 = other.end1.clone();
        }
        if results["End2"] {
            self.end2 = other.end2.clone();
        }
    }
}

impl Empty for TermCableType {
    fn is_empty(&self) -> bool {
        self == &Self::new()
    }
}

impl PartialEmpty for TermCableType {
    fn is_partial_empty(&self) -> bool {
        let tester = Self::new();
        self.manufacturer == tester.manufacturer
            && self.model == tester.model
            && self.part_number == tester.part_number
            && self.manufacturer_part_number == tester.manufacturer_part_number
            && self.supplier == tester.supplier
            && self.supplier_part_number == tester.supplier_part_number
            && self.description == tester.description
            && self.wire_cable == tester.wire_cable
            && self.nominal_length == tester.nominal_length
            && self.actual_length == tester.actual_length
            && self.end1 == tester.end1
            && self.end2 == tester.end2
    }
}

impl fmt::Display for TermCableType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Connector Type:")?;
        if let Some(manufacturer) = &self.manufacturer {
            write!(f, "Manufacturer: {manufacturer}")?;
        }
        if let Some(model) = &self.model {
            write!(f, "Model: {model}")?;
        }
        if let Some(part_number) = &self.part_number {
            write!(f, "Part Number: {part_number}")?;
        }
        if let Some(manufacturer_part_number) = &self.manufacturer_part_number {
            write!(f, "Manufacturer Part Number: {manufacturer_part_number}")?;
        }
        if let Some(supplier) = &self.supplier {
            write!(f, "Supplier: {supplier}")?;
        }
        if let Some(supplier_part_number) = &self.supplier_part_number {
            write!(f, "Supplier Part Number: {supplier_part_number}")?;
        }
        if let Some(description) = &self.description {
            write!(f, "Description: {description}")?;
        }
        match &self.wire_cable {
            WireCable::CableType(cable) => write!(f, "Cable Type: {}", cable.borrow())?,
            WireCable::WireType(wire) => write!(f, "Wire Type: {}", wire.borrow())?,
        }
        if let Some(nominal_length) = &self.nominal_length {
            //TODO: implement units functions to do proper conversions
            write!(f, "Nominal Length: {nominal_length}mm")?;
        }
        if let Some(actual_length) = &self.actual_length {
            //TODO: implement units functions to do proper conversions
            write!(f, "Actual Length: {actual_length} mm")?;
        }
        //TODO: implement loops for cable ends.
        Ok(())
    }
}
