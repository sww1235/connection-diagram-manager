use serde::{Deserialize, Serialize};

use crate::datatypes::svg::Svg;

/// `SchematicSymbolType` is the source code representation of the on-disk file format for an in-memory
/// [`SchematicSymbolType`](crate::datatypes::library_types::schematicSymbolType).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SchematicSymbolType {
    /// Short name for display. Can contain spaces/special characters.
    pub name: String,
    /// What to actually display when symbol is rendered on drawing.
    pub visual_representation: Svg,
    /// Optional description.
    pub description: Option<String>,
    /// If this symbol supports links. If true, schematic symbol instances
    /// will be allowed to define links between parent and child components.
    pub supports_links: bool,
}
