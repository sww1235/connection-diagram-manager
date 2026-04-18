use core::cmp::Ordering;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use log::{trace, warn};
use serde::{Deserialize, Serialize};

use crate::{
    datatypes::{
        library_types::{Library, cable_type, cable_type::CableLayer},
        project_types::{
            Project,
            ProjectData,
            connection::{Connection, Type as ConnectionType},
            wire::Wire,
        },
        schematic_connector::{
            AsConnector,
            ConnectionPoint,
            ConnectorType,
            multi_right_angle::MultiRightAngle,
            right_angle::RightAngle,
        },
        unit_helper::length::Length,
        util_types::{IECCodes, LineStyle, PhysicalLocation, UserFields},
    },
    error::{CableTypeError, Error, GUIRenderingError, LibraryError},
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

/// `CableCore` represents a core of a cable, which can either be a `Wire` or another `Cable`.
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum CableCore {
    /// `Wire`.
    Wire(Wire),
    /// `Cable`.
    Cable(Cable),
}

impl AsConnector for Cable {
    type Output = MultiRightAngle;
    #[inline]
    fn as_connector(&self, id: String, project_data: &Project) -> Result<MultiRightAngle, GUIRenderingError> {
        let mut end1_junction: ConnectionPoint = ConnectionPoint::default();
        let mut end2_junction: ConnectionPoint = ConnectionPoint::default();
        let mut end1_connections: Vec<ConnectorType> = Vec::new();
        let mut end2_connections: Vec<ConnectorType> = Vec::new();

        let mut end1_cable_connections: Vec<Connection> = Vec::new();
        let mut end2_cable_connections: Vec<Connection> = Vec::new();

        for connection in &project_data.connections {
            if connection.end1 == connection.end2 {
                warn! {"connection: {connection:?} has both ends assigned to the same entity."};
                continue;
            }
            //TODO: rethink this
            if let ConnectionType::Cable { cable_id, core_id } = &connection.end1
                && cable_id == &id
                && self.cores.contains_key(core_id)
            {
                end1_cable_connections.push(connection.clone());
            }
            if let ConnectionType::Cable { cable_id, core_id } = &connection.end2
                && cable_id == &id
                && self.cores.contains_key(core_id)
            {
                end2_cable_connections.push(connection.clone());
            }
        }

        Ok(MultiRightAngle::new(
            end1_junction,
            end1_connections,
            end2_junction,
            end2_connections,
            false,
            self.line_style.clone(),
        ))
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

        self.insert_cores(&cable_type.cores, library, None)?;
        Ok(())
    }
}

//TODO: provide a way to print core_ids for a cable
impl Cable {
    /// `insert_cores` handles the creation of all the individual wires/cables inside a cable and
    /// its cores.
    ///
    /// It is a recursive function.
    ///
    /// `super_id` passes through the core ID of the previous iteration so it gets concatenated
    /// correctly.
    #[expect(clippy::needless_pass_by_value, reason = "need to use unwrap_or_default()")]
    fn insert_cores(
        &mut self,
        reference_cores: &BTreeMap<String, cable_type::CableCore>,
        library: &Library,
        super_id: Option<String>,
    ) -> Result<(), LibraryError> {
        for (id, core) in reference_cores {
            match core {
                cable_type::CableCore::WireType { type_id, line_style } => {
                    let core_type = library.wire_types.get(type_id).ok_or(LibraryError::ValueNotFound {
                        id: type_id.to_owned(),
                        found_in: format!("wire instance {}", self.identifier).to_owned(),
                        library_type: "Wire Type".to_owned(),
                    })?;

                    trace! {"inserting wire core {id} into {}", self.identifier}
                    // migrate to trim_prefix() once stablized
                    // https://github.com/rust-lang/rust/issues/142312
                    let new_core_id = format!("{}.{id}", super_id.clone().unwrap_or_default());
                    #[expect(clippy::shadow_reuse, reason = "just stripping a prefix")]
                    let new_core_id = new_core_id.strip_prefix('.').unwrap_or(&new_core_id).to_owned();

                    self.cores.insert(
                        new_core_id,
                        CableCore::Wire(Wire {
                            wire_type: type_id.to_owned(),
                            identifier: format!("{}.{id}", super_id.clone().unwrap_or_default()),
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

                    trace! {"creating cable for cable core {id} of {}", self.identifier}
                    let mut cable = Cable {
                        cable_type: type_id.to_owned(),
                        identifier: format!("{}.{id}", super_id.clone().unwrap_or_default()),
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
                    match core_type.cores.len().cmp(&1) {
                        Ordering::Greater => {
                            // recursive call to handle inner cables
                            cable.insert_cores(&core_type.cores, library, super_id.clone())?;
                            // migrate to trim_prefix() once stablized
                            // https://github.com/rust-lang/rust/issues/142312
                            let new_core_id = format!("{}.{id}", super_id.clone().unwrap_or_default());
                            #[expect(clippy::shadow_reuse, reason = "just stripping a prefix")]
                            let new_core_id = new_core_id.strip_prefix('.').unwrap_or(&new_core_id).to_owned();
                            self.cores.insert(new_core_id, CableCore::Cable(cable));
                        }
                        Ordering::Equal => {
                            self.cores.insert(
                                format!("{}.{id}", super_id.clone().unwrap_or_default()),
                                CableCore::Cable(cable),
                            );
                        }
                        Ordering::Less => {
                            return Err(CableTypeError::NoCores(type_id.to_owned()).into());
                        }
                    }
                }
            }
        }

        if self.cores.is_empty() {
            return Err(CableTypeError::NoCores(self.cable_type.clone()).into());
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
