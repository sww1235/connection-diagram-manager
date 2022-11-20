use super::svg;
use serde::{Deserialize, Serialize};
//TODO: Make some of these fields enums
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConnectorType {
    pub manufacturer: String,
    pub model: String,
    pub part_number: String,
    pub manufacturer_part_number: String,
    pub supplier: String,
    pub supplier_part_number: String,
    pub description: String,
    pub mount_type: Vec<String>,
    pub panel_cutout: String,
    pub gender: String,
    pub height: f64,
    pub width: f64,
    pub depth: f64,
    pub diameter: f64,
    pub pin_count: i64,
    pub pins: Vec<String>,
    pub pin_labels: Vec<String>,
    pub pin_colors: Vec<String>,
    pub pin_signal_type: Vec<String>,
    pub visual_rep: svg::Svg,
    pub pin_visual_rep: svg::Svg,
}
