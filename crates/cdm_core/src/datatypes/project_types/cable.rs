use core::cmp::Ordering;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use egui::Pos2;
use log::trace;
use slotmap::Key;

use crate::{
    datatypes::{
        file_types,
        library_types::{Library, cable_type, cable_type::CableLayer},
        project_types::{InnerConnectionId, Project, ProjectData},
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
    error::{CableTypeError, Error, LibraryError},
    traits::FromFile,
};

/// `Cable` represents a particular instance of a `CableType`
/// It represents a physical item.
#[derive(Debug, PartialEq, Clone)]
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
    pub(crate) cores: BTreeMap<String, Core>,
    //TODO: rename connector and connectortype to something more distinct
    /// The schematic representation of this cable.
    pub(crate) connector: ConnectorType,
    /// The `LineStyle` of this cable. Initially copied from the `CableType`.
    pub(crate) line_style: LineStyle,
    /// vector of exterior insulation/shielding layers. Copied from the `CableType`.
    pub(crate) layers: Vec<CableLayer>,
    /// datafile the struct instance was read in from.
    pub(crate) contained_datafile_path: PathBuf,
}

impl From<file_types::cable::Cable> for Cable {
    #[inline]
    fn from(value: file_types::cable::Cable) -> Self {
        Self {
            cable_type: value.cable_type,
            identifier: value.identifier,
            description: value.description,
            length: value.length,
            pathway: value.pathway,
            physical_location: value.physical_location,
            iec_codes: value.iec_codes,
            user_fields: value.user_fields,
            cores: BTreeMap::new(),
            connector: ConnectorType::MultiRightAngle(MultiRightAngle::default()),
            line_style: LineStyle::default(),
            layers: Vec::new(),
            contained_datafile_path: PathBuf::new(),
        }
    }
}

//TODO: may not need InnerConnectionId here

/// `CableCore` represents a core of a cable.
#[derive(Debug, PartialEq, Clone)]
#[expect(clippy::exhaustive_enums, reason = "wrapper enum")]
pub enum Core {
    /// `Cable`.
    Cable { cable: Cable, connection_id: InnerConnectionId },
}

impl AsConnector for Cable {
    type Output = MultiRightAngle;

    #[inline]
    fn connector(&self) -> Self::Output {
        #[expect(
            clippy::wildcard_enum_match_arm,
            reason = "Only one option implemented at this point, but others may be in the future."
        )]
        match &self.connector {
            ConnectorType::MultiRightAngle(mra) => mra.clone(),
            #[expect(clippy::panic, reason = "The wildcard arm of the match should never happen currently")]
            _ => panic!(),
        }
    }

    #[inline]
    fn connector_mut(&mut self) -> &mut Self::Output {
        #[expect(
            clippy::wildcard_enum_match_arm,
            reason = "Only one option implemented at this point, but others may be in the future."
        )]
        match &mut self.connector {
            ConnectorType::MultiRightAngle(mra) => mra,
            #[expect(clippy::panic, reason = "The wildcard arm of the match should never happen currently")]
            _ => panic!(),
        }
    }

    #[inline]
    fn set_end1_position(&mut self, position: Pos2) {
        #[expect(
            clippy::wildcard_enum_match_arm,
            reason = "Only one option implemented at this point, but others may be in the future."
        )]
        match &mut self.connector {
            ConnectorType::MultiRightAngle(mra) => mra.end1_junction.set_position(position),
            #[expect(clippy::panic, reason = "The wildcard arm of the match should never happen currently")]
            _ => panic!(),
        }
    }
    #[inline]
    fn set_end2_position(&mut self, position: Pos2) {
        #[expect(
            clippy::wildcard_enum_match_arm,
            reason = "Only one option implemented at this point, but others may be in the future."
        )]
        match &mut self.connector {
            ConnectorType::MultiRightAngle(mra) => mra.end2_junction.set_position(position),
            #[expect(clippy::panic, reason = "The wildcard arm of the match should never happen currently")]
            _ => panic!(),
        }
    }
    #[inline]
    fn end1_position(&self) -> Pos2 {
        #[expect(
            clippy::wildcard_enum_match_arm,
            reason = "Only one option implemented at this point, but others may be in the future."
        )]
        match &self.connector {
            ConnectorType::MultiRightAngle(mra) => mra.end1_junction.position(),
            #[expect(clippy::panic, reason = "The wildcard arm of the match should never happen currently")]
            _ => panic!(),
        }
    }
    #[inline]
    fn end2_position(&self) -> Pos2 {
        #[expect(
            clippy::wildcard_enum_match_arm,
            reason = "Only one option implemented at this point, but others may be in the future."
        )]
        match &self.connector {
            ConnectorType::MultiRightAngle(mra) => mra.end2_junction.position(),
            #[expect(clippy::panic, reason = "The wildcard arm of the match should never happen currently")]
            _ => panic!(),
        }
    }

    //#[inline]
    //#[expect(clippy::too_many_lines, reason = "it has lines")]
    //fn as_connector(&self, id: String, project_data: &Project) -> Result<MultiRightAngle, GUIRenderingError> {
    //    let end1_junction: ConnectionPoint = ConnectionPoint::default();
    //    let end2_junction: ConnectionPoint = ConnectionPoint::default();
    //    let mut end1_connections: Vec<ConnectorType> = Vec::new();
    //    let mut end2_connections: Vec<ConnectorType> = Vec::new();

    //    let mut end1_cable_connections: Vec<Connection> = Vec::new();
    //    let mut end2_cable_connections: Vec<Connection> = Vec::new();

    //    for connection in &project_data.connections {
    //        if connection.end1 == connection.end2 {
    //            warn! {"connection: {connection:?} has both ends assigned to the same entity."};
    //            continue;
    //        }
    //        //TODO: rethink this
    //        if let ConnectionType::Cable { cable_id, core_id } = &connection.end1
    //            && cable_id == &id
    //            && self.cores.contains_key(core_id)
    //        {
    //            end1_cable_connections.push(connection.clone());
    //        }
    //        if let ConnectionType::Cable { cable_id, core_id } = &connection.end2
    //            && cable_id == &id
    //            && self.cores.contains_key(core_id)
    //        {
    //            end2_cable_connections.push(connection.clone());
    //        }
    //    }

    //    // These are only the cores in this current cable, not the outer cable.
    //    for (core_id, core) in &self.cores {
    //        match core {
    //            CableCore::Wire(wire) => {
    //                // we already know that end1_cable_connections, contains connections with
    //                // reference to cable in end1
    //                for connection in &end1_cable_connections {
    //                    #[expect(
    //                        clippy::match_same_arms,
    //                        reason = "Separating out some match elements makes the code read clearer"
    //                    )]
    //                    #[expect(clippy::wildcard_enum_match_arm, reason = "code still in development")]
    //                    match &connection.end2 {
    //                        ConnectionType::Wire { wire_id } if wire_id == core_id => {
    //                            //Do nothing.
    //                        }
    //                        // TODO: wire-wire, wire-cable and wire-term_cable connections
    //                        // not defined yet.
    //                        ConnectionType::Wire { .. } | ConnectionType::Cable { .. } |
    // ConnectionType::TermCable { .. } => {                            //todo!();
    //                        }

    //                        #[expect(clippy::arithmetic_side_effects, reason = "deal with it")]
    //                        ConnectionType::Equipment {
    //                            equipment_id,
    //                            connection_point_id,
    //                        } => {
    //                            let mut end: ConnectionPoint = ConnectionPoint::default();

    //                            trace! {"end equipment: {equipment_id}"};
    //                            #[expect(clippy::get_unwrap, reason = "temporary for testing")]
    //                            #[expect(clippy::unwrap_used, reason = "temporary for testing")]
    //                            let equipment = project_data.equipment.get(equipment_id).unwrap();
    //                            let symbol = equipment.schematic_symbol();
    //                            #[expect(clippy::get_unwrap, reason = "temporary for testing")]
    //                            #[expect(clippy::unwrap_used, reason = "temporary for testing")]
    //                            let connection_point = symbol.connections.get(connection_point_id).unwrap();
    //                            trace!(
    //                                "allowed_connection_directions: {:?}",
    //                                connection_point.allowed_connection_directions
    //                            );
    //                            end.directions = connection_point.allowed_connection_directions.clone();

    //                            //TODO: move this to a method on SchematicSymbol
    //                            //
    //                            //TODO: change connection point to contain a Pos2?
    //                            let connection_point_offset = Vec2::from((
    //                                symbol.scaled_size().x * (connection_point.x / 100.0),
    //                                symbol.scaled_size().y * (connection_point.y / 100.0),
    //                            ));
    //                            trace! {"symbol scaled_size: {}", symbol.scaled_size()};
    //                            trace! {"end connection_point: {connection_point:?}"};
    //                            trace! {"end connection_point_offset: {connection_point_offset}"};
    //                            end.position = symbol.position
    //                                + Vec2::from(( symbol.scaled_size().x * connection_point.x / 100.0,
    //                                  symbol.scaled_size().y * connection_point.y / 100.0,
    //                                ));
    //                            end1_connections.push(ConnectorType::RightAngle(RightAngle::new(
    //                                end1_junction.clone(),
    //                                end,
    //                                false,
    //                                self.line_style.clone(),
    //                            )));
    //                        }
    //                        _ => {}
    //                    }
    //                }
    //                for connection in &end2_cable_connections {
    //                    #[expect(
    //                        clippy::match_same_arms,
    //                        reason = "Separating out some match elements makes the code read clearer"
    //                    )]
    //                    #[expect(clippy::wildcard_enum_match_arm, reason = "code still in development")]
    //                    match &connection.end1 {
    //                        ConnectionType::Wire { wire_id } if wire_id == core_id => {
    //                            //Do nothing.
    //                        }
    //                        // TODO: wire-wire, wire-cable and wire-term_cable connections
    //                        // not defined yet.
    //                        ConnectionType::Wire { .. } | ConnectionType::Cable { .. } |
    // ConnectionType::TermCable { .. } => {                            //todo!();
    //                        }

    //                        #[expect(clippy::arithmetic_side_effects, reason = "deal with it")]
    //                        ConnectionType::Equipment {
    //                            equipment_id,
    //                            connection_point_id,
    //                        } => {
    //                            let mut end: ConnectionPoint = ConnectionPoint::default();

    //                            trace! {"end equipment: {equipment_id}"};
    //                            #[expect(clippy::get_unwrap, reason = "temporary for testing")]
    //                            #[expect(clippy::unwrap_used, reason = "temporary for testing")]
    //                            let equipment = project_data.equipment.get(equipment_id).unwrap();
    //                            let symbol = equipment.schematic_symbol();
    //                            #[expect(clippy::get_unwrap, reason = "temporary for testing")]
    //                            #[expect(clippy::unwrap_used, reason = "temporary for testing")]
    //                            let connection_point = symbol.connections.get(connection_point_id).unwrap();
    //                            trace!(
    //                                "allowed_connection_directions: {:?}",
    //                                connection_point.allowed_connection_directions
    //                            );
    //                            end.directions = connection_point.allowed_connection_directions.clone();

    //                            //TODO: move this to a method on SchematicSymbol
    //                            //
    //                            //TODO: change connection point to contain a Pos2?
    //                            let connection_point_offset = Vec2::from((
    //                                symbol.scaled_size().x * (connection_point.x / 100.0),
    //                                symbol.scaled_size().y * (connection_point.y / 100.0),
    //                            ));
    //                            trace! {"symbol scaled_size: {}", symbol.scaled_size()};
    //                            trace! {"end connection_point: {connection_point:?}"};
    //                            trace! {"end connection_point_offset: {connection_point_offset}"};
    //                            end.position = symbol.position
    //                                + Vec2::from(( symbol.scaled_size().x * connection_point.x / 100.0,
    //                                  symbol.scaled_size().y * connection_point.y / 100.0,
    //                                ));
    //                            end2_connections.push(ConnectorType::RightAngle(RightAngle::new(
    //                                end2_junction.clone(),
    //                                end,
    //                                false,
    //                                self.line_style.clone(),
    //                            )));
    //                        }
    //                        _ => {}
    //                    }
    //                }
    //            }

    //            CableCore::Cable(cable) => {
    //                //match cable.cores.len().cmp(&1) {
    //                //    Ordering::Greater => {
    //                //        // recursive call to handle inner cables

    //                //        // migrate to trim_prefix() once stablized
    //                //        // https://github.com/rust-lang/rust/issues/142312
    //                //    }
    //                //    Ordering::Equal => {
    //                //        self.cores.insert(
    //                //            format!("{}.{id}", super_id.clone().unwrap_or_default()),
    //                //            CableCore::Cable(cable),
    //                //        );
    //                //    }
    //                //    Ordering::Less => {
    //                //        return Err(CableTypeError::NoCores(type_id.to_owned()).into());
    //                //    }
    //                //}
    //            }
    //        }
    //    }

    //    Ok(MultiRightAngle::new(
    //        end1_junction,
    //        end1_connections,
    //        end2_junction,
    //        end2_connections,
    //        false,
    //        self.line_style.clone(),
    //    ))
    //}

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

        match self.cores.len().cmp(&1) {
            Ordering::Equal => {
                self.connector = ConnectorType::RightAngle(RightAngle::new(
                    ConnectionPoint::default(),
                    ConnectionPoint::default(),
                    false,
                    self.line_style.clone(),
                ));
            }
            Ordering::Less => {
                return Err(LibraryError::from(CableTypeError::NoCores(self.cable_type.clone())).into());
            }
            Ordering::Greater => {
                self.connector = ConnectorType::MultiRightAngle(MultiRightAngle::new(
                    ConnectionPoint::default(),
                    Vec::new(),
                    ConnectionPoint::default(),
                    Vec::new(),
                    false,
                    self.line_style.clone(),
                ));
            }
        }

        Ok(())
    }
    //TODO: add method to update connection reference ids from Project.
}

//TODO: provide a way to print core_ids for a cable
impl Cable {
    /// `insert_cores` handles the creation of all the individual cables inside a cable and
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
            let core_type = library.cable_types.get(&core.type_id).ok_or(LibraryError::ValueNotFound {
                id: core.type_id.clone(),
                found_in: format!("cable instance {}", self.identifier).to_owned(),
                library_type: "Cable Type".to_owned(),
            })?;

            trace! {"creating cable for cable core {id} of {}", self.identifier}
            let mut cable = Cable {
                cable_type: core.type_id.clone(),
                identifier: format!("{}.{id}", super_id.clone().unwrap_or_default()),
                description: None,
                length: self.length.clone(),
                physical_location: self.physical_location.clone(),
                iec_codes: None,
                user_fields: None,
                pathway: None,
                cores: BTreeMap::new(),
                layers: core_type.layers.clone(),
                line_style: core.line_style.clone().unwrap_or(core_type.line_style.clone()),
                connector: ConnectorType::MultiRightAngle(MultiRightAngle::default()),
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
                    self.cores.insert(
                        new_core_id,
                        Core::Cable {
                            cable,
                            connection_id: InnerConnectionId::null(),
                        },
                    );
                    self.connector = ConnectorType::MultiRightAngle(MultiRightAngle::new(
                        ConnectionPoint::default(),
                        Vec::new(),
                        ConnectionPoint::default(),
                        Vec::new(),
                        false,
                        self.line_style.clone(),
                    ));
                }
                Ordering::Equal => {
                    self.cores.insert(
                        format!("{}.{id}", super_id.clone().unwrap_or_default()),
                        Core::Cable {
                            cable,
                            connection_id: InnerConnectionId::null(),
                        },
                    );
                    self.connector = ConnectorType::RightAngle(RightAngle::new(
                        ConnectionPoint::default(),
                        ConnectionPoint::default(),
                        false,
                        self.line_style.clone(),
                    ));
                }
                Ordering::Less => {
                    return Err(CableTypeError::NoCores(core.type_id.clone()).into());
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
