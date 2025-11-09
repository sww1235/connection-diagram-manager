use std::collections::{HashMap, hash_map::Entry};

use crate::{error::Error, traits::FromFile};

/// merge hashmaps merges two hashmaps while checking uniqueness of keys.
///
/// Specialized function for merging hashmaps with data read in from datafiles
///
/// # Errors
///
/// Will error if there are duplicate keys found in `other` map
pub fn merge_hashmaps<V>(map: &mut HashMap<String, V>, other: HashMap<String, V>, other_file: &str) -> Result<(), Error>
where V: FromFile {
    for (key, value) in other {
        if let Entry::Vacant(e) = map.entry(key.clone()) {
            e.insert(value);
        } else {
            return Err(Error::DuplicateKey {
                key: key.clone(),
                //TODO: name these better
                file1: value.datafile().display().to_string(),
                other_file: other_file.to_string(),
            });
        }
    }
    Ok(())
}
