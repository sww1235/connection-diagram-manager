/// Right angle represents a right angle connection.
pub mod right_angle;

use serde::{Deserialize, Serialize};

use crate::{datatypes::project_types::Project, error::GUIRenderingError, traits::ProjectData};

//TODO: multiple connections on each end.

/// `IntoConnector` converts a linear entity into a connector for rendering.
pub trait AsConnector {
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
    fn as_connector(&self, id: String, project_data: &Project) -> Result<Self::Output, GUIRenderingError>
    where Self: ProjectData;
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
