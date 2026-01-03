use std::{io, path::PathBuf};

use thiserror::Error;

//TODO: flesh this out more
#[derive(Error, Debug)]
#[non_exhaustive]
#[expect(clippy::error_impl_error, reason = "main error type for library")]
#[expect(clippy::absolute_paths, reason = "keeping all sub-error types as absolute paths")]
/// `Error` is the list of errors and error types that can occur in the `cdm_core` library
pub enum Error {
    /// This error is used when duplicate keys are found in `Project` or `Library` structs during
    /// import
    #[error("Duplicate key {key} found in {origin_file} and {test_file}")]
    DuplicateKey {
        /// key of tested hashmap
        key: String,
        /// filepath of file first imported which is being compared against
        origin_file: PathBuf,
        /// filepath of file being checked for duplicates
        test_file: PathBuf,
    },
    /// Errors from `Library` code
    #[error(transparent)]
    LibraryError(#[from] LibraryError),
    /// Errors from `Project` code
    #[error(transparent)]
    ProjectError(#[from] ProjectError),
    /// Errors relating to linebreaking
    #[error(transparent)]
    ParagraphBreaking(#[from] paragraph_breaker::Error),
    /// Errors relating to input / output
    #[error(transparent)]
    IOError(#[from] io::Error),
    /// Errors relating to PDF creation and export
    #[error(transparent)]
    PDFError(#[from] pdf_helper::Error),
    /// Errors resulting from the USVG library
    #[error(transparent)]
    USVGError(#[from] usvg::Error),
    /// Errors resulting from PDF generation
    #[error(transparent)]
    PDFGenerationError(#[from] PDFError),
    /// Errors resulting from parsing unit name strings during Deserialize
    #[error(transparent)]
    UnitParsingError(#[from] UnitParsingError),
    /// Errors resulting from parsing config files
    #[error(transparent)]
    ConfigParsingError(#[from] figment::Error),
    /// Errors resulting from TOML file parsing
    #[error(transparent)]
    TomlError(#[from] toml::de::Error),
}

/// `LibraryError` is the list of errors that can occur within code related to `Library` data,
/// especially during parsing
#[derive(Debug, Error)]
#[non_exhaustive]
#[expect(clippy::module_name_repetitions, reason = "error types should have Error in the name")]
pub enum LibraryError {
    /// This error is used when a lookup is made in the library and the ID is not found
    #[error("The ID {id} of type {library_type} not found in library")]
    ValueNotFound {
        /// The ID of the the `Library` entry
        id: String,
        //TODO: switch to using a derived enum for library_type
        /// The type of the `Library` entry
        library_type: String,
    },
    /// This error is used when optional data is missing from library data when required for a
    /// certain operation in the program.
    #[error(
        "The Library entry of type {library_type} with ID {id} requires the following data to peform the operation requested: \
         {data_missing}"
    )]
    DataMissing {
        /// The ID of the the `Library` entry
        id: String,
        //TODO: switch to using a derived enum for library_type
        /// The type of the `Library` entry
        library_type: String,
        /// What data was missing from the `Library` entry
        data_missing: String,
    },
    /// `WireTypeError` are errors resulting from functions specific to `WireType`s.
    #[error(transparent)]
    WireTypeError(#[from] WireTypeError),
}
/// `WireTypeError` are errors resulting from functions specific to `WireType`s.
#[derive(Debug, Error)]
#[non_exhaustive]
#[expect(clippy::module_name_repetitions, reason = "error types should have Error in the name")]
pub enum WireTypeError {
    /// This error is used when failing to calculate Overall Cross Sectional Area
    #[error("Overall Cross Seectional Area calculation failed due to {0}")]
    UnableToCalculateOverallCrossSectionalArea(String),
}

/// `ProjectError` is the list of errors that can occur within code related to `Project` data,
/// especially during parsing
#[derive(Debug, Error)]
#[non_exhaustive]
#[expect(clippy::module_name_repetitions, reason = "error types should have Error in the name")]
pub enum ProjectError {
    /// This error is used when a lookup is made in the project and the ID is not found
    #[error("The ID {id} of type {project_type} not found in project")]
    ValueNotFound {
        /// The ID of the the `Project` entry
        id: String,
        //TODO: switch to using a derived enum for project_type
        /// The type of the `Project` entry
        project_type: String,
    },
    /// This error is used when optional data is missing from project data when required for a
    /// certain operation in the program.
    #[error(
        "The Project entry of type {project_type} with ID {id} requires the following data to peform the operation requested: \
         {data_missing}"
    )]
    DataMissing {
        /// The ID of the the `Project` entry
        id: String,
        //TODO: switch to using a derived enum for project_type
        /// The type of the `Project` entry
        project_type: String,
        /// What data was missing from the `Project` entry
        data_missing: String,
    },
    /// `ConnectionError` is the list of errors that result from processing connections
    #[error(transparent)]
    ConnectionError(#[from] ConnectionError),
}
/// `ConnectionError` is the list of errors that result from processing connections
#[derive(Debug, Error)]
#[non_exhaustive]
#[expect(clippy::module_name_repetitions, reason = "error types should have Error in the name")]
pub enum ConnectionError {
    /// Generic error for an invalid connection. Use the more specific errors listed below if
    /// possible. Make sure that `reason` starts with an uppper case letter.
    #[error("The connection between {end1} and {end2} in {project_file} is invalid. {reason}")]
    Invalid {
        /// The ID of one end of the connection
        end1: String,
        /// The ID of the other end of the connection
        end2: String,
        /// What project file this connection was found in.
        project_file: PathBuf,
        /// The reason the connection is invalid.
        reason: String,
    },
    // The whole map_or() chain, adds a space to the beginning of `message` if there is content so
    // there is correct spacing. That way the fmt string doesn't have to have an extra space in it
    // all the time.
    /// Error produced if both ends of the connection are the same type and not valid to connect.
    /// make sure that `message` starts with an uppper case letter.
    #[error("The connection between {end1} and {end2} in {project_file} is invalid because they are both the same type.\
    {}", message.clone().map_or(String::new(), |mut value| {value.insert_str(0, ""); value}))]
    SameType {
        /// The ID of one end of the connection
        end1: String,
        /// The ID of the other end of the connection
        end2: String,
        /// What project file this connection was found in.
        project_file: PathBuf,
        /// An optional message with more information for the user
        message: Option<String>,
    },
}

/// `PDFGenerationError` is the list of errors that can occur in `pdf_generation`
#[derive(Debug, Error)]
#[non_exhaustive]
#[expect(clippy::module_name_repetitions, reason = "error types should have Error in the name")]
pub enum PDFError {
    /// Error resulting from layout or rendering
    #[error("Layout Error: {0}")]
    LayoutError(String),
    /// Error resulting during PDF Creation
    #[error("PDF Creation Error: {0}")]
    PDFCreationError(String),
    /// If a pdf page isn't found when indexing
    #[error("PDF Page not found")]
    PDFPageNotFound,
}

/// `UnitParsingError` is the list of errors that can occur during Deserialization of units
#[derive(Debug, Error)]
#[non_exhaustive]
#[expect(clippy::module_name_repetitions, reason = "error types should have Error in the name")]
pub enum UnitParsingError {
    #[error("Unknown unit string {unit_string} for quantity type {quantity_type}")]
    /// Error resulting from an unknown unit string
    UnknownUnit {
        /// tested unit string
        unit_string: String,
        /// checked unit type
        quantity_type: String,
    },
    #[error("Unsupported unit string {unit_string} for quantity type {quantity_type}")]
    /// Error resulting from an unsupported unit string
    UnsupportedUnit {
        /// tested unit string
        unit_string: String,
        /// checked quantity type
        quantity_type: String,
    },
    #[error("Unable to store provided value of quantity type {quantity_type} into a {data_type}")]
    /// Error resulting from a value that is unable to be stored in the underlying datatype
    ValueError {
        /// provided quantity type
        quantity_type: String,
        /// underlying datatype
        data_type: String,
    },
}
