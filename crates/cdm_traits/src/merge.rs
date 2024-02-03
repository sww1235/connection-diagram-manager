/// `Merge` indicates that an object has the necessary utilities to merge itself with another
/// instance of the same object type.
pub trait Merge {
    /// `merge_prompt` assists the user in merging 2 object instances by prompting the user with
    /// the difference between the object, field by field, and providing sensible defaults.
    fn merge_prompt(&mut self, other: &Self, prompt_fn: fn(ComparedStruct) -> ComparedStruct);
    // pass a hashmap of string arrays, return a hashmap of integers which are the selected value
    // index out of the array, with keys as struct field names
}

/// `StructField` represents one field in a compared struct
#[derive(Debug, Default, PartialEq)]
pub struct StructField {
    /// `field_name` is the name of the field
    pub name: String,
    /// `equality` is true when the field in the compared structs are equal. Used for ignoring or
    /// highlighting fields in eventual presentation
    pub equality: bool,
    /// `self_string` is the debug representation of the field in the struct that the method was
    /// called on
    pub self_string: String,
    /// `other_string` is the debug representation of the field in the compared struct
    pub other_string: String,
    /// `use_other` indicates that the compared struct value is the one that should be used in the
    /// merged struct
    pub use_other: bool,
}

impl StructField {
    pub fn new() -> Self {
        Default::default()
    }
}
#[derive(Debug, Default, PartialEq)]
pub struct ComparedStruct {
    pub struct_name: String,
    pub fields: Vec<StructField>,
}

impl ComparedStruct {
    pub fn new() -> Self {
        Default::default()
    }
}
