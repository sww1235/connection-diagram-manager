use super::{cable_type, connector_type, wire_type};
use serde::{Deserialize, Serialize};

use std::fmt;

/// `TermCableType` represents a terminated cable with 2 ends and a connector on at least 1 end.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TermCableType {
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
    /// Underlying cable type of Terminated Cable
    #[serde(rename = "cable_type")]
    pub cable: Option<cable_type::CableType>,
    /// Underlying wire type of Terminated Cable
    #[serde(rename = "wire_type")]
    pub wire: Option<wire_type::WireType>,
    /// Nominal Length of Terminated Cable
    pub nominal_length: Option<u64>,
    /// Actual Length of Terminated Cable
    pub actual_length: Option<u64>,
    /// One end of Terminated Cable.
    #[serde(rename = "term_cable_connector")]
    pub end1: Option<Vec<TermCableConnector>>,
    /// The other end of Terminated Cable
    #[serde(rename = "term_cable_connector")]
    pub end2: Option<Vec<TermCableConnector>>,
}

/// TermCableConnectorTermination represents the connections between a pin of an individual
/// TermCableConnector and the individual core of the cable.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TermCableConnectorTermination {
    /// Core represents which individual wire inside a cable this pin is connected to
    pub core: Option<u64>,
    /// Pin represents which pin in the associated connector the core is connected to
    pub pin: Option<u64>,
}

/// TermCableConnector represents a connector on one end of a TermCable
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TermCableConnector {
    /// connector_type represents the connector type that is on the end of a TermCable
    pub connector_type: Option<connector_type::ConnectorType>,
    /// terminations represents the pin/core mapping for this connector
    pub terminations: Option<TermCableConnectorTermination>,
}
impl fmt::Display for TermCableType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Connector Type:")?;
        if let Some(manufacturer) = &self.manufacturer {
            write!(f, "Manufacturer: {}", manufacturer)?;
        }
        if let Some(model) = &self.model {
            write!(f, "Model: {}", model)?;
        }
        if let Some(part_number) = &self.part_number {
            write!(f, "Part Number: {}", part_number)?;
        }
        if let Some(manufacturer_part_number) = &self.manufacturer_part_number {
            write!(f, "Manufacturer Part Number: {}", manufacturer_part_number)?;
        }
        if let Some(supplier) = &self.supplier {
            write!(f, "Supplier: {}", supplier)?;
        }
        if let Some(supplier_part_number) = &self.supplier_part_number {
            write!(f, "Supplier Part Number: {}", supplier_part_number)?;
        }
        if let Some(description) = &self.description {
            write!(f, "Description: {}", description)?;
        }
        if let Some(cable) = &self.cable {
            write!(f, "Cable Type: {}", cable)?;
        }
        if let Some(wire) = &self.wire {
            write!(f, "Wire Type: {}", wire)?;
        }
        if let Some(nominal_length) = &self.nominal_length {
            //TODO: implement units functions to do proper conversions
            write!(f, "Nominal Length: {}mm", nominal_length)?;
        }
        if let Some(actual_length) = &self.actual_length {
            //TODO: implement units functions to do proper conversions
            write!(f, "Actual Length: {} mm", actual_length)?;
        }
        //TODO: implement loops for cable ends.
        Ok(())
    }
}
