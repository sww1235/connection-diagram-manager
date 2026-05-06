use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::datatypes::{
    library_types::term_cable_type::{Connector, WireCable},
    unit_helper::length::Length,
    util_types::{Catalog, LineStyle},
};

/// `TermCableType` is the source code representation of the on-disk file format for an in-memory
/// [`TermCableType`](crate::datatypes::library_types::term_cable_type::TermCableType).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TermCableType {
    /// Catalog information.
    pub catalog: Option<Catalog>,
    /// Underlying wire or cable type of Terminated Cable.
    pub wire_cable: WireCable,
    /// Nominal Length of Terminated Cable.
    pub nominal_length: Option<Length>,
    /// Actual Length of Terminated Cable.
    pub actual_length: Option<Length>,
    /// appearance in schematics.
    #[serde(default)]
    pub line_style: LineStyle,
    /// One end of Terminated Cable.
    pub end1: BTreeMap<String, Connector>,
    /// The other end of Terminated Cable.
    pub end2: BTreeMap<String, Connector>,
}
