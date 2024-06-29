use serde::{Deserialize, Serialize};

/// `Pathway` represents a physical instance of a pathway
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Pathway {
    /// Internal `id` of pathway instance
    pub id: String,
    /// Type of pathway
    #[serde(rename = "type")]
    pub path_type: String,
    /// structured identifier of pathway
    pub identifier: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// Length of pathway
    pub length: f64,
}
