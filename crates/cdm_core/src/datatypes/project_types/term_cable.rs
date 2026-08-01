use std::path::{Path, PathBuf};
use std::collections::BTreeMap;
use core::cmp::Ordering;

use egui::Pos2;
use log::trace;

use crate::{
    datatypes::{
        file_types,
        library_types::Library,
        library_types::term_cable_type::TermCableType,
        library_types::term_cable_type::self,
        project_types::ProjectData,
        project_types::connector::Connector,
        library_types::cable_type::{self, CableLayer},
        schematic_connector::{AsConnector, right_angle::RightAngle},
        unit_helper::length::Length,
        util_types::{IECCodes, PhysicalLocation, UserFields, LineStyle},
    },
    error::LibraryError,
    error::CableTypeError,
    traits::FromFile,
};

//TODO: add cores field to TermCable like cable

/// `TermCable` represents a particular instance of a `TermCableType`.
/// It represents a physical item.
#[derive(Debug, PartialEq, Clone)]
#[expect(clippy::partial_pub_fields, reason = "contained_datafile_path is not part of public API")]
pub struct TermCable {
    /// The `TermCableType` of this instance.
    pub term_cable_type: String,
    /// The structured name of the `TermCable` instance.
    pub identifier: String,
    /// Optional description.
    pub description: Option<String>,
    /// Length of `TermCable`, copied from `Library` data.
    length: Length,
    /// Pathway containing instance.
    pub pathway: Option<String>,
    /// physical location of `TermCable`.
    pub physical_location: Option<PhysicalLocation>,
    /// Fields for use with IEC project coding.
    pub iec_codes: Option<IECCodes>,
    /// User defined fields.
    pub user_fields: Option<UserFields>,
    /// The cores in this cable. Generated from the data in the associcated `CableType` of the
    /// `TermCableType`.
    ///
    /// Key of map is identifier of core within cable, and is unique within each cable.
    ///
    /// NOTE: this is a flat map of cores within cable, with dot joined ids. This does not maintain
    /// the internal structure of the cable.
    pub(crate) cores: BTreeMap<String, Core>,
    /// Flat map of all connectors on end1 of `TermCable`.
    pub(crate) end1_connectors: BTreeMap<String, TerminatedConnector>,
    /// Flat map of all connectors on end2 of `TermCable`.
    pub(crate) end2_connectors: BTreeMap<String, TerminatedConnector>,
    /// The `LineStyle` of this cable. Initially copied from the `CableType`.
    pub(crate) line_style: LineStyle,
    /// vector of exterior insulation/shielding layers. Copied from the `CableType` of the
    /// `TermCableType`.
    pub(crate) layers: Vec<CableLayer>,
    /// datafile the struct instance was read in from.
    pub(crate) contained_datafile_path: PathBuf,
}

impl From<file_types::term_cable::TermCable> for TermCable {
    #[inline]
    fn from(value: file_types::term_cable::TermCable) -> Self {
        Self {
            term_cable_type: value.term_cable_type,
            identifier: value.identifier,
            description: value.description,
            length: Length::default(),
            pathway: value.pathway,
            physical_location: value.physical_location,
            iec_codes: value.iec_codes,
            user_fields: value.user_fields,
            cores: BTreeMap::new(),
            end1_connectors: BTreeMap::new(),
            end2_connectors: BTreeMap::new(),
            line_style: LineStyle::default(),
            layers: Vec::new(),
            contained_datafile_path: PathBuf::new(),
        }
    }
}

For any connector, treat it similar to a piece of equipment where it has defined connection points in the schematic symbol.

Also have it contain a list of mating connectors from the connector type

a cable can terminate to either a connector, a connection point on a piece of equipment, or a terminal

TermCables have the connectors included, but the cores are identical to normal cables

//TODO: for schematic connector implementation, look at which cores have all their cores assigned
//to connector pins, and then only create schematic connecors for the cores with connectors.

impl TermCable {
    /// `insert_cores` handles the creation of all the individual `Core`s within a `TermCable` and
    /// nested `Core`s.
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
            let core_type = library.cable_types.get(&ref_core.cable_type).ok_or(LibraryError::ValueNotFound {
                id: ref_core.cable_type.clone(),
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
                        core_id_stripped.to_owned(),
                        Core {
                            cable_type: ref_core.cable_type.clone(),
                            layers: core_type.layers.clone(),
                            line_style,
                            connector,
                        },
                    );
                }
                Ordering::Less => {
                    return Err(CableTypeError::NoCores(ref_core.cable_type.clone()).into());
                }
            }
        }

        if self.cores.is_empty() {
            return Err(CableTypeError::NoCores(self.term_cable_type.clone()).into());
        }

        Ok(())

    }

    /// Insert `Connectors` into `TermCable` and update their `Termination`s.
    /// # Errors
    ///
    /// Will error if values needed in the function are not found in project or library data.
    #[inline]
    fn insert_connectors(&mut self, library: &Library, term_cable_type: &TermCableType) -> Result<(), LibraryError> {
        for (id, ref_connector) in &term_cable_type.end1 {
            let mut terminations: Vec<Termination> = Vec::new();
            for termination in &ref_connector.terminations {
                if self.cores.contains_key(&termination.core) {
                    terminations.push(termination.clone().into());
                }

            }
            let 
        }

        for (id, ref_connector) in &term_cable_type.end2 {
            
        }

        

        Ok(())

    }

    /// Updates internal fields of `TermCable` instance from library data.
    ///
    /// # Errors
    ///
    /// Will return errors if values needed in the function are not found in project or library
    /// data.
    #[inline]
    pub fn update_data_from_library(&mut self, library: &Library) -> Result<(), LibraryError> {
        let term_cable_type = library.term_cable_types.get(&self.term_cable_type).ok_or(LibraryError::ValueNotFound {
                id: self.term_cable_type.clone(),
                found_in: format!("term cable instance {}", self.identifier).to_owned(),
                library_type: "Term Cable Type".to_owned(),
            })?;

        let underlying_cable_type = library.cable_types.get(&term_cable_type.cable_type).ok_or(LibraryError::ValueNotFound {
                id: term_cable_type.cable_type.clone(),
                found_in: format!("term cable instance {}", self.identifier).to_owned(),
                library_type: "Term Cable Type".to_owned(),
            })?;


        self.line_style = term_cable_type.line_style.clone();

        self.layers = underlying_cable_type.layers.clone();

        self.cores = BTreeMap::new();

        self.insert_cores(&underlying_cable_type.cores, library, None)?;

        self.insert_connectors(library, term_cable_type)?;

        Ok(())
    }

    /// length of `TermCableType`.
    ///
    /// # Errors
    ///
    /// Will error if `term_cable_type` id not found in provided library.
    #[inline(never)]
    pub fn len(&self, library: &Library) -> Result<Length, LibraryError> {
        let term_cable_type = library
            .term_cable_types
            .get(&self.term_cable_type)
            .ok_or(LibraryError::ValueNotFound {
                id: self.term_cable_type.clone(),
                //TODO: figure out how to insert the ID of the term cable here
                found_in: "term_cable".to_owned(),
                library_type: "Term Cable Type".to_owned(),
            })?;

        Ok(term_cable_type
            .actual_length
            .clone()
            .unwrap_or(term_cable_type.nominal_length.clone().unwrap_or_default()))
    }
}

//TODO: Provide a configuration option to render all cores of cable, or just ones fully terminated
//in a connector.
//
//Need to figure out a good way to draw cables that don't have the same number of terminations on
//each end.
//
//The configuration option would select between rendering `Core`s or `ConnectorCore`s.
/// `Core` represents a core of a `Cable`.
///
/// Connector may not be populated on all cores based on configuration options.
#[expect(clippy::partial_pub_fields, reason = "connector is not part of public API")]
#[derive(Debug, PartialEq, Clone)]
pub struct Core {
    /// The `CableType` key of this core.
    pub cable_type: String,
    /// The `LineStyle` of this cable. Initially copied from the `CableType`.
    pub line_style: LineStyle,
    /// vector of exterior insulation/shielding layers. Copied from the `CableType`.
    pub layers: Vec<CableLayer>,
    /// Schematic connector that is used to render this `Core`.
    ///
    /// Only used when rendering `Core`s directly, not `ConnectorCore`s.
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

//TODO: look at how to better implement this in a generic way for normal connector::Connector's, so
//we can show which pins a particular cable is connected to in general for field terminated cables.

/// `Connector` represents a connector on one end of a `TermCable`.
#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
pub struct TerminatedConnector {
    /// `connector` represents a connector that is on the end of a `TermCable`.
    pub connector: Connector,
    /// `terminations` represents the pin/core mapping for this connector.
    pub terminations: Vec<Termination>
}

/// [`Termination`] represents the connections between a pin of an individual
/// [`Connector`] and the individual core of the cable.
#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
pub struct Termination {
    /// `Core` represents which individual core inside a cable this pin is connected to.
    ///
    /// This is the id used in the TOML file.
    pub core: String,
    /// `Pin` represents which pin in the associated connector the core is connected to.
    pub pin: String,
}

impl From<term_cable_type::Termination> for Termination {
    #[inline]
    fn from(value: term_cable_type::Termination) -> Self {
        Self {
            core: value.core, 
            pin: value.pin,
        }
    }
}

impl FromFile for TermCable {
    #[inline]
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
    #[inline]
    fn set_datafile(&mut self, datafile_path: &Path) {
        self.contained_datafile_path = datafile_path.to_path_buf();
    }
}
impl ProjectData for TermCable {}

impl ProjectData for Core {}
