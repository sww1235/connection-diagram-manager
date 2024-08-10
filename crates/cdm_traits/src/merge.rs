/// `Merge` indicates that an object has the necessary utilities to merge itself with another
/// instance of the same object type.
pub trait Merge
where
    Self: Clone,
{
    /// `merge_prompt` assists the user in merging 2 object instances by prompting the user with
    /// the difference between the object, field by field, and providing sensible defaults.
    ///
    /// # Errors
    ///
    /// TODO: format this better
    ///
    /// - `DataMergeError`: means that the function failed to merge the two structs
    fn merge_prompt(
        &mut self,
        other: &Self,
        prompt_fn: fn(ComparedStruct) -> ComparedStruct,
    ) -> Result<(), Error>;
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
    /// Create new `StructField`
    #[must_use]
    pub fn new() -> Self {
        StructField::default()
    }
}
/// `ComparedStruct` is a container for comparision information about a struct
#[derive(Debug, Default, PartialEq)]
pub struct ComparedStruct {
    /// struct name
    pub struct_name: String,
    /// list of fields in the struct
    pub fields: Vec<StructField>,
}

impl ComparedStruct {
    /// Create new `ComparedStruct`
    #[must_use]
    pub fn new() -> Self {
        ComparedStruct::default()
    }
}

/// `Error` is the list of errors that are used in this trait definition
#[derive(Debug)]
//#[non_exhaustive]
pub enum Error {
    ///Failure to merge two different structs
    ///
    ///This is caused when attempting to merge 2 different structs with different IDs
    DataMergeError {
        /// datatype type
        datatype: String,
        /// ID of struct called as self
        self_id: String,
        /// ID of struct called as other
        other_id: String,
        /// datafile path of struct called as self
        self_path: String,
        /// datafile path of struct called as other
        other_path: String,
    },
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::DataMergeError {
                ref datatype,
                ref self_id,
                ref other_id,
                ref self_path,
                ref other_path,
            } => {
                write!(f, "Attempting to merge two structs of type {datatype} with IDs {self_id} and {other_id} which don't match. They came from datafiles {self_path} and {other_path}. Please check this data and clean it up, as this should not have happened.")
            }
        }
    }
}
