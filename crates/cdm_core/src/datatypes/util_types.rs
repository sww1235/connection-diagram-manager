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
    /// A cable consisting of 2 wires/cables bonded to each other in a figure 8 layout inside the
    /// same external jacket.
    Figure8,
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
    /// width of connector
    pub width: Length,
    /// depth of connector
    pub depth: Option<Length>,
    /// diameter of circular connectors
    pub diameter: Option<Length>,
}

//TODO: make defaults for these part of application and project configuration file
/// Style information for linear items
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LineStyle {
    /// Primary `Color` of line
    pub color: Option<Color>,
    /// Secondary `Color` of line
    pub secondary_color: Option<Color>,
    /// Thickness or width of line
    pub line_thickness: Option<Length>,
    /// array of lengths/percentages of dashes and gaps
    /// uses same specification as SVG stroke-dasharray field.
    pub line_appearance: Option<Vec<u64>>,
}
/// Custom fields for user specified data. Not parsed
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[expect(missing_docs)]
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
    /// Location code for IEC Coding
    pub location: Option<String>,
    /// Installation code for IEC coding
    pub installation: Option<String>,
}

/// Representation of a physical location on a particular planet
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PhysicalLocation {
    /// Most specific part of address. Should include things like apartment number etc.
    street_address: Option<String>,
    /// City or town
    city: Option<String>,
    /// state or province
    state: Option<String>,
    /// machine readable code for mail sorting area
    zip_code: Option<String>,
    /// Latitude represented as a Rational64 value to avoid float loss of precision issues
    latitude: Rational64,
    /// Longitude represented as a Rational64 value to avoid float loss of precision issues
    longitude: Rational64,
    /// A user specified structured identifier
    structured_location_id: Option<String>,
    /// Name of the country in which this physical address resides
    country: Option<String>,
    /// Name of planet in which this physical address resides
    planet: Option<String>,
    /// Name or description of building in which this physical address resides.
    building: Option<String>,
}
