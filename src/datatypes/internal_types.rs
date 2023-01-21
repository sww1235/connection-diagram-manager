///`cable_type` represents a cable with multiple cores
pub mod cable_type;
/// `connector_type` represents a connector
pub mod connector_type;
/// `equipment_connector represents a connector instance on a piece of equipment
pub mod equipment_connector;
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

/// `equipment` represents an instance of an EquipmentType. This is a physical item
/// you hold in your hand.
pub mod equipment;

use log::{trace, warn};

use std::cell::RefCell;
use std::rc::Rc;

use super::file_types::DataFile;

/// `Library` represents all library data used in program
#[derive(Debug, Default)]
pub struct Library {
    /// contains all wire types read in from file, and/or added in via program logic
    pub wire_types: Vec<Rc<RefCell<wire_type::WireType>>>,
    /// contains all cable types read in from file, and/or added in via program logic
    pub cable_types: Vec<Rc<RefCell<cable_type::CableType>>>,
    /// contains all terminated cable types read in from file, and/or added in via program logic
    pub term_cable_types: Vec<Rc<RefCell<term_cable_type::TermCableType>>>,
    /// contains all location types read in from file, and/or added in via program logic
    pub location_types: Vec<Rc<RefCell<location_type::LocationType>>>,
    /// contains all connector types read in from file, and/or added in via program logic
    pub connector_types: Vec<Rc<RefCell<connector_type::ConnectorType>>>,
    /// contains all equipment types read in from file, and/or added in via program logic
    pub equipment_types: Vec<Rc<RefCell<equipment_type::EquipmentType>>>,
    /// contains all pathway types read in from file
    pub pathway_types: Vec<Rc<RefCell<pathway_type::PathwayType>>>,
}

/// `Project` represents all project specific data used in program
#[derive(Debug, Default)]
pub struct Project {
    //TODO: create structs for individual values
}

impl Library {
    ///Initializes an empty Datastore
    pub fn new() -> Library {
        let datastore = Library {
            wire_types: Vec::new(),
            cable_types: Vec::new(),
            term_cable_types: Vec::new(),
            location_types: Vec::new(),
            connector_types: Vec::new(),
            equipment_types: Vec::new(),
            pathway_types: Vec::new(),
        };
        datastore
    }
    /// inserts the correct values from a datafile into a `Library` struct
    pub fn from_datafile(&mut self, datafile: DataFile) {
        // wire_types
        if let Some(wire_types) = datafile.wire_types {
            for (k, v) in wire_types {
                //TODO: need to check if key == ID for each element of vector, maybe switch back to
                //hashmaps for performance reasons?
                if self.wire_types.contains_key(&k) {
                    warn! {"WireType : {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, filepath.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted WireType : {}, value: {:#?} into main datastore.",k,v}
                    self.wire_types.insert(k, v);
                }
            }
        }
        // cable_types
        if let Some(cable_types) = datafile.cable_types {
            for (k, v) in cable_types {
                if self.cable_types.contains_key(&k) {
                    warn! {"CableType : {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, filepath.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted CableType: {}, value: {:#?} into main datastore.",k,v}
                    self.cable_types.insert(k, v);
                }
            }
        }

        // term_cable_types
        if let Some(term_cable_types) = datafile.term_cable_types {
            for (k, v) in term_cable_types {
                if self.term_cable_types.contains_key(&k) {
                    warn! {"TermCableType : {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, filepath.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted TermCableType: {}, value: {:#?} into main datastore.",k,v}
                    self.term_cable_types.insert(k, v);
                }
            }
        }

        // location_types
        if let Some(location_types) = datafile.location_types {
            for (k, v) in location_types {
                if self.location_types.contains_key(&k) {
                    warn! {"LocationType : {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, filepath.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted LocationType: {}, value: {:#?} into main datastore.",k,v}
                    self.location_types.insert(k, v);
                }
            }
        }

        // connector_types
        if let Some(connector_types) = datafile.connector_types {
            for (k, v) in connector_types {
                if self.connector_types.contains_key(&k) {
                    warn! {"ConnectorType : {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, filepath.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted ConnectorType: {}, value: {:#?} into main datastore.",k,v}
                    self.connector_types.insert(k, v);
                }
            }
        }

        // equipment_types
        if let Some(equipment_types) = datafile.equipment_types {
            for (k, v) in equipment_types {
                if self.equipment_types.contains_key(&k) {
                    warn! {"EquipmentType : {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, filepath.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted EquipmentType: {}, value: {:#?} into main datastore.",k,v}
                    self.equipment_types.insert(k, v);
                }
            }
        }

        // pathway_types
        if let Some(pathway_types) = datafile.pathway_types {
            for (k, v) in pathway_types {
                if self.pathway_types.contains_key(&k) {
                    warn! {"PathwayType : {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, filepath.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted PathwayType: {}, value: {:#?} into main datastore.",k,v}
                    self.pathway_types.insert(k, v);
                }
            }
        }
    }
}

impl Project {
    pub fn from_datafile(&mut self, datafile: DataFile) {
        // wire_types
        if let Some(wire_types) = datafile.wire_types {
            for (k, v) in wire_types {
                if self.wire_types.contains_key(&k) {
                    warn! {"WireType : {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, filepath.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted WireType : {}, value: {:#?} into main datastore.",k,v}
                    self.wire_types.insert(k, v);
                }
            }
        }
        // cable_types
        if let Some(cable_types) = datafile.cable_types {
            for (k, v) in cable_types {
                if self.cable_types.contains_key(&k) {
                    warn! {"CableType : {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, filepath.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted CableType: {}, value: {:#?} into main datastore.",k,v}
                    self.cable_types.insert(k, v);
                }
            }
        }

        // term_cable_types
        if let Some(term_cable_types) = datafile.term_cable_types {
            for (k, v) in term_cable_types {
                if self.term_cable_types.contains_key(&k) {
                    warn! {"TermCableType : {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, filepath.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted TermCableType: {}, value: {:#?} into main datastore.",k,v}
                    self.term_cable_types.insert(k, v);
                }
            }
        }

        // location_types
        if let Some(location_types) = datafile.location_types {
            for (k, v) in location_types {
                if self.location_types.contains_key(&k) {
                    warn! {"LocationType : {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, filepath.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted LocationType: {}, value: {:#?} into main datastore.",k,v}
                    self.location_types.insert(k, v);
                }
            }
        }

        // connector_types
        if let Some(connector_types) = datafile.connector_types {
            for (k, v) in connector_types {
                if self.connector_types.contains_key(&k) {
                    warn! {"ConnectorType : {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, filepath.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted ConnectorType: {}, value: {:#?} into main datastore.",k,v}
                    self.connector_types.insert(k, v);
                }
            }
        }

        // equipment_types
        if let Some(equipment_types) = datafile.equipment_types {
            for (k, v) in equipment_types {
                if self.equipment_types.contains_key(&k) {
                    warn! {"EquipmentType : {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, filepath.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted EquipmentType: {}, value: {:#?} into main datastore.",k,v}
                    self.equipment_types.insert(k, v);
                }
            }
        }

        // pathway_types
        if let Some(pathway_types) = datafile.pathway_types {
            for (k, v) in pathway_types {
                if self.pathway_types.contains_key(&k) {
                    warn! {"PathwayType : {} with contents: {:#?} has already been loaded. Found again in file {}. Check this and merge if necessary", k, v, filepath.display()}
                    //TODO: do something: ignore dupe, prompt user for merge, try to merge
                    //automatically
                } else {
                    trace! {"Inserted PathwayType: {}, value: {:#?} into main datastore.",k,v}
                    self.pathway_types.insert(k, v);
                }
            }
        }
    }
}
