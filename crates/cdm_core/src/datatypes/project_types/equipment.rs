use core::mem;
use std::{
    collections::{BTreeMap, HashSet},
    path::{Path, PathBuf},
};

use egui::Pos2;
use log::{trace, warn};
use slotmap::SparseSecondaryMap;
use xml::{EventReader, EventWriter, reader::XmlEvent as ReaderEvent, writer::XmlEvent as WriterEvent};

use crate::{
    datatypes::{
        file_types,
        library_types::Library,
        project_types::{
            InnerConnectionId,
            Project,
            ProjectData,
            cable::CableCore,
            connection::{EndDesignation, Type as ConnectionType},
        },
        schematic_symbol::{ConnectionDirection, SchematicRepresentation, SchematicSymbol, SymbolConnection},
        util_types::{IECCodes, PhysicalLocation, SymbolStyle, UserFields},
    },
    error::{Error, LibraryError, SVGModificationError, SVGValidationError},
    traits::FromFile,
};

/// `Equipment` represents a particular instance of an `EquipmentType`.
/// This is the physical unit you would hold in your hand.
#[derive(Debug, PartialEq, Clone)]
#[expect(clippy::partial_pub_fields, reason = "some fields are not part of the public API")]
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
    ///
    /// Styling can also be defined in the SVG itself, but will be overrriden if this is defined.
    pub symbol_style: Option<SymbolStyle>,
    /// Stores ids of connections that are connected to an equipment instance to avoid iterating over the
    /// entire `project.connections` map every time.
    ///
    /// The mapped value indicates if the connection end that represents the equipment instance is
    /// `End1` or `End2`.
    pub(crate) connections: SparseSecondaryMap<InnerConnectionId, EndDesignation>,
    /// Schematic symbol instance that is updated from the library and contains updated data unique
    /// to this instance. This also has the `symbol_style` applied if `Some()`.
    ///
    /// This field is designed to cache the final symbol used for display so the update methods are
    /// not running every frame.
    pub(crate) schematic_symbol: SchematicSymbol,
    /// datafile the struct instance was read in from.
    pub(crate) contained_datafile_path: PathBuf,
}

impl From<file_types::equipment::Equipment> for Equipment {
    #[inline]
    fn from(value: file_types::equipment::Equipment) -> Self {
        Self {
            equipment_type: value.equipment_type,
            identifier: value.identifier,
            mounting_type: value.mounting_type,
            enclosure: value.enclosure,
            mount_point: value.mount_point,
            description: value.description,
            physical_location: value.physical_location,
            iec_codes: value.iec_codes,
            user_fields: value.user_fields,
            symbol_style: value.symbol_style,
            connections: SparseSecondaryMap::new(),
            schematic_symbol: SchematicSymbol::default(),
            contained_datafile_path: PathBuf::new(),
        }
    }
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
    #[inline]
    fn schematic_symbol(&self) -> SchematicSymbol {
        //TODO: don't have the warning symbol as default? Currently Svg::default().
        self.schematic_symbol.clone()
    }

    #[inline]
    fn schematic_symbol_mut(&mut self) -> &mut SchematicSymbol {
        //TODO: don't have the warning symbol as default? Currently Svg::default().
        &mut self.schematic_symbol
    }

    #[inline]
    fn update_symbol_scale(&mut self, scale: f32) {
        self.schematic_symbol.scale = scale;
    }

    #[inline]
    fn set_symbol_position(&mut self, position: Pos2) {
        self.schematic_symbol.position = position;
    }

    #[inline]
    fn symbol_position(&self) -> Pos2 {
        self.schematic_symbol.position
    }

    #[inline]
    fn update_schematic_symbol_from_library(
        &mut self,
        library: &Library,
        symbol_selector: Option<usize>,
        entity_id: String,
    ) -> Result<(), Error> {
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

        let schematic_symbol_svg = library
            .schematic_symbol_types
            .get(schematic_symbol_type_id)
            .ok_or(LibraryError::ValueNotFound {
                id: schematic_symbol_type_id.clone(),
                found_in: format!("equipment instance {}", self.identifier).to_owned(),
                library_type: "Schematic Symbol".to_owned(),
            })?
            .visual_representation
            .clone();

        let schematic_symbol = SchematicSymbol {
            symbol_type: schematic_symbol_type_id.to_owned(),
            visual_representation: schematic_symbol_svg,
            identifier: entity_id,
            connections: BTreeMap::new(),
            position: Pos2::default(),
            scale: 1.0,
        };

        self.schematic_symbol = schematic_symbol;

        Ok(())
    }

    #[inline(never)]
    #[expect(clippy::too_many_lines, reason = "Its a long function, deal with it.")]
    fn update_symbol_data(&mut self, library: &Library, project: &Project) -> Result<(), Error> {
        // TODO: look at reader config
        let svg_data = self.schematic_symbol.visual_representation.get_data_mut();
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
            let event = event?.clone();
            // If next_event is None, this is the last element in the iterator
            // TODO: check if event = EndDocument and error if not here.
            let Some(next_event) = reader.peek() else { break };
            #[expect(clippy::shadow_reuse, reason = "unwrapping error")]
            let next_event = next_event.clone()?;
            //trace! {"Event: {event:#?}"};
            //trace! {"Next Event: {next_event:#?}"};

            match event {
                #[expect(clippy::ref_patterns, reason = "can't get this to work otherwise")]
                ReaderEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                } => {
                    trace! {"StartElement name: {}", name.local_name};

                    // Modify tagged text elements within SVG

                    // first check for text element
                    match name.local_name.as_str() {
                        "text" => {
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
                                // pop the next element off the reader stack.
                                reader.nth(0);
                            }
                            //https://stackoverflow.com/a/37700229
                            //
                            // Checking for duplicate attributes on text element
                            let unique_attributes: HashSet<_> =
                                attributes.iter().map(|attr| attr.name.local_name.as_str()).collect();

                            if unique_attributes.len() < attributes.len() {
                                //TODO: return list of duplicate attributes here as well
                                return Err(SVGModificationError::DuplicateAttributes.into());
                            }

                            // This attribute indicates
                            // if this text element is related to a connection point on the symbol.
                            //
                            // This is used to display data on the symbol for a specific connection
                            // point.
                            let connection_indication_attribute = "data-connection-point";

                            let connection_point_id: Option<String> = attributes
                                .iter()
                                .find(|attr| attr.name.local_name.as_str() == connection_indication_attribute)
                                .map(|attr| attr.value.clone());

                            // Write out StartElement first
                            // the else path here should never happen.
                            let Some(writer_output) = event.as_writer_event() else {
                                continue;
                            };
                            writer.write(writer_output)?;
                            // Now finally loop through attributes and update text value where
                            // appropriate.
                            for attr in attributes {
                                match attr.name.local_name.as_str() {
                                    "data-ref-des" => {
                                        // Then finally write out character data with updated values after writing
                                        // start element
                                        let character_event = WriterEvent::Characters(&self.identifier);
                                        writer.write(character_event)?;
                                    }
                                    "data-manufacturer" => {
                                        let character_event = WriterEvent::Characters(
                                            &equipment_type.catalog.as_ref().map_or(String::new(), |catalog| {
                                                catalog.manufacturer.clone().unwrap_or_default()
                                            }),
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
                                        let character_event =
                                            WriterEvent::Characters(&self.description.clone().unwrap_or_default());
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
                                    //TODO: need to figure out cables here
                                    //
                                    // This text displays the identifier of the wire/cable/etc that
                                    // is connected to this connection point.
                                    "data-connection-point-identifier" => {
                                        let mut identifier = String::new();
                                        if let Some(ref connection_point_id_inner) = connection_point_id {
                                            // these connection_keys should all refer to
                                            // connections that already have Equipment as the other
                                            // end.
                                            //
                                            // which_end refers to the end the Equipment was
                                            // associated with.
                                            for (connection_key, which_end) in &self.connections {
                                                let connection = project.connections.get(connection_key).unwrap();
                                                match which_end {
                                                    EndDesignation::End1 => {
                                                        if let ConnectionType::Equipment {
                                                            connection_point_id: equip_connection_point_id,
                                                            ..
                                                        } = &connection.end1
                                                            && equip_connection_point_id == connection_point_id_inner
                                                        {
                                                            #[expect(
                                                                clippy::wildcard_enum_match_arm,
                                                                reason = "only need to handle these 3 variants here"
                                                            )]
                                                            match &connection.end2 {
                                                                ConnectionType::Wire { wire_id } => {
                                                                    identifier =
                                                                        project.wires.get(wire_id).unwrap().identifier.clone();
                                                                }
                                                                ConnectionType::Cable { cable_id, core_id } => {
                                                                    match &project
                                                                        .cables
                                                                        .get(cable_id)
                                                                        .unwrap()
                                                                        .cores
                                                                        .get(core_id)
                                                                        .unwrap()
                                                                    {
                                                                        CableCore::Cable(cable) => {
                                                                            identifier = cable.identifier.clone();
                                                                        }
                                                                        CableCore::Wire(wire) => {
                                                                            identifier = wire.identifier.clone();
                                                                        }
                                                                    }
                                                                    //TODO: need to correctly pull
                                                                    //identifier for core
                                                                    //identifier =
                                                                    // project.cables.get(cable_id).unwrap().
                                                                    // cores.
                                                                    // get(core_id).unwrap().0.identifier.
                                                                    // clone();
                                                                }
                                                                ConnectionType::TermCable { cable_id } => {
                                                                    //TODO: need to correctly pull
                                                                    //identifier for core
                                                                    //identifier =
                                                                    // project.cables.get(cable_id).unwrap().
                                                                    // cores.
                                                                    // get(core_id).unwrap().0.identifier.
                                                                    // clone();
                                                                }
                                                                _ => {}
                                                            }
                                                        }
                                                    }
                                                    EndDesignation::End2 => {
                                                        if let ConnectionType::Equipment {
                                                            connection_point_id: equip_connection_point_id,
                                                            ..
                                                        } = &connection.end2
                                                            && equip_connection_point_id == connection_point_id_inner
                                                        {
                                                            #[expect(
                                                                clippy::wildcard_enum_match_arm,
                                                                reason = "only need to handle these 3 variants here"
                                                            )]
                                                            match &connection.end2 {
                                                                ConnectionType::Wire { wire_id } => {
                                                                    identifier =
                                                                        project.wires.get(wire_id).unwrap().identifier.clone();
                                                                }
                                                                ConnectionType::Cable { cable_id, core_id } => {
                                                                    //TODO: need to correctly pull
                                                                    //identifier for core
                                                                    //identifier =
                                                                    // project.cables.get(cable_id).unwrap().
                                                                    // cores.
                                                                    // get(core_id).unwrap().0.identifier.
                                                                    // clone();
                                                                }
                                                                ConnectionType::TermCable { cable_id } => {
                                                                    //TODO: need to correctly pull
                                                                    //identifier for core
                                                                    //identifier =
                                                                    // project.cables.get(cable_id).unwrap().
                                                                    // cores.
                                                                    // get(core_id).unwrap().0.identifier.
                                                                    // clone();
                                                                }
                                                                _ => {}
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        let character_event = WriterEvent::Characters(&identifier);
                                        writer.write(character_event)?;
                                    }
                                    "data-connection-point-label" => {
                                        //TODO:
                                    }
                                    "data-terminal-identifier" => {
                                        let mut identifier: Option<String> = None;
                                        if let Some(ref connection_point_id_inner) = connection_point_id {
                                            for (key, connection) in &project.connections {
                                                if let ConnectionType::Equipment {
                                                    equipment_id,
                                                    connection_point_id: equip_connection_point_id,
                                                } = &connection.end1
                                                    && equip_connection_point_id == connection_point_id_inner
                                                    && let ConnectionType::TerminalStrip {
                                                        term_strip_id,
                                                        element_id,
                                                    } = &connection.end2
                                                {
                                                    identifier = Some(element_id.clone());
                                                }

                                                if let ConnectionType::Equipment {
                                                    equipment_id,
                                                    connection_point_id: equip_connection_point_id,
                                                } = &connection.end2
                                                    && equip_connection_point_id == connection_point_id_inner
                                                    && let ConnectionType::TerminalStrip {
                                                        term_strip_id,
                                                        element_id,
                                                    } = &connection.end1
                                                {
                                                    identifier = Some(element_id.clone());
                                                }
                                            }
                                        }
                                        let character_event = WriterEvent::Characters(&identifier.unwrap_or_default());
                                        writer.write(character_event)?;
                                    }
                                    // other attributes
                                    _ => {}
                                }
                            }
                        }
                        // which elements to match for connections
                        //
                        // Here, allow duplicate attributes such as data-connection-point-type
                        "circle" => {
                            if attributes
                                .iter()
                                .any(|attr| attr.name.local_name.as_str() == "data-connection-point")
                            {
                                trace! {"found data-connection-point"};
                                let mut connection: SymbolConnection = SymbolConnection::default();
                                let mut connection_id = String::new();

                                for attr in attributes {
                                    match attr.name.local_name.as_str() {
                                        "data-connection-point" => {
                                            connection_id = attr.value.clone();
                                        }
                                        "data-connection-point-type" => {
                                            if attr.value.is_empty() {
                                                return Err(SVGValidationError::BlankAttributeValue(
                                                    attr.name.local_name.clone(),
                                                )
                                                .into());
                                            }
                                            match attr.value.as_str() {
                                                "left" => {
                                                    connection.allowed_connection_directions.remove(&ConnectionDirection::NONE);
                                                    connection.allowed_connection_directions.insert(ConnectionDirection::LEFT);
                                                }
                                                "right" => {
                                                    connection.allowed_connection_directions.remove(&ConnectionDirection::NONE);
                                                    connection.allowed_connection_directions.insert(ConnectionDirection::RIGHT);
                                                }
                                                "top" => {
                                                    connection.allowed_connection_directions.remove(&ConnectionDirection::NONE);
                                                    connection.allowed_connection_directions.insert(ConnectionDirection::TOP);
                                                }
                                                "bottom" => {
                                                    connection.allowed_connection_directions.remove(&ConnectionDirection::NONE);
                                                    connection.allowed_connection_directions.insert(ConnectionDirection::BOTTOM);
                                                }
                                                "all" => {
                                                    connection.allowed_connection_directions.remove(&ConnectionDirection::NONE);
                                                    connection.allowed_connection_directions.extend(ConnectionDirection::all());
                                                }
                                                x => {
                                                    return Err(SVGValidationError::AttributeValueInvalid(
                                                        attr.name.local_name.clone(),
                                                        x.to_owned(),
                                                    )
                                                    .into());
                                                }
                                            }
                                        }

                                        "cx" => {
                                            if attr.value.is_empty() {
                                                return Err(SVGValidationError::BlankAttributeValue(
                                                    attr.name.local_name.clone(),
                                                )
                                                .into());
                                            }

                                            if !attr.value.ends_with('%') {
                                                return Err(SVGValidationError::AttributeMustBePercentage(
                                                    attr.name.local_name.clone(),
                                                )
                                                .into());
                                            }
                                            connection.x = attr.value.replace('%', "").as_str().parse::<f32>()?;
                                        }
                                        "cy" => {
                                            if attr.value.is_empty() {
                                                return Err(SVGValidationError::BlankAttributeValue(
                                                    attr.name.local_name.clone(),
                                                )
                                                .into());
                                            }

                                            if !attr.value.ends_with('%') {
                                                return Err(SVGValidationError::AttributeMustBePercentage(
                                                    attr.name.local_name.clone(),
                                                )
                                                .into());
                                            }
                                            connection.y = attr.value.replace('%', "").as_str().parse::<f32>()?;
                                        }
                                        // other attributes
                                        _ => {}
                                    }
                                }
                                // Making sure there is indeed a value assigned to the connection_id after processing attributes.
                                if connection_id == String::new() {
                                    return Err(
                                        SVGValidationError::BlankAttributeValue("data-connection-point".to_owned()).into()
                                    );
                                }
                                self.schematic_symbol.connections.insert(connection_id, connection);
                            }

                            //TODO:
                            let Some(writer_output) = event.as_writer_event() else {
                                continue;
                            };
                            writer.write(writer_output)?;
                        }

                        "rect" => {
                            warn! {"rect connection points not implemented yet."}
                            //TODO: copy logic for circle, but return center point based on x, y,
                            //width, height
                        }
                        _ => {
                            let Some(writer_output) = event.as_writer_event() else {
                                continue;
                            };
                            writer.write(writer_output)?;
                        }
                    }
                }
                ReaderEvent::EndDocument => break,
                // EndDocument is the only Discriminent currently that is None
                ReaderEvent::StartDocument { .. }
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

        self.schematic_symbol.visual_representation.set_data(&output_string);

        Ok(())
    }
}

impl ProjectData for Equipment {}
