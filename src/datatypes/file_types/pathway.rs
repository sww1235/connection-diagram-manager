use serde::{Deserialize, Serialize};

use std::fmt;

/// `Pathway` represents a physical instance of a pathway
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Pathway {
    /// Internal `id` of pathway instance
    pub id: String,
    /// Type of pathway
    pub path_type: String,
    /// structured identifier of pathway
    pub identifier: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// Length of pathway
    pub length: f64,
}

impl fmt::Display for Pathway {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Pathway Instance: {}", &self.id)?;
        writeln!(f, "Pathway Type: {}", &self.path_type)?;
        if let Some(identifier) = &self.identifier {
            writeln!(f, "Pathway Identifier: {identifier}")?;
        }
        writeln!(f, "Length: {}", &self.length)?;

        if let Some(description) = &self.description {
            writeln!(f, "Description: {description}")?;
        }
        Ok(())
    }
}
