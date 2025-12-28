use core::str::FromStr as _;
use std::{fmt, path::PathBuf};

use log::trace;
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
pub struct Svg {
    /// Tree is a `[usvg::Tree]` used for easy interpretation of SVG
    tree: Tree,
    /// If provided string is a filepath to a SVG file stored elsewhere
    filepath: Option<PathBuf>,
}

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
        self.tree
    }
    #[must_use]
    #[inline]
    /// Create a `Svg` from a `[usvg::Tree]`
    pub fn from_tree(tree: Tree) -> Self {
        Self { tree, filepath: None }
    }
}

impl Serialize for Svg {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let write_options = WriteOptions::default();
        serializer.serialize_str(self.tree.to_string(&write_options).as_str())
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
        // try to parse and validate a filepath, then load SVG from that
        // If that fails, attempt to parse SVG from string directly.
        // If both fail, return error
        trace! {"{}", std::env::current_dir().unwrap().display()}
        #[expect(clippy::unwrap_used, reason = "Infallible")]
        let svg: Svg = match PathBuf::from_str(v).unwrap().canonicalize() {
            Ok(path) => {
                let path = path
                    .canonicalize()
                    .map_err(|e| E::custom(format!("failed to canonicalize filepath: {e}")))?;
                let image_bytes = std::fs::read(&path).map_err(|e| E::custom(format!("failed to read file: {e}")))?;
                let image_str = str::from_utf8(&image_bytes)
                    .map_err(|e| E::custom(format!("failed to parse image bytes into UTF8 string: {e}")))?;
                let tree = match Tree::from_str(image_str, &options) {
                    Ok(tree) => tree,
                    Err(tree_err) => {
                        return Err(E::custom(format!(
                            "Failed to parse data in {} as SVG data. {tree_err}",
                            path.display()
                        )));
                    }
                };

                Svg {
                    tree,
                    filepath: Some(path),
                }
            }
            Err(path_err) => {
                trace! {"failed to parse {v} as path"};

                match Tree::from_str(v, &options) {
                    Ok(tree) => Svg { tree, filepath: None },
                    Err(tree_err) => {
                        return Err(E::custom(format!(
                            "Failed to parse provided SVG data as either path {path_err} or SVG tree {tree_err}"
                        )));
                    }
                }
            }
        };

        Ok(svg)
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
        Svg {
            tree: Tree::from_str(default_svg_string, &options).unwrap(),
            filepath: None,
        }
    }
}

//TODO: Decide if it makes sense to keep this or not.
#[expect(clippy::missing_trait_methods, reason = "using default impl")]
impl PartialEq for Svg {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.tree.to_string(&Self::write_options()) == other.tree.to_string(&Self::write_options())
            && self.filepath == other.filepath
    }
}
