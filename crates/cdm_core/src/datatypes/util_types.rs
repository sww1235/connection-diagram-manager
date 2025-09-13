use serde::{Deserialize, Serialize};

use crate::datatypes::{color::Color, unit_helper::Length};

/// Cross section of wire or cable
#[non_exhaustive]
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CrossSection {
    /// A wire or cable with an oval or flat cross section
    Oval,
    /// A wire or cable with a circular cross section
    #[default]
    Circular,
    /// A cable consisting of 2 or more wires/cables bonded to each other not inside the same
    /// external jacket.
    Siamese,
}

/// Common Catalog information for Library Types
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Catalog {
    /// manufacturer name
    pub manufacturer: Option<String>,
    /// connector model description
    pub model: Option<String>,
    /// free text field for larger descriptions
    pub description: Option<String>,
    /// [internal] part number
    pub part_number: Option<String>,
    /// manufacturer part number
    pub manufacturer_part_number: Option<String>,
    /// supplier name
    pub supplier: Option<String>,
    /// supplier part number
    pub supplier_part_number: Option<String>,
}

/// Common Dimension information for Library Types
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Dimension {
    /// height of connector
    pub height: Length,
    height_unit: String,
    /// width of connector
    pub width: Length,
    width_unit: String,
    /// depth of connector
    pub depth: Option<Length>,
    depth_unit: Option<String>,
    /// diameter of circular connectors
    pub diameter: Option<Length>,
    diameter_unit: Option<String>,
}

//TODO: make defaults for these part of application and project configuration file
/// Style information for linear items
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LineStyle {
    color: Option<Color>,
    secondary_color: Option<Color>,
    line_thickness: Option<Length>,
    line_thickness_unit: Option<String>,
    /// array of lengths/percentages of dashes and gaps
    /// uses same specification as SVG stroke-dasharray field.
    line_appearance: Option<Vec<u64>>,
}
