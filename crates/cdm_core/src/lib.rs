//! `cdm_core` contains datatypes, functions
//! and logic that are used by the binary implementations
//! of the program

/// shared logic between GUI and CLI binaries
pub mod bin_logic;
/// `config` contains configuration structs that are used for
/// the application binaries
pub mod config;
/// `datatypes` contains all custom types for the
/// `cdm_core` library.
///
/// It also contains functions for parsing data files
/// and the main `Data` type
pub mod datatypes;
/// Functions for navigating directories
pub mod directory_navigator;
/// contains error types for application
pub mod error;
/// utility functions for working with `Path`s and `PathBuf`s
pub(crate) mod path_utils;
/// `pdf_generation` contains the functions used to generate a PDF
/// of `Projects`
pub mod pdf_generation;
/// `project_config` contains definitions for the project specific configuration options
pub mod project_config;
/// Traits used in `cdm_core`
pub mod traits;
/// utility functions that don't have a good home elsewhere
pub(crate) mod util_functions;
