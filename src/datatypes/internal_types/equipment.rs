use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use super::equipment_type::EquipmentType;

/// `Equipment` represents a particular instance of an `EquipmentType`.
/// This is the physical unit you would hold in your hand
#[derive(Debug, Default)]
pub struct Equipment {
    /// Internal `id` of equipment instance
    pub id: String,
    /// The type of equipment of the instance
    pub equip_type: Rc<RefCell<EquipmentType>>,
    /// The structured name of the equipment
    pub identifier: Option<String>,
    /// The particular mounting type of this instance
    /// must be in list of mounting types defined in `equip_type.mounting_type`
    pub mounting_type: Option<String>,
    /// The individual location
    pub location: Option<String>, //TODO: Change to Option<location::Location>
    /// Description
    pub description: Option<String>,
}

impl fmt::Display for Equipment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Equipment Instance:")?;
        if let Some(manufacturer) = &self.equip_type.borrow().manufacturer {
            writeln!(f, "Manufacturer: {manufacturer}")?;
        }
        //TODO: Decide how much data from Equiptype we want to display for instance
        if let Some(model) = &self.equip_type.borrow().model {
            writeln!(f, "Model: {model}")?;
        }
        if let Some(part_number) = &self.equip_type.borrow().part_number {
            writeln!(f, "Part Number: {part_number}")?;
        }
        if let Some(manufacturer_part_number) = &self.equip_type.borrow().manufacturer_part_number {
            writeln!(f, "Manufacturer Part Number: {manufacturer_part_number}")?;
        }
        if let Some(supplier) = &self.equip_type.borrow().supplier {
            writeln!(f, "Supplier: {supplier}")?;
        }
        if let Some(supplier_part_number) = &self.equip_type.borrow().supplier_part_number {
            writeln!(f, "Supplier Part Number: {supplier_part_number}")?;
        }
        if let Some(identifier) = &self.identifier {
            writeln!(f, "Equipment Identifier: {identifier}")?;
        }
        if let Some(mounting_type) = &self.mounting_type {
            writeln!(f, "Mounting Type: {mounting_type}")?;
        }
        if let Some(location) = &self.location {
            writeln!(f, "Location: {location}")?;
        }
        if let Some(description) = &self.description {
            writeln!(f, "Description: {description}")?;
        }
        Ok(())
    }
}
