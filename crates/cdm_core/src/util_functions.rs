use std::collections::{HashMap, hash_map::Entry};

use crate::{error::Error, traits::FromFile};

/// merge hashmaps merges two hashmaps while checking uniqueness of keys.
///
/// Specialized function for merging hashmaps with data read in from datafiles
///
/// # Errors
///
/// Will error if there are duplicate keys found in `test_map`
pub fn merge_hashmaps<V>(
    origin_map: &mut HashMap<String, V>,
    test_map: HashMap<String, V>,
    test_file: &str,
) -> Result<(), Error>
where
    V: FromFile,
{
    for (key, value) in test_map {
        if let Entry::Vacant(e) = origin_map.entry(key.clone()) {
            e.insert(value);
        } else {
            return Err(Error::DuplicateKey {
                key: key.clone(),
                origin_file: value.datafile().display().to_string(),
                test_file: test_file.to_string(),
            });
        }
    }
    Ok(())
}
