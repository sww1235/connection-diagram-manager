use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{
    datatypes::{
        library_types::{Library, cable_type, cable_type::CableLayer},
        project_types::{Project, ProjectData, wire::Wire},
        schematic_connector::{AsConnector, right_angle::RightAngle},
        unit_helper::length::Length,
        util_types::{IECCodes, LineStyle, PhysicalLocation, UserFields},
    },
    error::{Error, GUIRenderingError, LibraryError},
    traits::FromFile,
};

/// `Cable` represents a particular instance of a `CableType`
/// It represents a physical item.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::partial_pub_fields, reason = "contained_datafile_path is not part of public API")]
pub struct Cable {
    /// The `CableType` key of this instance.
    pub cable_type: String,
    /// The structured name of the `Cable` instance.
    pub identifier: String,
    /// Optional description.
    pub description: Option<String>,
    /// length of wire or cable.
    pub length: Length,
    /// Pathway key containing instance.
    pub pathway: Option<String>,
    /// physical location of Cable.
    pub physical_location: Option<PhysicalLocation>,
    /// Fields for use with IEC project coding.
    pub iec_codes: Option<IECCodes>,
    /// User defined fields.
    pub user_fields: Option<UserFields>,
    /// The cores in this cable. Generated from the data in the associcated `CableType`.
    ///
    /// Key of map is identifier of core within cable, and is unique within each cable.
    #[serde(skip)]
    pub(crate) cores: BTreeMap<String, CableCore>,
    /// The `LineStyle` of this cable. Initially copied from the `CableType`.
    #[serde(skip)]
    pub(crate) line_style: LineStyle,
    /// vector of exterior insulation/shielding layers. Copied from the `CableType`.
    #[serde(skip)]
    pub(crate) layers: Vec<CableLayer>,
    /// datafile the struct instance was read in from.
    #[serde(skip)]
    pub(crate) contained_datafile_path: PathBuf,
}

#[derive(Debug, PartialEq, Clone)]
#[expect(clippy::exhaustive_enums, reason = "only two options make sense")]
pub(crate) enum CableCore {
    /// `Wire`
    Wire(Wire),
    /// `Cable`
    Cable(Cable),
}

impl AsConnector for Cable {
    type Output = RightAngle;
    #[inline]
    fn as_connector(&self, id: String, project_data: &Project) -> Result<RightAngle, GUIRenderingError> {

    }

    #[inline]
    fn update_data_from_library(&mut self, library: &Library) -> Result<(), Error> {
        let cable_type = library.cable_types.get(&self.cable_type).ok_or(LibraryError::ValueNotFound {
            id: self.cable_type.clone(),
            found_in: format!("cable instance {}", self.identifier).to_owned(),
            library_type: "Cable Type".to_owned(),
        })?;

        self.line_style = cable_type.line_style.clone();

        self.layers = cable_type.layers.clone();

        self.cores = BTreeMap::new();

        self.insert_cores(&cable_type.cores, library)?;
        Ok(())
    }
}

impl Cable {
    /// `insert_cores` handles the creation of all the individual wires/cables inside a cable and
    /// its cores.
    ///
    /// It is a recursive function.
    fn insert_cores(
        &mut self,
        reference_cores: &BTreeMap<String, cable_type::CableCore>,
        library: &Library,
    ) -> Result<(), LibraryError> {
        for (id, core) in reference_cores {
            match core {
                cable_type::CableCore::WireType { type_id, line_style } => {
                    let core_type = library.wire_types.get(type_id).ok_or(LibraryError::ValueNotFound {
                        id: type_id.to_owned(),
                        found_in: format!("wire instance {}", self.identifier).to_owned(),
                        library_type: "Wire Type".to_owned(),
                    })?;

                    self.cores.insert(
                        id.to_owned(),
                        CableCore::Wire(Wire {
                            wire_type: type_id.to_owned(),
                            identifier: format!("{}.{}", self.identifier, id),
                            description: None,
                            length: self.length.clone(),
                            physical_location: self.physical_location.clone(),
                            iec_codes: None,
                            user_fields: None,
                            pathway: None,
                            end1_connector_type: None,
                            end2_connector_type: None,
                            line_style: line_style.clone().unwrap_or(core_type.line_style.clone()),
                            contained_datafile_path: self.contained_datafile_path.clone(),
                        }),
                    );
                }
                cable_type::CableCore::CableType { type_id, line_style } => {
                    let core_type = library.cable_types.get(type_id).ok_or(LibraryError::ValueNotFound {
                        id: type_id.to_owned(),
                        found_in: format!("cable instance {}", self.identifier).to_owned(),
                        library_type: "Cable Type".to_owned(),
                    })?;

                    let mut cable = Cable {
                        cable_type: type_id.to_owned(),
                        identifier: format!("{}.{}", self.identifier, id),
                        description: None,
                        length: self.length.clone(),
                        physical_location: self.physical_location.clone(),
                        iec_codes: None,
                        user_fields: None,
                        pathway: None,
                        cores: BTreeMap::new(),
                        layers: core_type.layers.clone(),
                        line_style: line_style.clone().unwrap_or(core_type.line_style.clone()),
                        contained_datafile_path: self.contained_datafile_path.clone(),
                    };

                    // recursive call to handle inner cables
                    cable.insert_cores(&core_type.cores, library)?;

                    self.cores.insert(id.to_owned(), CableCore::Cable(cable));
                }
            }
        }

        Ok(())
    }
}

impl FromFile for Cable {
    #[inline]
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
    #[inline]
    fn set_datafile(&mut self, datafile_path: &Path) {
        self.contained_datafile_path = datafile_path.to_path_buf();
    }
}

impl ProjectData for Cable {}
