use serde::{Deserialize, Serialize};

/// `WireCable` represents a particular instance of a `WireType`, `CableType` or `TermCableType`.
/// It represents a physical item.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
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
    /// length of wire or cable
    pub length: Option<f64>,
    /// Pathway containing instance
    pub pathway: Option<String>,
}
