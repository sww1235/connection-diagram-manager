use egui::{Pos2, Rect, Sense, Stroke, Ui, Vec2, response::Response, widgets::Widget};
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
        Rect::from_two_pos(self.end1.position, self.end2.position)
    }
}

impl Widget for &mut RightAngle {
    #[inline]
    fn ui(self, ui: &mut Ui) -> Response {
        let sense_settings = Sense::click_and_drag();
        let mut response = ui.response();
        let painter = ui.painter();

        response.sense = sense_settings;
        //TODO: use painter.add and Shape::dashed_line_with_offset instead if dashed line.

        if self.end1.directions.is_subset(&ConnectionDirection::horizontal())
            && self.end2.directions.is_subset(&ConnectionDirection::horizontal())
        {
            trace! {"right/left:right/left"}
            let end1_midpoint = Pos2::new(self.midpoint.x, self.end1.position.y);
            let end2_midpoint = Pos2::new(self.midpoint.x, self.end2.position.y);
            let line_points: Vec<Pos2> = vec![self.end1.position, end1_midpoint, end2_midpoint, self.end2.position];
            painter.line(line_points, Into::<Stroke>::into(self.line_style.clone()));
        } else if self.end1.directions.is_subset(&ConnectionDirection::vertical())
            && self.end2.directions.is_subset(&ConnectionDirection::vertical())
        {
            trace! {"top/bottom:top/bottom"}
            let end1_midpoint = Pos2::new(self.end1.position.x, self.midpoint.y);
            let end2_midpoint = Pos2::new(self.end2.position.x, self.midpoint.y);
            let line_points: Vec<Pos2> = vec![self.end1.position, end1_midpoint, end2_midpoint, self.end2.position];
            painter.line(line_points, Into::<Stroke>::into(self.line_style.clone()));
        } else if self.end1.directions.is_subset(&ConnectionDirection::horizontal())
            && self.end2.directions.is_subset(&ConnectionDirection::vertical())
        {
            trace! {"right/left:top/bottom"} //TODO
        } else if self.end1.directions.is_subset(&ConnectionDirection::vertical())
            && self.end2.directions.is_subset(&ConnectionDirection::horizontal())
        {
            trace! {"top/bottom:right/left"} //TODO
        } else {
            error! {"unsupported direction combination"}
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
            error! {"unsupported direction combination"}
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
}
