use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::datatypes::{internal_types::svg::Svg, unit_helper::Length, util_types::Catalog};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MountingRailType {
    /// Catalog information
    pub catalog: Option<Catalog>,
    /// overall height of rail
    /// rail center point will be at
    /// rail_height / 2
    pub rail_height: Length,
    /// total height of center/recessed section of mounting rail
    /// centered on total height
    pub rail_center_height: Length,
    /// does mounting rail have slots
    pub slots: bool,
    /// are slots rounded or rectangular
    pub rounded_slots: bool,
    /// linear distance between origin and center of first slot
    /// will also be used for the distance between the last slot
    /// and the end of the rail.
    pub first_slot_center: Option<Length>,
    /// linear center to center distance between slots.
    pub slot_center_to_center: Option<Length>,
    /// Length of slot, includes length of rounded ends
    pub slot_length: Option<Length>,
    /// Height of slot
    pub slot_height: Option<Length>,
    /// the length of rail as specified by the manufacturer/supplier part number
    pub standard_rail_length: Length,
    /// User specified minimum length.
    /// If not specified, will be set to 2x the first_slot_center distance
    /// if instance length is set smaller than default minimum_rail_length
    /// and no_partial_holes is false, then minimum_rail_length
    /// will be ignored.
    pub minimum_rail_length: Option<Length>,
    /// extend rail so there are no partial holes
    pub no_partial_holes: Option<bool>,
    /// distance between top center_line and origin
    pub top_rail_center_height: Option<Length>,
    /// distance between bottom center_line and origin
    pub bottom_rail_center_height: Option<Length>,
    /// distance between origin and slot vertical center
    /// positive above origin, negative below origin
    pub slot_vertical_center: Option<Length>,
    /// SVG files for start, end and middle of mounting rail
    /// minimum rail length should be set to the length of the
    /// start and end SVGs to not cause graphical issues
    /// if minimum rail length is not set, the middle SVG
    /// might get cut off unexpectedly.
    ///
    /// the start, middle and end images should not have lines where they join
    /// so when the images are placed together, there is no overlap.
    pub start_image: Option<Svg>,
    pub middle_image: Option<Svg>,
    pub end_image: Option<Svg>,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}
