use serde::{Deserialize, Serialize};

use std::fmt;

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

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Location Instance:")?;
        writeln!(f, "Internal ID: {}", &self.id)?;
        writeln!(f, "LocationType: {}", &self.location_type)?;
        if let Some(identifier) = &self.identifier {
            writeln!(f, "Location Identifier: {identifier}")?;
        }
        if let Some(physical_location) = &self.physical_location {
            writeln!(f, "Physical Location of location: {physical_location}")?;
        }
        if let Some(description) = &self.description {
            writeln!(f, "Description: {description}")?;
        }
        Ok(())
    }
}
