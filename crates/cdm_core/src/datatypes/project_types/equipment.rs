use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    datatypes::{
        library_types::Library,
        svg::Svg,
        util_types::{IECCodes, PhysicalLocation, SymbolStyle, UserFields},
    },
    error::LibraryError,
    traits::{FromFile, ProjectData, SchematicRepresentation},
};

/// `Equipment` represents a particular instance of an `EquipmentType`.
/// This is the physical unit you would hold in your hand
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::partial_pub_fields, reason = "contained_datafile_path is not part of public API")]
pub struct Equipment {
    /// The type of equipment of the instance
    pub equipment_type: String,
    /// The structured name of the equipment
    pub identifier: Option<String>,
    /// The particular mounting type of this instance
    /// must be in list of mounting types defined in `equip_type.mounting_type`.
    /// Validated on import
    pub mounting_type: Option<String>,
    /// The containing `Enclosure` ID
    pub enclosure: Option<String>,
    /// The ID of the `MountPoint` within the `Enclosure`
    pub mount_point: Option<String>,
    /// The physical location of this piece of equipment
    pub physical_location: Option<PhysicalLocation>,
    /// fields for IEC coding
    pub iec_codes: Option<IECCodes>,
    /// Description
    pub description: Option<String>,
    /// Optional user Fields
    pub user_fields: Option<UserFields>,
    /// Optional styling data for schematic symbol
    pub symbol_style: Option<SymbolStyle>,
    /// datafile the struct instance was read in from
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
    fn schematic_symbol(&self, library: &Library, symbol_selector: Option<usize>) -> Result<Svg, LibraryError> {
        let equipment_type = library
            .equipment_types
            .get(&self.equipment_type)
            .ok_or(LibraryError::ValueNotFound {
                id: self.equipment_type.clone(),
                //TODO: figure out how to insert the ID of the equipment here
                found_in: "equipment".to_owned(),
                library_type: "Equipment Type".to_owned(),
            })?;
        let equipment_schematic_symbols = equipment_type.schematic_symbols.clone();
        if equipment_schematic_symbols.is_empty() {
            return Err(LibraryError::DataMissing {
                id: self.equipment_type.clone(),
                //TODO: figure out how to insert the ID of the equipment here
                found_in: "equipment".to_owned(),
                library_type: "Equipment Type".to_owned(),
                data_missing: "Schematic Symbols".to_owned(),
            });
        }

        let schematic_symbol_type_id =
            equipment_schematic_symbols
                .get(symbol_selector.unwrap_or(0))
                .ok_or(LibraryError::DataMissing {
                    id: self.equipment_type.clone(),
                    //TODO: figure out how to insert the ID of the equipment here
                    found_in: "equipment".to_owned(),
                    library_type: "Equipment Type".to_owned(),
                    data_missing: "At least one schematic symbol needs to be specified".to_owned(),
                })?;
        let schematic_symbol = library
            .schematic_symbol_types
            .get(schematic_symbol_type_id)
            .ok_or(LibraryError::ValueNotFound {
                id: schematic_symbol_type_id.clone(),
                //TODO: figure out how to insert the ID of the equipment here
                found_in: "equipment".to_owned(),
                library_type: "Schematic Symbol".to_owned(),
            })?
            .visual_representation
            .clone();

        Ok(schematic_symbol)
    }
}

impl ProjectData for Equipment {}
