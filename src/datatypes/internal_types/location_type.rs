use std::fmt;

//TODO: create physical location stuff
/// LocationType represents a type/model of location.
///
/// Examples of LocationType include junction boxes, racks, panels, etc.
/// It does not include places these are located.
#[derive(Debug, Default)]
pub struct LocationType {
    /// Internal ID of `LocationType`
    pub id: String,
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
    pub width: Option<f64>,
    /// Height of locationType
    pub height: Option<f64>,
    /// Depth of locationType
    pub depth: Option<f64>,
    /// Usable Width of locationType
    pub usable_width: Option<f64>,
    /// Usable Height of locationType
    pub usable_height: Option<f64>,
    /// Usable Depth of locationType
    pub usable_depth: Option<f64>,
}
impl fmt::Display for LocationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Connector Type:")?;
        if let Some(manufacturer) = &self.manufacturer {
            writeln!(f, "Manufacturer: {}", manufacturer)?;
        }
        if let Some(model) = &self.model {
            writeln!(f, "Model: {}", model)?;
        }
        if let Some(part_number) = &self.part_number {
            writeln!(f, "Part Number: {}", part_number)?;
        }
        if let Some(manufacturer_part_number) = &self.manufacturer_part_number {
            writeln!(f, "Manufacturer Part Number: {}", manufacturer_part_number)?;
        }
        if let Some(supplier) = &self.supplier {
            writeln!(f, "Supplier: {}", supplier)?;
        }
        if let Some(supplier_part_number) = &self.supplier_part_number {
            writeln!(f, "Supplier Part Number: {}", supplier_part_number)?;
        }
        if let Some(description) = &self.description {
            writeln!(f, "Description: {}", description)?;
        }
        if let Some(material) = &self.material {
            writeln!(f, "Material: {}", material)?;
        }
        if let Some(width) = &self.width {
            writeln!(f, "Width: {}", width)?;
        }
        if let Some(height) = &self.height {
            writeln!(f, "Height: {}", height)?;
        }
        if let Some(depth) = &self.depth {
            writeln!(f, "Depth: {}", depth)?;
        }
        if let Some(usable_width) = &self.usable_width {
            writeln!(f, "Usable Width: {}", usable_width)?;
        }
        if let Some(usable_height) = &self.usable_height {
            writeln!(f, "Usable Height: {}", usable_height)?;
        }
        if let Some(usable_depth) = &self.usable_depth {
            writeln!(f, "Usable Depth: {}", usable_depth)?;
        }
        Ok(())
    }
}
