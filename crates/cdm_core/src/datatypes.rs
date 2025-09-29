/// `color` is used to define common colors, as well as allow custom colors to be defined
pub mod color;
/// `library_types` contains the types that are contained in a library
pub mod library_types;
/// `project_types` contains the types that are contained in a project
pub mod project_types;
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
