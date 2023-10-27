/// `Compare` provides a method to compare two instances of a type and provide a string result for
/// visual comparison and selection
///
pub trait Compare: PartialEq {
    /// `compare` is a method to produce a human readable comparison between two instances of a
    /// type if they are not equal.
    /// This is implemented for some basic primative types in this module and can be derived by a
    /// proc-macro crate TODO: fill in name here
    /// TODO: maybe refactor to use struct instead of tuples
    fn compare(&self, other: &Self, compare_result: Option<CompareResult>)
        -> Option<CompareResult>;
}

pub struct CompareResult {
    pub struct_name: Option<String>,
    pub base_result: Option<CompareResultBase>,
    pub field_variations: Option<Vec<CompareResultField>>,
}
pub struct CompareResultField {
    pub field_name: String,
    pub self_value: String,
    pub other_value: String,
    pub use_other: bool,
}

pub struct CompareResultBase {
    pub self_value: String,
    pub other_value: String,
}

impl Compare for String {
    fn compare(
        &self,
        other: &Self,
        compare_result: Option<CompareResult>,
    ) -> Option<CompareResult> {
        // compare_result should never be Some() on base types, so it is ignored
        if self == other {
            return None;
        } else {
            Some(CompareResult {
                struct_name: None,
                base_result: Some(CompareResultBase {
                    self_value: self.to_string(),
                    other_value: other.to_string(),
                }),
                field_variations: None,
            })
        }
    }
}

impl Compare for f64 {
    fn compare(
        &self,
        other: &Self,
        compare_result: Option<CompareResult>,
    ) -> Option<CompareResult> {
        if self == other {
            return None;
        } else {
            Some(CompareResult {
                struct_name: None,
                base_result: Some(CompareResultBase {
                    self_value: self.to_string(),
                    other_value: other.to_string(),
                }),
                field_variations: None,
            })
        }
    }
}
