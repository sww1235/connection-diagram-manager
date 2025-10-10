use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::datatypes::{
    unit_helper::Length,
    util_types::{IECCodes, PhysicalLocation, UserFields},
};

/// `Location` represents a physical instance of a locationType
/// TODO: add page/sheet number for pdf generation and printing
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Enclosure {
    /// Type of location
    pub enclosure_type: String,
    /// structured identifier of location
    pub identifier: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// physical location of Pathway
    pub physical_location: Option<PhysicalLocation>,
    /// Fields for use with IEC project coding
    pub iec_codes: Option<IECCodes>,
    /// User defined fields
    pub user_fields: Option<UserFields>,
    /// `mount_point` - Actual locations of associated equipment within location
    pub mount_points: HashMap<String, MountPoint>,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}
/// `MountPoint` represents a particular physical x/y/z within an `Enclosure`
///
/// Examples of `MountPoint`s include
/// - coordinate pairs on a backplane
/// - Individual DIN rails on a backplane, and then distance along DIN rail //TODO fix this
/// - Individual keystone slots on a panel
/// - Rack units/Sub rack units within a panel

//TODO: add mounting rails?
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MountPoint {
    /// Optional mounting rail
    pub mounting_rail: Option<String>,
    /// Distance from left side of parent location
    pub x: Length,
    /// Distance from bottom of parent location
    pub y: Length,
    /// Distance from left side of location
    pub distance: Length,
}
