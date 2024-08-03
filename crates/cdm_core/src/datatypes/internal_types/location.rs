use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::rc::Rc;

use dimensioned::ucum;

use cdm_macros::{Empty, Merge, PartialEmpty};

use super::location_type::LocationType;

/// `Location` represents a physical instance of a locationType
#[derive(Debug, Default, PartialEq, Merge, PartialEmpty, Empty)]
pub struct Location {
    /// Internal `id` of location instance
    pub id: String,
    /// Type of location
    pub location_type: Rc<RefCell<LocationType>>,
    /// structured identifier of location
    pub identifier: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// Physical Location
    pub physical_location: Option<String>,
    /// `SubLocation` - Actual locations of associated equipment within location
    /// Hashmap enforces unique keys
    pub sub_locations: HashMap<String, SubLocation>,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}
impl Location {
    /// Creates an empty instance of `Location`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
/// `SubLocation` represents a particular physical x/y/z within a `Location`
///
/// Examples of `SubLocations` include
/// - coordinate pairs on a backplane
/// - Individual DIN rails on a backplane, and then distance along DIN rail //TODO fix this
/// - Individual keystone slots on a panel
/// - Rack units/Sub rack units within a panel

// TODO: Finish this. Should equipment be assigned to a sublocation vs a location, or both, or
// neither? Should sublocations be the same thing as a location?
#[derive(Debug, Clone, Default, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub struct SubLocation {
    /// Distance from left side of parent location
    pub x: ucum::Meter<f64>, //TODO: Units?
    /// Distance from bottom of parent location
    pub y: ucum::Meter<f64>,
    /// Distance from back of parent location
    pub z: ucum::Meter<f64>,
}

impl SubLocation {
    /// Crates an empty instance of `SubLocation`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Location Instance:")?;
        writeln!(f, "Internal ID: {}", &self.id)?;
        if let Some(manufacturer) = &self.location_type.borrow().manufacturer {
            writeln!(f, "Manufacturer: {manufacturer}")?;
        }
        //TODO: Decide how much data from Equiptype we want to display for instance
        if let Some(model) = &self.location_type.borrow().model {
            writeln!(f, "Model: {model}")?;
        }
        if let Some(part_number) = &self.location_type.borrow().part_number {
            writeln!(f, "Part Number: {part_number}")?;
        }
        if let Some(manufacturer_part_number) =
            &self.location_type.borrow().manufacturer_part_number
        {
            writeln!(f, "Manufacturer Part Number: {manufacturer_part_number}")?;
        }
        if let Some(supplier) = &self.location_type.borrow().supplier {
            writeln!(f, "Supplier: {supplier}")?;
        }
        if let Some(supplier_part_number) = &self.location_type.borrow().supplier_part_number {
            writeln!(f, "Supplier Part Number: {supplier_part_number}")?;
        }
        if let Some(identifier) = &self.identifier {
            writeln!(f, "Location Identifier: {identifier}")?;
        }
        if let Some(physical_location) = &self.physical_location {
            writeln!(f, "Physical Location of location: {physical_location}")?;
        }
        if let Some(description) = &self.description {
            writeln!(f, "Description: {description}")?;
        }
        Ok(())
    }
}

impl fmt::Display for SubLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)?;

        Ok(())
    }
}
