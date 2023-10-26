
/// `Empty` indicates that an object can be checked for `PartialEq` to `Object::new()`
pub trait Empty {
    /// `is_empty` checks to see if self is `PartialEq` to `Object::new()`
    fn is_empty(&self) -> bool;
}
