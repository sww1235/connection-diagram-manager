use core::{convert::Infallible, str::FromStr};
use std::{env, fmt, fs, path::PathBuf};

use log::trace;
use serde::{
    de::{self, Deserialize, Deserializer, Visitor},
    ser::{Serialize, Serializer},
};
use usvg::{Error as USvgParseError, Options as USvgParseOptions, Tree};

//mod rwxmltree;

//TODO: implement svg validation rules here
//
//TODO: provide a method of specifying the units of the SVG file

/// Svg represents a full SVG image.
#[derive(Debug, Clone)]
pub struct Svg {
    /// a raw XML doc imported and modified.
    svg_data: String,
    /// If provided string is a filepath to a SVG file stored elsewhere.
    filepath: Option<PathBuf>,
}

impl Svg {
    /// Outputs a `[usvg::Tree]` of the `svg_data` contained within the `Svg`.
    ///
    /// # Errors
    ///
    /// Will error if the conversion to `[usvg::Tree]` fails.
    #[inline]
    pub fn get_tree(&self) -> Result<Tree, USvgParseError> {
        Tree::from_data(self.svg_data.as_bytes(), &USvgParseOptions::default())
    }

    /// Returns the XML data within the `Svg`.
    #[inline]
    #[must_use]
    pub fn get_data(&self) -> String {
        self.svg_data.clone()
    }

    /// Sets the XML data witin the `Svg`.
    #[inline]
    pub fn set_data(&mut self, new_data: &str) {
        self.svg_data = new_data.to_owned();
    }
}

impl Serialize for Svg {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        //TODO: handle filepaths
        serializer.serialize_str(&self.svg_data)
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

/// Visitor struct for Deserializer trait.
struct SvgVisitor;

#[expect(clippy::missing_trait_methods, reason = "using default impl")]
impl Visitor<'_> for SvgVisitor {
    type Value = Svg;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a SVG in string format or a filepath")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where E: de::Error {
        // try to parse and validate a filepath, then load SVG from that
        // If that fails, attempt to parse SVG from string directly.
        // If both fail, return error
        trace! {"Current Directory for SVG canonical_path: {}",
        env::current_dir().map_err(|err| E::custom(
                format!("failed to find current directory. Something went seriously wrong. {err}")))?.display()};
        #[expect(irrefutable_let_patterns, reason = "either this or a match statement")]
        let svg: Svg = if let Ok(path) = PathBuf::from_str(v) {
            let canonical_path = path
                .canonicalize()
                .map_err(|err| E::custom(format!("failed to canonicalize filepath: {err}")))?;
            let image_bytes = fs::read(&canonical_path).map_err(|err| E::custom(format!("failed to read file: {err}")))?;
            let image_str = str::from_utf8(&image_bytes)
                .map_err(|err| E::custom(format!("failed to parse image bytes into UTF8 string: {err}")))?
                .to_owned();
            Svg {
                svg_data: image_str,
                filepath: Some(canonical_path),
            }
        }
        //If filepath parsing fails, it should be an SVG
        else {
            trace! {"failed to parse {v} as path"};
            Svg {
                svg_data: v.to_owned(),
                filepath: None,
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
        Svg {
            svg_data: default_svg_string.to_owned(),
            filepath: None,
        }
    }
}

//TODO: Decide if it makes sense to keep this or not.
#[expect(clippy::missing_trait_methods, reason = "using default impl")]
impl PartialEq for Svg {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.svg_data == other.svg_data && self.filepath == other.filepath
    }
}

impl FromStr for Svg {
    type Err = Infallible;

    //TODO: validation
    /// Creates a `Svg` from a string representation of an Svg. No validation is performed
    /// currently.
    #[inline]
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            svg_data: text.to_owned(),
            filepath: None,
        })
    }
}
