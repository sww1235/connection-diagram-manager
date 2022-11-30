use serde::{Deserialize, Serialize};

/// Svg represents a full SVG image
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Svg(Option<String>);
