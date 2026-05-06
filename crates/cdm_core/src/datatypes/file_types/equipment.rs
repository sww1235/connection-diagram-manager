use serde::{Deserialize, Serialize};

use crate::datatypes::util_types::{IECCodes, PhysicalLocation, SymbolStyle, UserFields};

/// `Equipment` is the source code representation of the on-disk file format for an in-memory
/// [`Equipment`](crate::datatypes::project_types::equipment::Equipment).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Equipment {
    /// The type of equipment of the instance.
    pub equipment_type: String,
    /// The structured name of the equipment.
    pub identifier: String,
    /// The particular mounting type of this instance
    /// must be in list of mounting types defined in `equip_type.mounting_type`.
    ///
    /// Validated on import.
    pub mounting_type: Option<String>,
    /// The containing `Enclosure` ID.
    pub enclosure: Option<String>,
    /// The ID of the `MountPoint` within the `Enclosure`.
    pub mount_point: Option<String>,
    /// The physical location of this piece of equipment.
    pub physical_location: Option<PhysicalLocation>,
    /// fields for IEC coding.
    pub iec_codes: Option<IECCodes>,
    /// Description.
    pub description: Option<String>,
    /// Optional user Fields.
    pub user_fields: Option<UserFields>,
    /// Optional styling data for schematic symbol.
    ///
    /// Styling can also be defined in the SVG itself, but will be overrriden if this is defined.
    pub symbol_style: Option<SymbolStyle>,
}
