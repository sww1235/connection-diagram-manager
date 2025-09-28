/// `color` is used to define common colors, as well as allow custom colors to be defined
pub mod color;
///`internal_types` contains all representations of data used internally in the program.
pub mod internal_types;
/// `project` contains the types that define a project.
pub mod project;
/// `svg` represents a complete SVG image
pub mod svg;
/// `unit_helper` contains wrapper types around UOM units, to allow tracking what unit was defined
/// in the data file easier.
///
/// Also contains conversion helper functions
pub mod unit_helper;
///`util_types` contains utility types that are used in multiple other different types and files,
///including generic enums.
pub mod util_types;
