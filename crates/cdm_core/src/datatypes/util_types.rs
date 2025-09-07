use std::fmt;

use serde::{Deserialize, Serialize};

/// Cross section of wire or cable
#[non_exhaustive]
#[derive(Debug, Default, PartialEq, Clone)]
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

impl fmt::Display for CrossSection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CrossSection::Oval => write! {f, "Oval"},
            CrossSection::Circular => write! {f, "Circular"},
            CrossSection::Siamese => write! {f, "Siamese"},
        }
    }
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
