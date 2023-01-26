use serde::{Deserialize, Serialize};

use std::fmt;

//TODO: maybe split this up into separate structs

/// `WireCable` represents a particular instance of a `WireType`, `CableType` or `TermCableType`.
/// It represents a physical item.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WireCable {
    /// Internal `id` of wire or cable instance
    pub id: String,
    /// The type of wire of this instance
    pub wire: Option<String>,
    /// The type of cable of this instance
    pub cable: Option<String>,
    /// The type of term_cable of this instance
    pub term_cable: Option<String>,
    /// The structured name of the wire/cable/termcable instance
    pub identifier: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// length of wire or cable TODO: figure out how to return term_cable_length if it is defined
    pub length: Option<f64>,
    /// Pathway containing instance
    pub pathway: Option<String>,
}

impl fmt::Display for WireCable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(wire) = &self.wire {
            writeln!(f, "WireType: {}", wire)?;
        }
        if let Some(cable) = &self.cable {
            writeln!(f, "CableType: {}", cable)?;
        }
        if let Some(term_cable) = &self.term_cable {
            writeln!(f, "TermCableType: {}", term_cable)?;
        }
        if let Some(length) = &self.length {
            writeln!(f, "Length: {}", length)?;
        }
        if let Some(identifier) = &self.identifier {
            writeln!(f, "Equipment Identifier: {}", identifier)?;
        }
        if let Some(pathway) = &self.pathway {
            writeln!(f, "Pathway: {}", pathway)?;
        }
        if let Some(description) = &self.description {
            writeln!(f, "Description: {}", description)?;
        }
        Ok(())
    }
}
