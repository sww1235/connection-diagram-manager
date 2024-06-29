use serde::{Deserialize, Serialize};

use std::collections::HashMap;

/// `Location` represents a physical instance of a pathway
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Location {
    /// Internal `id` of location instance
    pub id: String,
    /// Type of location
    #[serde(rename = "type")]
    pub location_type: String,
    /// structured identifier of location
    pub identifier: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// Physical Location
    pub physical_location: Option<String>,
    /// Sub Locations. Hashmap enforces unique keys
    pub sub_locations: HashMap<String, SubLocation>,
}

/// Unique coordinate triplet within `Location`
/// TODO: decide on units
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct SubLocation {
    /// Distance from left side of parent location
    pub x: f64,
    /// Distance from bottom of parent location
    pub y: f64,
    /// Distance from back of parent location
    pub z: f64,
}
