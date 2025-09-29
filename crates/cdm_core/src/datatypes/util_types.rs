use num_rational::Rational64;
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
#[expect(clippy::partial_pub_fields)]
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
    pub color: Option<Color>,
    pub secondary_color: Option<Color>,
    pub line_thickness: Option<Length>,
    pub line_thickness_unit: Option<String>,
    /// array of lengths/percentages of dashes and gaps
    /// uses same specification as SVG stroke-dasharray field.
    pub line_appearance: Option<Vec<u64>>,
}
/// Custom fields for user specified data. Not parsed
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UserFields {
    pub user0: Option<String>,
    pub user1: Option<String>,
    pub user2: Option<String>,
    pub user3: Option<String>,
    pub user4: Option<String>,
    pub user5: Option<String>,
    pub user6: Option<String>,
    pub user7: Option<String>,
    pub user8: Option<String>,
    pub user9: Option<String>,
}

/// Fields to support IEC coding of assets
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IECCodes {
    pub location: Option<String>,
    pub installation: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PhysicalLocation {
    street_address: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zip_code: Option<String>,
    latitude: Rational64,
    longitude: Rational64,
    structured_location_id: Option<String>,
    planet: Option<String>,
    building: Option<String>,
}
