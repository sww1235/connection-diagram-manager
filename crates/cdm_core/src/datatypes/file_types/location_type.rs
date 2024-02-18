use serde::{Deserialize, Serialize};

//TODO: create physical location stuff
/// `LocationType` represents a type/model of location.
///
/// Examples of `LocationType` include junction boxes, racks, panels, etc.
/// It does not include places these are located.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct LocationType {
    /// Manufacturer of LocationType
    pub manufacturer: Option<String>,
    /// Model of LocationType
    pub model: Option<String>,
    /// Part Number of LocationType
    pub part_number: Option<String>,
    /// Manufacturer's Part Number
    pub manufacturer_part_number: Option<String>,
    /// Supplier of Part Number
    pub supplier: Option<String>,
    /// Supplier's Part Number
    pub supplier_part_number: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// Main material of LocationType
    pub material: Option<String>,
    /// Width of locationType
    pub width: f64,
    /// Height of locationType
    pub height: f64,
    /// Depth of locationType
    pub depth: f64,
    /// Usable Width of locationType
    pub usable_width: f64,
    /// Usable Height of locationType
    pub usable_height: f64,
    /// Usable Depth of locationType
    pub usable_depth: f64,
}
