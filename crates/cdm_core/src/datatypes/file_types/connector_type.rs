use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::datatypes::{
    color::Color,
    library_types::connector_type::ConnectorPin,
    svg::Svg,
    util_types::{Catalog, Dimension},
};

/// `ConnectorType` is the source code representation of the on-disk file format for an in-memory
/// [`ConnectorType`](crate::datatypes::library_types::connector_type::ConnectorType).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ConnectorType {
    /// Catalog information.
    pub catalog: Option<Catalog>,
    /// Dimensional information of connector.
    pub dimensions: Option<Dimension>,
    /// Mounting method of connector.
    ///
    /// Cable, PCB through hole, PCB surface mount, panel.
    pub mount_type: Option<String>,
    /// Panel Cutout of connector if it is panel mounted.
    ///
    /// D, A, etc.
    pub panel_cutout: Option<String>,
    /// Gender of connector.
    ///
    /// Male, Female, RPMale, RPFemale, Hermaphrodidic, unknown.
    pub gender: Option<String>,
    /// connector color.
    pub color: Option<Color>,
    /// component designator.
    pub component_designator: Option<String>,
    /// Vector of schematic symbols that can represent this `connector_type`.
    pub schematic_symbols: Option<Vec<String>>,
    /// Optional list of other connector types this one can mate with.
    pub connector_type_mate: Option<Vec<String>>,
    /// pins inside connector.
    ///
    /// Pin index is not guaranteed to be the same. Use `ConnectorPin.id` for confirming equality.
    pub pins: BTreeMap<String, ConnectorPin>,
    /// overall diagram of connector TODO: figure out what angle this should be.
    pub visual_representation: Option<Svg>,
}
