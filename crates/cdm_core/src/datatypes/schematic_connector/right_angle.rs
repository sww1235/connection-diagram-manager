use std::collections::HashSet;

use egui::{Pos2, Rect, Sense, Stroke, Ui, Vec2, epaint::MarginF32, response::Response, widgets::Widget};
use log::{error, trace};

use crate::datatypes::{
    color::Color,
    schematic_connector::{ConnectionPoint, SchematicConnector},
    schematic_symbol::ConnectionDirection,
    util_types::LineStyle,
};

/// `RightAngle` is a connector that either has a `Z` or `S` shape using right angles or an `L`
/// shape.
///
/// If `end1_direction` and `end2_direction` are both subsets of `ConnectionDirection::horizontal()`
/// then the connection will render as 2 horizontal lines, one each extending from each of the ends
/// of the connection. These will extend to the midpoint in the `x` direction between the two ends
/// and then will be joined with a vertical line.
///
/// If `end1_direction` and `end2_direction` are both subsets of `ConnectionDirection::vertical()`
/// then the connection will render as 2 vertical lines, one each extending from each of the ends
/// of the connection. These will extend to the midpoint in the `y` direction between the two ends
/// and then will be joined with a horizontal line.
///
/// If `end1_direction` and `end2_direction` are not subsets of the same orientation
/// `ConnectionDirection`, then the connection will render as a right angled line, with one
/// horizontal and one vertical line. Which is which will depend on the `end_direction`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct RightAngle {
    /// One end of the connection.
    pub end1: ConnectionPoint,
    /// The other end of the connection.
    pub end2: ConnectionPoint,
    /// The midpoint of the line. Depending on direction, only the `x` or `y` coordinates are used.
    pub midpoint: Pos2,
    /// If the connection is allowed to render past its bounds based on directions.
    ///
    /// Has no effect if opposing directions are specified.
    pub overflow: bool,
    /// The visual appearance of the connector.
    pub line_style: LineStyle,
}

impl SchematicConnector for RightAngle {
    #[inline]
    fn bounding_rect(&self) -> Rect {
        Rect::from_points(&[self.end1.position, self.end2.position, self.midpoint])
    }
}

impl Widget for &mut RightAngle {
    #[inline]
    #[expect(clippy::similar_names, reason = "partial false positive")]
    #[expect(clippy::arithmetic_side_effects, reason = "UI code")]
    fn ui(self, ui: &mut Ui) -> Response {
        let sense = Sense::click_and_drag();
        let response: Response;
        let painter = ui.painter();

        //TODO: add configuration option for this
        let buffer_buffer: f32 = 2.0;
        let interaction_buffer_amount = self.line_style.line_thickness + buffer_buffer;

        let vertical_interaction_buffer = MarginF32 {
            left: 0.0,
            right: 0.0,
            top: interaction_buffer_amount,
            bottom: interaction_buffer_amount,
        };
        let horizontal_interaction_buffer = MarginF32 {
            left: interaction_buffer_amount,
            right: interaction_buffer_amount,
            top: 0.0,
            bottom: 0.0,
        };

        //TODO: use painter.add and Shape::dashed_line_with_offset instead if dashed line.

        //debug! {"RightAngle::ui() end_1 directions: {:?}", self.end1.directions};
        //debug! {"RightAngle::ui() end_2 directions: {:?}", self.end2.directions};

        if self.end1.directions.is_subset(&ConnectionDirection::horizontal())
            && self.end2.directions.is_subset(&ConnectionDirection::horizontal())
        {
            //trace! {"right/left:right/left"}
            let stroke = Into::<Stroke>::into(self.line_style.clone());
            trace! {"STROKE: {stroke:?}"}
            let end1_midpoint = Pos2::new(self.midpoint.x, self.end1.position.y);
            let end2_midpoint = Pos2::new(self.midpoint.x, self.end2.position.y);
            let line_points: Vec<Pos2> = vec![self.end1.position, end1_midpoint, end2_midpoint, self.end2.position];
            painter.line(line_points.clone(), stroke);
            //let line1_points: Vec<Pos2> = vec![self.end1.position, end1_midpoint];
            let line2_points: Vec<Pos2> = vec![end1_midpoint, end2_midpoint];
            //let line3_points: Vec<Pos2> = vec![end2_midpoint, self.end2.position];

            //let line1_rect: Rect = Rect::from_points(&line1_points) + vertical_interaction_buffer;
            let line2_rect: Rect = Rect::from_points(&line2_points) + horizontal_interaction_buffer;
            //let line3_rect: Rect = Rect::from_points(&line3_points) + vertical_interaction_buffer;

            // Horizontal line
            //let line1_response = ui.allocate_rect(line1_rect, sense);
            // Vertical line
            let line2_response = ui.allocate_rect(line2_rect, sense);
            // Horizontal line
            //let line3_response = ui.allocate_rect(line3_rect, sense);
            response = line2_response;
        } else if self.end1.directions.is_subset(&ConnectionDirection::vertical())
            && self.end2.directions.is_subset(&ConnectionDirection::vertical())
        {
            //trace! {"top/bottom:top/bottom"}
            let stroke = Into::<Stroke>::into(self.line_style.clone());
            trace! {"STROKE: {stroke:?}"}
            let end1_midpoint = Pos2::new(self.end1.position.x, self.midpoint.y);
            let end2_midpoint = Pos2::new(self.end2.position.x, self.midpoint.y);
            let line_points: Vec<Pos2> = vec![self.end1.position, end1_midpoint, end2_midpoint, self.end2.position];
            painter.line(line_points.clone(), stroke);
            //let line1_points: Vec<Pos2> = vec![self.end1.position, end1_midpoint];
            let line2_points: Vec<Pos2> = vec![end1_midpoint, end2_midpoint];
            //let line3_points: Vec<Pos2> = vec![end2_midpoint, self.end2.position];

            //let line1_rect: Rect = Rect::from_points(&line1_points) + horizontal_interaction_buffer;
            let line2_rect: Rect = Rect::from_points(&line2_points) + vertical_interaction_buffer;
            //let line3_rect: Rect = Rect::from_points(&line3_points) + horizontal_interaction_buffer;

            // vertical line
            //let line1_response = ui.allocate_rect(line1_rect, sense);
            // horizontal line
            let line2_response = ui.allocate_rect(line2_rect, sense);
            // vertical line
            //let line3_response = ui.allocate_rect(line3_rect, sense);
            response = line2_response;
        } else if self.end1.directions.is_subset(&ConnectionDirection::horizontal())
            && self.end2.directions.is_subset(&ConnectionDirection::vertical())
        {
            trace! {"right/left:top/bottom"} //TODO
            response = ui.response();
        } else if self.end1.directions.is_subset(&ConnectionDirection::vertical())
            && self.end2.directions.is_subset(&ConnectionDirection::horizontal())
        {
            trace! {"top/bottom:right/left"} //TODO
            response = ui.response();
        } else {
            error! {"RA ui() fn: unsupported direction combination"}
            response = ui.response();
        }
        response
    }
}

impl RightAngle {
    /// Creates a new `RightAngle` connector.
    #[must_use]
    #[inline]
    pub fn new(end1: ConnectionPoint, end2: ConnectionPoint, overflow: bool, line_style: LineStyle) -> Self {
        let midpoint = if end1.directions.is_subset(&ConnectionDirection::horizontal())
            && end2.directions.is_subset(&ConnectionDirection::horizontal())
        {
            Pos2::new(f32::midpoint(end1.position.x, end2.position.x), 0.0)
        } else if end1.directions.is_subset(&ConnectionDirection::vertical())
            && end2.directions.is_subset(&ConnectionDirection::vertical())
        {
            Pos2::new(0.0, f32::midpoint(end1.position.y, end2.position.y))
        } else if end1.directions.is_subset(&ConnectionDirection::horizontal())
            && end2.directions.is_subset(&ConnectionDirection::vertical())
        {
            trace! {"right/left:top/bottom"} //TODO
            Pos2::new(end1.position.x, end2.position.y)
        } else if end1.directions.is_subset(&ConnectionDirection::vertical())
            && end2.directions.is_subset(&ConnectionDirection::horizontal())
        {
            trace! {"top/bottom:right/left"} //TODO
            Pos2::new(end1.position.y, end2.position.x)
        } else {
            error! {"new() fn: unsupported direction combination"}
            //TODO: replace with Pos2::NAN once migrated to egui 3.34.1
            Pos2::new(f32::NAN, f32::NAN)
        };

        Self {
            end1,
            end2,
            midpoint,
            overflow,
            line_style,
        }
    }

    /// Set end positions of Connector.
    #[inline]
    pub fn set_end_positions(&mut self, end1: Pos2, end2: Pos2) {
        self.end1.position = end1;
        self.end2.position = end2;
    }
    /// Set end1 position of Connector.
    #[inline]
    pub fn set_end1_position(&mut self, end1: Pos2) {
        self.end1.position = end1;
    }
    /// Set end2 position of Connector.
    #[inline]
    pub fn set_end2_position(&mut self, end2: Pos2) {
        self.end2.position = end2;
    }
    /// Move end1 position of Connector.
    #[inline]
    #[expect(clippy::arithmetic_side_effects, reason = "/shrug")]
    pub fn move_end1_position(&mut self, delta: Vec2) {
        self.end1.position += delta;
    }
    /// Move end2 position of Connector.
    #[inline]
    #[expect(clippy::arithmetic_side_effects, reason = "/shrug")]
    pub fn move_end2_position(&mut self, delta: Vec2) {
        self.end2.position += delta;
    }
    //TODO: add overflow logic here
    /// Set midpoint position for Connector.
    #[inline]
    pub fn set_midpoint(&mut self, midpoint: Pos2) {
        self.midpoint = midpoint;
    }

    //TODO: add overflow logic here
    /// Change midpoint position for Connector.
    #[inline]
    #[expect(clippy::arithmetic_side_effects, reason = "/shrug")]
    pub fn move_midpoint(&mut self, delta: Vec2) {
        self.midpoint += delta;
    }

    /// Return midpoint position for Connector.
    #[inline]
    pub fn midpoint(&mut self) -> Pos2 {
        self.midpoint
    }

    /// Set `LineStyle` of Connector.
    #[inline]
    pub fn set_line_style(&mut self, line_style: LineStyle) {
        self.line_style = line_style;
    }

    /// Set `Color` of `Stroke` of Connector.
    #[inline]
    pub fn set_color(&mut self, color: Color) {
        self.line_style.color = color;
    }

    /// Set allowed connection directions for end1.
    #[inline]
    pub fn set_end1_connection_directions(&mut self, allowed_directions: &HashSet<ConnectionDirection>) {
        self.end1.set_allowed_connection_directions(allowed_directions);
    }

    /// Set allowed connection directions for end2.
    #[inline]
    pub fn set_end2_connection_directions(&mut self, allowed_directions: &HashSet<ConnectionDirection>) {
        self.end2.set_allowed_connection_directions(allowed_directions);
    }
}

impl Default for RightAngle {
    #[inline]
    fn default() -> Self {
        Self {
            end1: ConnectionPoint::default(),
            end2: ConnectionPoint::default(),
            midpoint: Pos2::default(),
            overflow: false,
            line_style: LineStyle::default(),
        }
    }
}
