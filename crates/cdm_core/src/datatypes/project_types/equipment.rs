use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    datatypes::util_types::{IECCodes, PhysicalLocation, UserFields},
    traits::FromFile,
};

/// `Equipment` represents a particular instance of an `EquipmentType`.
/// This is the physical unit you would hold in your hand
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::partial_pub_fields)]
pub struct Equipment {
    /// The type of equipment of the instance
    pub equipment_type: String,
    /// The structured name of the equipment
    pub identifier: Option<String>,
    /// The particular mounting type of this instance
    /// must be in list of mounting types defined in `equip_type.mounting_type`.
    /// Validated on import
    pub mounting_type: Option<String>,
    /// The containing `Enclosure`
    pub enclosure: Option<String>,
    /// The ID of the `MountPoint` within the `Enclosure`
    pub mount_point: Option<String>,
    /// The physical location of this piece of equipment
    pub physical_location: Option<PhysicalLocation>,
    /// fields for IEC coding
    pub iec_codes: Option<IECCodes>,
    /// Description
    pub description: Option<String>,
    /// Optional user Fields
    pub user_fields: Option<UserFields>,
    /// datafile the struct instance was read in from
    #[serde(skip)]
    pub(crate) contained_datafile_path: PathBuf,
}

impl FromFile for Equipment {
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
    fn set_datafile(&mut self, datafile_path: &Path) {
        self.contained_datafile_path = datafile_path.to_path_buf();
    }
}
