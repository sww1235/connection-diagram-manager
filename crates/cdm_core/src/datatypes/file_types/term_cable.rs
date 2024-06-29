use serde::{Deserialize, Serialize};

/// `Cable` represents a particular instance of a `TermCableType`.
/// It represents a physical item.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct TermCable {
    /// Internal `id` of `TermCable` instance
    pub id: String,
    /// The `TermCableType` of this instance
    #[serde(rename = "type")]
    pub term_cable_type: String,
    /// The structured name of the `TermCable` instance
    pub identifier: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// Pathway containing instance
    pub pathway: Option<String>,
}
