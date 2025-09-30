use serde::{Deserialize, Serialize};
use usvg::Tree;

//TODO: implement svg validation rules here
//
//TODO: switch to using usvg/romxmltree instead of just a string
//
//TODO: provide a method of specifying the units of the SVG file

/// Svg represents a full SVG image
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Svg(pub Tree);

fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
where S: Serializer
{
    serializer.serialize_string(*self.value
}

