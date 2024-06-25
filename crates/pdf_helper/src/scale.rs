use std::fmt;

/// `ScalingFactor` is used to represent scaling when laying out PDF documents.
/// It is a ratio or quotient represented as a:b or a/b.
///
/// Default is 1:1
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ScalingFactor {
    pub a: u16,
    pub b: u16,
}

impl fmt::Display for ScalingFactor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.a, self.b)
    }
}

impl Default for ScalingFactor {
    fn default() -> Self {
        Self { a: 1, b: 1 }
    }
}
