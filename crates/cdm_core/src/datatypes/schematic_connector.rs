/// `multi_right_angle` represents a right angle connection where each end can split off and have
/// many separate connection points.
///
/// Mainly used for cables.
pub mod multi_right_angle;
/// `right_angle` represents a right angle connection between two connection points.
pub mod right_angle;

use std::collections::HashSet;

use egui::{Color32, Pos2, Rect, Sense, Ui, Vec2, response::Response, widgets::Widget};
use serde::{Deserialize, Serialize};

use crate::{
    datatypes::{
        color::Color,
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

    /// Returns a reference to the `SchematicConnector` of the implementing datatype.
    fn connector(&self) -> Self::Output;

    /// Returns a mutable reference to the `SchematicConnector` of the implementing datatype.
    fn connector_mut(&mut self) -> &mut Self::Output;

    /// Sets the position of the primary `end1` of the connector.
    ///
    /// If the connector branches, then this should set the position of the `end1` branch point or
    /// junction.
    fn set_end1_position(&mut self, position: Pos2);

    /// Sets the position of the primary `end2` of the connector.
    ///
    /// If the connector branches, then this should set the position of the `end2` branch point or
    /// junction.
    fn set_end2_position(&mut self, position: Pos2);

    /// Returns the position of the primary `end1` of the connector.
    ///
    /// If the connector branches, then this should return the position of the `end1` branch point or
    /// junction.
    fn end1_position(&self) -> Pos2;

    /// Returns the position of the primary `end2` of the connector.
    ///
    /// If the connector branches, then this should return the position of the `end2` branch point or
    /// junction.
    fn end2_position(&self) -> Pos2;

    /// Updates the data embedded in `Self` from its library representation.
    ///
    /// Also inserts `SchematicConnector`s as appropriate.
    ///
    /// # Errors
    ///
    /// Shall error if the id of `&self.entity_type` is not found in the provided library or other
    /// implementation specific errors.
    #[expect(clippy::result_large_err, reason = "Using main Error type")]
    fn update_data_from_library(&mut self, library: &Library) -> Result<(), Error>;
}

/// Marker trait for the various types of `SchematicConnectors`.
///
/// Also provides some common methods.
pub trait SchematicConnector {
    /// Returns a `Rect` that is the bounding box of this `SchematicConnector`.
    #[must_use]
    fn bounding_rect(&self) -> Rect;
}

/// `SchematicConnector Type`.
#[non_exhaustive]
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum TypeFlag {
    /// A Connector drawn at right angles.
    #[default]
    RightAngle,
    /// A Connector drawn directly between the two ends.
    Straight,
}
/// An enum to allow storing different `SchematicConnector`s in one `Vec`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectorType {
    /// `RightAngle` contains a `RightAngle` connector.
    RightAngle(right_angle::RightAngle),
    /// `MultiRightAngle` contains a `MultiRightAngle` connector.
    MultiRightAngle(multi_right_angle::MultiRightAngle),
}

impl Widget for &mut ConnectorType {
    #[inline]
    fn ui(self, ui: &mut Ui) -> Response {
        match self {
            ConnectorType::RightAngle(widget) => widget.ui(ui),
            ConnectorType::MultiRightAngle(widget) => widget.ui(ui),
        }
    }
}

impl SchematicConnector for ConnectorType {
    #[inline]
    fn bounding_rect(&self) -> Rect {
        match self {
            ConnectorType::RightAngle(ra) => ra.bounding_rect(),
            ConnectorType::MultiRightAngle(mra) => mra.bounding_rect(),
        }
    }
}

/// `ConnectionPoint` represents a connection point of a `SchematicConnector`.
#[non_exhaustive]
#[derive(Debug, PartialEq, Clone)]
pub struct ConnectionPoint {
    /// ID of `ConnectionPoint`. Should be unique within a `SchematicSymbol`.
    pub id: String,
    /// The coordinates of the connection in screen coordinates.
    pub position: Pos2,
    /// The allowed directions for `SchematicConnections` to render from this `ConnectionPoint`.
    pub directions: HashSet<ConnectionDirection>,
    /// Radius of the circle representing this `ConnectionPoint`.
    pub radius: f32,
    /// Fill color.
    pub color: Color,
}

impl Default for ConnectionPoint {
    #[inline]
    fn default() -> Self {
        Self {
            id: String::new(),
            position: Pos2::ZERO,
            directions: HashSet::from([ConnectionDirection::NONE]),
            radius: 1.0,
            color: Color::RED,
        }
    }
}

impl ConnectionPoint {
    /// Creates a new `ConnectionPoint`.
    #[must_use]
    #[inline]
    pub fn new(id: &str, position: Pos2, directions: HashSet<ConnectionDirection>, radius: f32, color: Color) -> Self {
        Self {
            id: id.to_owned(),
            position,
            directions,
            radius,
            color,
        }
    }

    /// Returns a `Rect` that is the bounding box of this `ConnectionPoint`.
    #[must_use]
    #[inline]
    pub fn bounding_rect(&self) -> Rect {
        Rect::from_center_size(self.position, Vec2::new(0.0, self.radius))
    }

    /// Move position of `ConnectionPoint`.
    #[inline]
    #[expect(clippy::arithmetic_side_effects, reason = "/shrug")]
    pub fn move_position(&mut self, delta: Vec2) {
        self.position += delta;
    }

    /// Set position of `ConnectionPoint`.
    #[inline]
    pub fn set_position(&mut self, position: Pos2) {
        self.position = position;
    }

    /// Returns position of `ConnectionPoint`.
    #[must_use]
    #[inline]
    pub fn position(&self) -> Pos2 {
        self.position
    }
}

impl Widget for &mut ConnectionPoint {
    #[inline]
    fn ui(self, ui: &mut Ui) -> Response {
        let sense_settings = Sense::click_and_drag();
        let mut response = ui.response();
        let painter = ui.painter();

        response.sense = sense_settings;

        painter.circle_filled(self.position, self.radius, Into::<Color32>::into(self.color.clone()));

        response
    }
}
