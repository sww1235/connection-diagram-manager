///`cable_type` represents a cable with multiple cores
pub mod cable_type;
/// `connector_type` represents a connector
pub mod connector_type;
/// `enclosure_type` represents a type of enclosure
pub mod enclosure_type;
/// `equipment_type` represents a type of equipment
pub mod equipment_type;
/// `mounting_rail_type` represents a type of mounting rail such as DIN rail or unistrut
pub mod mounting_rail_type;
/// `pathway_type` represents a type of pathway for wires or cables
pub mod pathway_type;
/// `schematic_symbol_type` represents a type of schematic symbol
pub mod schematic_symbol_type;
/// `term_cable_type` represents a cable that has connectors assembled on to it
pub mod term_cable_type;
/// `terminal_type` represents a type of terminal
/// This module also includes related defintions including accessories and jumpers
pub mod terminal_type;
/// `wire_type` represents an individual wire with optional insulation
pub mod wire_type;

use std::{collections::BTreeMap, path::Path};

use serde::{Deserialize, Serialize};

use crate::{error::Error, traits::FromFile as _, util_functions};

/// `Library` represents all library data used in program
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Library {
    /// contains all cable types read in from file, and/or added in via program logic
    pub cable_types: BTreeMap<String, cable_type::CableType>,
    /// contains all connector types read in from file, and/or added in via program logic
    pub connector_types: BTreeMap<String, connector_type::ConnectorType>,
    /// contains all enclosure types read in from file, and/or added in via program logic
    pub enclosure_types: BTreeMap<String, enclosure_type::EnclosureType>,
    /// contains all equipment types read in from file, and/or added in via program logic
    pub equipment_types: BTreeMap<String, equipment_type::EquipmentType>,
    /// contains all mounting rail types read in from file, and/or added in via program logic
    pub mounting_rail_types: BTreeMap<String, mounting_rail_type::MountingRailType>,
    /// contains all pathway types read in from file, and/or added in via program logic
    pub pathway_types: BTreeMap<String, pathway_type::PathwayType>,
    /// contains all schematic symbol types read in from file and/or added in via program logic
    pub schematic_symbol_types: BTreeMap<String, schematic_symbol_type::SchematicSymbolType>,
    /// contains all terminated cable types read in from file, and/or added in via program logic
    pub term_cable_types: BTreeMap<String, term_cable_type::TermCableType>,
    /// contains all terminal types read in from file, and/or added in via program logic
    pub terminal_types: BTreeMap<String, terminal_type::TerminalType>,
    /// contains all terminal strip jumper types read in from file, and/or added in via program
    /// logic
    pub terminal_strip_jumper_types: BTreeMap<String, terminal_type::TerminalStripJumperType>,
    /// contains all terminal accessory types read in from file, and/or added in via program logic
    pub terminal_accessory_types: BTreeMap<String, terminal_type::TerminalAccessoryType>,
    /// contains all terminal strip accessory types read in from file, and/or added in via program
    /// logic
    pub terminal_strip_accessory_types: BTreeMap<String, terminal_type::TerminalStripAccessoryType>,
    /// contains all wire types read in from file, and/or added in via program logic
    pub wire_types: BTreeMap<String, wire_type::WireType>,
}

//TODO: implement validation function for all datatypes

impl Library {
    /// Merges two instances of `Library`, validating that there are no key conflicts between the
    /// two instances
    ///
    /// # Errors
    ///
    /// Will error if there are duplicate keys found in `other` map
    #[inline(never)]
    pub fn merge(&mut self, test_map: Library, test_file: &Path) -> Result<(), Error> {
        util_functions::merge_btreemaps(&mut self.cable_types, test_map.cable_types, test_file)?;
        util_functions::merge_btreemaps(&mut self.connector_types, test_map.connector_types, test_file)?;
        util_functions::merge_btreemaps(&mut self.enclosure_types, test_map.enclosure_types, test_file)?;
        util_functions::merge_btreemaps(&mut self.equipment_types, test_map.equipment_types, test_file)?;
        util_functions::merge_btreemaps(&mut self.mounting_rail_types, test_map.mounting_rail_types, test_file)?;
        util_functions::merge_btreemaps(&mut self.pathway_types, test_map.pathway_types, test_file)?;
        util_functions::merge_btreemaps(&mut self.term_cable_types, test_map.term_cable_types, test_file)?;
        util_functions::merge_btreemaps(&mut self.terminal_types, test_map.terminal_types, test_file)?;
        util_functions::merge_btreemaps(
            &mut self.terminal_strip_jumper_types,
            test_map.terminal_strip_jumper_types,
            test_file,
        )?;
        util_functions::merge_btreemaps(
            &mut self.terminal_accessory_types,
            test_map.terminal_accessory_types,
            test_file,
        )?;
        util_functions::merge_btreemaps(
            &mut self.terminal_strip_accessory_types,
            test_map.terminal_strip_accessory_types,
            test_file,
        )?;
        util_functions::merge_btreemaps(&mut self.wire_types, test_map.wire_types, test_file)?;
        Ok(())
    }

    /// Inserts datafile path into all structs in the called library
    #[inline(never)]
    pub fn add_datafile_paths(&mut self, datafile_path: &Path) {
        // Cable Types
        if !self.cable_types.is_empty() {
            for cable_type in self.cable_types.values_mut() {
                cable_type.set_datafile(datafile_path);
            }
        }
        // Connector Types
        if !self.connector_types.is_empty() {
            for connector_type in self.connector_types.values_mut() {
                connector_type.set_datafile(datafile_path);
            }
        }
        // Enclosure Types
        if !self.enclosure_types.is_empty() {
            for enclosure_type in self.enclosure_types.values_mut() {
                enclosure_type.set_datafile(datafile_path);
            }
        }
        // Equipment Types
        if !self.equipment_types.is_empty() {
            for equipment_type in self.equipment_types.values_mut() {
                equipment_type.set_datafile(datafile_path);
            }
        }
        // Mounting Rail Types
        if !self.mounting_rail_types.is_empty() {
            for mounting_rail_type in self.mounting_rail_types.values_mut() {
                mounting_rail_type.set_datafile(datafile_path);
            }
        }
        // Pathway Types
        if !self.pathway_types.is_empty() {
            for pathway_type in self.pathway_types.values_mut() {
                pathway_type.set_datafile(datafile_path);
            }
        }
        // Schematic Symbol Types
        if !self.schematic_symbol_types.is_empty() {
            for schematic_symbol_type in self.schematic_symbol_types.values_mut() {
                schematic_symbol_type.set_datafile(datafile_path);
            }
        }
        // Term Cable Types
        if !self.term_cable_types.is_empty() {
            for term_cable_type in self.term_cable_types.values_mut() {
                term_cable_type.set_datafile(datafile_path);
            }
        }
        // Terminal Types
        if !self.terminal_types.is_empty() {
            for terminal_type in self.terminal_types.values_mut() {
                terminal_type.set_datafile(datafile_path);
            }
        }
        // Terminal Strip Jumper Types
        if !self.terminal_strip_jumper_types.is_empty() {
            for terminal_strip_jumper_type in self.terminal_strip_jumper_types.values_mut() {
                terminal_strip_jumper_type.set_datafile(datafile_path);
            }
        }
        // Terminal Accessory Types
        if !self.terminal_accessory_types.is_empty() {
            for terminal_accessory_type in self.terminal_accessory_types.values_mut() {
                terminal_accessory_type.set_datafile(datafile_path);
            }
        }
        // Terminal Strip Accessory Types
        if !self.terminal_strip_accessory_types.is_empty() {
            for terminal_strip_accessory_type in self.terminal_strip_accessory_types.values_mut() {
                terminal_strip_accessory_type.set_datafile(datafile_path);
            }
        }
        // Wire Types
        if !self.wire_types.is_empty() {
            for wire_type in self.wire_types.values_mut() {
                wire_type.set_datafile(datafile_path);
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::too_many_lines)]
mod tests {
    use std::{collections::BTreeMap, fs, path::PathBuf};

    use num_rational::Rational64;
    use pretty_assertions::assert_eq;
    use uom::si::{
        area::{square_inch, square_micrometer, square_millimeter},
        electric_potential::volt,
        length::{inch, millimeter, point_computer},
        rational64,
        temperature_interval::degree_celsius,
    };

    use crate::{
        datatypes::{
            color::Color,
            library_types::{
                Library,
                cable_type::{CableCore, CableLayer, CableType, LayerType},
                wire_type::WireType,
            },
            unit_helper::{
                cross_sectional_area::CrossSectionalArea,
                electric_potential::ElectricPotential,
                length::Length,
                temperature_interval::TemperatureInterval,
            },
            util_types::{Catalog, CrossSection, Dimension, LineStyle},
        },
        traits::FromFile,
    };

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
    fn read_datafile_library_basic() {}

    // TODO: For cables:
    // [x] minimal realistic (soow14_3)
    // [x] minimal with multiple layers
    // [x] minimal with 1 layer and 1 core
    // [x] full with multiple cores and multiple layers
    // [x] Vary the cross section
    // [x] incorrect unit string
    // [x] try with a few different unit strings
    // [x] multiple cables in one file
    // [x] multiple cables in one file, with one cable containing cores of the other cables
    // [x] merging 2 libraries together with cables
    // [x] wires and cables in the same file
    #[test]
    /// Test importing a realistic minimal example file
    ///
    /// No validation of string keys within library
    fn read_datafile_library_cable_minimal_realistic() {
        let soow14_3 = CableType {
            cross_sect_area: CrossSectionalArea {
                original_unit: "square inch".to_string(),
                value: rational64::Area::new::<square_inch>(Rational64::new(14_389_229, 64_008_858)),
            },
            cross_section: CrossSection::Circular,
            layers: vec![CableLayer {
                layer_number: 1,
                layer_type: LayerType::Jacket,
                material: None,
                ac_electric_potential_rating: None,
                dc_electric_potential_rating: None,
                temperature_rating: None,
                rating: None,
                thickness: None,
                color: None,
            }],
            cores: {
                let mut cores = BTreeMap::new();
                cores.insert("green".to_string(), CableCore::WireType("soow14_green_inner".to_string()));
                cores.insert("white".to_string(), CableCore::WireType("soow14_white_inner".to_string()));
                cores.insert("black".to_string(), CableCore::WireType("soow14_black_inner".to_string()));
                cores
            },
            cable_type_code: None,
            catalog: None,
            dimensions: None,
            line_style: None,
            contained_datafile_path: PathBuf::from("../../resources/test/library_tests/cable_type_test_minimal_realistic.toml")
                .canonicalize()
                .unwrap(),
        };
        let test_library = Library {
            cable_types: {
                let mut cable_types = BTreeMap::new();
                cable_types.insert("soow14_3".to_string(), soow14_3);
                cable_types
            },
            connector_types: BTreeMap::new(),
            enclosure_types: BTreeMap::new(),
            equipment_types: BTreeMap::new(),
            mounting_rail_types: BTreeMap::new(),
            pathway_types: BTreeMap::new(),
            schematic_symbol_types: BTreeMap::new(),
            term_cable_types: BTreeMap::new(),
            terminal_types: BTreeMap::new(),
            terminal_strip_jumper_types: BTreeMap::new(),
            terminal_accessory_types: BTreeMap::new(),
            terminal_strip_accessory_types: BTreeMap::new(),
            wire_types: BTreeMap::new(),
        };
        let library_filepath = PathBuf::from("../../resources/test/library_tests/cable_type_test_minimal_realistic.toml")
            .canonicalize()
            .unwrap();
        let library_file_contents = fs::read_to_string(&library_filepath).unwrap();
        let mut library_file: Library = toml::from_str(&library_file_contents).unwrap();
        library_file.add_datafile_paths(&library_filepath);
        assert_eq!(test_library, library_file);
    }
    #[test]
    /// Test importing a cable with only required values
    ///
    /// No validation of string keys within library
    fn read_datafile_library_cable_minimal() {
        let soow14_1 = CableType {
            cross_sect_area: CrossSectionalArea {
                original_unit: "square inch".to_string(),
                value: rational64::Area::new::<square_inch>(Rational64::new(14_389_229, 64_008_858)),
            },
            cross_section: CrossSection::Circular,
            layers: vec![CableLayer {
                layer_number: 1,
                layer_type: LayerType::Jacket,
                material: None,
                ac_electric_potential_rating: None,
                dc_electric_potential_rating: None,
                temperature_rating: None,
                rating: None,
                thickness: None,
                color: None,
            }],
            cores: {
                let mut cores = BTreeMap::new();
                cores.insert(
                    "flamingo".to_string(),
                    CableCore::WireType("soow14_flamingo_inner".to_string()),
                );
                cores
            },
            cable_type_code: None,
            catalog: None,
            dimensions: None,
            line_style: None,
            contained_datafile_path: PathBuf::from("../../resources/test/library_tests/cable_type_test_minimal.toml")
                .canonicalize()
                .unwrap(),
        };
        let test_library = Library {
            cable_types: {
                let mut cable_types = BTreeMap::new();
                cable_types.insert("soow14_1".to_string(), soow14_1);
                cable_types
            },
            connector_types: BTreeMap::new(),
            enclosure_types: BTreeMap::new(),
            equipment_types: BTreeMap::new(),
            mounting_rail_types: BTreeMap::new(),
            pathway_types: BTreeMap::new(),
            schematic_symbol_types: BTreeMap::new(),
            term_cable_types: BTreeMap::new(),
            terminal_types: BTreeMap::new(),
            terminal_strip_jumper_types: BTreeMap::new(),
            terminal_accessory_types: BTreeMap::new(),
            terminal_strip_accessory_types: BTreeMap::new(),
            wire_types: BTreeMap::new(),
        };
        let library_filepath = PathBuf::from("../../resources/test/library_tests/cable_type_test_minimal.toml")
            .canonicalize()
            .unwrap();
        let library_file_contents = fs::read_to_string(&library_filepath).unwrap();
        let mut library_file: Library = toml::from_str(&library_file_contents).unwrap();
        library_file.add_datafile_paths(&library_filepath);
        assert_eq!(test_library, library_file);
    }
    #[test]
    /// Test importing a cable with minimal values and multiple layers defined
    ///
    /// No validation of string keys within library
    fn read_datafile_library_cable_multi_layer() {
        let triax_rg11 = CableType {
            cross_sect_area: CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(137, 1)),
            },
            cross_section: CrossSection::Circular,
            layers: vec![
                CableLayer {
                    layer_number: 1,
                    layer_type: LayerType::Insulation,
                    material: Some("High Density Polyethylene Foam".to_string()),
                    ac_electric_potential_rating: None,
                    dc_electric_potential_rating: None,
                    temperature_rating: None,
                    rating: None,
                    thickness: Some(Length {
                        original_unit: "inch".to_string(),
                        value: rational64::Length::new::<inch>(Rational64::new(31, 250)),
                    }),
                    color: Some(Color::White),
                },
                CableLayer {
                    layer_number: 2,
                    layer_type: LayerType::Shield,
                    material: Some("Bare Copper".to_string()),
                    ac_electric_potential_rating: None,
                    dc_electric_potential_rating: None,
                    temperature_rating: None,
                    rating: None,
                    thickness: None,
                    color: None,
                },
                CableLayer {
                    layer_number: 3,
                    layer_type: LayerType::Jacket,
                    material: Some("Polyethylene".to_string()),
                    ac_electric_potential_rating: None,
                    dc_electric_potential_rating: None,
                    temperature_rating: None,
                    rating: None,
                    thickness: Some(Length {
                        original_unit: "inch".to_string(),
                        value: rational64::Length::new::<inch>(Rational64::new(79, 1000)),
                    }),
                    color: Some(Color::Black),
                },
                CableLayer {
                    layer_number: 4,
                    layer_type: LayerType::Shield,
                    material: Some("Bare Copper".to_string()),
                    ac_electric_potential_rating: None,
                    dc_electric_potential_rating: None,
                    temperature_rating: None,
                    rating: None,
                    thickness: None,
                    color: None,
                },
                CableLayer {
                    layer_number: 5,
                    layer_type: LayerType::Jacket,
                    material: Some("Polyvinyl Chloride".to_string()),
                    ac_electric_potential_rating: None,
                    dc_electric_potential_rating: None,
                    temperature_rating: Some(TemperatureInterval {
                        original_unit: "degree Celsius".to_string(),
                        value: rational64::TemperatureInterval::new::<degree_celsius>(Rational64::new(80, 1)),
                    }),
                    rating: None,
                    thickness: Some(Length {
                        original_unit: "inch".to_string(),
                        value: rational64::Length::new::<inch>(Rational64::new(13, 25)),
                    }),
                    color: Some(Color::Yellow),
                },
            ],
            cores: {
                let mut cores = BTreeMap::new();
                cores.insert(
                    "inner".to_string(),
                    CableCore::WireType("triax_15awg_copper_inner".to_string()),
                );
                cores
            },
            cable_type_code: Some("RG11".to_string()),
            catalog: None,
            dimensions: None,
            line_style: None,
            contained_datafile_path: PathBuf::from("../../resources/test/library_tests/cable_type_test_multi_layer.toml")
                .canonicalize()
                .unwrap(),
        };
        let test_library = Library {
            cable_types: {
                let mut cable_types = BTreeMap::new();
                cable_types.insert("triax_rg11".to_string(), triax_rg11);
                cable_types
            },
            connector_types: BTreeMap::new(),
            enclosure_types: BTreeMap::new(),
            equipment_types: BTreeMap::new(),
            mounting_rail_types: BTreeMap::new(),
            pathway_types: BTreeMap::new(),
            schematic_symbol_types: BTreeMap::new(),
            term_cable_types: BTreeMap::new(),
            terminal_types: BTreeMap::new(),
            terminal_strip_jumper_types: BTreeMap::new(),
            terminal_accessory_types: BTreeMap::new(),
            terminal_strip_accessory_types: BTreeMap::new(),
            wire_types: BTreeMap::new(),
        };
        let library_filepath = PathBuf::from("../../resources/test/library_tests/cable_type_test_multi_layer.toml")
            .canonicalize()
            .unwrap();
        let library_file_contents = fs::read_to_string(&library_filepath).unwrap();
        let mut library_file: Library = toml::from_str(&library_file_contents).unwrap();
        library_file.add_datafile_paths(&library_filepath);
        assert_eq!(test_library, library_file);
    }
    #[test]
    /// Test importing a cable with all values defined
    ///
    /// No validation of string keys within library
    fn read_datafile_library_cable_full() {
        let soow14_3 = CableType {
            cross_sect_area: CrossSectionalArea {
                original_unit: "square inch".to_string(),
                value: rational64::Area::new::<square_inch>(Rational64::new(14_389_229, 64_008_858)),
            },
            cross_section: CrossSection::Circular,
            cable_type_code: Some("SOOW".to_string()),
            layers: vec![CableLayer {
                layer_number: 1,
                layer_type: LayerType::Jacket,
                material: Some("Clorinated Polyethylene".to_string()),
                ac_electric_potential_rating: Some(ElectricPotential {
                    original_unit: "volt".to_string(),
                    value: rational64::ElectricPotential::new::<volt>(Rational64::new(600, 1)),
                }),
                dc_electric_potential_rating: Some(ElectricPotential {
                    original_unit: "volt".to_string(),
                    value: rational64::ElectricPotential::new::<volt>(Rational64::new(600, 1)),
                }),
                temperature_rating: Some(TemperatureInterval {
                    original_unit: "degree Celsius".to_string(),
                    value: rational64::TemperatureInterval::new::<degree_celsius>(Rational64::new(90, 1)),
                }),
                rating: Some("UL Listed".to_string()),
                thickness: Some(Length {
                    original_unit: "mm".to_string(),
                    value: rational64::Length::new::<millimeter>(Rational64::new(203, 100)),
                }),
                color: Some(Color::Black),
            }],
            cores: {
                let mut cores = BTreeMap::new();
                cores.insert("green".to_string(), CableCore::WireType("soow14_green_inner".to_string()));
                cores.insert("white".to_string(), CableCore::WireType("soow14_white_inner".to_string()));
                cores.insert("black".to_string(), CableCore::WireType("soow14_black_inner".to_string()));
                cores
            },
            line_style: Some(LineStyle {
                color: Some(Color::Black),
                secondary_color: Some(Color::Red),
                line_thickness: Some(Length {
                    original_unit: "point (computer)".to_string(),
                    value: rational64::Length::new::<point_computer>(Rational64::new(20, 1)),
                }),
                line_appearance: Some([4, 1].to_vec()),
            }),
            dimensions: Some(Dimension {
                height: Length {
                    original_unit: "inch".to_string(),
                    value: rational64::Length::new::<inch>(Rational64::new(13, 25)),
                },
                width: Length {
                    original_unit: "inch".to_string(),
                    value: rational64::Length::new::<inch>(Rational64::new(13, 25)),
                },
                depth: None,
                diameter: Some(Length {
                    original_unit: "inch".to_string(),
                    value: rational64::Length::new::<inch>(Rational64::new(13, 25)),
                }),
            }),
            catalog: Some(Catalog {
                manufacturer: Some("Priority Wire & Cable".to_string()),
                model: Some("14 AWG SOOW 600V Portable Power Cord 3 Conductor".to_string()),
                description: Some("14 AWG SOOW cable 3 Conductor".to_string()),
                //totally made up
                part_number: Some("00-500-3".to_string()),
                manufacturer_part_number: Some("14-03SOOW".to_string()),
                //not true
                supplier: Some("Wire & Cable Your Way".to_string()),
                supplier_part_number: Some("SOO14s3-FT".to_string()),
            }),
            contained_datafile_path: PathBuf::from("../../resources/test/library_tests/cable_type_test_full_realistic.toml")
                .canonicalize()
                .unwrap(),
        };
        let test_library = Library {
            cable_types: {
                let mut cable_types = BTreeMap::new();
                cable_types.insert("soow14_3".to_string(), soow14_3);
                cable_types
            },
            connector_types: BTreeMap::new(),
            enclosure_types: BTreeMap::new(),
            equipment_types: BTreeMap::new(),
            mounting_rail_types: BTreeMap::new(),
            pathway_types: BTreeMap::new(),
            schematic_symbol_types: BTreeMap::new(),
            term_cable_types: BTreeMap::new(),
            terminal_types: BTreeMap::new(),
            terminal_strip_jumper_types: BTreeMap::new(),
            terminal_accessory_types: BTreeMap::new(),
            terminal_strip_accessory_types: BTreeMap::new(),
            wire_types: BTreeMap::new(),
        };
        let library_filepath = PathBuf::from("../../resources/test/library_tests/cable_type_test_full_realistic.toml")
            .canonicalize()
            .unwrap();
        let library_file_contents = fs::read_to_string(&library_filepath).unwrap();
        let mut library_file: Library = toml::from_str(&library_file_contents).unwrap();
        library_file.add_datafile_paths(&library_filepath);
        assert_eq!(test_library, library_file);
    }
    #[test]
    #[expect(non_snake_case)]
    //TODO: duplicate this and test with direct AWG
    /// This tests the following:
    /// - multiple cable definitions in one file
    /// - multiple cable definitions in one file, with one cable referencing cores of the other
    /// - wires and cables together in the same file
    ///
    /// this is a relatively realistic test, with a mix of filled in values, multiple items in
    /// btreemaps, and validation of btreemap keys
    fn read_datafile_library_cable_multi_cable_wire() {
        let datafile_path = PathBuf::from("../../resources/test/library_tests/cable_type_test_multicore_realistic.toml")
            .canonicalize()
            .unwrap();
        let belden_638AFJ = CableType {
            cross_sect_area: CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(688, 1)),
            },
            cross_section: CrossSection::Circular,
            layers: vec![CableLayer {
                layer_number: 1,
                layer_type: LayerType::Jacket,
                material: Some("Polyvinyl Chloride".to_string()),
                ac_electric_potential_rating: Some(ElectricPotential {
                    original_unit: "volt".to_string(),
                    value: rational64::ElectricPotential::new::<volt>(Rational64::new(300, 1)),
                }),
                dc_electric_potential_rating: Some(ElectricPotential {
                    original_unit: "volt".to_string(),
                    value: rational64::ElectricPotential::new::<volt>(Rational64::new(300, 1)),
                }),
                temperature_rating: Some(TemperatureInterval {
                    original_unit: "degree Celsius".to_string(),
                    value: rational64::TemperatureInterval::new::<degree_celsius>(Rational64::new(75, 1)),
                }),
                thickness: None,
                rating: Some("UL Listed, Plenum, Flamarrest®".to_string()),
                color: Some(Color::Yellow),
            }],
            cores: {
                let mut cores = BTreeMap::new();
                cores.insert(
                    "card_reader".to_string(),
                    CableCore::CableType("belden_638AFJ_card_reader_3_pair_inner".to_string()),
                );
                cores.insert(
                    "door_contact".to_string(),
                    CableCore::CableType("belden_638AFJ_door_contact_inner".to_string()),
                );
                cores.insert("rex".to_string(), CableCore::CableType("belden_638AFJ_rex_inner".to_string()));
                cores.insert(
                    "lock_power".to_string(),
                    CableCore::CableType("belden_638AFJ_lock_power_inner".to_string()),
                );
                cores
            },
            cable_type_code: None,
            line_style: Some(LineStyle {
                color: Some(Color::Yellow),
                secondary_color: None,
                line_thickness: Some(Length {
                    original_unit: "point (computer)".to_string(),
                    value: rational64::Length::new::<point_computer>(Rational64::new(20, 1)),
                }),
                line_appearance: Some([4, 2].to_vec()),
            }),
            dimensions: Some(Dimension {
                height: Length {
                    original_unit: "mm".to_string(),
                    value: rational64::Length::new::<millimeter>(Rational64::new(74, 5)),
                },
                width: Length {
                    original_unit: "mm".to_string(),
                    value: rational64::Length::new::<millimeter>(Rational64::new(74, 5)),
                },
                depth: None,
                diameter: Some(Length {
                    original_unit: "mm".to_string(),
                    value: rational64::Length::new::<millimeter>(Rational64::new(74, 5)),
                }),
            }),
            catalog: Some(Catalog {
                manufacturer: Some("Belden".to_string()),
                model: Some("638AFJ".to_string()),
                description: Some("Access Control, 16c (#18-3pr, #16-4c, #18-6c), Shielded, Outer Jacket, CMP".to_string()),
                //totally made up
                part_number: Some("00-1500-16".to_string()),
                manufacturer_part_number: Some("638AFJ".to_string()),
                supplier: Some("Digikey".to_string()),
                supplier_part_number: Some("BEL8706-1000-ND".to_string()),
            }),
            contained_datafile_path: datafile_path.clone(),
        };

        let belden_638AFJ_card_reader_3_pair_inner = CableType {
            cross_sect_area: CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(1683, 10)),
            },
            cross_section: CrossSection::Circular,
            cable_type_code: None,
            catalog: None,
            line_style: None,
            layers: vec![
                CableLayer {
                    layer_number: 1,
                    layer_type: LayerType::Shield,
                    material: Some("Bi-Laminate (Alum+Poly) Tape".to_string()),
                    ac_electric_potential_rating: None,
                    dc_electric_potential_rating: None,
                    temperature_rating: None,
                    thickness: None,
                    rating: None,
                    color: None,
                },
                CableLayer {
                    layer_number: 2,
                    layer_type: LayerType::Jacket,
                    material: Some("Polyvinyl Chloride".to_string()),
                    ac_electric_potential_rating: None,
                    dc_electric_potential_rating: None,
                    temperature_rating: None,
                    thickness: None,
                    rating: Some("Flamarrest®".to_string()),
                    color: Some(Color::Orange),
                },
            ],
            cores: {
                let mut cores = BTreeMap::new();
                cores.insert(
                    "black".to_string(),
                    CableCore::WireType("belden_638AFJ_18AWG_black_inner".to_string()),
                );
                cores.insert(
                    "red".to_string(),
                    CableCore::WireType("belden_638AFJ_18AWG_red_inner".to_string()),
                );
                cores.insert(
                    "white".to_string(),
                    CableCore::WireType("belden_638AFJ_18AWG_white_inner".to_string()),
                );
                cores.insert(
                    "green".to_string(),
                    CableCore::WireType("belden_638AFJ_18AWG_green_inner".to_string()),
                );
                cores.insert(
                    "brown".to_string(),
                    CableCore::WireType("belden_638AFJ_18AWG_brown_inner".to_string()),
                );
                cores.insert(
                    "orange".to_string(),
                    CableCore::WireType("belden_638AFJ_18AWG_orange_inner".to_string()),
                );
                cores
            },
            dimensions: Some(Dimension {
                height: Length {
                    original_unit: "mm".to_string(),
                    value: rational64::Length::new::<millimeter>(Rational64::new(183, 25)),
                },
                width: Length {
                    original_unit: "mm".to_string(),
                    value: rational64::Length::new::<millimeter>(Rational64::new(183, 25)),
                },
                depth: None,
                diameter: Some(Length {
                    original_unit: "mm".to_string(),
                    value: rational64::Length::new::<millimeter>(Rational64::new(183, 25)),
                }),
            }),
            contained_datafile_path: datafile_path.clone(),
        };
        let belden_638AFJ_door_contact_inner = CableType {
            cross_sect_area: CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(573, 10)),
            },
            cross_section: CrossSection::Circular,
            cable_type_code: None,
            catalog: None,
            line_style: None,
            layers: vec![
                CableLayer {
                    layer_number: 1,
                    layer_type: LayerType::Shield,
                    material: Some("Bi-Laminate (Alum+Poly) Tape".to_string()),
                    ac_electric_potential_rating: None,
                    dc_electric_potential_rating: None,
                    temperature_rating: None,
                    thickness: None,
                    rating: None,
                    color: None,
                },
                CableLayer {
                    layer_number: 2,
                    layer_type: LayerType::Jacket,
                    material: Some("Polyvinyl Chloride".to_string()),
                    ac_electric_potential_rating: None,
                    dc_electric_potential_rating: None,
                    temperature_rating: None,
                    thickness: None,
                    rating: Some("Flamarrest®".to_string()),
                    color: Some(Color::White),
                },
            ],
            cores: {
                let mut cores = BTreeMap::new();
                cores.insert(
                    "black".to_string(),
                    CableCore::WireType("belden_638AFJ_18AWG_black_inner".to_string()),
                );
                cores.insert(
                    "red".to_string(),
                    CableCore::WireType("belden_638AFJ_18AWG_red_inner".to_string()),
                );
                cores
            },
            dimensions: Some(Dimension {
                height: Length {
                    original_unit: "mm".to_string(),
                    value: rational64::Length::new::<millimeter>(Rational64::new(427, 100)),
                },
                width: Length {
                    original_unit: "mm".to_string(),
                    value: rational64::Length::new::<millimeter>(Rational64::new(427, 100)),
                },
                depth: None,
                diameter: Some(Length {
                    original_unit: "mm".to_string(),
                    value: rational64::Length::new::<millimeter>(Rational64::new(427, 100)),
                }),
            }),
            contained_datafile_path: datafile_path.clone(),
        };

        let belden_638AFJ_rex_inner = CableType {
            cross_sect_area: CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(3927, 50)),
            },
            cross_section: CrossSection::Circular,
            cable_type_code: None,
            catalog: None,
            line_style: None,
            layers: vec![
                CableLayer {
                    layer_number: 1,
                    layer_type: LayerType::Shield,
                    material: Some("Bi-Laminate (Alum+Poly) Tape".to_string()),
                    ac_electric_potential_rating: None,
                    dc_electric_potential_rating: None,
                    temperature_rating: None,
                    thickness: None,
                    rating: None,
                    color: None,
                },
                CableLayer {
                    layer_number: 2,
                    layer_type: LayerType::Jacket,
                    material: Some("Polyvinyl Chloride".to_string()),
                    ac_electric_potential_rating: None,
                    dc_electric_potential_rating: None,
                    temperature_rating: None,
                    thickness: None,
                    rating: Some("Flamarrest®".to_string()),
                    color: Some(Color::Blue),
                },
            ],
            cores: {
                let mut cores = BTreeMap::new();
                cores.insert(
                    "black".to_string(),
                    CableCore::WireType("belden_638AFJ_18AWG_black_inner".to_string()),
                );
                cores.insert(
                    "red".to_string(),
                    CableCore::WireType("belden_638AFJ_18AWG_red_inner".to_string()),
                );
                cores.insert(
                    "white".to_string(),
                    CableCore::WireType("belden_638AFJ_18AWG_white_inner".to_string()),
                );
                cores.insert(
                    "green".to_string(),
                    CableCore::WireType("belden_638AFJ_18AWG_green_inner".to_string()),
                );
                cores
            },
            dimensions: Some(Dimension {
                height: Length {
                    original_unit: "mm".to_string(),
                    value: rational64::Length::new::<millimeter>(Rational64::new(5, 1)),
                },
                width: Length {
                    original_unit: "mm".to_string(),
                    value: rational64::Length::new::<millimeter>(Rational64::new(5, 1)),
                },
                depth: None,
                diameter: Some(Length {
                    original_unit: "mm".to_string(),
                    value: rational64::Length::new::<millimeter>(Rational64::new(5, 1)),
                }),
            }),
            contained_datafile_path: datafile_path.clone(),
        };

        let belden_638AFJ_lock_power_inner = CableType {
            cross_sect_area: CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(1017, 10)),
            },
            cross_section: CrossSection::Circular,
            cable_type_code: None,
            catalog: None,
            line_style: None,
            layers: vec![
                CableLayer {
                    layer_number: 1,
                    layer_type: LayerType::Shield,
                    material: Some("Bi-Laminate (Alum+Poly) Tape".to_string()),
                    ac_electric_potential_rating: None,
                    dc_electric_potential_rating: None,
                    temperature_rating: None,
                    thickness: None,
                    rating: None,
                    color: None,
                },
                CableLayer {
                    layer_number: 2,
                    layer_type: LayerType::Jacket,
                    material: Some("Polyvinyl Chloride".to_string()),
                    ac_electric_potential_rating: None,
                    dc_electric_potential_rating: None,
                    temperature_rating: None,
                    thickness: None,
                    rating: Some("Flamarrest®".to_string()),
                    color: Some(Color::Grey),
                },
            ],
            cores: {
                let mut cores = BTreeMap::new();
                cores.insert(
                    "black".to_string(),
                    CableCore::WireType("belden_638AFJ_16AWG_black_inner".to_string()),
                );
                cores.insert(
                    "red".to_string(),
                    CableCore::WireType("belden_638AFJ_16AWG_red_inner".to_string()),
                );
                cores.insert(
                    "white".to_string(),
                    CableCore::WireType("belden_638AFJ_16AWG_white_inner".to_string()),
                );
                cores.insert(
                    "green".to_string(),
                    CableCore::WireType("belden_638AFJ_16AWG_green_inner".to_string()),
                );
                cores
            },
            dimensions: Some(Dimension {
                height: Length {
                    original_unit: "mm".to_string(),
                    value: rational64::Length::new::<millimeter>(Rational64::new(569, 100)),
                },
                width: Length {
                    original_unit: "mm".to_string(),
                    value: rational64::Length::new::<millimeter>(Rational64::new(569, 100)),
                },
                depth: None,
                diameter: Some(Length {
                    original_unit: "mm".to_string(),
                    value: rational64::Length::new::<millimeter>(Rational64::new(569, 100)),
                }),
            }),
            contained_datafile_path: datafile_path.clone(),
        };

        let belden_638AFJ_18AWG_black_inner = WireType {
            material: "Copper".to_string(),
            insulated: true,
            insulation_material: Some("Polyvinyl Chloride".to_string()),
            nominal_cross_section: None,
            conductor_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(8229, 10000)),
            }),
            stranded: true,
            num_strands: 7,
            strand_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(1281, 10000)),
            }),
            insulation_rating: Some("Flamarrest®".to_string()),
            insulation_color: Some(Color::Black),
            catalog: None,
            wire_type_code: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_thickness: None,
            line_style: None,
            secondary_insulation_color: None,
            contained_datafile_path: datafile_path.clone(),
        };
        let belden_638AFJ_18AWG_red_inner = WireType {
            material: "Copper".to_string(),
            insulated: true,
            insulation_material: Some("Polyvinyl Chloride".to_string()),
            nominal_cross_section: None,
            conductor_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(8229, 10000)),
            }),
            stranded: true,
            num_strands: 7,
            strand_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(1281, 10000)),
            }),
            insulation_rating: Some("Flamarrest®".to_string()),
            insulation_color: Some(Color::Red),
            catalog: None,
            wire_type_code: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_thickness: None,
            line_style: None,
            secondary_insulation_color: None,
            contained_datafile_path: datafile_path.clone(),
        };
        let belden_638AFJ_18AWG_white_inner = WireType {
            material: "Copper".to_string(),
            insulated: true,
            insulation_material: Some("Polyvinyl Chloride".to_string()),
            nominal_cross_section: None,
            conductor_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(8229, 10000)),
            }),
            stranded: true,
            num_strands: 7,
            strand_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(1281, 10000)),
            }),
            insulation_rating: Some("Flamarrest®".to_string()),
            insulation_color: Some(Color::White),
            catalog: None,
            wire_type_code: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_thickness: None,
            line_style: None,
            secondary_insulation_color: None,
            contained_datafile_path: datafile_path.clone(),
        };
        let belden_638AFJ_18AWG_green_inner = WireType {
            material: "Copper".to_string(),
            insulated: true,
            insulation_material: Some("Polyvinyl Chloride".to_string()),
            nominal_cross_section: None,
            conductor_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(8229, 10000)),
            }),
            stranded: true,
            num_strands: 7,
            strand_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(1281, 10000)),
            }),
            insulation_rating: Some("Flamarrest®".to_string()),
            insulation_color: Some(Color::Green),
            catalog: None,
            wire_type_code: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_thickness: None,
            line_style: None,
            secondary_insulation_color: None,
            contained_datafile_path: datafile_path.clone(),
        };
        let belden_638AFJ_18AWG_brown_inner = WireType {
            material: "Copper".to_string(),
            insulated: true,
            insulation_material: Some("Polyvinyl Chloride".to_string()),
            nominal_cross_section: None,
            conductor_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(8229, 10000)),
            }),
            stranded: true,
            num_strands: 7,
            strand_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(1281, 10000)),
            }),
            insulation_rating: Some("Flamarrest®".to_string()),
            insulation_color: Some(Color::Brown),
            catalog: None,
            wire_type_code: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_thickness: None,
            line_style: None,
            secondary_insulation_color: None,
            contained_datafile_path: datafile_path.clone(),
        };
        let belden_638AFJ_18AWG_orange_inner = WireType {
            material: "Copper".to_string(),
            insulated: true,
            insulation_material: Some("Polyvinyl Chloride".to_string()),
            nominal_cross_section: None,
            conductor_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(8229, 10000)),
            }),
            stranded: true,
            num_strands: 7,
            strand_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(1281, 10000)),
            }),
            insulation_rating: Some("Flamarrest®".to_string()),
            insulation_color: Some(Color::Orange),
            catalog: None,
            wire_type_code: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_thickness: None,
            line_style: None,
            secondary_insulation_color: None,
            contained_datafile_path: datafile_path.clone(),
        };
        let belden_638AFJ_16AWG_black_inner = WireType {
            material: "Copper".to_string(),
            insulated: true,
            insulation_material: Some("Polyvinyl Chloride".to_string()),
            nominal_cross_section: None,
            conductor_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(327, 250)),
            }),
            stranded: true,
            num_strands: 19,
            strand_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(647, 10000)),
            }),
            insulation_rating: Some("Flamarrest®".to_string()),
            insulation_color: Some(Color::Black),
            catalog: None,
            wire_type_code: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_thickness: None,
            line_style: None,
            secondary_insulation_color: None,
            contained_datafile_path: datafile_path.clone(),
        };
        let belden_638AFJ_16AWG_red_inner = WireType {
            material: "Copper".to_string(),
            insulated: true,
            insulation_material: Some("Polyvinyl Chloride".to_string()),
            nominal_cross_section: None,
            conductor_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(327, 250)),
            }),
            stranded: true,
            num_strands: 19,
            strand_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(647, 10000)),
            }),
            insulation_rating: Some("Flamarrest®".to_string()),
            insulation_color: Some(Color::Red),
            catalog: None,
            wire_type_code: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_thickness: None,
            line_style: None,
            secondary_insulation_color: None,
            contained_datafile_path: datafile_path.clone(),
        };
        let belden_638AFJ_16AWG_white_inner = WireType {
            material: "Copper".to_string(),
            insulated: true,
            insulation_material: Some("Polyvinyl Chloride".to_string()),
            nominal_cross_section: None,
            conductor_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(327, 250)),
            }),
            stranded: true,
            num_strands: 19,
            strand_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(647, 10000)),
            }),
            insulation_rating: Some("Flamarrest®".to_string()),
            insulation_color: Some(Color::White),
            catalog: None,
            wire_type_code: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_thickness: None,
            line_style: None,
            secondary_insulation_color: None,
            contained_datafile_path: datafile_path.clone(),
        };
        let belden_638AFJ_16AWG_green_inner = WireType {
            material: "Copper".to_string(),
            insulated: true,
            insulation_material: Some("Polyvinyl Chloride".to_string()),
            nominal_cross_section: None,
            conductor_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(327, 250)),
            }),
            stranded: true,
            num_strands: 19,
            strand_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(647, 10000)),
            }),
            insulation_rating: Some("Flamarrest®".to_string()),
            insulation_color: Some(Color::Green),
            catalog: None,
            wire_type_code: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_thickness: None,
            line_style: None,
            secondary_insulation_color: None,
            contained_datafile_path: datafile_path.clone(),
        };

        let test_library = Library {
            cable_types: {
                let mut cable_types = BTreeMap::new();
                cable_types.insert("belden_638AFJ".to_string(), belden_638AFJ);
                cable_types.insert(
                    "belden_638AFJ_card_reader_3_pair_inner".to_string(),
                    belden_638AFJ_card_reader_3_pair_inner,
                );
                cable_types.insert(
                    "belden_638AFJ_door_contact_inner".to_string(),
                    belden_638AFJ_door_contact_inner,
                );
                cable_types.insert("belden_638AFJ_rex_inner".to_string(), belden_638AFJ_rex_inner);
                cable_types.insert("belden_638AFJ_lock_power_inner".to_string(), belden_638AFJ_lock_power_inner);
                cable_types
            },
            connector_types: BTreeMap::new(),
            enclosure_types: BTreeMap::new(),
            equipment_types: BTreeMap::new(),
            mounting_rail_types: BTreeMap::new(),
            pathway_types: BTreeMap::new(),
            schematic_symbol_types: BTreeMap::new(),
            term_cable_types: BTreeMap::new(),
            terminal_types: BTreeMap::new(),
            terminal_strip_jumper_types: BTreeMap::new(),
            terminal_accessory_types: BTreeMap::new(),
            terminal_strip_accessory_types: BTreeMap::new(),
            wire_types: {
                let mut wire_types = BTreeMap::new();
                wire_types.insert("belden_638AFJ_18AWG_black_inner".to_string(), belden_638AFJ_18AWG_black_inner);
                wire_types.insert("belden_638AFJ_18AWG_red_inner".to_string(), belden_638AFJ_18AWG_red_inner);
                wire_types.insert("belden_638AFJ_18AWG_white_inner".to_string(), belden_638AFJ_18AWG_white_inner);
                wire_types.insert("belden_638AFJ_18AWG_green_inner".to_string(), belden_638AFJ_18AWG_green_inner);
                wire_types.insert("belden_638AFJ_18AWG_brown_inner".to_string(), belden_638AFJ_18AWG_brown_inner);
                wire_types.insert(
                    "belden_638AFJ_18AWG_orange_inner".to_string(),
                    belden_638AFJ_18AWG_orange_inner,
                );

                wire_types.insert("belden_638AFJ_16AWG_black_inner".to_string(), belden_638AFJ_16AWG_black_inner);
                wire_types.insert("belden_638AFJ_16AWG_red_inner".to_string(), belden_638AFJ_16AWG_red_inner);
                wire_types.insert("belden_638AFJ_16AWG_white_inner".to_string(), belden_638AFJ_16AWG_white_inner);
                wire_types.insert("belden_638AFJ_16AWG_green_inner".to_string(), belden_638AFJ_16AWG_green_inner);

                wire_types
            },
        };
        let library_filepath = PathBuf::from("../../resources/test/library_tests/cable_type_test_multicore_realistic.toml")
            .canonicalize()
            .unwrap();
        let library_file_contents = fs::read_to_string(&library_filepath).unwrap();
        let mut library_file: Library = toml::from_str(&library_file_contents).unwrap();
        library_file.add_datafile_paths(&library_filepath);
        assert_eq!(test_library, library_file);
    }

    #[test]
    /// This tests the following:
    /// - different cross sections
    /// - multiple cable definitions in one file, with one cable referencing cores of the other
    /// - wires and cables together in the same file
    ///
    /// this is a relatively realistic test, with a mix of filled in values, multiple items in
    /// btreemaps, and validation of btreemap keys
    fn read_datafile_library_cable_alt_cross_section() {
        let datafile_path = PathBuf::from("../../resources/test/library_tests/cable_type_test_figure8_aerial_fiber.toml")
            .canonicalize()
            .unwrap();
        let fig8_fiber = CableType {
            cross_sect_area: CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(1918, 5)),
            },
            cross_section: CrossSection::Figure8,
            layers: vec![CableLayer {
                layer_number: 1,
                layer_type: LayerType::Jacket,
                material: Some("Polyethylene".to_string()),
                ac_electric_potential_rating: None,
                dc_electric_potential_rating: None,
                temperature_rating: None,
                thickness: Some(Length {
                    original_unit: "mm".to_string(),
                    value: rational64::Length::new::<millimeter>(Rational64::new(3, 2)),
                }),
                rating: None,
                color: Some(Color::Black),
            }],
            cores: {
                let mut cores = BTreeMap::new();
                cores.insert(
                    "messenger_wire".to_string(),
                    CableCore::WireType("corning_steel_messenger_025".to_string()),
                );
                cores.insert(
                    "fiber_cable".to_string(),
                    CableCore::CableType("corning_altos_os2_12_strand_fiber".to_string()),
                );
                cores
            },
            cable_type_code: None,
            line_style: None,
            dimensions: Some(Dimension {
                height: Length {
                    original_unit: "mm".to_string(),
                    value: rational64::Length::new::<millimeter>(Rational64::new(221, 10)),
                },
                width: Length {
                    original_unit: "mm".to_string(),
                    value: rational64::Length::new::<millimeter>(Rational64::new(21, 2)),
                },
                depth: None,
                diameter: Some(Length {
                    original_unit: "mm".to_string(),
                    value: rational64::Length::new::<millimeter>(Rational64::new(21, 2)),
                }),
            }),
            catalog: Some(Catalog {
                manufacturer: Some("Corning".to_string()),
                model: Some("ALTOS® Figure-8 Loose Tube, Gel-Free Cable  12 F, Single-mode (OS2)".to_string()),
                description: Some("12 strand single mode OS2 fiber cable with messenger wire. OSP rated.".to_string()),
                //totally made up
                part_number: Some("00-1500-12".to_string()),
                manufacturer_part_number: Some("012EUA-T4101D20".to_string()),
                supplier: Some("LAN Shack".to_string()),
                supplier_part_number: Some("FBTF-CA-OSP-AWM-SM-12".to_string()),
            }),
            contained_datafile_path: datafile_path.clone(),
        };

        let corning_altos_os2_12_strand_fiber = CableType {
            cross_sect_area: CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(803, 5)),
            },
            cross_section: CrossSection::Circular,
            cable_type_code: None,
            catalog: None,
            line_style: None,
            layers: vec![CableLayer {
                layer_number: 1,
                layer_type: LayerType::WaterBlocking,
                material: Some("Water blocking tape".to_string()),
                ac_electric_potential_rating: None,
                dc_electric_potential_rating: None,
                temperature_rating: None,
                thickness: None,
                rating: None,
                color: None,
            }],
            cores: {
                let mut cores = BTreeMap::new();
                cores.insert(
                    "1oclock".to_string(),
                    CableCore::WireType("corning_filling_element".to_string()),
                );
                cores.insert(
                    "3oclock".to_string(),
                    CableCore::CableType("corning_12_strand_buffer_tube".to_string()),
                );
                cores.insert(
                    "5oclock".to_string(),
                    CableCore::WireType("corning_filling_element".to_string()),
                );
                cores.insert(
                    "7oclock".to_string(),
                    CableCore::WireType("corning_filling_element".to_string()),
                );
                cores.insert(
                    "9oclock".to_string(),
                    CableCore::WireType("corning_filling_element".to_string()),
                );
                cores.insert(
                    "11oclock".to_string(),
                    CableCore::WireType("corning_filling_element".to_string()),
                );
                cores.insert(
                    "central_element".to_string(),
                    CableCore::WireType("corning_dialectric_central_member".to_string()),
                );
                cores
            },
            dimensions: None,
            contained_datafile_path: datafile_path.clone(),
        };
        let corning_12_strand_buffer_tube = CableType {
            cross_sect_area: CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(491, 100)),
            },
            cross_section: CrossSection::Circular,
            cable_type_code: None,
            catalog: None,
            line_style: None,
            layers: vec![CableLayer {
                layer_number: 1,
                layer_type: LayerType::Jacket,
                material: Some("Buffer Tube".to_string()),
                ac_electric_potential_rating: None,
                dc_electric_potential_rating: None,
                temperature_rating: None,
                thickness: None,
                rating: None,
                color: Some(Color::Blue),
            }],
            cores: {
                let mut cores = BTreeMap::new();
                cores.insert(
                    "blue".to_string(),
                    CableCore::WireType("corning_os2_fiber_strand_blue".to_string()),
                );
                cores.insert(
                    "orange".to_string(),
                    CableCore::WireType("corning_os2_fiber_strand_orange".to_string()),
                );
                cores.insert(
                    "green".to_string(),
                    CableCore::WireType("corning_os2_fiber_strand_green".to_string()),
                );
                cores.insert(
                    "brown".to_string(),
                    CableCore::WireType("corning_os2_fiber_strand_brown".to_string()),
                );
                cores.insert(
                    "slate".to_string(),
                    CableCore::WireType("corning_os2_fiber_strand_slate".to_string()),
                );
                cores.insert(
                    "white".to_string(),
                    CableCore::WireType("corning_os2_fiber_strand_white".to_string()),
                );
                cores.insert(
                    "red".to_string(),
                    CableCore::WireType("corning_os2_fiber_strand_red".to_string()),
                );
                cores.insert(
                    "black".to_string(),
                    CableCore::WireType("corning_os2_fiber_strand_black".to_string()),
                );
                cores.insert(
                    "yellow".to_string(),
                    CableCore::WireType("corning_os2_fiber_strand_yellow".to_string()),
                );
                cores.insert(
                    "violet".to_string(),
                    CableCore::WireType("corning_os2_fiber_strand_violet".to_string()),
                );
                cores.insert(
                    "rose".to_string(),
                    CableCore::WireType("corning_os2_fiber_strand_rose".to_string()),
                );
                cores.insert(
                    "aqua".to_string(),
                    CableCore::WireType("corning_os2_fiber_strand_aqua".to_string()),
                );
                cores
            },
            dimensions: None,
            contained_datafile_path: datafile_path.clone(),
        };

        let corning_steel_messenger = WireType {
            material: "Steel".to_string(),
            insulated: false,
            insulation_material: None,
            nominal_cross_section: None,
            conductor_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(317, 10)),
            }),
            stranded: true,
            num_strands: 7,
            strand_cross_sect_area: None,
            insulation_rating: None,
            insulation_color: None,
            catalog: None,
            wire_type_code: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_thickness: None,
            line_style: None,
            secondary_insulation_color: None,
            contained_datafile_path: datafile_path.clone(),
        };

        let corning_filling_element = WireType {
            material: "Water blocking yarn".to_string(),
            insulated: false,
            insulation_material: None,
            nominal_cross_section: None,
            conductor_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(491, 100)),
            }),
            stranded: false,
            num_strands: 1,
            strand_cross_sect_area: None,
            insulation_rating: None,
            insulation_color: None,
            catalog: None,
            wire_type_code: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_thickness: None,
            line_style: None,
            secondary_insulation_color: None,
            contained_datafile_path: datafile_path.clone(),
        };

        let corning_dialectric_central_member = WireType {
            material: "Glass Reinforced Plastic Dialectric".to_string(),
            insulated: false,
            insulation_material: None,
            nominal_cross_section: None,
            conductor_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(491, 100)),
            }),
            stranded: false,
            num_strands: 1,
            strand_cross_sect_area: None,
            insulation_rating: None,
            insulation_color: None,
            catalog: None,
            wire_type_code: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_thickness: None,
            line_style: None,
            secondary_insulation_color: None,
            contained_datafile_path: datafile_path.clone(),
        };

        let corning_os2_fiber_strand_blue = WireType {
            material: "Glass".to_string(),
            insulated: true,
            insulation_material: Some("Acrylate".to_string()),
            nominal_cross_section: None,
            conductor_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square micrometer".to_string(),
                value: rational64::Area::new::<square_micrometer>(Rational64::new(12272, 1)),
            }),
            stranded: false,
            num_strands: 1,
            strand_cross_sect_area: None,
            insulation_rating: None,
            insulation_color: Some(Color::Blue),
            catalog: None,
            wire_type_code: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_thickness: None,
            line_style: None,
            secondary_insulation_color: None,
            contained_datafile_path: datafile_path.clone(),
        };

        let corning_os2_fiber_strand_orange = WireType {
            material: "Glass".to_string(),
            insulated: true,
            insulation_material: Some("Acrylate".to_string()),
            nominal_cross_section: None,
            conductor_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square micrometer".to_string(),
                value: rational64::Area::new::<square_micrometer>(Rational64::new(12272, 1)),
            }),
            stranded: false,
            num_strands: 1,
            strand_cross_sect_area: None,
            insulation_rating: None,
            insulation_color: Some(Color::Orange),
            catalog: None,
            wire_type_code: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_thickness: None,
            line_style: None,
            secondary_insulation_color: None,
            contained_datafile_path: datafile_path.clone(),
        };

        let corning_os2_fiber_strand_green = WireType {
            material: "Glass".to_string(),
            insulated: true,
            insulation_material: Some("Acrylate".to_string()),
            nominal_cross_section: None,
            conductor_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square micrometer".to_string(),
                value: rational64::Area::new::<square_micrometer>(Rational64::new(12272, 1)),
            }),
            stranded: false,
            num_strands: 1,
            strand_cross_sect_area: None,
            insulation_rating: None,
            insulation_color: Some(Color::Green),
            catalog: None,
            wire_type_code: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_thickness: None,
            line_style: None,
            secondary_insulation_color: None,
            contained_datafile_path: datafile_path.clone(),
        };

        let corning_os2_fiber_strand_brown = WireType {
            material: "Glass".to_string(),
            insulated: true,
            insulation_material: Some("Acrylate".to_string()),
            nominal_cross_section: None,
            conductor_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square micrometer".to_string(),
                value: rational64::Area::new::<square_micrometer>(Rational64::new(12272, 1)),
            }),
            stranded: false,
            num_strands: 1,
            strand_cross_sect_area: None,
            insulation_rating: None,
            insulation_color: Some(Color::Brown),
            catalog: None,
            wire_type_code: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_thickness: None,
            line_style: None,
            secondary_insulation_color: None,
            contained_datafile_path: datafile_path.clone(),
        };

        let corning_os2_fiber_strand_slate = WireType {
            material: "Glass".to_string(),
            insulated: true,
            insulation_material: Some("Acrylate".to_string()),
            nominal_cross_section: None,
            conductor_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square micrometer".to_string(),
                value: rational64::Area::new::<square_micrometer>(Rational64::new(12272, 1)),
            }),
            stranded: false,
            num_strands: 1,
            strand_cross_sect_area: None,
            insulation_rating: None,
            insulation_color: Some(Color::Slate),
            catalog: None,
            wire_type_code: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_thickness: None,
            line_style: None,
            secondary_insulation_color: None,
            contained_datafile_path: datafile_path.clone(),
        };

        let corning_os2_fiber_strand_white = WireType {
            material: "Glass".to_string(),
            insulated: true,
            insulation_material: Some("Acrylate".to_string()),
            nominal_cross_section: None,
            conductor_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square micrometer".to_string(),
                value: rational64::Area::new::<square_micrometer>(Rational64::new(12272, 1)),
            }),
            stranded: false,
            num_strands: 1,
            strand_cross_sect_area: None,
            insulation_rating: None,
            insulation_color: Some(Color::White),
            catalog: None,
            wire_type_code: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_thickness: None,
            line_style: None,
            secondary_insulation_color: None,
            contained_datafile_path: datafile_path.clone(),
        };

        let corning_os2_fiber_strand_red = WireType {
            material: "Glass".to_string(),
            insulated: true,
            insulation_material: Some("Acrylate".to_string()),
            nominal_cross_section: None,
            conductor_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square micrometer".to_string(),
                value: rational64::Area::new::<square_micrometer>(Rational64::new(12272, 1)),
            }),
            stranded: false,
            num_strands: 1,
            strand_cross_sect_area: None,
            insulation_rating: None,
            insulation_color: Some(Color::Red),
            catalog: None,
            wire_type_code: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_thickness: None,
            line_style: None,
            secondary_insulation_color: None,
            contained_datafile_path: datafile_path.clone(),
        };

        let corning_os2_fiber_strand_black = WireType {
            material: "Glass".to_string(),
            insulated: true,
            insulation_material: Some("Acrylate".to_string()),
            nominal_cross_section: None,
            conductor_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square micrometer".to_string(),
                value: rational64::Area::new::<square_micrometer>(Rational64::new(12272, 1)),
            }),
            stranded: false,
            num_strands: 1,
            strand_cross_sect_area: None,
            insulation_rating: None,
            insulation_color: Some(Color::Black),
            catalog: None,
            wire_type_code: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_thickness: None,
            line_style: None,
            secondary_insulation_color: None,
            contained_datafile_path: datafile_path.clone(),
        };

        let corning_os2_fiber_strand_yellow = WireType {
            material: "Glass".to_string(),
            insulated: true,
            insulation_material: Some("Acrylate".to_string()),
            nominal_cross_section: None,
            conductor_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square micrometer".to_string(),
                value: rational64::Area::new::<square_micrometer>(Rational64::new(12272, 1)),
            }),
            stranded: false,
            num_strands: 1,
            strand_cross_sect_area: None,
            insulation_rating: None,
            insulation_color: Some(Color::Yellow),
            catalog: None,
            wire_type_code: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_thickness: None,
            line_style: None,
            secondary_insulation_color: None,
            contained_datafile_path: datafile_path.clone(),
        };

        let corning_os2_fiber_strand_violet = WireType {
            material: "Glass".to_string(),
            insulated: true,
            insulation_material: Some("Acrylate".to_string()),
            nominal_cross_section: None,
            conductor_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square micrometer".to_string(),
                value: rational64::Area::new::<square_micrometer>(Rational64::new(12272, 1)),
            }),
            stranded: false,
            num_strands: 1,
            strand_cross_sect_area: None,
            insulation_rating: None,
            insulation_color: Some(Color::Violet),
            catalog: None,
            wire_type_code: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_thickness: None,
            line_style: None,
            secondary_insulation_color: None,
            contained_datafile_path: datafile_path.clone(),
        };

        let corning_os2_fiber_strand_rose = WireType {
            material: "Glass".to_string(),
            insulated: true,
            insulation_material: Some("Acrylate".to_string()),
            nominal_cross_section: None,
            conductor_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square micrometer".to_string(),
                value: rational64::Area::new::<square_micrometer>(Rational64::new(12272, 1)),
            }),
            stranded: false,
            num_strands: 1,
            strand_cross_sect_area: None,
            insulation_rating: None,
            insulation_color: Some(Color::Rose),
            catalog: None,
            wire_type_code: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_thickness: None,
            line_style: None,
            secondary_insulation_color: None,
            contained_datafile_path: datafile_path.clone(),
        };

        let corning_os2_fiber_strand_aqua = WireType {
            material: "Glass".to_string(),
            insulated: true,
            insulation_material: Some("Acrylate".to_string()),
            nominal_cross_section: None,
            conductor_cross_sect_area: Some(CrossSectionalArea {
                original_unit: "square micrometer".to_string(),
                value: rational64::Area::new::<square_micrometer>(Rational64::new(12272, 1)),
            }),
            stranded: false,
            num_strands: 1,
            strand_cross_sect_area: None,
            insulation_rating: None,
            insulation_color: Some(Color::Aqua),
            catalog: None,
            wire_type_code: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_thickness: None,
            line_style: None,
            secondary_insulation_color: None,
            contained_datafile_path: datafile_path.clone(),
        };

        let test_library = Library {
            cable_types: {
                let mut cable_types = BTreeMap::new();
                cable_types.insert("fig8_fiber".to_string(), fig8_fiber);
                cable_types.insert(
                    "corning_altos_os2_12_strand_fiber".to_string(),
                    corning_altos_os2_12_strand_fiber,
                );
                cable_types.insert("corning_12_strand_buffer_tube".to_string(), corning_12_strand_buffer_tube);
                cable_types
            },
            connector_types: BTreeMap::new(),
            enclosure_types: BTreeMap::new(),
            equipment_types: BTreeMap::new(),
            mounting_rail_types: BTreeMap::new(),
            pathway_types: BTreeMap::new(),
            schematic_symbol_types: BTreeMap::new(),
            term_cable_types: BTreeMap::new(),
            terminal_types: BTreeMap::new(),
            terminal_strip_jumper_types: BTreeMap::new(),
            terminal_accessory_types: BTreeMap::new(),
            terminal_strip_accessory_types: BTreeMap::new(),
            wire_types: {
                let mut wire_types = BTreeMap::new();
                wire_types.insert("corning_os2_fiber_strand_blue".to_string(), corning_os2_fiber_strand_blue);
                wire_types.insert("corning_os2_fiber_strand_orange".to_string(), corning_os2_fiber_strand_orange);
                wire_types.insert("corning_os2_fiber_strand_green".to_string(), corning_os2_fiber_strand_green);
                wire_types.insert("corning_os2_fiber_strand_brown".to_string(), corning_os2_fiber_strand_brown);
                wire_types.insert("corning_os2_fiber_strand_slate".to_string(), corning_os2_fiber_strand_slate);
                wire_types.insert("corning_os2_fiber_strand_white".to_string(), corning_os2_fiber_strand_white);
                wire_types.insert("corning_os2_fiber_strand_red".to_string(), corning_os2_fiber_strand_red);
                wire_types.insert("corning_os2_fiber_strand_black".to_string(), corning_os2_fiber_strand_black);
                wire_types.insert("corning_os2_fiber_strand_yellow".to_string(), corning_os2_fiber_strand_yellow);
                wire_types.insert("corning_os2_fiber_strand_violet".to_string(), corning_os2_fiber_strand_violet);
                wire_types.insert("corning_os2_fiber_strand_rose".to_string(), corning_os2_fiber_strand_rose);
                wire_types.insert("corning_os2_fiber_strand_aqua".to_string(), corning_os2_fiber_strand_aqua);
                wire_types.insert("corning_steel_messenger_025".to_string(), corning_steel_messenger);
                wire_types.insert("corning_filling_element".to_string(), corning_filling_element);
                wire_types.insert(
                    "corning_dialectric_central_member".to_string(),
                    corning_dialectric_central_member,
                );

                wire_types
            },
        };
        let library_filepath = PathBuf::from("../../resources/test/library_tests/cable_type_test_figure8_aerial_fiber.toml")
            .canonicalize()
            .unwrap();
        let library_file_contents = fs::read_to_string(&library_filepath).unwrap();
        let mut library_file: Library = toml::from_str(&library_file_contents).unwrap();
        library_file.add_datafile_paths(&library_filepath);
        assert_eq!(test_library, library_file);
    }
    #[test]
    /// Test importing a realistic minimal example file
    ///
    /// No validation of string keys within library
    fn read_datafile_library_cable_library_merging() {
        let soow14_3 = CableType {
            cross_sect_area: CrossSectionalArea {
                original_unit: "square inch".to_string(),
                value: rational64::Area::new::<square_inch>(Rational64::new(14_389_229, 64_008_858)),
            },
            cross_section: CrossSection::Circular,
            layers: vec![CableLayer {
                layer_number: 1,
                layer_type: LayerType::Jacket,
                material: None,
                ac_electric_potential_rating: None,
                dc_electric_potential_rating: None,
                temperature_rating: None,
                rating: None,
                thickness: None,
                color: None,
            }],
            cores: {
                let mut cores = BTreeMap::new();
                cores.insert("green".to_string(), CableCore::WireType("soow14_green_inner".to_string()));
                cores.insert("white".to_string(), CableCore::WireType("soow14_white_inner".to_string()));
                cores.insert("black".to_string(), CableCore::WireType("soow14_black_inner".to_string()));
                cores
            },
            cable_type_code: None,
            catalog: None,
            dimensions: None,
            line_style: None,
            contained_datafile_path: PathBuf::from("../../resources/test/library_tests/cable_type_test_minimal_realistic.toml")
                .canonicalize()
                .unwrap(),
        };
        let triax_rg11 = CableType {
            cross_sect_area: CrossSectionalArea {
                original_unit: "square millimeter".to_string(),
                value: rational64::Area::new::<square_millimeter>(Rational64::new(137, 1)),
            },
            cross_section: CrossSection::Circular,
            layers: vec![
                CableLayer {
                    layer_number: 1,
                    layer_type: LayerType::Insulation,
                    material: Some("High Density Polyethylene Foam".to_string()),
                    ac_electric_potential_rating: None,
                    dc_electric_potential_rating: None,
                    temperature_rating: None,
                    rating: None,
                    thickness: Some(Length {
                        original_unit: "inch".to_string(),
                        value: rational64::Length::new::<inch>(Rational64::new(31, 250)),
                    }),
                    color: Some(Color::White),
                },
                CableLayer {
                    layer_number: 2,
                    layer_type: LayerType::Shield,
                    material: Some("Bare Copper".to_string()),
                    ac_electric_potential_rating: None,
                    dc_electric_potential_rating: None,
                    temperature_rating: None,
                    rating: None,
                    thickness: None,
                    color: None,
                },
                CableLayer {
                    layer_number: 3,
                    layer_type: LayerType::Jacket,
                    material: Some("Polyethylene".to_string()),
                    ac_electric_potential_rating: None,
                    dc_electric_potential_rating: None,
                    temperature_rating: None,
                    rating: None,
                    thickness: Some(Length {
                        original_unit: "inch".to_string(),
                        value: rational64::Length::new::<inch>(Rational64::new(79, 1000)),
                    }),
                    color: Some(Color::Black),
                },
                CableLayer {
                    layer_number: 4,
                    layer_type: LayerType::Shield,
                    material: Some("Bare Copper".to_string()),
                    ac_electric_potential_rating: None,
                    dc_electric_potential_rating: None,
                    temperature_rating: None,
                    rating: None,
                    thickness: None,
                    color: None,
                },
                CableLayer {
                    layer_number: 5,
                    layer_type: LayerType::Jacket,
                    material: Some("Polyvinyl Chloride".to_string()),
                    ac_electric_potential_rating: None,
                    dc_electric_potential_rating: None,
                    temperature_rating: Some(TemperatureInterval {
                        original_unit: "degree Celsius".to_string(),
                        value: rational64::TemperatureInterval::new::<degree_celsius>(Rational64::new(80, 1)),
                    }),
                    rating: None,
                    thickness: Some(Length {
                        original_unit: "inch".to_string(),
                        value: rational64::Length::new::<inch>(Rational64::new(13, 25)),
                    }),
                    color: Some(Color::Yellow),
                },
            ],
            cores: {
                let mut cores = BTreeMap::new();
                cores.insert(
                    "inner".to_string(),
                    CableCore::WireType("triax_15awg_copper_inner".to_string()),
                );
                cores
            },
            cable_type_code: Some("RG11".to_string()),
            catalog: None,
            dimensions: None,
            line_style: None,
            contained_datafile_path: PathBuf::from("../../resources/test/library_tests/cable_type_test_multi_layer.toml")
                .canonicalize()
                .unwrap(),
        };
        let test_library = Library {
            cable_types: {
                let mut cable_types = BTreeMap::new();
                cable_types.insert("soow14_3".to_string(), soow14_3);
                cable_types.insert("triax_rg11".to_string(), triax_rg11);
                cable_types
            },
            connector_types: BTreeMap::new(),
            enclosure_types: BTreeMap::new(),
            equipment_types: BTreeMap::new(),
            mounting_rail_types: BTreeMap::new(),
            pathway_types: BTreeMap::new(),
            schematic_symbol_types: BTreeMap::new(),
            term_cable_types: BTreeMap::new(),
            terminal_types: BTreeMap::new(),
            terminal_strip_jumper_types: BTreeMap::new(),
            terminal_accessory_types: BTreeMap::new(),
            terminal_strip_accessory_types: BTreeMap::new(),
            wire_types: BTreeMap::new(),
        };
        let library_filepath_1 = PathBuf::from("../../resources/test/library_tests/cable_type_test_minimal_realistic.toml")
            .canonicalize()
            .unwrap();
        let library_file_contents_1 = fs::read_to_string(&library_filepath_1).unwrap();
        let library_filepath_2 = PathBuf::from("../../resources/test/library_tests/cable_type_test_multi_layer.toml")
            .canonicalize()
            .unwrap();
        let library_file_contents_2 = fs::read_to_string(&library_filepath_2).unwrap();
        let mut library_file2: Library = toml::from_str(&library_file_contents_2).unwrap();
        library_file2.add_datafile_paths(&library_filepath_2);
        let mut library_file: Library = toml::from_str(&library_file_contents_1).unwrap();
        library_file.add_datafile_paths(&library_filepath_1);
        library_file.merge(library_file2, &library_filepath_2).unwrap();
        assert_eq!(test_library, library_file);
    }
}
