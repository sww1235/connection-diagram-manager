use thiserror::Error;

use paragraph_breaker::Error as ParagraphError;
use pdf_helper::Error as PDFError;
use usvg::Error as USVGError;

//TODO: flesh this out more
#[derive(Error, Debug)]
pub enum Error {
    /// This error is used when a lookup is made in the library and the ID is not found
    #[error("The ID {0} not found in library")]
    LibraryValueNotFound(String),
    /// This error is used when a lookup is made in the project and the ID is not found
    #[error("The ID {0} not found in project")]
    ProjectValueNotFound(String),
    /// This error is used when optional data is missing from library data when required for a certain operation in
    /// the program.
    #[error("The Library entry with ID: {id} requires the following data to peform the operation requested: {data_missing}")]
    LibraryDataMissing { id: String, data_missing: String },
    /// This error is used when optional data is missing from project data when required for a certain operation in
    /// the program.
    #[error("The Library entry with ID: {id} requires the following data to peform the operation requested: {data_missing}")]
    ProjectDataMissing { id: String, data_missing: String },
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
