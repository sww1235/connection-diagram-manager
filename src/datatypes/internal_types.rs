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

use std::cell::RefCell;
use std::rc::Rc;

/// `Datastore` represents all data that is read from all source files
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
}
