use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{
    datatypes::{
        unit_helper::length::Length,
        util_types::{IECCodes, PhysicalLocation, UserFields},
    },
    traits::{FromFile, ProjectData},
};

/// `Location` represents a physical instance of a locationType
/// TODO: add page/sheet number for pdf generation and printing
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::partial_pub_fields)]
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
    pub mount_points: BTreeMap<String, MountPoint>,
    /// datafile the struct instance was read in from
    #[serde(skip)]
    pub(crate) contained_datafile_path: PathBuf,
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
#[non_exhaustive]
pub enum MountPoint {
    /// Coordinate pair represents a x,y coordinate pair on a backplane or enclosure
    CoordinatePair {
        /// Distance from left side of parent location
        x: Length,
        /// Distance from bottom of parent location
        y: Length,
    },
    /// `MountingRail` represents both an instance of a mounting rail within an enclosure, and the
    /// location of the equipment mounted on it.
    MountingRail {
        /// Distance from left side of parent location to left end of Mounting Rail
        x: Length,
        /// Distance from bottom of parent location to horizontal center of Mounting Rail
        y: Length,
        /// mounting rail
        mounting_rail: String,
        /// Distance from left side of location
        distance: Length,
    },
    //TODO: slot panel and rack panel
}

impl FromFile for Enclosure {
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
    fn set_datafile(&mut self, datafile_path: &Path) {
        self.contained_datafile_path = datafile_path.to_path_buf();
    }
}

impl ProjectData for Enclosure {}
