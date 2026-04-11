use core::cmp::Ordering;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use egui::{Color32, Pos2, Stroke, Vec2};
use log::{trace, warn};
use serde::{Deserialize, Serialize};

use crate::{
    datatypes::{
        library_types::Library,
        project_types::{
            Project,
            ProjectData,
            connection::{Connection, Type as ConnectionType},
        },
        schematic_connector::{AsConnector, right_angle::RightAngle},
        schematic_symbol::{ConnectionDirection, SchematicRepresentation as _},
        unit_helper::length::Length,
        util_types::{IECCodes, LineStyle, PhysicalLocation, UserFields},
    },
    error::{Error, GUIRenderingError, LibraryError},
    traits::FromFile,
};

/// `Wire` represents a particular instance of a `WireType`.
/// It represents a physical item.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::partial_pub_fields, reason = "contained_datafile_path is not part of public API")]
pub struct Wire {
    /// The `WireType` of this instance.
    pub wire_type: String,
    /// The structured name of the `Wire` instance. This can be used as a wire number or other
    /// identifier.
    pub identifier: String,
    /// Optional description.
    pub description: Option<String>,
    /// length of wire.
    pub length: Length,
    /// physical location of Wire.
    pub physical_location: Option<PhysicalLocation>,
    /// Fields for use with IEC project coding.
    pub iec_codes: Option<IECCodes>,
    /// User defined fields.
    pub user_fields: Option<UserFields>,
    /// Pathway containing instance.
    pub pathway: Option<String>,
    /// One end of `Wire` / Cable.
    pub end1_connector_type: Option<String>,
    /// The other end of `Wire`.
    pub end2_connector_type: Option<String>,
    /// Styling info for the connector that represents this wire.
    #[serde(skip)]
    line_style: LineStyle,
    /// datafile the struct instance was read in from.
    #[serde(skip)]
    pub(crate) contained_datafile_path: PathBuf,
}

impl AsConnector for Wire {
    type Output = RightAngle;
    #[inline]
    fn as_connector(&self, id: String, project_data: &Project) -> Result<RightAngle, GUIRenderingError> {
        let mut end1: (Pos2, HashSet<ConnectionDirection>) = (Pos2::ZERO, HashSet::from([ConnectionDirection::NONE]));
        let mut end2: (Pos2, HashSet<ConnectionDirection>) = (Pos2::ZERO, HashSet::from([ConnectionDirection::NONE]));

        let mut wire_connections: Vec<Connection> = Vec::new();

        //TODO:
        //
        //First need to find all connections that reference this wire
        //
        //If more than 2, bird strike, log it, and pick 2 at random?
        //
        //Then set end 1 and end 2 based on those two connections

        for connection in &project_data.connections {
            if connection.end1 == connection.end2 {
                warn! {"connection: {connection:?} has both ends assigned to the same id."};
                continue;
            }
            if let ConnectionType::Wire { wire_id } = &connection.end1
                && wire_id == &id
            {
                wire_connections.push(connection.clone());
            }
            if let ConnectionType::Wire { wire_id } = &connection.end2
                && wire_id == &id
            {
                wire_connections.push(connection.clone());
            }
        }
        #[expect(clippy::indexing_slicing, reason = "size of vec validated in outer match")]
        match wire_connections.len().cmp(&2) {
            Ordering::Equal => {
                let wire_end_connections = [(&mut end1, &wire_connections[0]), (&mut end2, &wire_connections[1])];

                for (end, connection) in wire_end_connections {
                    let connection_ends = [&connection.end1, &connection.end2];
                    for connection_end in connection_ends {
                        #[expect(
                            clippy::match_same_arms,
                            reason = "Separating out some match elements makes the code read clearer"
                        )]
                        #[expect(clippy::wildcard_enum_match_arm, reason = "code still in development")]
                        match connection_end {
                            ConnectionType::Wire { wire_id } if wire_id == &id => {
                                //Do nothing.
                            }
                            // TODO: wire-wire, wire-cable and wire-term_cable connections
                            // not defined yet.
                            ConnectionType::Wire { .. } | ConnectionType::Cable { .. } | ConnectionType::TermCable { .. } => {
                                //todo!();
                            }

                            #[expect(clippy::arithmetic_side_effects, reason = "deal with it")]
                            ConnectionType::Equipment {
                                equipment_id,
                                connection_point_id,
                            } => {
                                trace! {"end1 equipment: {equipment_id}"};
                                #[expect(clippy::get_unwrap, reason = "temporary for testing")]
                                #[expect(clippy::unwrap_used, reason = "temporary for testing")]
                                let equipment = project_data.equipment.get(equipment_id).unwrap();
                                let symbol = equipment.schematic_symbol();
                                #[expect(clippy::get_unwrap, reason = "temporary for testing")]
                                #[expect(clippy::unwrap_used, reason = "temporary for testing")]
                                let connection_point = symbol.connections.get(connection_point_id).unwrap();
                                trace!(
                                    "allowed_connection_directions: {:?}",
                                    connection_point.allowed_connection_directions
                                );
                                end.1 = connection_point.allowed_connection_directions.clone();

                                //TODO: move this to a method on SchematicSymbol
                                //
                                //TODO: change connection point to contain a Pos2?
                                let connection_point_offset = Vec2::from((
                                    symbol.scaled_size().x * (connection_point.x / 100.0),
                                    symbol.scaled_size().y * (connection_point.y / 100.0),
                                ));
                                trace! {"symbol scaled_size: {}", symbol.scaled_size()};
                                trace! {"end1 connection_point: {connection_point:?}"};
                                trace! {"end1 connection_point_offset: {connection_point_offset}"};
                                end.0 = symbol.position
                                    + Vec2::from((
                                        symbol.scaled_size().x * connection_point.x / 100.0,
                                        symbol.scaled_size().y * connection_point.y / 100.0,
                                    ));
                            }

                            _ => {}
                        }
                    }
                }

                Ok(RightAngle::new(
                    end1.0,
                    end1.1,
                    end2.0,
                    end2.1,
                    false,
                    Into::<Stroke>::into(self.line_style.clone()),
                ))
            }
            Ordering::Greater => Err(GUIRenderingError::IncorrectNumberOfConnectionsDefined {
                comparison: "Greater".to_owned(),
                affected_entity: format!("{self:?}"),
            }),
            Ordering::Less => Err(GUIRenderingError::IncorrectNumberOfConnectionsDefined {
                comparison: "Less".to_owned(),
                affected_entity: format!("{self:?}"),
            }),
        }
    }

    #[inline]
    fn update_data_from_library(&mut self, library: &Library) -> Result<(), Error> {
        let wire_type = library.wire_types.get(&self.wire_type).ok_or(LibraryError::ValueNotFound {
            id: self.wire_type.clone(),
            found_in: format!("wire instance {}", self.identifier).to_owned(),
            library_type: "Wire Type".to_owned(),
        })?;

        self.line_style = wire_type.line_style.clone();
        Ok(())
    }
}

impl FromFile for Wire {
    #[inline]
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
    #[inline]
    fn set_datafile(&mut self, datafile_path: &Path) {
        self.contained_datafile_path = datafile_path.to_path_buf();
    }
}

impl ProjectData for Wire {}
