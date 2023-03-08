use super::{connector_type::ConnectorType, svg::Svg, Empty, Mergable, PartialEmpty};

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

//TODO: Make some of these fields enums
/// EquipmentType represents a type of equipment
///
/// Anything from a rackmount piece of gear to an outlet or terminal block
#[derive(Debug, Default, PartialEq)]
pub struct EquipmentType {
    //TODO: add dimensions here
    /// Internal ID of `EquipmentType`
    pub id: String,
    /// Manufacturer of Equipment
    pub manufacturer: Option<String>,
    /// Model of Equipment
    pub model: Option<String>,
    /// Part Number of Equipment
    pub part_number: Option<String>,
    /// Manufacturer's Part Number
    pub manufacturer_part_number: Option<String>,
    /// Supplier of Equipment
    pub supplier: Option<String>,
    /// Supplier's Part Number
    pub supplier_part_number: Option<String>,
    /// Optional text description
    pub description: Option<String>,
    /// List of mounting options for equipment
    pub mount_type: Option<String>,
    /// Equipment Type (audio, video, mix, lighting, networking, patch panel, power)
    pub equip_type: Option<String>,
    /// faces represents a visual representation of each face of a piece of equipment
    pub faces: Option<Vec<EquipFace>>,
    /// visual representation of the equipment
    // TODO: figure out what angle to standardize on, or
    // just rely on the face vis_rep
    pub visual_rep: Svg,
}

/// `EquipFace` represents one physical face of equipment.
///
/// May have 2 faces for something like a patch panel, or 6 for a cube, or 1 for an unrolled
/// sphere, etc.
#[derive(Debug, Default, PartialEq, Clone)]
pub struct EquipFace {
    /// Name/ID of equipment face
    pub name: String,
    /// visual representation of equipment face, without connectors
    pub vis_rep: Option<Svg>,
    /// all connectors that are on this face of equipment
    pub connectors: Option<Vec<EquipConnector>>,
}

//TODO: Make some of these fields enums
/// EquipmentConnector represents an instance of a [`ConnectorType`](super::connector_type::ConnectorType) in
/// a EquipmentType
#[derive(Debug, Default, PartialEq, Clone)]
pub struct EquipConnector {
    /// Internal ID of `EquipmentConnector`
    /// ConnectorType
    pub connector_type: Rc<RefCell<ConnectorType>>,
    /// electrical direction, used for basic rule mapping, (input, output, power input, power
    /// output, bidirectiona, passive)
    pub direction: Option<String>,
    /// location of connector on face from left of visrep. Origin is bottom left
    pub x: u64,
    /// location of connector on face from bottom of visrep. Origin is bottom left
    pub y: u64,
}
impl EquipmentType {
    /// Creates an empty instance of `EquipmentType`
    pub fn new() -> Self {
        Self {
            id: String::new(),
            manufacturer: None,
            model: None,
            part_number: None,
            manufacturer_part_number: None,
            supplier: None,
            supplier_part_number: None,
            description: None,
            mount_type: None,
            equip_type: None,
            faces: None,
            visual_rep: Svg::new(),
        }
    }
}

impl Mergable for EquipmentType {
    fn merge_prompt(
        &mut self,
        other: &Self,
        prompt_fn: fn(HashMap<String, [String; 2]>) -> HashMap<String, bool>,
    ) {
        //TODO: maybe check for partial_empty/empty here on other
        let mut input_map: HashMap<String, [String; 2]> = HashMap::new();
        if self.id != other.id {
            panic! {"attempting to merge structs with different IDs. This shouldn't have happened."}
        }
        if self.manufacturer != other.manufacturer {
            input_map.insert(
                "Manufacturer".to_string(),
                [
                    {
                        if let Some(manufacturer) = self.manufacturer.clone() {
                            manufacturer
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(manufacturer) = other.manufacturer.clone() {
                            manufacturer
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.model != other.model {
            input_map.insert(
                "Model".to_string(),
                [
                    {
                        if let Some(model) = self.model.clone() {
                            model
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(model) = other.model.clone() {
                            model
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.part_number != other.part_number {
            input_map.insert(
                "Part Number".to_string(),
                [
                    {
                        if let Some(part_number) = self.part_number.clone() {
                            part_number
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(part_number) = other.part_number.clone() {
                            part_number
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.manufacturer_part_number != other.manufacturer_part_number {
            input_map.insert(
                "Manufacturer Part Number".to_string(),
                [
                    {
                        if let Some(manufacturer_part_number) =
                            self.manufacturer_part_number.clone()
                        {
                            manufacturer_part_number
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(manufacturer_part_number) =
                            other.manufacturer_part_number.clone()
                        {
                            manufacturer_part_number
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.supplier != other.supplier {
            input_map.insert(
                "Supplier".to_string(),
                [
                    {
                        if let Some(supplier) = self.supplier.clone() {
                            supplier
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(supplier) = other.supplier.clone() {
                            supplier
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.supplier_part_number != other.supplier_part_number {
            input_map.insert(
                "Supplier Part Number".to_string(),
                [
                    {
                        if let Some(supplier_part_number) = self.supplier_part_number.clone() {
                            supplier_part_number
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(supplier_part_number) = other.supplier_part_number.clone() {
                            supplier_part_number
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.description != other.description {
            input_map.insert(
                "Description".to_string(),
                [
                    {
                        if let Some(description) = self.description.clone() {
                            description
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(description) = other.description.clone() {
                            description
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.mount_type != other.mount_type {
            input_map.insert(
                "Mount Type".to_string(),
                [
                    {
                        if let Some(mount_type) = self.mount_type.clone() {
                            mount_type
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(mount_type) = other.mount_type.clone() {
                            mount_type
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.equip_type != other.equip_type {
            input_map.insert(
                "Equipment Type".to_string(),
                [
                    {
                        if let Some(equip_type) = self.equip_type.clone() {
                            equip_type
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(equip_type) = other.equip_type.clone() {
                            equip_type
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.faces != other.faces {
            let mut self_string = String::new();
            let mut other_string = String::new();
            if let Some(self_faces) = &self.faces {
                for face in self_faces {
                    self_string.push('(');
                    self_string.push_str(face.name.as_str());
                    self_string.push_str(", Connectors:  ");

                    if let Some(connectors) = &face.connectors {
                        let mut connector_string = String::new();
                        for connector in connectors {
                            connector_string.push('(');
                            connector_string
                                .push_str(connector.connector_type.borrow().id.as_str());
                            connector_string.push_str(", Direction: ");

                            if let Some(direction) = &connector.direction {
                                connector_string.push_str(direction.as_str());
                            }

                            connector_string.push_str(", X: ");
                            connector_string.push_str(connector.x.to_string().as_str());
                            connector_string.push_str(", Y: ");
                            connector_string.push_str(connector.y.to_string().as_str());
                            connector_string.push_str(")\t");
                        }
                        self_string.push_str(connector_string.as_str());
                    }

                    self_string.push_str(")\t");
                }
            } else {
                self_string = String::new();
            }
            if let Some(other_faces) = &other.faces {
                for face in other_faces {
                    other_string.push('(');
                    other_string.push_str(face.name.as_str());
                    other_string.push_str(", Connectors:  ");
                    if let Some(connectors) = &face.connectors {
                        let mut connector_string = String::new();
                        for connector in connectors {
                            connector_string.push('(');
                            connector_string
                                .push_str(connector.connector_type.borrow().id.as_str());
                            connector_string.push_str(", Direction: ");

                            if let Some(direction) = &connector.direction {
                                connector_string.push_str(direction.as_str());
                            }

                            connector_string.push_str(", X: ");
                            connector_string.push_str(connector.x.to_string().as_str());
                            connector_string.push_str(", Y: ");
                            connector_string.push_str(connector.y.to_string().as_str());
                            connector_string.push_str(")\t");
                        }
                        other_string.push_str(connector_string.as_str());
                    }
                    other_string.push_str(")\t");
                }
            } else {
                other_string = String::new();
            }
            input_map.insert("Faces".to_string(), [self_string, other_string]);
        }
        if self.visual_rep != other.visual_rep {
            input_map.insert(
                "Visual Representation".to_string(),
                [self.visual_rep.to_string(), other.visual_rep.to_string()],
            );
        }

        let results = prompt_fn(input_map);
        // false means don't replace value in self struct
        if results["Manufacturer"] {
            self.manufacturer = other.manufacturer.clone();
        }
        if results["Model"] {
            self.model = other.model.clone();
        }
        if results["Part Number"] {
            self.part_number = other.part_number.clone();
        }
        if results["Manufacturer Part Number"] {
            self.manufacturer_part_number = other.manufacturer_part_number.clone();
        }
        if results["Supplier"] {
            self.supplier = other.supplier.clone();
        }
        if results["Supplier Part Number"] {
            self.supplier_part_number = other.supplier_part_number.clone();
        }
        if results["Description"] {
            self.description = other.description.clone();
        }
        if results["Mount Type"] {
            self.mount_type = other.mount_type.clone();
        }
        if results["Equipment Type"] {
            self.equip_type = other.equip_type.clone();
        }
        if results["Faces"] {
            self.faces = other.faces.clone();
        }
        if results["Visual Representation"] {
            self.visual_rep = other.visual_rep.clone();
        }
    }
}

impl Empty for EquipmentType {
    fn is_empty(&self) -> bool {
        self == &Self::new()
    }
}

impl PartialEmpty for EquipmentType {
    fn is_partial_empty(&self) -> bool {
        let tester = Self::new();
        self.manufacturer == tester.manufacturer
            && self.model == tester.model
            && self.part_number == tester.part_number
            && self.manufacturer_part_number == tester.manufacturer_part_number
            && self.supplier == tester.supplier
            && self.supplier_part_number == tester.supplier_part_number
            && self.description == tester.description
            && self.mount_type == tester.mount_type
            && self.equip_type == tester.equip_type
            && self.faces == tester.faces
            && self.visual_rep == tester.visual_rep
    }
}

impl fmt::Display for EquipConnector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Equipment Connector:")?;
        writeln!(f, "Connector: {}", &self.connector_type.borrow())?;
        if let Some(direction) = &self.direction {
            writeln!(f, "Direction: {direction}")?;
        }
        writeln!(f, "X coordinate: {}", self.x)?;
        writeln!(f, "Y coordinate: {}", self.y)?;
        Ok(())
    }
}
impl fmt::Display for EquipmentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Equipment Type:")?;
        if let Some(manufacturer) = &self.manufacturer {
            write!(f, "Manufacturer: {manufacturer}")?;
        }
        if let Some(model) = &self.model {
            write!(f, "Model: {model}")?;
        }
        if let Some(part_number) = &self.part_number {
            write!(f, "Part Number: {part_number}")?;
        }
        if let Some(manufacturer_part_number) = &self.manufacturer_part_number {
            write!(f, "Manufacturer Part Number: {manufacturer_part_number}")?;
        }
        if let Some(supplier) = &self.supplier {
            write!(f, "Supplier: {supplier}")?;
        }
        if let Some(supplier_part_number) = &self.supplier_part_number {
            write!(f, "Supplier Part Number: {supplier_part_number}")?;
        }
        if let Some(description) = &self.description {
            write!(f, "Description: {description}")?;
        }
        if let Some(mount_type) = &self.mount_type {
            write!(f, "Mount Type: {mount_type}")?;
        }
        if let Some(equip_type) = &self.equip_type {
            write!(f, "Equipment Type: {equip_type}")?;
        }
        //TODO: implement loops over faces and connectors
        //TODO: implement svg validation rules here
        Ok(())
    }
}
