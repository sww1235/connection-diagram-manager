use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use uom::si::length::millimeter;
use usvg::Tree;

use crate::{
    datatypes::{
        library_types::Library,
        svg::Svg,
        unit_helper::Length,
        util_types::{IECCodes, PhysicalLocation, UserFields},
    },
    error::Error,
    traits::FromFile,
};

/// `MountingRail` represents an individual mounting rail in a project
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::partial_pub_fields)]
pub struct MountingRail {
    /// ID of type of mounting rail
    pub mounting_rail_type: String,
    /// Length of mounting rail
    pub length: Length,
    /// physical location of `MountingRail`
    pub physical_location: Option<PhysicalLocation>,
    /// Fields for use with IEC project coding
    pub iec_codes: Option<IECCodes>,
    /// User defined fields
    pub user_fields: Option<UserFields>,
    /// datafile the struct instance was read in from
    #[serde(skip)]
    pub(crate) contained_datafile_path: PathBuf,
}

impl FromFile for MountingRail {
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
    fn set_datafile(&mut self, datafile_path: &Path) {
        self.contained_datafile_path = datafile_path.to_path_buf();
    }
}

impl MountingRail {
    /// Output a generated SVG either based on the parameters in `MountingRail` and
    /// `MountingRailType` or the `start_image`, `middle_image` and `end_image` parameters
    ///
    /// # Errors
    ///
    /// Will error if `mounting_rail_type` not found in library
    #[expect(clippy::format_push_string)]
    pub fn vis_rep(&self, library: &Library) -> Result<Svg, Error> {
        // because usvg is a read only parsing library, I can't build the SVG programatically and
        // instead have to bastardize creation of it via string concatenation and parsing

        let rail_type = library
            .mounting_rail_types
            .get(&self.mounting_rail_type)
            .ok_or(Error::LibraryValueNotFound {
                id: self.mounting_rail_type.clone(),
                library_type: "Mounting Rail Type".to_string(),
            })?;
        let mut svg_string = String::new();

        if rail_type.start_image.is_some() && rail_type.middle_image.is_some() && rail_type.end_image.is_some() {
            //TODO: decide if this even makes sense to keep this option.
            //
            //TODO: check and error on only some of these specified
        } else {
            // using mm here for all units within generated SVG
            let rail_height = rail_type.rail_height.value.get::<millimeter>();
            let rail_length = self.length.value.get::<millimeter>();
            svg_string.push_str("<svg version=\"1.1\"\n");
            svg_string.push_str(&format!("width=\"{rail_length}mm\" height=\"{rail_height}mm\""));
            svg_string.push_str("xmlns=\"http://www.w3.org/2000/svg\">");
            // push outer rectangle
            svg_string.push_str("<rect width=\"{rail_length}mm\" height=\"{rail_height}mm\" />");
            if rail_type.rail_center_height.value < rail_type.rail_height.value {
                //TODO: render smaller rectangle here
            }
            if rail_type.slots {
                //TODO calculate numebr of slots
                //TODO loop through slots {
                if rail_type.rounded_slots {
                    //TODO: render ovals
                } else {
                    //TODO: render rectangles
                }
                //}
            }
            svg_string.push_str("</svg>");
        }
        let output = Svg::from_tree(Tree::from_str(&svg_string, &Svg::parse_options())?);
        Ok(output)
    }
    /// If rail length is standard
    ///
    /// # Errors
    ///
    /// Will error if `mounting_rail_type` is not found in library
    pub fn is_standard_length(&self, library: &Library) -> Result<bool, Error> {
        let rail_type = library
            .mounting_rail_types
            .get(&self.mounting_rail_type)
            .ok_or(Error::LibraryValueNotFound {
                id: self.mounting_rail_type.clone(),
                library_type: "Mounting Rail Type".to_string(),
            })?;
        Ok(rail_type.standard_rail_length.value == self.length.value)
    }
}
