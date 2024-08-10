/// `PartialEmpty` indicates that an object can be checked to be almost `PartialEq` to `Object::new()`,
/// excepting the id field
pub trait PartialEmpty {
    /// `is_partial_empty` checks to see if self is almost `PartialEq` to `Object::new()` but id can be
    /// different
    fn is_partial_empty(&self) -> bool;
}
