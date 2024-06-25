use serde::{Deserialize, Serialize};

/// `PathwayType` represents a route for wires and cables to take from one
/// [`LocationType`](super::location_type::LocationType) to another.
///
/// Examples of Pathways include, conduit, cable tray, free air
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PathwayType {
    /// Manufacturer of Pathway Type
    pub manufacturer: Option<String>,
    /// Model of Pathway Type
    pub model: Option<String>,
    /// Part Number of Pathway Type
    pub part_number: Option<String>,
    /// Manufacturer's Part Number of Pathway Type
    pub manufacturer_part_number: Option<String>,
    /// Supplier of Pathway Type
    pub supplier: Option<String>,
    /// Supplier's Part Number of Pathway Type
    pub supplier_part_number: Option<String>,
    /// Optional description text
    pub description: Option<String>,
    /// actual size of pathway
    pub size: Option<String>,
    /// Trade Size of pathway
    pub trade_size: Option<String>,
    /// Height of pathway in mm
    pub height: f64,
    /// Width of pathway in mm
    pub width: f64,
    /// Inner cross sectional area of pathway
    pub cross_sect_area: f64,
    /// Main material of pathway
    pub material: Option<String>,
}
