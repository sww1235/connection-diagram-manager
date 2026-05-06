use serde::{Deserialize, Serialize};

use crate::datatypes::{
    unit_helper::length::Length,
    util_types::{IECCodes, PhysicalLocation, UserFields},
};

/// `Wire` is the source code representation of the on-disk file format for an in-memory
/// [`Wire`](crate::datatypes::project_types::wire::Wire).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Wire {
    /// The `WireType` of this instance.
    pub wire_type: String,
    /// The structured name of the `Wire` instance. This can be used as a wire number or other
    /// identifier.
    pub identifier: String,
    /// Optional description.
    pub description: Option<String>,
    /// length of wire.
    pub length: Length,
    /// physical location of Wire.
    pub physical_location: Option<PhysicalLocation>,
    /// Fields for use with IEC project coding.
    pub iec_codes: Option<IECCodes>,
    /// User defined fields.
    pub user_fields: Option<UserFields>,
    /// Pathway containing instance.
    pub pathway: Option<String>,
    /// An optional single pin connector on one end of this `Wire`.
    pub end1_connector_type: Option<String>,
    /// An optional single pin connector on the other end of this `Wire`.
    pub end2_connector_type: Option<String>,
}
