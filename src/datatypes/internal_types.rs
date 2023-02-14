///`cable_type` represents a cable with multiple cores
pub mod cable_type;
/// `connector_type` represents a connector
pub mod connector_type;
/// `equipment_type` represents a type of equipment
pub mod equipment_type;
/// `location_type` represents a type of location
pub mod location_type;
/// `pathway_type` represents a type of pathway for wires or cables
pub mod pathway_type;
/// `svg` represents a complete SVG image
pub mod svg;
/// `term_cable_type` represents a cable that has connectors assembled on to it
pub mod term_cable_type;
/// `wire_type` represents an individual wire with optional insulation
pub mod wire_type;

/// `equipment` represents an instance of an `EquipmentType`. This is a physical item
/// you hold in your hand.
pub mod equipment;
/// `location` represents an instance of a `LocationType`
pub mod location;
/// `pathway` represents an instance of a `PathwayType`
pub mod pathway;
/// `wire_cable` represents an instance of either a `WireType`, `CableType` or `TermCableType`
pub mod wire_cable;

use log::{trace, warn};

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::file_types::DataFile;
use super::util_types::CrossSection;
use cable_type::CableCore;
use svg::Svg;

/// `Library` represents all library data used in program
#[derive(Debug, Default, PartialEq)]
pub struct Library {
    /// contains all wire types read in from file, and/or added in via program logic
    pub wire_types: HashMap<String, Rc<RefCell<wire_type::WireType>>>,
    /// contains all cable types read in from file, and/or added in via program logic
    pub cable_types: HashMap<String, Rc<RefCell<cable_type::CableType>>>,
    /// contains all terminated cable types read in from file, and/or added in via program logic
    pub term_cable_types: HashMap<String, Rc<RefCell<term_cable_type::TermCableType>>>,
    /// contains all location types read in from file, and/or added in via program logic
    pub location_types: HashMap<String, Rc<RefCell<location_type::LocationType>>>,
    /// contains all connector types read in from file, and/or added in via program logic
    pub connector_types: HashMap<String, Rc<RefCell<connector_type::ConnectorType>>>,
    /// contains all equipment types read in from file, and/or added in via program logic
    pub equipment_types: HashMap<String, Rc<RefCell<equipment_type::EquipmentType>>>,
    /// contains all pathway types read in from file
    pub pathway_types: HashMap<String, Rc<RefCell<pathway_type::PathwayType>>>,
}

/// `Project` represents all project specific data used in program
#[derive(Debug, Default, PartialEq)]
pub struct Project {
    /// `equipment` contains all equipment instances read in from files, and/or added in via program logic
    pub equipment: HashMap<String, Rc<RefCell<equipment::Equipment>>>,
    /// `wire_cables` contains all wire, cable and termcable instances read in from files, and/or
    /// added in via program logic
    pub wire_cables: HashMap<String, Rc<RefCell<wire_cable::WireCable>>>,
    /// `pathways`contains all pathway instances read in from files and/or added in via program
    /// logic
    pub pathways: HashMap<String, Rc<RefCell<pathway::Pathway>>>,
    /// `locations` contains all location instances read in from files and/or added in via program
    /// logic
    pub locations: HashMap<String, Rc<RefCell<location::Location>>>,
}

impl Library {
    ///Initializes an empty `Library`
    pub fn new() -> Self {
        Library {
            wire_types: HashMap::new(),
            cable_types: HashMap::new(),
            term_cable_types: HashMap::new(),
            location_types: HashMap::new(),
            connector_types: HashMap::new(),
            equipment_types: HashMap::new(),
            pathway_types: HashMap::new(),
        }
    }
    /// inserts the correct values from a datafile into a `Library` struct
    pub fn from_datafile(&mut self, datafile: DataFile) {
        // wire_types
        if let Some(wire_types) = datafile.wire_types {
            for (k, v) in &wire_types {
                if self.wire_types.contains_key(k) {
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                    warn! {"WireType: {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, datafile.file_path.display()}
                } else {
                    trace! {"Inserted WireType: {}, value: {:#?} into main library.",k,v}
                    self.wire_types.insert(
                        k.to_string(),
                        Rc::new(RefCell::new(wire_type::WireType {
                            id: k.to_string(),
                            manufacturer: wire_types[k].manufacturer.clone(),
                            model: wire_types[k].model.clone(),
                            part_number: wire_types[k].part_number.clone(),
                            manufacturer_part_number: wire_types[k]
                                .manufacturer_part_number
                                .clone(),
                            supplier: wire_types[k].supplier.clone(),
                            supplier_part_number: wire_types[k].supplier_part_number.clone(),
                            material: wire_types[k].material.clone(),
                            insulated: wire_types[k].insulated,
                            insulation_material: wire_types[k].insulation_material.clone(),
                            wire_type_code: wire_types[k].wire_type_code.clone(),
                            conductor_cross_sect_area: wire_types[k].conductor_cross_sect_area,
                            overall_cross_sect_area: wire_types[k].overall_cross_sect_area,
                            stranded: wire_types[k].stranded,
                            num_strands: wire_types[k].num_strands,
                            strand_cross_sect_area: wire_types[k].strand_cross_sect_area,
                            insul_volt_rating: wire_types[k].insul_volt_rating,
                            insul_temp_rating: wire_types[k].insul_temp_rating,
                            insul_color: wire_types[k].insul_color.clone(),
                        })),
                    );
                }
            }
        }
        // cable_types
        if let Some(cable_types) = datafile.cable_types {
            for (k, v) in &cable_types {
                if self.cable_types.contains_key(k) {
                    warn! {"CableType: {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, datafile.file_path.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted CableType: {}, value: {:#?} into main datastore.",k,v}
                    self.cable_types.insert(
                        k.to_string(),
                        Rc::new(RefCell::new(cable_type::CableType {
                            id: k.to_string(),
                            manufacturer: cable_types[k].manufacturer.clone(),
                            model: cable_types[k].model.clone(),
                            part_number: cable_types[k].part_number.clone(),
                            manufacturer_part_number: cable_types[k].manufacturer_part_number.clone(),
                            supplier: cable_types[k].supplier.clone(),
                            supplier_part_number: cable_types[k].supplier_part_number.clone(),
                            cable_type_code: cable_types[k].cable_type_code.clone(),
                            cross_sect_area: cable_types[k].cross_sect_area,
                            height: cable_types[k].height,
                            width: cable_types[k].width,
                            diameter: cable_types[k].diameter,
                            cross_section: {
                                match cable_types[k].cross_section.to_uppercase().as_str() {
                                   "OVAL" => CrossSection::Oval,
                                   "CIRCULAR" => CrossSection::Circular,
                                   "SIAMESE" => CrossSection::Siamese,
                                   //TODO: handle this better
                                   _ => panic! {"Cross Section: {} in CableType: {} in file: {} not recognized. Check your spelling and try again.",cable_types[k].cross_section, k, datafile.file_path.display() }
                                }
                            },
                            cable_core: {
                               let mut cable_core_map = HashMap::new();
                                for (core_id, core_type_str) in &cable_types[k].cable_core {
                                    //TODO: this could result in issues where the cable type is in
                                    //the file, but not read before it is checked for here.
                                if self.wire_types.contains_key(core_type_str) {
                                    cable_core_map.insert(core_id.to_string(), CableCore::WireType(self.wire_types[core_type_str].clone()));

                                } else if self.cable_types.contains_key(core_type_str) {
                                    cable_core_map.insert(core_id.to_string(), CableCore::CableType(self.cable_types[core_type_str].clone()));

                                } else {
                                    //TODO: handle this more gracefully
                                    panic!{"can't find CableCore Type: {} referenced in CableType: {} in datafile: {}, in any file or library imported into Project.", core_type_str, k, datafile.file_path.display()}
                                }

                                }
                                cable_core_map
                    },
                    insul_layers: {
                        let mut new_layers = Vec::new();
                        for layer in &cable_types[k].insul_layers {
                            let new_layer = cable_type::CableLayer {
                                    layer_number: layer.layer_number,
                                    layer_type: layer.layer_type.clone(),
                                    material: layer.material.clone(),
                                    volt_rating: layer.volt_rating,
                                    temp_rating: layer.temp_rating,
                                    color: layer.color.clone(),
                                };
                        new_layers.push(new_layer);
                        }
                        new_layers
                    },
                        })),
                    );
                }
            }
        }

        // pathway_types
        if let Some(pathway_types) = datafile.pathway_types {
            for (k, v) in &pathway_types {
                if self.pathway_types.contains_key(k) {
                    warn! {"PathwayType : {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, datafile.file_path.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted PathwayType: {}, value: {:#?} into main datastore.",k,v}
                    self.pathway_types.insert(
                        k.to_string(),
                        Rc::new(RefCell::new(pathway_type::PathwayType {
                            id: k.to_string(),
                            manufacturer: pathway_types[k].manufacturer.clone(),
                            model: pathway_types[k].model.clone(),
                            part_number: pathway_types[k].part_number.clone(),
                            manufacturer_part_number: pathway_types[k]
                                .manufacturer_part_number
                                .clone(),
                            supplier: pathway_types[k].supplier.clone(),
                            supplier_part_number: pathway_types[k].supplier_part_number.clone(),
                            description: pathway_types[k].description.clone(),
                            size: pathway_types[k].size.clone(),
                            trade_size: pathway_types[k].trade_size.clone(),
                            // no clone needed since numeric types have easy copy implementation
                            cross_sect_area: pathway_types[k].cross_sect_area,
                            material: pathway_types[k].material.clone(),
                        })),
                    );
                }
            }
        }
        // location_types
        if let Some(location_types) = datafile.location_types {
            for (k, v) in &location_types {
                if self.location_types.contains_key(k) {
                    warn! {"LocationType : {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, datafile.file_path.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted LocationType: {}, value: {:#?} into main datastore.",k,v}
                    self.location_types.insert(
                        k.to_string(),
                        Rc::new(RefCell::new(location_type::LocationType {
                            id: k.to_string(),
                            manufacturer: location_types[k].manufacturer.clone(),
                            model: location_types[k].model.clone(),
                            part_number: location_types[k].part_number.clone(),
                            manufacturer_part_number: location_types[k]
                                .manufacturer_part_number
                                .clone(),
                            supplier: location_types[k].supplier.clone(),
                            supplier_part_number: location_types[k].supplier_part_number.clone(),
                            description: location_types[k].description.clone(),
                            material: location_types[k].material.clone(),
                            height: location_types[k].height,
                            width: location_types[k].width,
                            depth: location_types[k].depth,
                            usable_width: location_types[k].usable_width,
                            usable_height: location_types[k].usable_height,
                            usable_depth: location_types[k].usable_depth,
                        })),
                    );
                }
            }
        }

        // connector_types
        if let Some(connector_types) = datafile.connector_types {
            for (k, v) in &connector_types {
                if self.connector_types.contains_key(k) {
                    warn! {"ConnectorType : {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, datafile.file_path.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted ConnectorType: {}, value: {:#?} into main datastore.",k,v}
                    self.connector_types.insert(
                        k.to_string(),
                        Rc::new(RefCell::new(connector_type::ConnectorType {
                            id: k.to_string(),
                            manufacturer: connector_types[k].manufacturer.clone(),
                            model: connector_types[k].model.clone(),
                            part_number: connector_types[k].part_number.clone(),
                            manufacturer_part_number: connector_types[k]
                                .manufacturer_part_number
                                .clone(),
                            supplier: connector_types[k].supplier.clone(),
                            supplier_part_number: connector_types[k].supplier_part_number.clone(),
                            description: connector_types[k].description.clone(),
                            mount_type: connector_types[k].mount_type.clone(),
                            panel_cutout: connector_types[k].panel_cutout.clone(),
                            gender: connector_types[k].gender.clone(),
                            height: connector_types[k].height,
                            width: connector_types[k].width,
                            depth: connector_types[k].depth,
                            diameter: connector_types[k].diameter,
                            pins: {
                                let mut new_pins = Vec::new();
                                for pin in &connector_types[k].pins {
                                    let new_pin = connector_type::ConnectorPin {
                                        id: pin.id.clone(),
                                        label: pin.label.clone(),
                                        signal_type: pin.signal_type.clone(),
                                        color: pin.color.clone(),
                                        visual_rep: pin.visual_rep.clone().map(Svg::from),
                                        gender: pin.gender.clone(),
                                    };
                                    new_pins.push(new_pin);
                                }
                                new_pins
                            },
                            visual_rep: Svg::from(connector_types[k].visual_rep.clone()),
                        })),
                    );
                }
            }
        }
        // term_cable_types
        if let Some(term_cable_types) = datafile.term_cable_types {
            for (k, v) in &term_cable_types {
                if self.term_cable_types.contains_key(k) {
                    warn! {"TermCableType : {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, datafile.file_path.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted TermCableType: {}, value: {:#?} into main datastore.",k,v}
                    self.term_cable_types.insert(k.to_string(),
                        Rc::new(RefCell::new(term_cable_type::TermCableType {
                            id: k.to_string(),
                            manufacturer: term_cable_types[k].manufacturer.clone(),
                            model: term_cable_types[k].model.clone(),
                            part_number: term_cable_types[k].part_number.clone(),
                            manufacturer_part_number: term_cable_types[k].manufacturer_part_number.clone(),
                            supplier: term_cable_types[k].supplier.clone(),
                            supplier_part_number: term_cable_types[k].supplier_part_number.clone(),
                            description: term_cable_types[k].description.clone(),
                            wire_cable: {

                                if term_cable_types[k].wire.is_some() && term_cable_types[k].cable.is_some() {
                                    panic! {"Both wire and cable values of TermCableType {k} are specified. Please correct this."}
                                } else if term_cable_types[k].wire.is_none() && term_cable_types[k].cable.is_none() {

                                    panic! {"Neither wire or cable values of TermCableType {k} are specified. Please correct this."}
                                } else {
                                    #[allow(clippy::collapsible_else_if)] // This would change the
                                                                          // meaning of the logic
                                    if let Some(wire_type) = term_cable_types[k].wire.clone() {
                                        if self.wire_types.contains_key(&wire_type) {
                                            term_cable_type::WireCable::WireType(self.wire_types[&wire_type].clone())
                                        } else {panic!{"WireType: {} in TermCableType: {} specified in datafile: {} is not found in any library either read from datafiles, or implemented in program logic. Check your spelling", wire_type, k, datafile.file_path.display()}}


                                    } else if let Some(cable_type) = &term_cable_types[k].cable {

                                        if self.cable_types.contains_key(cable_type) {
                                            term_cable_type::WireCable::CableType(self.cable_types[cable_type].clone())
                                        } else {
panic!{"WireType: {} in TermCableType: {} specified in datafile: {} is not found in any library either read from datafiles, or implemented in program logic. Check your spelling", cable_type, k, datafile.file_path.display()}}


                                        } else {
                                        //TODO: fix this
                                    panic! {"Neither wire or cable values of TermCableType {k} are specified. Please correct this."}
                                    }
                                }
                            },
                            nominal_length: term_cable_types[k].nominal_length,
                            actual_length:  term_cable_types[k].actual_length,
                            end1:  {
                                let mut new_end1 = Vec::new();
                                for connector in &term_cable_types[k].end1 {
                                   let new_connector = term_cable_type::TermCableConnector {
                                        connector_type: {
                                            if self.connector_types.contains_key(&connector.connector_type) {
                                                self.connector_types[&connector.connector_type].clone()
                                            } else {
                                                panic! {"End 1 of TermCableType: {} in datafile: {}, contains ConnectorType: {} that does not exist in any library data, either read from file, or created via program logic. Please check your spelling.", k, datafile.file_path.display(), &connector.connector_type}
                                            }
                                        },
                                        terminations: {
                                        if let Some(terminations) = &connector.terminations {
                                            let mut new_terminations = Vec::new();
                                            for termination in terminations {
                                                let new_termination = term_cable_type::TermCableConnectorTermination {
                                                    core: termination.core,
                                                    pin: termination.pin,
                                                };
                                                new_terminations.push(new_termination);
                                            }
                                            Some(new_terminations)
                                        } else {None}
                                        },
                                   };
                                   new_end1.push(new_connector);
                                }
                            new_end1
                            },
                            end2: {
                                let mut new_end2 = Vec::new();
                                for connector in &term_cable_types[k].end2 {
                                   let new_connector = term_cable_type::TermCableConnector {
                                        connector_type: {
                                            if self.connector_types.contains_key(&connector.connector_type) {
                                                self.connector_types[&connector.connector_type].clone()
                                            } else {
                                                panic! {"End 2 of TermCableType: {} in datafile: {}, contains ConnectorType: {} that does not exist in any library data, either read from file, or created via program logic. Please check your spelling.", k, datafile.file_path.display(), &connector.connector_type}
                                            }
                                        },
                                        terminations: {
                                        if let Some(terminations) = &connector.terminations {
                                            let mut new_terminations = Vec::new();
                                            for termination in terminations {
                                                let new_termination = term_cable_type::TermCableConnectorTermination {
                                                    core: termination.core,
                                                    pin: termination.pin,
                                                };
                                                new_terminations.push(new_termination);
                                            }
                                            Some(new_terminations)
                                        } else {None}
                                        },
                                   };
                                   new_end2.push(new_connector);
                                }
                            new_end2
                            },
                    },
                        )));
                }
            }
        }

        // equipment_types
        if let Some(equipment_types) = datafile.equipment_types {
            for (k, v) in &equipment_types {
                if self.equipment_types.contains_key(k) {
                    warn! {"EquipmentType : {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, datafile.file_path.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted EquipmentType: {}, value: {:#?} into main datastore.",k,v}
                    self.equipment_types.insert(k.to_string(),
                        Rc::new(RefCell::new(equipment_type::EquipmentType {
                            id: k.to_string(),
                            manufacturer: equipment_types[k].manufacturer.clone(),
                            model: equipment_types[k].model.clone(),
                            part_number: equipment_types[k].part_number.clone(),
                            manufacturer_part_number: equipment_types[k].manufacturer_part_number.clone(),
                            supplier: equipment_types[k].supplier.clone(),
                            supplier_part_number: equipment_types[k].supplier_part_number.clone(),
                            description: equipment_types[k].description.clone(),
                            mount_type: equipment_types[k].mount_type.clone(),
                            equip_type: equipment_types[k].equip_type.clone(),
                            faces: {
                                if let Some(faces) = &equipment_types[k].faces {
                                    let mut new_faces = Vec::new();
                                   for face in faces {
                                       let new_face = equipment_type::EquipFace {
                                            name: face.name.to_string(),
                                            vis_rep: face.vis_rep.clone().map(Svg::from),
                                            connectors: {
                                              if let Some(connectors) = &face.connectors {
                                                let mut new_connectors = Vec::new();
                                                for connector in connectors {
                                                    let new_connector = equipment_type::EquipConnector {
                                                        connector: {
                                                        if self.connector_types.contains_key(&connector.connector) {
                                                            self.connector_types[&connector.connector].clone()
                                                        } else {
                                                            panic! {"connectorType: {} in equipment: {} from datafile: {} not found in any Library file imported into project or added via program logic. Check your spelling.", &connector.connector, &k, datafile.file_path.display(), }
                                                        }
                                                        },
                                                        direction: connector.direction.clone(),
                                                        x: connector.x,
                                                        y: connector.y,
                                                    };
                                                    new_connectors.push(new_connector);
                                                }
                                               Some(new_connectors)
                                              } else {None}
                                            },
                                       };
                                    new_faces.push(new_face);
                                   }
                                    Some(new_faces)
                                } else {
                                    None
                                }
                            },
                            visual_rep: Svg::from(equipment_types[k].visual_rep.clone()),
                    },
                        )));
                }
            }
        }
    }
}

impl Project {
    ///Initializes an empty `Project`
    pub fn new() -> Self {
        Project {
            locations: HashMap::new(),
            equipment: HashMap::new(),
            pathways: HashMap::new(),
            wire_cables: HashMap::new(),
        }
    }

    /// `from_datafile` takes a `DataFile` and a `Library` and imports all Project data found
    /// within, into the `Project` struct this method is called on. It will check `Library` for
    /// defined types to assign as references within the various project data imported from
    /// `datafile`
    pub fn from_datafile(&mut self, datafile: DataFile, library: &Library) {
        // wire_cables
        if let Some(wire_cables) = datafile.wire_cables {
            for (k, v) in &wire_cables {
                if self.wire_cables.contains_key(k) {
                    warn! {"WireCable: {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, datafile.file_path.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted WireCable: {}, value: {:#?} into main project.",k,v}
                    self.wire_cables.insert(
                        k.to_string(),
                        Rc::new(RefCell::new(wire_cable::WireCable {
                            id: k.to_string(),
                            ctw_type: {
// Checking to make sure only one of wire, cable, or term_cable are set
                                if
                                    (wire_cables[k].wire.is_some() && wire_cables[k].cable.is_some() && wire_cables[k].term_cable.is_some())
                                    ||
                                    (wire_cables[k].wire.is_some() && wire_cables[k].cable.is_some())
                                    ||
                                    (wire_cables[k].cable.is_some() && wire_cables[k].term_cable.is_some())
                                    ||
                                    (wire_cables[k].wire.is_some() && wire_cables[k].term_cable.is_some())

                                {
                                    panic! {"More than one of wire, cable and term_cable of WireCable {} are specified. Please correct this.", &k}
                                } else if wire_cables[k].wire.is_none() && wire_cables[k].cable.is_none() && wire_cables[k].term_cable.is_none() {

                                    panic! {"Neither wire, cable or term_cable values of WireCable {} are specified. Please correct this.", &k}
                                } else {
                                    // at this point, only one of wire, cable and term_cable should
                                    // be set.
                                    //
                                    // clone string here to avoid moving value out of hashmap.
                                    if let Some(wire_type) = wire_cables[k].wire.clone() {
                                        if library.wire_types.contains_key(&wire_type) {
                                            wire_cable::WireCableType::WireType(library.wire_types[&wire_type].clone())
                                        } else {panic!{"WireType: {} in WireCable: {} specified in datafile: {} is not found in any library either read from datafiles, or implemented in program logic. Check your spelling", wire_type, k, datafile.file_path.display()}}


                                    // clone string here to avoid moving value out of hashmap.
                                    } else if let Some(cable_type) = wire_cables[k].cable.clone() {

                                        if library.cable_types.contains_key(&cable_type) {
                                            wire_cable::WireCableType::CableType(library.cable_types[&cable_type].clone())
                                        } else {
panic!{"CableType: {} in WireCable: {} specified in datafile: {} is not found in any library either read from datafiles, or implemented in program logic. Check your spelling", cable_type, k, datafile.file_path.display()}}


                                    // clone string here to avoid moving value out of hashmap.
                                        } else if let Some(term_cable_type) = wire_cables[k].term_cable.clone() {

                                        if library.term_cable_types.contains_key(&term_cable_type) {
                                            wire_cable::WireCableType::TermCableType(library.term_cable_types[&term_cable_type].clone())
                                        } else {
panic!{"TermCableType: {} in WireCable: {} specified in datafile: {} is not found in any library either read from datafiles, or implemented in program logic. Check your spelling", term_cable_type, k, datafile.file_path.display()}}


                                        } else {
                                        //TODO: fix this
                                    panic! {"Neither wire, cable or termcable type values of WireCable {} are specified. Please correct this.", &k}
                                    }
                                }

                            },
                            identifier: wire_cables[k].identifier.clone(),
                            description: wire_cables[k].description.clone(),
                            length: wire_cables[k].length,
                            pathway: {
                                    // clone string here to avoid moving value out of hashmap.
                                if let Some(pathway) = wire_cables[k].pathway.clone() {
                                    if self.pathways.contains_key(k) {
                                        Some(self.pathways[k].clone())
                                    } else {
                                        panic! {"WireCable: {} is assigned to Pathway: {} in datafile: {}, that doesn't exist in any library either read in from datafile, or added via program logic. Check your spelling", k, pathway, datafile.file_path.display()}
                                    }
                                } else {
                                    None
                                }
                            },
                        })),
                    );
                }
            }
        }
        // locations
        if let Some(locations) = datafile.locations {
            for (k, v) in &locations {
                if self.locations.contains_key(k) {
                    warn! {"Location: {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, datafile.file_path.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted Location: {}, value: {:#?} into main project.",k,v}
                    self.locations.insert(k.to_string(),
                        Rc::new(
                            RefCell::new(
                                   location::Location {
                                    id: k.to_string(),
                                    location_type: {
                                        if library.location_types.contains_key(&locations[k].location_type) {
                                            library.location_types[&locations[k].location_type].clone()
                                        } else {
                                                //TODO: handle this more intelligently
                                                panic! {"Failed to find LocationType: {} used in Location: {} in file {}, in any imported library dictionary or file. Please check spelling, or add it, if this was not intentional.", locations[k].location_type, &k, datafile.file_path.display() }
                                            }


                                    },
                                    identifier: locations[k].identifier.clone(),
                                    description: locations[k].description.clone(),
                                    physical_location: locations[k].physical_location.clone(),
                                   }
                                )
                            )




                        );
                }
            }
        }

        // equipment
        if let Some(equipment) = datafile.equipment {
            for (k, v) in &equipment {
                if self.equipment.contains_key(k) {
                    warn! {"Equipment: {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, datafile.file_path.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted Equipment: {}, value: {:#?} into main project.",k,v}
                    self.equipment.insert(k.to_string(),
                        Rc::new(
                            RefCell::new(
                                    equipment::Equipment {
                                       id: k.to_string(),
                                       equip_type: {
                                            if library.equipment_types.contains_key(&equipment[k].equipment_type) {
                                                library.equipment_types[&equipment[k].equipment_type].clone()
                                            } else {
                                                //TODO: handle this more intelligently
                                                panic! {"Failed to find EquipmentType: {} used in Equipment: {} in file {}, in any imported library dictionary or file. Please check spelling, or add it, if this was not intentional.", equipment[k].equipment_type, &k, datafile.file_path.display() }
                                            }
                                       },
                                       identifier: equipment[k].identifier.clone(),
                                       mounting_type: equipment[k].mounting_type.clone(),
                                       location: {
                                                  // clone string here to avoid moving value out of hashmap.
                                           if let Some(file_location) = equipment[k].location.clone() {
                                                if self.locations.contains_key(&file_location) {
                                                    Some(self.locations[k].clone())
                                                } else {
                                                    //TODO: handle this more intelligently

                                        panic! {"Location: {} is assigned to Equipment: {} in datafile: {}, that doesn't exist in any library either read in from datafile, or added via program logic. Check your spelling", k, file_location, datafile.file_path.display()}
                                           }

                                           } else {
                                               None
                                           }
                                       },
                                       description: equipment[k].description.clone(),
                                    }
                                )
                            )
                        );
                }
            }
        }

        // pathway
        if let Some(pathways) = datafile.pathways {
            for (k, v) in &pathways {
                if self.pathways.contains_key(k) {
                    warn! {"Pathway : {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, datafile.file_path.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted Pathway: {}, value: {:#?} into main datastore.",k,v}
                    self.pathways.insert(k.to_string(),
                        Rc::new(
                            RefCell::new(
                                pathway::Pathway {
                                id: k.to_string(),
                                path_type: {
                                            if library.pathway_types.contains_key(&pathways[k].path_type) {
                                                library.pathway_types[&pathways[k].path_type].clone()
                                            } else {
                                                //TODO: handle this more intelligently
                                                panic! {"Failed to find PathwayType: {} used in Pathway: {} in file {}, in any imported library dictionary or file. Please check spelling, or add it, if this was not intentional.", pathways[k].path_type, &k, datafile.file_path.display() }
                                            }
                                       },
                                       identifier: pathways[k].identifier.clone(),
                                       description: pathways[k].description.clone(),
                                       length: pathways[k].length,

                                }

                                )
                            )
                        );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    #[test]
    fn new_library() {
        assert_eq!(
            Library::new(),
            Library {
                wire_types: HashMap::new(),
                cable_types: HashMap::new(),
                term_cable_types: HashMap::new(),
                location_types: HashMap::new(),
                connector_types: HashMap::new(),
                equipment_types: HashMap::new(),
                pathway_types: HashMap::new(),
            }
        )
    }

    #[test]
    fn new_project() {
        assert_eq!(
            Project::new(),
            Project {
                locations: HashMap::new(),
                equipment: HashMap::new(),
                pathways: HashMap::new(),
                wire_cables: HashMap::new(),
            }
        )
    }

    // TODO:  testing ideas (for both project and library):
    // - test import of datafile containing each individual object
    // - test import of basic datafile, minimal amount of data necessary
    // - test import of full datafile, with multiple defined dictionary entries for each dictionary
    // - test failure of multiple of the top level dicts defined in one file
    // - test to make sure only one of wire, cable, termcable is set in project parsing, both
    // positive and negative
    // - test importing a cable/termcable type with a missing wiretype (also for equipment, etc)
    // - test complicated term_cable
    // - test all project datatypes with both present and absent library values
    // - test all panics
    // - test library types that refer to each other, make sure objects are always parsed in
    // correct order
    // - same with project types, except with both library and project assets
    #[test]
    fn read_datafile_library() {}

    #[test]
    fn read_datafile_project() {}
}
