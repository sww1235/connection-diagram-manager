use super::compare;
use std::collections::HashMap;
/// `Merge` indicates that an object has the necessary utilities to merge itself with another
/// instance of the same object type.
pub trait Merge {
    /// `merge_prompt` assists the user in merging 2 object instances by prompting the user with
    /// the difference between the object, field by field, and providing sensible defaults.
    fn merge_prompt(
        &mut self,
        other: &Self,
        prompt_fn: fn(compare::CompareResult) -> compare::CompareResult,
    );
    // pass a hashmap of string arrays, return a hashmap of integers which are the selected value
    // index out of the array, with keys as struct field names
}
