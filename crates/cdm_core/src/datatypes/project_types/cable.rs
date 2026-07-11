use core::cmp::Ordering;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use egui::Pos2;
use log::trace;

//use slotmap::Key as _;
use crate::{
    datatypes::{
        file_types,
        library_types::{Library, cable_type, cable_type::CableLayer},
        project_types::ProjectData,
        schematic_connector::{AsConnector, right_angle::RightAngle},
        unit_helper::length::Length,
        util_types::{IECCodes, LineStyle, PhysicalLocation, UserFields},
    },
    error::{CableTypeError, Error, LibraryError},
    traits::FromFile,
};

/// `Cable` represents a particular instance of a `CableType`
/// It represents a physical item.
#[derive(Debug, PartialEq, Clone)]
#[expect(clippy::partial_pub_fields, reason = "contained_datafile_path is not part of public API")]
pub struct Cable {
    /// The `CableType` key of this instance.
    pub cable_type: String,
    /// The structured name of the `Cable` instance.
    pub identifier: String,
    /// Optional description.
    pub description: Option<String>,
    /// length of wire or cable.
    pub length: Length,
    /// Pathway key containing instance.
    pub pathway: Option<String>,
    /// physical location of Cable.
    pub physical_location: Option<PhysicalLocation>,
    /// Fields for use with IEC project coding.
    pub iec_codes: Option<IECCodes>,
    /// User defined fields.
    pub user_fields: Option<UserFields>,
    /// The cores in this cable. Generated from the data in the associcated `CableType`.
    ///
    /// Key of map is identifier of core within cable, and is unique within each cable.
    ///
    /// NOTE: this is a flat map of cores within cable, with dot joined ids. This does not maintain
    /// the internal structure of the cable.
    pub(crate) cores: BTreeMap<String, Core>,
    //TODO: rename connector and connectortype to something more distinct
    ///// The schematic representation of this cable.
    //pub(crate) connector: ConnectorType,
    /// The `LineStyle` of this cable. Initially copied from the `CableType`.
    pub(crate) line_style: LineStyle,
    /// vector of exterior insulation/shielding layers. Copied from the `CableType`.
    pub(crate) layers: Vec<CableLayer>,
    /// datafile the struct instance was read in from.
    pub(crate) contained_datafile_path: PathBuf,
}

impl From<file_types::cable::Cable> for Cable {
    #[inline]
    fn from(value: file_types::cable::Cable) -> Self {
        Self {
            cable_type: value.cable_type,
            identifier: value.identifier,
            description: value.description,
            length: value.length,
            pathway: value.pathway,
            physical_location: value.physical_location,
            iec_codes: value.iec_codes,
            user_fields: value.user_fields,
            cores: BTreeMap::new(),
            //connector: ConnectorType::MultiRightAngle(MultiRightAngle::default()),
            line_style: LineStyle::default(),
            layers: Vec::new(),
            contained_datafile_path: PathBuf::new(),
        }
    }
}

//impl AsConnector for Cable {
//    type Output = MultiRightAngle;
//
//    #[inline]
//    fn connector(&self) -> Self::Output {
//        #[expect(
//            clippy::wildcard_enum_match_arm,
//            reason = "Only one option implemented at this point, but others may be in the future."
//        )]
//        match &self.connector {
//            ConnectorType::MultiRightAngle(mra) => mra.clone(),
//            #[expect(clippy::panic, reason = "The wildcard arm of the match should never happen currently")]
//            _ => panic!(),
//        }
//    }
//
//    #[inline]
//    fn connector_mut(&mut self) -> &mut Self::Output {
//        #[expect(
//            clippy::wildcard_enum_match_arm,
//            reason = "Only one option implemented at this point, but others may be in the future."
//        )]
//        match &mut self.connector {
//            ConnectorType::MultiRightAngle(mra) => mra,
//            #[expect(clippy::panic, reason = "The wildcard arm of the match should never happen currently")]
//            _ => panic!(),
//        }
//    }
//
//    #[inline]
//    fn set_end1_position(&mut self, position: Pos2) {
//        #[expect(
//            clippy::wildcard_enum_match_arm,
//            reason = "Only one option implemented at this point, but others may be in the future."
//        )]
//        match &mut self.connector {
//            //TODO: update this to use a method directly on mra.
//            ConnectorType::MultiRightAngle(mra) => mra.end1_junction.set_position(position),
//            #[expect(clippy::panic, reason = "The wildcard arm of the match should never happen currently")]
//            _ => panic!(),
//        }
//    }
//    #[inline]
//    fn set_end2_position(&mut self, position: Pos2) {
//        #[expect(
//            clippy::wildcard_enum_match_arm,
//            reason = "Only one option implemented at this point, but others may be in the future."
//        )]
//        match &mut self.connector {
//            //TODO: update this to use a method directly on mra.
//            ConnectorType::MultiRightAngle(mra) => mra.end2_junction.set_position(position),
//            #[expect(clippy::panic, reason = "The wildcard arm of the match should never happen currently")]
//            _ => panic!(),
//        }
//    }
//    #[inline]
//    fn end1_position(&self) -> Pos2 {
//        #[expect(
//            clippy::wildcard_enum_match_arm,
//            reason = "Only one option implemented at this point, but others may be in the future."
//        )]
//        match &self.connector {
//            //TODO: update this to use a method directly on mra.
//            ConnectorType::MultiRightAngle(mra) => mra.end1_junction.position(),
//            #[expect(clippy::panic, reason = "The wildcard arm of the match should never happen currently")]
//            _ => panic!(),
//        }
//    }
//    #[inline]
//    fn end2_position(&self) -> Pos2 {
//        #[expect(
//            clippy::wildcard_enum_match_arm,
//            reason = "Only one option implemented at this point, but others may be in the future."
//        )]
//        match &self.connector {
//            //TODO: update this to use a method directly on mra.
//            ConnectorType::MultiRightAngle(mra) => mra.end2_junction.position(),
//            #[expect(clippy::panic, reason = "The wildcard arm of the match should never happen currently")]
//            _ => panic!(),
//        }
//    }
//
//    //TODO: add method to update connection reference ids from Project.
//}

//TODO: provide a way to print core_ids for a cable
impl Cable {
    /// `insert_cores` handles the creation of all the individual cables inside a cable and
    /// its cores.
    ///
    /// `super_id` passes through the core ID of the previous iteration so it gets concatenated
    /// correctly.
    ///
    /// # Errors
    ///
    /// Will error if values needed in the function are not found in project or library data.
    #[inline]
    fn insert_cores(
        &mut self,
        reference_cores: &BTreeMap<String, cable_type::Core>,
        library: &Library,
        super_id: Option<&str>,
    ) -> Result<(), LibraryError> {
        for (id, ref_core) in reference_cores {
            let core_type = library.cable_types.get(&ref_core.type_id).ok_or(LibraryError::ValueNotFound {
                id: ref_core.type_id.clone(),
                found_in: format!("cable instance {}", self.identifier).to_owned(),
                library_type: "Cable Type".to_owned(),
            })?;

            let core_id = format! {"{}.{id}", super_id.unwrap_or_default()};
            // migrate to trim_prefix() once stablized
            // https://github.com/rust-lang/rust/issues/142312
            let core_id_stripped = core_id.strip_prefix('.').unwrap_or(&core_id);
            trace! {"creating core for cable core {core_id_stripped} of {}", self.identifier}
            match core_type.cores.len().cmp(&1) {
                Ordering::Greater => {
                    // recursive call to handle inner cables
                    self.insert_cores(&core_type.cores, library, Some(core_id_stripped))?;
                }
                Ordering::Equal => {
                    let line_style = ref_core.line_style.clone().unwrap_or(core_type.line_style.clone());
                    let connector = RightAngle {
                        line_style: line_style.clone(),
                        ..RightAngle::default()
                    };
                    self.cores.insert(
                        id.to_owned(),
                        Core {
                            cable_type: ref_core.type_id.clone(),
                            layers: core_type.layers.clone(),
                            line_style,
                            connector,
                        },
                    );
                }
                Ordering::Less => {
                    return Err(CableTypeError::NoCores(ref_core.type_id.clone()).into());
                }
            }
        }

        if self.cores.is_empty() {
            return Err(CableTypeError::NoCores(self.cable_type.clone()).into());
        }

        Ok(())
    }

    /// Updates internal fields of `Cable` instance from library data.
    ///
    /// # Errors
    ///
    /// Will return errors if values needed in the function are not found in project or library
    /// data.
    #[inline]
    #[expect(clippy::result_large_err, reason = "deal with it")]
    pub fn update_data_from_library(&mut self, library: &Library) -> Result<(), Error> {
        let cable_type = library.cable_types.get(&self.cable_type).ok_or(LibraryError::ValueNotFound {
            id: self.cable_type.clone(),
            found_in: format!("cable instance {}", self.identifier).to_owned(),
            library_type: "Cable Type".to_owned(),
        })?;

        self.line_style = cable_type.line_style.clone();

        self.layers = cable_type.layers.clone();

        self.cores = BTreeMap::new();

        // passing outermost list of cores
        self.insert_cores(&cable_type.cores, library, None)?;

        //match self.cores.len().cmp(&1) {
        //    Ordering::Equal => {
        //        let connector_inst = RightAngle {
        //            line_style: self.line_style.clone(),
        //            ..Default::default()
        //        };
        //        self.connector = ConnectorType::RightAngle(connector_inst);
        //    }
        //    Ordering::Less => {
        //        return Err(LibraryError::from(CableTypeError::NoCores(self.cable_type.clone())).into());
        //    }
        //    Ordering::Greater => {
        //        let mut connector_inst = MultiRightAngle {
        //            line_style: self.line_style.clone(),
        //            ..Default::default()
        //        };
        //        // flat_map of cores within cable
        //        for (id, core) in &self.cores {

        //        }

        //        self.connector = ConnectorType::MultiRightAngle(connector_inst);
        //    }
        //}

        Ok(())
    }

    /// Returns the flatmap of cores in this cable.
    #[must_use]
    #[inline]
    pub fn cores(&self) -> BTreeMap<String, Core> {
        self.cores.clone()
    }

    /// Mutable access to flatmap of cores in this cable.
    #[inline]
    pub fn cores_mut(&mut self) -> &mut BTreeMap<String, Core> {
        &mut self.cores
    }

    /// Return identifier of cable core.
    #[must_use]
    #[inline]
    pub fn core_identifier(&self, core_id: &str) -> String {
        let core_identifier = format! {"{}-{}", self.identifier, core_id};
        core_identifier
    }
}

/// `Core` represents a core of a `Cable`.
#[derive(Debug, PartialEq, Clone)]
#[expect(clippy::partial_pub_fields, reason = "connector is not part of public API")]
pub struct Core {
    /// The `CableType` key of this core.
    pub cable_type: String,
    /// The `LineStyle` of this cable. Initially copied from the `CableType`.
    pub line_style: LineStyle,
    /// vector of exterior insulation/shielding layers. Copied from the `CableType`.
    pub layers: Vec<CableLayer>,
    /// the `SchematicConnector` for this core.
    pub(crate) connector: RightAngle,
}

impl AsConnector for Core {
    type Output = RightAngle;

    #[inline]
    fn connector(&self) -> Self::Output {
        self.connector.clone()
    }

    #[inline]
    fn connector_mut(&mut self) -> &mut Self::Output {
        &mut self.connector
    }

    #[inline]
    fn set_end1_position(&mut self, position: Pos2) {
        self.connector.set_end1_position(position);
    }
    #[inline]
    fn set_end2_position(&mut self, position: Pos2) {
        self.connector.set_end2_position(position);
    }
    #[inline]
    fn end1_position(&self) -> Pos2 {
        //TODO: update this to use a method directly on connector.
        self.connector.end1.position()
    }
    #[inline]
    fn end2_position(&self) -> Pos2 {
        //TODO: update this to use a method directly on connector.
        self.connector.end2.position()
    }

    //TODO: add method to update connection reference ids from Project.
}

impl FromFile for Cable {
    #[inline]
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
    #[inline]
    fn set_datafile(&mut self, datafile_path: &Path) {
        self.contained_datafile_path = datafile_path.to_path_buf();
    }
}

impl ProjectData for Cable {}

impl ProjectData for Core {}
