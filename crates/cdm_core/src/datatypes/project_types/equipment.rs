use core::mem;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use log::trace;
use serde::{Deserialize, Serialize};
use xml::{
    EventReader,
    EventWriter,
    attribute::OwnedAttribute,
    reader::XmlEvent as ReaderEvent,
    writer::XmlEvent as WriterEvent,
};

use crate::{
    datatypes::{
        library_types::Library,
        svg::Svg,
        util_types::{IECCodes, PhysicalLocation, SymbolStyle, UserFields},
    },
    error::{Error, LibraryError, SVGModificationError},
    traits::{FromFile, ProjectData, SchematicRepresentation},
};

/// `Equipment` represents a particular instance of an `EquipmentType`.
/// This is the physical unit you would hold in your hand.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::partial_pub_fields, reason = "contained_datafile_path is not part of public API")]
pub struct Equipment {
    /// The type of equipment of the instance.
    pub equipment_type: String,
    /// The structured name of the equipment.
    pub identifier: String,
    /// The particular mounting type of this instance
    /// must be in list of mounting types defined in `equip_type.mounting_type`.
    ///
    /// Validated on import.
    pub mounting_type: Option<String>,
    /// The containing `Enclosure` ID.
    pub enclosure: Option<String>,
    /// The ID of the `MountPoint` within the `Enclosure`.
    pub mount_point: Option<String>,
    /// The physical location of this piece of equipment.
    pub physical_location: Option<PhysicalLocation>,
    /// fields for IEC coding.
    pub iec_codes: Option<IECCodes>,
    /// Description.
    pub description: Option<String>,
    /// Optional user Fields.
    pub user_fields: Option<UserFields>,
    /// Optional styling data for schematic symbol.
    pub symbol_style: Option<SymbolStyle>,
    /// datafile the struct instance was read in from.
    #[serde(skip)]
    pub(crate) contained_datafile_path: PathBuf,
}

impl FromFile for Equipment {
    #[inline]
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
    #[inline]
    fn set_datafile(&mut self, datafile_path: &Path) {
        self.contained_datafile_path = datafile_path.to_path_buf();
    }
}

impl Equipment {
    //TODO: Figure out better way of storing and referring to connectors on equipment
    //pub fn connector_list(&self, library: &Library) -> Result<BTreeMap<String, FaceConnector>,
    // LibraryError> {
    //
    //}
}

impl SchematicRepresentation for Equipment {
    #[inline(never)]
    fn schematic_symbol(&self, library: &Library, symbol_selector: Option<usize>) -> Result<(Svg, String), Error> {
        let equipment_type = library
            .equipment_types
            .get(&self.equipment_type)
            .ok_or(LibraryError::ValueNotFound {
                id: self.equipment_type.clone(),
                found_in: format!("equipment instance {}", self.identifier).to_owned(),
                library_type: "Equipment Type".to_owned(),
            })?;

        let equipment_schematic_symbols = equipment_type.schematic_symbols.clone();

        if equipment_schematic_symbols.is_empty() {
            return Err(LibraryError::DataMissing {
                id: self.equipment_type.clone(),
                found_in: format!("equipment instance {}", self.identifier).to_owned(),
                library_type: "Equipment Type".to_owned(),
                data_missing: "Schematic Symbols".to_owned(),
            }
            .into());
        }

        let schematic_symbol_type_id =
            equipment_schematic_symbols
                .get(symbol_selector.unwrap_or(0))
                .ok_or(LibraryError::DataMissing {
                    id: self.equipment_type.clone(),
                    found_in: format!("equipment instance {}", self.identifier).to_owned(),
                    library_type: "Equipment Type".to_owned(),
                    data_missing: "At least one schematic symbol needs to be specified".to_owned(),
                })?;

        let mut schematic_symbol = library
            .schematic_symbol_types
            .get(schematic_symbol_type_id)
            .ok_or(LibraryError::ValueNotFound {
                id: schematic_symbol_type_id.clone(),
                found_in: format!("equipment instance {}", self.identifier).to_owned(),
                library_type: "Schematic Symbol".to_owned(),
            })?
            .visual_representation
            .clone();

        let uri = format!("bytes://{schematic_symbol_type_id}.svg").to_string();

        self.update_symbol_data(library, &mut schematic_symbol)?;

        Ok((schematic_symbol, uri))
    }

    #[inline(never)]
    #[expect(clippy::too_many_lines, reason = "Its a long function, deal with it.")]
    fn update_symbol_data(&self, library: &Library, svg: &mut Svg) -> Result<(), Error> {
        // TODO: look at reader config
        let svg_data = &svg.get_data();
        let equipment_type = library
            .equipment_types
            .get(&self.equipment_type)
            .ok_or(LibraryError::ValueNotFound {
                id: self.equipment_type.clone(),
                found_in: format!("equipment instance {}", self.identifier).to_owned(),
                library_type: "Equipment Type".to_owned(),
            })?;
        let mut reader = EventReader::from_str(svg_data).into_iter().peekable();
        let mut out_buffer: Vec<u8> = Vec::new();

        //TODO: look at writer config
        let mut writer = EventWriter::new(&mut out_buffer);

        trace! {"updating symbol data"};

        while let Some(event) = reader.next() {
            #[expect(clippy::shadow_reuse, reason = "unwrapping error")]
            let event = event?;
            // If next_event is None, this is the last element in the iterator
            // TODO: check if event = EndDocument and error if not here.
            let Some(next_event) = reader.peek() else { continue };
            #[expect(clippy::shadow_reuse, reason = "unwrapping error")]
            let next_event = next_event.clone()?;
            trace! {"Event: {event:#?}"};
            trace! {"Next Event: {next_event:#?}"};

            match event {
                #[expect(clippy::ref_patterns, reason = "can't get this to work otherwise")]
                ReaderEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                } => {
                    trace! {"StartElement name: {}", name.local_name};

                    // Then finally write out character data with updated values after writing
                    // start element

                    // Modify tagged text elements within SVG

                    // first check for text element
                    if &name.local_name == "text" {
                        // check if next event is a character event
                        // and skip if it is
                        //
                        // this should skip the current and next elements in the iterator.
                        //
                        // The current element is already saved via the let event = line above,
                        // in theory.
                        //
                        // https://stackoverflow.com/a/59045627
                        if mem::discriminant(&next_event) == (mem::discriminant(&ReaderEvent::Characters(String::new()))) {
                            reader.nth(1);
                        }
                        //https://stackoverflow.com/a/37700229
                        //
                        // Checking for duplicate attributes
                        let unique_attributes: HashSet<_> = attributes.iter().map(|attr| attr.name.local_name.as_str()).collect();

                        if unique_attributes.len() < attributes.len() {
                            //TODO: return list of duplicate attributes here as well
                            return Err(SVGModificationError::DuplicateAttributes.into());
                        }

                        // Validate that only 1 data attribute is present that affects the
                        // value of the SVG text element

                        let mut duplicate_attr_detected = false;
                        for attr in attributes {
                            if duplicate_attr_detected {
                                return Err(SVGModificationError::ConflictingAttributes.into());
                            }
                            //TODO: validate that all data- attributes should be unique
                            duplicate_attr_detected = attr.name.local_name.starts_with("data-");
                        }

                        // Now finally loop through attributes and update text value where
                        // appropriate.

                        // the else path here should never happen.
                        let Some(writer_output) = event.as_writer_event() else {
                            continue;
                        };
                        writer.write(writer_output)?;
                        for attr in attributes {
                            match attr.name.local_name.as_str() {
                                "data-ref-des" => {
                                    let character_event = WriterEvent::Characters(&self.identifier);
                                    writer.write(character_event)?;
                                }
                                "data-manufacturer" => {
                                    let character_event = WriterEvent::Characters(
                                        &equipment_type
                                            .catalog
                                            .as_ref()
                                            .map_or(String::new(), |catalog| catalog.manufacturer.clone().unwrap_or_default()),
                                    );
                                    writer.write(character_event)?;
                                }
                                "data-model" => {
                                    let character_event = WriterEvent::Characters(
                                        &equipment_type
                                            .catalog
                                            .as_ref()
                                            .map_or(String::new(), |catalog| catalog.model.clone().unwrap_or_default()),
                                    );
                                    writer.write(character_event)?;
                                }
                                "data-description" => {
                                    let character_event = WriterEvent::Characters(&self.description.clone().unwrap_or_default());
                                    writer.write(character_event)?;
                                }
                                "data-installation" => {
                                    let character_event = WriterEvent::Characters(
                                        &self
                                            .iec_codes
                                            .as_ref()
                                            .map_or(String::new(), |codes| codes.installation.clone().unwrap_or_default()),
                                    );
                                    writer.write(character_event)?;
                                }
                                "data-location" => {
                                    let character_event = WriterEvent::Characters(
                                        &self
                                            .iec_codes
                                            .as_ref()
                                            .map_or(String::new(), |codes| codes.location.clone().unwrap_or_default()),
                                    );
                                    writer.write(character_event)?;
                                }
                                "data-rating" => {
                                    let character_event =
                                        WriterEvent::Characters(&equipment_type.rating.clone().unwrap_or_default());
                                    writer.write(character_event)?;
                                }
                                // other attributes
                                _ => {}
                            }
                        }
                    }
                }
                // EndDocument is the only Discriminent currently that is None
                ReaderEvent::StartDocument { .. }
                | ReaderEvent::EndDocument
                | ReaderEvent::ProcessingInstruction { .. }
                | ReaderEvent::EndElement { .. }
                | ReaderEvent::Doctype { .. }
                | ReaderEvent::CData(..)
                | ReaderEvent::Comment(..)
                | ReaderEvent::Characters(..)
                | ReaderEvent::Whitespace(..) => {
                    let Some(writer_output) = event.as_writer_event() else {
                        continue;
                    };
                    writer.write(writer_output)?;
                }
            }
        }

        let output_string = str::from_utf8(&out_buffer)?.to_owned();
        //trace!{"{}", output_string};

        svg.set_data(&output_string);

        Ok(())
    }
}

impl ProjectData for Equipment {}
