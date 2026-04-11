/// Right angle represents a right angle connection.
pub mod right_angle;

use core::cmp::Ordering;
use std::collections::HashSet;

use egui::Pos2;
use serde::{Deserialize, Serialize};

use crate::{
    datatypes::{
        library_types::Library,
        project_types::{Project, ProjectData},
        schematic_symbol::ConnectionDirection,
    },
    error::{Error, GUIRenderingError},
};

//TODO: multiple connections on each end.

/// `IntoConnector` converts a linear entity into a connector for rendering.
pub trait AsConnector
where Self: ProjectData
{
    /// Type of Connector returned from function.
    type Output: SchematicConnector;
    /// returns a `SchematicConnector` representation of a linear entity.
    ///
    /// # Errors
    ///
    /// Will return an `Err` if `Self` has greater or fewer than 2 connections defined overall.
    ///
    /// For things like `Cables` that might have more than 2 connections overall defined, this is
    /// checked at the core level.
    fn as_connector(&self, id: String, project_data: &Project) -> Result<Self::Output, GUIRenderingError>;

    /// Updates the data embedded in `Self` from its library representation.
    ///
    /// # Errors
    ///
    /// Shall error if the id of `&self.entity_type` is not found in the provided library or other
    /// implementation specific errors.
    #[expect(clippy::result_large_err, reason = "Using main Error type")]
    fn update_data_from_library(&mut self, library: &Library) -> Result<(), Error>;
}

/// Marker trait for the various types of `SchematicConnectors`.
pub trait SchematicConnector {}

/// `SchematicConnector Type`.
#[non_exhaustive]
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum Type {
    /// A Connector drawn at right angles.
    #[default]
    RightAngle,
    /// A Connector drawn directly between the two ends.
    Straight,
}

/// `ConnectionPoint` represents a connection point of a `SchematicConnector`.
#[non_exhaustive]
pub struct ConnectionPoint {
    pub id: String,
    /// The coordinates of the connection in screen coordinates.
    pub position: Pos2,
    /// The allowed directions for `SchematicConnections` to render from this `ConnectionPoint`.
    pub directions: HashSet<ConnectionDirection>,
}

impl Default for ConnectionPoint {
    fn default() -> Self {
        Self {
            id: String::new(),
            position: Pos2::ZERO,
            directions: HashSet::from([ConnectionDirection::NONE]),
        }
    }
}

impl Ord for ConnectionPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        let x_ordering: Ordering = self.position.x.total_cmp(&other.position.x);
        let y_ordering: Ordering = self.position.y.total_cmp(&other.position.y);

        x_ordering.then(y_ordering)
    }
}

impl PartialOrd for ConnectionPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ConnectionPoint {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for ConnectionPoint {}
