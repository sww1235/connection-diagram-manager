use core::{convert::Infallible, str::FromStr};
use std::{env, fmt, fs, path::PathBuf};

use log::trace;
use serde::{
    de::{self, Deserialize, Deserializer, Visitor},
    ser::{Serialize, Serializer},
};
use usvg::{Error as USvgParseError, Options as USvgParseOptions, Tree};
use xml::{EventReader, reader::XmlEvent as ReaderEvent};

use crate::error::{Error, SVGValidationError};

//mod rwxmltree;

//TODO: implement svg validation rules here
//
//TODO: provide a method of specifying the units of the SVG file

/// Svg represents a full SVG image.
#[derive(Debug, Clone)]
pub struct Svg {
    /// Raw XML text data of SVG.
    data: String,
    /// If provided string is a filepath to a SVG file stored elsewhere.
    filepath: Option<PathBuf>,
    /// Width of `viewBox` in raw SVG.
    original_width: Option<f32>,
    /// Height of `viewBox` in raw SVG.
    original_height: Option<f32>,
}

impl Svg {
    /// Outputs a `[usvg::Tree]` of the `svg_data` contained within the `Svg`.
    ///
    /// # Errors
    ///
    /// Will error if the conversion to `[usvg::Tree]` fails.
    #[inline]
    pub fn get_tree(&self) -> Result<Tree, USvgParseError> {
        Tree::from_data(self.data.as_bytes(), &USvgParseOptions::default())
    }

    /// Returns the XML data within the `Svg`.
    #[inline]
    #[must_use]
    pub fn get_data(&self) -> String {
        self.data.clone()
    }

    /// Returns a mutable reference to the XML data within the `Svg`.
    #[inline]
    #[must_use]
    pub fn get_data_mut(&mut self) -> &mut String {
        &mut self.data
    }

    /// Sets the XML data witin the `Svg`.
    #[inline]
    pub fn set_data(&mut self, new_data: &str) {
        self.data = new_data.to_owned();
    }

    /// Gets the original height of the SVG before scaling.
    #[inline]
    #[must_use]
    pub fn get_original_height(&self) -> Option<f32> {
        self.original_height
    }

    /// Gets the original width of the SVG before scaling.
    #[inline]
    #[must_use]
    pub fn get_original_width(&self) -> Option<f32> {
        self.original_width
    }
}

impl Serialize for Svg {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        //TODO: handle filepaths
        serializer.serialize_str(&self.data)
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
        let mut svg: Svg = if let Ok(path) = PathBuf::from_str(v) {
            let canonical_path = path
                .canonicalize()
                .map_err(|err| E::custom(format!("failed to canonicalize filepath: {err}")))?;
            let image_bytes = fs::read(&canonical_path).map_err(|err| E::custom(format!("failed to read file: {err}")))?;
            let image_str = str::from_utf8(&image_bytes)
                .map_err(|err| E::custom(format!("failed to parse image bytes into UTF8 string: {err}")))?
                .to_owned();
            Svg {
                data: image_str,
                filepath: Some(canonical_path),
                original_width: None,
                original_height: None,
            }
        }
        //If filepath parsing fails, it should be an SVG
        else {
            trace! {"failed to parse {v} as path"};
            Svg {
                data: v.to_owned(),
                filepath: None,
                original_width: None,
                original_height: None,
            }
        };
        validate_and_update_svg(&mut svg).map_err(|err| E::custom(format!("Invalid SVG: {err}")))?;

        Ok(svg)
    }
}

impl Default for Svg {
    #[inline]
    fn default() -> Self {
        // modified from https://upload.wikimedia.org/wikipedia/commons/d/dd/Achtung.svg
        //
        // This graphic may be updated at any time and should not be relied on in application
        // usage.
        let default_svg_string = r##"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg xmlns="http://www.w3.org/2000/svg" xml:space="preserve" height="550.45" width="627.77" viewBox="0 0 627.77 550.45">
<path fill="#ea0000" d="m614.57 504.94l-279.4-483.94c-4.38-7.588-12.47-12.262-21.23-12.262s-16.85 4.674-21.23 12.258l-279.41 483.94c-4.375 7.58-4.375 16.93 0.003 24.52 4.379 7.58 12.472 12.25 21.23 12.25h558.81c8.76 0 16.86-4.67 21.23-12.25 4.38-7.59 4.38-16.94 0-24.52z"/>
<polygon points="93.977 482.88 533.9 482.88 313.94 101.89" fill="#fff"/>
<path d="m291.87 343.36c1.21 11.49 3.21 20.04 6.02 25.66 2.81 5.63 7.82 8.43 15.04 8.43h2.01c7.22 0 12.24-2.8 15.04-8.43 2.81-5.62 4.82-14.17 6.02-25.66l6.42-88.75c1.21-17.3 1.81-29.71 1.81-37.25 0-10.25-2.91-18.25-8.73-23.99-5.53-5.46-13.38-8.59-21.56-8.59s-16.04 3.13-21.57 8.59c-5.81 5.74-8.72 13.74-8.72 23.99 0 7.54 0.6 19.95 1.8 37.25l6.42 88.75z"/>
<circle cy="430.79" cx="313.94" r="30.747"/>
</svg>
        "##;
        Svg {
            data: default_svg_string.to_owned(),
            filepath: None,
            original_width: Some(627.77),
            original_height: Some(550.45),
        }
    }
}

//TODO: Decide if it makes sense to keep this or not.
#[expect(clippy::missing_trait_methods, reason = "using default impl")]
impl PartialEq for Svg {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.filepath == other.filepath
    }
}

impl FromStr for Svg {
    type Err = Infallible;

    //TODO: validation
    //TODO: parse out viewBox width and height and add it here.
    /// Creates a `Svg` from a string representation of an Svg. No validation is performed
    /// currently.
    #[inline]
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            data: text.to_owned(),
            filepath: None,
            original_width: None,
            original_height: None,
        })
    }
}
/// Validate that `Svg.data` contains valid SVG data.
///
/// Updates the dimension fields of an SVG from the parsed data.
#[expect(clippy::result_large_err, reason = "Yes it is, deal with it.")]
fn validate_and_update_svg(svg: &mut Svg) -> Result<(), Error> {
    //Let usvg do some initial SVG validation for us.
    //
    //roxmltree parsing will error on duplicate attributes...
    let _: usvg::Tree = usvg::Tree::from_str(&svg.data, &usvg::Options::default())?;

    let reader = EventReader::from_str(&svg.data);
    trace! {"updating SVG height and width."}

    for event in reader {
        #[expect(clippy::shadow_reuse, reason = "unwrapping error")]
        let event = event?.clone();

        // Using match here in case I ever need to check or update more things here.
        match event {
            ReaderEvent::StartElement { name, attributes, .. } if name.local_name == "svg" => {
                for attr in attributes {
                    // Using match here in case I ever need to check or update more things here.
                    #[expect(clippy::single_match, reason = "Futureproofing")]
                    #[expect(clippy::indexing_slicing, reason = "already validated in function")]
                    match attr.name.local_name.as_str() {
                        "viewBox" => {
                            // I don't like that I can't chain these two into one method
                            // call chain
                            let temp = attr.value.replace(',', " ");
                            let values: Vec<f32> = temp
                                .split_whitespace()
                                .map(|num| f32::from_str(num).unwrap_or(f32::NAN))
                                .collect();
                            if values.len() > 4 {
                                return Err(SVGValidationError::InvalidViewPort(
                                    "viewPort contained more than 4 sub-attributes".to_owned(),
                                )
                                .into());
                            }
                            if values.contains(&f32::NAN) {
                                return Err(SVGValidationError::InvalidNumber.into());
                            }
                            // At this point we should have a vector of 4 "valid" f64 numbers
                            //
                            // Require viewport to start from top left
                            if values[0] != 0.0_f32 || values[1] != 0.0_f32 {
                                return Err(
                                    SVGValidationError::InvalidViewPort("viewPort does not start at 0 0".to_owned()).into(),
                                );
                            }
                            svg.original_width = Some(values[2]);
                            svg.original_height = Some(values[3]);
                        }
                        // Don't care about other attributes currently
                        _ => {}
                    }
                }
            }
            // don't care about any other events currently here
            //#[expect(clippy::wildcard_enum_match_arm, reason = "Only care about StartElement, but using match for ergonomics")]
            _ => {}
        }
    }
    Ok(())
}
