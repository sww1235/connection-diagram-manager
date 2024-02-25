//! `cdm_core` contains datatypes, functions
//! and logic that are used by the binary implementations
//! of the program

/// `config` contains configuration structs that are used for
/// the application binaries
pub mod config;
/// `datatypes` contains all custom types for the
/// `cdm_core` library.
///
/// It also contains functions for parsing data files
/// and the main `Data` type
pub mod datatypes;

/// `pdf_generation` contains the functions used to generate a PDF
/// of `Projects`
pub mod pdf_generation;

pub use cdm_macros;
