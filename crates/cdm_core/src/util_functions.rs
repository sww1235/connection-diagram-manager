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
pub fn merge_btreemaps<V>(
    origin_map: &mut BTreeMap<String, V>,
    test_map: BTreeMap<String, V>,
    test_file: &Path,
) -> Result<(), Error>
where
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
                key: key.clone(),
                origin_file: value.datafile().clone(),
                test_file: test_file.to_path_buf(),
            });
        }
    }
    Ok(())
}
