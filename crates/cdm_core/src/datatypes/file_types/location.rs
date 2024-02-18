use serde::{Deserialize, Serialize};

/// `Location` represents a physical instance of a pathway
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Location {
    /// Internal `id` of location instance
    pub id: String,
    /// Type of location
    pub location_type: String,
    /// structured identifier of location
    pub identifier: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// Physical Location
    pub physical_location: Option<String>,
    //TODO: add sub locations
}
