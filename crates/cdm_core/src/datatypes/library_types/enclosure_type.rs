use std::path::{Path, PathBuf};

use crate::{
    datatypes::{
        color::Color,
        file_types,
        library_types::LibraryData,
        svg::Svg,
        unit_helper::length::Length,
        util_types::{Catalog, Dimension},
    },
    traits::FromFile,
};

/// `EnclosureType` represents a type/model of location.
///
/// Examples of `EnclosureType` include junction boxes, racks, panels, etc.
/// It does not include places these are located.
#[derive(Debug, PartialEq, Clone)]
#[expect(clippy::partial_pub_fields, reason = "contained_datafile_path is not part of public API")]
pub struct EnclosureType {
    /// Catalog information.
    pub catalog: Option<Catalog>,
    /// Dimensional information of enclosure.
    pub dimensions: Dimension,
    /// Main material of `EnclosureType`.
    pub material: Option<String>,
    /// Usable Width.
    pub usable_width: Length,
    /// Usable Height.
    pub usable_height: Length,
    /// Usable Depth.
    pub usable_depth: Option<Length>,
    /// Other rating information for enclosure.
    pub rating: Option<String>,
    /// Visual representation of Enclosure.
    pub visual_representation: Svg,
    /// Primary color of enclosure.
    pub color: Option<Color>,
    /// datafile the struct instance was read in from.
    pub(crate) contained_datafile_path: PathBuf,
}

impl From<file_types::enclosure_type::EnclosureType> for EnclosureType {
    #[inline]
    fn from(value: file_types::enclosure_type::EnclosureType) -> Self {
        Self {
            catalog: value.catalog,
            dimensions: value.dimensions,
            material: value.material,
            usable_width: value.usable_width,
            usable_height: value.usable_height,
            usable_depth: value.usable_depth,
            rating: value.rating,
            visual_representation: value.visual_representation.unwrap_or_default(),
            color: value.color,
            contained_datafile_path: PathBuf::new(),
        }
    }
}

impl FromFile for EnclosureType {
    #[inline]
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
    #[inline]
    fn set_datafile(&mut self, datafile_path: &Path) {
        self.contained_datafile_path = datafile_path.to_path_buf();
    }
}

impl LibraryData for EnclosureType {}
