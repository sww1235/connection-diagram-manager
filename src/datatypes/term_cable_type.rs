use super::{cable_type, connector_type, wire_type};
use serde::{Deserialize, Serialize};

use std::fmt;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TermCableType {
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub part_number: Option<String>,
    pub manufacturer_part_number: Option<String>,
    pub supplier: Option<String>,
    pub supplier_part_number: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "cable_type")]
    pub cable: Option<cable_type::CableType>,
    #[serde(rename = "wire_type")]
    pub wire: Option<wire_type::WireType>,
    pub nominal_length: Option<u64>,
    pub actual_length: Option<u64>,
    #[serde(rename = "term_cable_connector")]
    pub end1: Option<Vec<TermCableConnector>>,
    #[serde(rename = "term_cable_connector")]
    pub end2: Option<Vec<TermCableConnector>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TermCableConnectorTermination {
    pub core: Option<u64>,
    pub pin: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TermCableConnector {
    pub connector_type: Option<connector_type::ConnectorType>,
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
