use serde::{Deserialize, Serialize};

use crate::datatypes::util_types::SymbolStyle;

/// `Connector` is the source code representation of the on-disk file format for an in-memory
/// [`Connector`](crate::datatypes::project_types::connector::Connector).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Connector {
    /// The type of this connector instance.
    pub connector_type: String,
    /// Optional styling data for schematic symbol.
    pub symbol_style: Option<SymbolStyle>,
}
