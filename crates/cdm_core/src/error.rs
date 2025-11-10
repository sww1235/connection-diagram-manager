use paragraph_breaker::Error as ParagraphError;
use pdf_helper::Error as PDFError;
use thiserror::Error;
use usvg::Error as USVGError;

//TODO: flesh this out more
#[derive(Error, Debug)]
#[non_exhaustive]
/// `Error` is the list of errors and error types that can occur in the `cdm_core` library
pub enum Error {
    /// This error is used when a lookup is made in the library and the ID is not found
    #[error("The ID {id} of type {library_type} not found in library")]
    LibraryValueNotFound {
        /// The ID of the the `Library` entry
        id: String,
        //TODO: switch to using a derived enum for library_type
        /// The type of the `Library` entry
        library_type: String,
    },
    /// This error is used when a lookup is made in the project and the ID is not found
    #[error("The ID {id} of type {project_type} not found in project")]
    ProjectValueNotFound {
        /// The ID of the the `Library` entry
        id: String,
        //TODO: switch to using a derived enum for project_type
        /// The type of the `Project` entry
        project_type: String,
    },
    /// This error is used when optional data is missing from library data when required for a
    /// certain operation in the program.
    #[error(
        "The Library entry of type {library_type} with ID {id} requires the following data to peform the operation requested: \
         {data_missing}"
    )]
    LibraryDataMissing {
        /// The ID of the the `Library` entry
        id: String,
        //TODO: switch to using a derived enum for library_type
        /// The type of the `Library` entry
        library_type: String,
        /// What data was missing from the `Library` entry
        data_missing: String,
    },
    /// This error is used when optional data is missing from project data when required for a
    /// certain operation in the program.
    #[error(
        "The Project entry of type {project_type} with ID {id} requires the following data to peform the operation requested: \
         {data_missing}"
    )]
    ProjectDataMissing {
        /// The ID of the the `Project` entry
        id: String,
        //TODO: switch to using a derived enum for project_type
        /// The type of the `Project` entry
        project_type: String,
        /// What data was missing from the `Project` entry
        data_missing: String,
    },
    /// This error is used when duplicate keys are found in `Project` or `Library` structs during
    /// import
    #[error("Duplicate key {key} found in {origin_file} and {test_file}")]
    DuplicateKey {
        /// key of tested hashmap
        key: String,
        /// filepath of file first imported which is being compared against
        origin_file: String,
        /// filepath of file being checked for duplicates
        test_file: String,
    },
    /// Errors relating to linebreaking
    #[error(transparent)]
    ParagraphBreaking(#[from] ParagraphError),
    /// Errors relating to input / output
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    /// Errors relating to PDF creation and export
    #[error(transparent)]
    PDFError(#[from] PDFError),
    /// Errors resulting from the USVG library
    #[error(transparent)]
    USVGError(#[from] USVGError),
    /// Errors resulting from PDF generation
    #[error(transparent)]
    PDFGenerationError(#[from] PDFGenerationError),
}

/// `PDFGenerationError` is the list of errors that can occur in `pdf_generation`
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PDFGenerationError {
    /// Error resulting from layout or rendering
    #[error("Layout Error: {0}")]
    LayoutError(String),
    #[error("PDF Creation Error: {0}")]
    /// Error resulting durin PDF Creation
    PDFCreationError(String),
}
