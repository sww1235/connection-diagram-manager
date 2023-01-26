use serde::{Deserialize, Serialize};

use std::fmt;

/// `Equipment` represents a particular instance of an `EquipmentType`.
/// This is the physical unit you would hold in your hand
///
/// Not all fields have to be populated, and some are
/// mainly provided for logical reasons rather than
/// functional (model/part number/manufacturer part number
/// may all be equivalent in some cases)
#[derive(Serialize, Deserialize, Debug, Default, Clone)]

pub struct Equipment {
    //Reminder: don't need id field here, since that is stored in hashmap key.
    /// The type of equipment of the instance
    pub equipment_type: String,
    /// The structured name of the equipment
    pub identifier: Option<String>,
    /// The particular mounting type of this instance
    /// must be in list of mounting types defined in `equip_type.mounting_type`
    pub mounting_type: Option<String>,
    /// The individual location
    pub location: Option<String>,
    /// Description
    pub description: Option<String>,
}

impl fmt::Display for Equipment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "File Equipment Instance:")?;

        writeln!(f, "Equipment Type String: {}", &self.equipment_type)?;
        if let Some(identifier) = &self.identifier {
            writeln!(f, "Identifier: {}", identifier)?;
        }
        if let Some(mounting_type) = &self.mounting_type {
            writeln!(f, "Mounting Type: {}", mounting_type)?;
        }
        if let Some(location) = &self.location {
            writeln!(f, "Location: {}", location)?;
        }
        if let Some(description) = &self.description {
            writeln!(f, "Description: {}", description)?;
        }
        Ok(())
    }
}
