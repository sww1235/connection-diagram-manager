use serde::{Deserialize, Serialize};

use crate::datatypes::{
    project_types::terminal_strip::{Element, Jumper},
    util_types::{IECCodes, PhysicalLocation, UserFields},
};

/// `TerminalStrip` is the source code representation of the on-disk file format for an in-memory
/// [`TerminalStrip`](crate::datatypes::project_types::terminal_strip::TerminalStrip).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TerminalStrip {
    /// structured identifier of terminal strip.
    pub identifier: Option<String>,
    /// Optional description.
    pub description: Option<String>,
    /// containing enclosure.
    pub enclosure: Option<String>,
    /// mounting rail.
    pub mounting_rail: Option<String>,
    /// physical location of `TerminalStrip`.
    pub physical_location: Option<PhysicalLocation>,
    /// Fields for use with IEC project coding.
    pub iec_codes: Option<IECCodes>,
    /// User defined fields.
    pub user_fields: Option<UserFields>,
    /// terminals and accessories defined in terminal strip.
    pub elements: Vec<Element>,
    /// Jumpers in terminal strip.
    pub jumpers: Vec<Jumper>,
}
