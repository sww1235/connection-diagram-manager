use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor},
    ser::{Serialize, Serializer},
};
use usvg::{Options as ParseOptions, Tree, WriteOptions};

//TODO: implement svg validation rules here
//
//TODO: provide a method of specifying the units of the SVG file

/// Svg represents a full SVG image
#[derive(Debug, Clone)]
pub struct Svg(Tree);

impl Svg {
    #[must_use]
    #[inline]
    /// Standard `[usvg::WriteOptions]` used when writing SVGs to strings
    pub fn write_options() -> WriteOptions {
        WriteOptions::default()
    }
    #[must_use]
    #[inline]
    /// Standard `[usvg::Options]` used when parsing SVG strings
    pub fn parse_options() -> ParseOptions<'static> {
        ParseOptions::default()
    }
    #[must_use]
    #[inline]
    /// Gets the underlying `[usvg::Tree]` in `Svg`
    pub fn get_tree(self) -> Tree {
        self.0
    }
    #[must_use]
    #[inline]
    /// Create a `Svg` from a `[usvg::Tree]`
    pub fn from_tree(tree: Tree) -> Self {
        Self(tree)
    }
}

impl Serialize for Svg {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let write_options = WriteOptions::default();
        serializer.serialize_str(self.0.to_string(&write_options).as_str())
    }
}

#[expect(clippy::missing_trait_methods, reason = "using default impl")]
impl<'de> Deserialize<'de> for Svg {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Svg, D::Error>
    where D: Deserializer<'de> {
        deserializer.deserialize_str(SvgVisitor)
    }
}

/// Visitor struct for Deserializer trait
struct SvgVisitor;

#[expect(clippy::missing_trait_methods, reason = "using default impl")]
impl Visitor<'_> for SvgVisitor {
    type Value = Svg;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a SVG in string format")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where E: de::Error {
        let options = ParseOptions::default();
        let tree = match Tree::from_str(v, &options) {
            Ok(tree) => tree,
            Err(err) => return Err(E::custom(format!("SVG parsing error {err}"))),
        };

        Ok(Svg(tree))
    }
}

impl Default for Svg {
    #[inline]
    fn default() -> Self {
        let default_svg_string = r#"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg xmlns="http://www.w3.org/2000/svg" width="640" height="120">
</svg>
        "#;

        let options = ParseOptions::default();
        #[expect(clippy::unwrap_used, reason = "a known SVG string should never fail to parse")]
        Svg(Tree::from_str(default_svg_string, &options).unwrap())
    }
}

//TODO: Decide if it makes sense to keep this or not.
#[expect(clippy::missing_trait_methods, reason = "using default impl")]
impl PartialEq for Svg {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.to_string(&Self::write_options()) == other.0.to_string(&Self::write_options())
    }
}
