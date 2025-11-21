use std::collections::{BTreeMap, btree_map::Entry};
use std::path::Path;

use crate::{error::Error, traits::FromFile};

/// merge btreemaps merges two btreemaps while checking uniqueness of keys.
///
/// Specialized function for merging btreemaps with data read in from datafiles
///
/// # Errors
///
/// Will error if there are duplicate keys found in `test_map`
pub fn merge_btreemaps<U,V>(
    origin_map: &mut BTreeMap<U, V>,
    test_map: BTreeMap<U, V>,
    test_file: &Path,
) -> Result<(), Error>
where
    U: Ord + Clone + ToString,
    V: FromFile,
{
    // don't need to do any work if test map is empty
    if test_map.is_empty() {
        return Ok(());
    }
    for (key, value) in test_map {
        if let Entry::Vacant(e) = origin_map.entry(key.clone()) {
            e.insert(value);
        } else {
            return Err(Error::DuplicateKey {
                key: key.clone().to_string(),
                origin_file: value.datafile().clone(),
                test_file: test_file.to_path_buf(),
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
use std::collections::{BTreeMap};
use std::path::Path;

use num_rational::Rational64;

use uom::si::{rational64, area::square_millimeter};

use crate::datatypes::library_types::Library;
use crate::datatypes::library_types::wire_type::WireType;
use crate::datatypes::unit_helper::CrossSectionalArea;


    #[test]
    fn no_duplicates() {
        let test_path = Path::new("testpath.toml");
        let wiretype1 = WireType {
            catalog: None,
            wire_type_code: None,
            material: "Copper".to_string(),
            insulated: false,
            insulation_material: None,
            insulation_thickness: None,
            conductor_cross_sect_area: CrossSectionalArea {original_unit: "square_millimeter".to_string(), value: rational64::Area::new::<square_millimeter>(Rational64::new(4,1))},
            overall_cross_sect_area: None,
            stranded: true,
            num_strands: 4,
            strand_cross_sect_area: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_rating: None,
            insulation_color: None,
            secondary_insulation_color: None,
            line_style: None,
            contained_datafile_path: test_path.to_path_buf(),
        };
        let wiretype2 = WireType {
            catalog: None,
            wire_type_code: None,
            material: "Copper".to_string(),
            insulated: false,
            insulation_material: None,
            insulation_thickness: None,
            conductor_cross_sect_area: CrossSectionalArea {original_unit: "square_millimeter".to_string(), value: rational64::Area::new::<square_millimeter>(Rational64::new(4,1))},
            overall_cross_sect_area: None,
            stranded: true,
            num_strands: 7,
            strand_cross_sect_area: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_rating: None,
            insulation_color: None,
            secondary_insulation_color: None,
            line_style: None,
            contained_datafile_path: test_path.to_path_buf(),
        };
        let wiretype3 = WireType {
            catalog: None,
            wire_type_code: None,
            material: "Copper".to_string(),
            insulated: false,
            insulation_material: None,
            insulation_thickness: None,
            conductor_cross_sect_area: CrossSectionalArea {original_unit: "square_millimeter".to_string(), value: rational64::Area::new::<square_millimeter>(Rational64::new(4,1))},
            overall_cross_sect_area: None,
            stranded: true,
            num_strands: 2,
            strand_cross_sect_area: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_rating: None,
            insulation_color: None,
            secondary_insulation_color: None,
            line_style: None,
            contained_datafile_path: test_path.to_path_buf(),
        };
        let mut map1: BTreeMap<usize, WireType> = BTreeMap::new();
        map1.insert(1, wiretype1.clone());
        map1.insert(2, wiretype2.clone());
        map1.insert(3, wiretype3.clone());

        let mut map2: BTreeMap<usize, WireType> = BTreeMap::new();
        map2.insert(4, wiretype1.clone());
        map2.insert(5, wiretype2.clone());
        map2.insert(6, wiretype3.clone());

        let mut map3: BTreeMap<usize, WireType> = BTreeMap::new();
        map3.insert(1, wiretype1.clone());
        map3.insert(2, wiretype2.clone());
        map3.insert(3, wiretype3.clone());
        map3.insert(4, wiretype1.clone());
        map3.insert(5, wiretype2.clone());
        map3.insert(6, wiretype3.clone());

        super::merge_btreemaps(&mut map1, map2, test_path).unwrap();
        assert_eq!(map1, map3);
    }
    #[test]
    #[should_panic(expected = "DuplicateKey")]
    fn duplicates() {
        let test_path = Path::new("testpath.toml");
        let wiretype1 = WireType {
            catalog: None,
            wire_type_code: None,
            material: "Copper".to_string(),
            insulated: false,
            insulation_material: None,
            insulation_thickness: None,
            conductor_cross_sect_area: CrossSectionalArea {original_unit: "square_millimeter".to_string(), value: rational64::Area::new::<square_millimeter>(Rational64::new(4,1))},
            overall_cross_sect_area: None,
            stranded: true,
            num_strands: 4,
            strand_cross_sect_area: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_rating: None,
            insulation_color: None,
            secondary_insulation_color: None,
            line_style: None,
            contained_datafile_path: test_path.to_path_buf(),
        };
        let wiretype2 = WireType {
            catalog: None,
            wire_type_code: None,
            material: "Copper".to_string(),
            insulated: false,
            insulation_material: None,
            insulation_thickness: None,
            conductor_cross_sect_area: CrossSectionalArea {original_unit: "square_millimeter".to_string(), value: rational64::Area::new::<square_millimeter>(Rational64::new(4,1))},
            overall_cross_sect_area: None,
            stranded: true,
            num_strands: 7,
            strand_cross_sect_area: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_rating: None,
            insulation_color: None,
            secondary_insulation_color: None,
            line_style: None,
            contained_datafile_path: test_path.to_path_buf(),
        };
        let wiretype3 = WireType {
            catalog: None,
            wire_type_code: None,
            material: "Copper".to_string(),
            insulated: false,
            insulation_material: None,
            insulation_thickness: None,
            conductor_cross_sect_area: CrossSectionalArea {original_unit: "square_millimeter".to_string(), value: rational64::Area::new::<square_millimeter>(Rational64::new(4,1))},
            overall_cross_sect_area: None,
            stranded: true,
            num_strands: 2,
            strand_cross_sect_area: None,
            ac_insulation_potential_rating: None,
            dc_insulation_potential_rating: None,
            insulation_temperature_rating: None,
            insulation_rating: None,
            insulation_color: None,
            secondary_insulation_color: None,
            line_style: None,
            contained_datafile_path: test_path.to_path_buf(),
        };
        let mut map1: BTreeMap<usize, WireType> = BTreeMap::new();
        map1.insert(1, wiretype1.clone());
        map1.insert(2, wiretype2.clone());
        map1.insert(3, wiretype3.clone());

        let mut map2: BTreeMap<usize, WireType> = BTreeMap::new();
        map2.insert(1, wiretype1.clone());
        map2.insert(2, wiretype2.clone());
        map2.insert(3, wiretype3.clone());

        super::merge_btreemaps(&mut map1, map2, test_path).unwrap();
    }
}
