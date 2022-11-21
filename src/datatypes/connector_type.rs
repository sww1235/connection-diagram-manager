use super::svg;
use serde::{Deserialize, Serialize};
//TODO: Make some of these fields enums
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConnectorType {
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub part_number: Option<String>,
    pub manufacturer_part_number: Option<String>,
    pub supplier: Option<String>,
    pub supplier_part_number: Option<String>,
    pub description: Option<String>,
    pub mount_type: Option<Vec<String>>,
    pub panel_cutout: Option<String>,
    pub gender: Option<String>,
    pub height: Option<f64>,
    pub width: Option<f64>,
    pub depth: Option<f64>,
    pub diameter: Option<f64>,
    pub pin_count: Option<u64>,
    pub pins: Option<Vec<String>>,
    pub pin_labels: Option<Vec<String>>,
    pub pin_colors: Option<Vec<String>>,
    pub pin_signal_type: Option<Vec<String>>,
    pub visual_rep: Option<svg::Svg>,
    pub pin_visual_rep: Option<svg::Svg>,
}
