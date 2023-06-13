use std::collections::HashMap;

/// `Mergable` indicates that an object has the necessary utilities to merge itself with another
/// instance of the same object type.
pub trait Mergable {
    /// `merge_prompt` assists the user in merging 2 object instances by prompting the user with
    /// the difference between the object, field by field, and providing sensible defaults.
    fn merge_prompt(
        &mut self,
        other: &Self,
        prompt_fn: fn(HashMap<String, [String; 2]>) -> HashMap<String, bool>,
    );
    // pass a hashmap of string arrays, return a hashmap of integers which are the selected value
    // index out of the array, with keys as struct field names
}

/// `Empty` indicates that an object can be checked for `PartialEq` to `Object::new()`
pub trait Empty {
    /// `is_empty` checks to see if self is `PartialEq` to `Object::new()`
    fn is_empty(&self) -> bool;
}

/// `PartialEmpty` indicates that an object can be checked to be almost `PartialEq` to `Object::new()`,
/// excepting the id field
pub trait PartialEmpty {
    /// `is_partial_empty` checks to see if self is almost `PartialEq` to `Object::new()` but id can be
    /// different
    fn is_partial_empty(&self) -> bool;
}
