use core::iter::chain;

use egui::{CursorIcon, Pos2, Rect, Sense, Ui, Vec2, response::Response, widgets::Widget};
use log::{trace, warn};

use crate::datatypes::{
    color::Color,
    schematic_connector::{ConnectionPoint, ConnectorType, SchematicConnector, right_angle::RightAngle},
    util_types::LineStyle,
};

/// `MultiRightAngle` is a connector that primarily has either a `Z` or `S` shape using right angles or an `L`
/// shape.
///
/// It differs from `RightAngle` in that each end of the connector can branch out to many different
/// connection points.
///
/// The main connector follows the same rules as `RightAngle` for directionality, as do all the
/// sub-connectors.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct MultiRightAngle {
    /// The split point of one end of the connection in screen coordinates.
    pub end1_junction: ConnectionPoint,
    /// All connections defined for this end of the `SchematicConnector`.
    ///
    /// The `end1` `ConnectionPoint` of `RightAngle`, in this Vec are assumed to be connected to
    /// the `end1_junction` and the `end2` `ConnectionPoint` is connected to another entity.
    pub end1_connections: Vec<ConnectorType>,
    /// The split point of the other of the connection in screen coordinates.
    pub end2_junction: ConnectionPoint,
    /// All connections defined for this end of the `SchematicConnector`.
    ///
    /// The `end1` `ConnectionPoint` of `RightAngle`, in this Vec are assumed to be connected to
    /// the `end2_junction` and the `end2` `ConnectionPoint` is connected to another entity.
    pub end2_connections: Vec<ConnectorType>,
    /// If the connection is allowed to render past its bounds based on directions.
    ///
    /// Has no effect if opposing directions are specified.
    pub overflow: bool,
    /// The visual appearance of the connector.
    pub line_style: LineStyle,
}

impl SchematicConnector for MultiRightAngle {
    #[inline]
    fn bounding_rect(&self) -> Rect {
        let connector_rects: Vec<Rect> = chain(&self.end1_connections, &self.end2_connections)
            .map(ConnectorType::containing_rect)
            .collect();
        let mut connection_points: Vec<Pos2> = connector_rects
            .iter()
            .flat_map(|rect| [rect.left_top(), rect.right_bottom()].to_vec())
            .collect();

        connection_points.push(self.end1_junction.position);
        connection_points.push(self.end2_junction.position);
        Rect::from_points(&connection_points)
    }
}

impl Widget for &mut MultiRightAngle {
    #[inline]
    fn ui(self, ui: &mut Ui) -> Response {
        let sense_settings = Sense::FOCUSABLE;
        let mut response = ui.response();
        //let painter = ui.painter();

        response.sense = sense_settings;
        //TODO: use painter.add and Shape::dashed_line_with_offset instead if dashed line.

        // First render main connector

        let mut main_connector = RightAngle::new(
            self.end1_junction.clone(),
            self.end2_junction.clone(),
            self.overflow,
            self.line_style.clone(),
        );

        let main_connector_response = ui.place(main_connector.bounding_rect(), &mut main_connector);
        if main_connector_response.hovered() {
            // This should be CursorIcon::Grab but it is not implemented yet.
            // See https://github.com/not-fl3/miniquad/issues/171#issuecomment-773394249

            ui.output_mut(|output| output.cursor_icon = CursorIcon::PointingHand);
        }
        if main_connector_response.dragged() {
            // This should be CursorIcon::Grabbing but it is not implemented yet.
            // See https://github.com/not-fl3/miniquad/issues/171#issuecomment-773394249

            ui.output_mut(|output| output.cursor_icon = CursorIcon::Move);
            trace!("main connector dragged");

            main_connector.move_midpoint(main_connector_response.drag_delta());
        }

        // then render the junctions themselves
        let end1_junction_response = ui.place(self.end1_junction.bounding_rect(), &mut self.end1_junction);
        let end2_junction_response = ui.place(self.end2_junction.bounding_rect(), &mut self.end2_junction);

        if end1_junction_response.hovered() {
            // This should be CursorIcon::Grab but it is not implemented yet.
            // See https://github.com/not-fl3/miniquad/issues/171#issuecomment-773394249

            ui.output_mut(|output| output.cursor_icon = CursorIcon::PointingHand);
        }
        if end1_junction_response.dragged() {
            // This should be CursorIcon::Grabbing but it is not implemented yet.
            // See https://github.com/not-fl3/miniquad/issues/171#issuecomment-773394249

            ui.output_mut(|output| output.cursor_icon = CursorIcon::Move);
            trace!("end1_junction dragged");
            let drag_delta = end1_junction_response.drag_delta();
            self.end1_junction.move_connection_point(drag_delta);
            main_connector.move_end1_position(drag_delta);
            for connection in &mut self.end1_connections {
                // Only updating end1 position here because that is the end of RightAngle
                // linked to end1_junction.
                match connection {
                    ConnectorType::RightAngle(ra) => {
                        ra.move_end1_position(drag_delta);
                    }
                    ConnectorType::MultiRightAngle(mra) => {
                        todo!()
                    }
                }
            }
        }
        if end2_junction_response.hovered() {
            // This should be CursorIcon::Grab but it is not implemented yet.
            // See https://github.com/not-fl3/miniquad/issues/171#issuecomment-773394249

            ui.output_mut(|output| output.cursor_icon = CursorIcon::PointingHand);
        }
        if end2_junction_response.dragged() {
            // This should be CursorIcon::Grabbing but it is not implemented yet.
            // See https://github.com/not-fl3/miniquad/issues/171#issuecomment-773394249

            ui.output_mut(|output| output.cursor_icon = CursorIcon::Move);
            trace!("end2_junction dragged");
            let drag_delta = end2_junction_response.drag_delta();
            self.end2_junction.move_connection_point(drag_delta);
            main_connector.move_end2_position(drag_delta);
            for connection in &mut self.end2_connections {
                // Only updating end1 position here because that is the end of RightAngle
                // linked to end2_junction.
                match connection {
                    ConnectorType::RightAngle(ra) => {
                        ra.move_end1_position(drag_delta);
                    }
                    ConnectorType::MultiRightAngle(mra) => {
                        todo!()
                    }
                }
            }
        }

        // Then render all the individual connectors for end1
        for connection in &mut self.end1_connections {
            let inner_response = ui.place(connection.containing_rect(), &mut *connection);
            if inner_response.hovered() {
                // This should be CursorIcon::Grab but it is not implemented yet.
                // See https://github.com/not-fl3/miniquad/issues/171#issuecomment-773394249

                ui.output_mut(|output| output.cursor_icon = CursorIcon::PointingHand);
            }

            if inner_response.dragged() {
                // This should be CursorIcon::Grabbing but it is not implemented yet.
                // See https://github.com/not-fl3/miniquad/issues/171#issuecomment-773394249

                ui.output_mut(|output| output.cursor_icon = CursorIcon::Move);

                match connection {
                    ConnectorType::RightAngle(ra) => {
                        ra.move_midpoint(inner_response.drag_delta());
                    }
                    ConnectorType::MultiRightAngle(mra) => {
                        todo!()
                    }
                }
            }
        }
        // Then render all the individual connectors for end2
        for connection in &mut self.end2_connections {
            let inner_response = ui.place(connection.containing_rect(), &mut *connection);
            if inner_response.hovered() {
                // This should be CursorIcon::Grab but it is not implemented yet.
                // See https://github.com/not-fl3/miniquad/issues/171#issuecomment-773394249

                ui.output_mut(|output| output.cursor_icon = CursorIcon::PointingHand);
            }

            if inner_response.dragged() {
                // This should be CursorIcon::Grabbing but it is not implemented yet.
                // See https://github.com/not-fl3/miniquad/issues/171#issuecomment-773394249

                ui.output_mut(|output| output.cursor_icon = CursorIcon::Move);

                match connection {
                    ConnectorType::RightAngle(ra) => {
                        ra.move_midpoint(inner_response.drag_delta());
                    }
                    ConnectorType::MultiRightAngle(mra) => {
                        todo!()
                    }
                }
            }
        }

        response
    }
}

impl MultiRightAngle {
    /// Creates a new `RightAngle` connector.
    #[must_use]
    #[inline]
    pub fn new(
        end1_junction: ConnectionPoint,
        end1_connections: Vec<ConnectorType>,
        end2_junction: ConnectionPoint,
        end2_connections: Vec<ConnectorType>,
        overflow: bool,
        line_style: LineStyle,
    ) -> Self {
        //let midpoint = if end1_junction.directions.is_subset(&ConnectionDirection::horizontal())
        //    && end2_junction.directions.is_subset(&ConnectionDirection::horizontal())
        //{
        //    Pos2::new(f32::midpoint(end1_junction.position.x, end2_junction.position.x), 0.0)
        //} else if end1_junction.directions.is_subset(&ConnectionDirection::vertical())
        //    && end2_junction.directions.is_subset(&ConnectionDirection::vertical())
        //{
        //    Pos2::new(0.0, f32::midpoint(end1_junction.position.y, end2_junction.position.y))
        //} else if end1_junction.directions.is_subset(&ConnectionDirection::horizontal())
        //    && end2_junction.directions.is_subset(&ConnectionDirection::vertical())
        //{
        //    trace! {"right/left:top/bottom"} //TODO
        //    Pos2::new(end1_junction.position.x, end2_junction.position.y)
        //} else if end1_junction.directions.is_subset(&ConnectionDirection::vertical())
        //    && end2_junction.directions.is_subset(&ConnectionDirection::horizontal())
        //{
        //    trace! {"top/bottom:right/left"} //TODO
        //    Pos2::new(end1_junction.position.y, end2_junction.position.x)
        //} else {
        //    error! {"unsupported direction combination"}
        //    //TODO: replace with Pos2::NAN once migrated to egui 3.34.1
        //    Pos2::new(f32::NAN, f32::NAN)
        //};

        Self {
            end1_junction,
            end1_connections,
            end2_junction,
            end2_connections,
            overflow,
            line_style,
        }
    }

    /// Set end positions of Connector.
    #[inline]
    pub fn set_end_junction_positions(&mut self, end1: Pos2, end2: Pos2) {
        self.end1_junction.position = end1;
        self.end2_junction.position = end2;
    }
    ////TODO: add overflow logic here
    ///// Set midpoint position for Connector.
    //#[inline]
    //pub fn set_midpoint(&mut self, midpoint: Pos2) {
    //    self.midpoint = midpoint;
    //}

    ////TODO: add overflow logic here
    ///// Change `midpoint` position for Connector.
    //#[inline]
    //#[expect(clippy::arithmetic_side_effects, reason = "/shrug")]
    //pub fn move_midpoint(&mut self, delta: Vec2) {
    //    self.midpoint += delta;
    //}

    //TODO: add overflow logic here
    /// Move `end1_junction` position for Connector.
    #[inline]
    #[expect(clippy::arithmetic_side_effects, reason = "/shrug")]
    pub fn move_end1_junction(&mut self, delta: Vec2) {
        self.end1_junction.position += delta;
    }

    //TODO: add overflow logic here
    /// Move `end2_junction` position for Connector.
    #[inline]
    #[expect(clippy::arithmetic_side_effects, reason = "/shrug")]
    pub fn move_end2_junction(&mut self, delta: Vec2) {
        self.end2_junction.position += delta;
    }

    /// Set `Stroke` of Connector.
    #[inline]
    pub fn set_stroke(&mut self, line_style: LineStyle) {
        self.line_style = line_style;
    }

    /// Set `Color` of `Stroke` of Connector.
    #[inline]
    pub fn set_color(&mut self, color: Color) {
        self.line_style.color = color;
    }
}
