//! `cdm_core` contains datatypes, functions
//! and logic that are used by the binary implementations
//! of the program

#![warn(missing_docs)]
/// `config` contains configuration structs that are used for
/// the application binaries
pub mod config;
/// `datatypes` contains all custom types for the
/// `cdm_core` library.
///
/// It also contains functions for parsing data files
/// and the main `Data` type
pub mod datatypes;
/// `pdf_helper` contains functions to generate PDF files.
/// It wraps `lopdf` with custom functions specific to
/// producing schematics and reports.
#[cfg(feature = "cli")]
pub mod pdf_helper;

#[cfg(feature = "cli")]
/// `paragraph_breaking` contains an implementation of a simplified version of the Knuth-Plass
/// algorithm
pub mod paragraph_breaking;
