use serde::{Deserialize, Serialize};

use super::super::internal_types::svg::Svg as IntSvg;

/// Svg represents a full SVG image
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Svg(pub String);

impl From<IntSvg> for Svg {
    fn from(int_svg: IntSvg) -> Self {
        Svg(int_svg.0)
    }
}
