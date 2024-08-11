use serde::{Deserialize, Serialize};

/// `LocationType` represents a type/model of location.
///
/// Examples of `LocationType` include junction boxes, racks, panels, etc.
/// It does not include places these are located.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct LocationType {
    /// Manufacturer of Location Type
    pub manufacturer: Option<String>,
    /// Model of Location Type
    pub model: Option<String>,
    /// Part Number of Location Type
    pub part_number: Option<String>,
    /// Manufacturer's Part Number
    pub manufacturer_part_number: Option<String>,
    /// Supplier of Part Number
    pub supplier: Option<String>,
    /// Supplier's Part Number
    pub supplier_part_number: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// Main material of Location Type
    pub material: Option<String>,
    /// Width of Location Type
    pub width: f64,
    /// Height of Location Type
    pub height: f64,
    /// Depth of Location Type
    pub depth: f64,
    /// Usable Width of Location Type
    pub usable_width: f64,
    /// Usable Height of Location Type
    pub usable_height: f64,
    /// Usable Depth of Location Type
    pub usable_depth: f64,
}
