use serde::{Deserialize, Serialize};

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
    #[serde(rename = "type")]
    pub equipment_type: String,
    /// The structured name of the equipment
    pub identifier: Option<String>,
    /// The particular mounting type of this instance
    /// must be in list of mounting types defined in `equip_type.mounting_type`
    pub mounting_type: Option<String>,
    /// The contained location
    pub location: String,
    /// The sublocation wthin the location
    pub sub_location: String,
    /// Description
    pub description: Option<String>,
}
