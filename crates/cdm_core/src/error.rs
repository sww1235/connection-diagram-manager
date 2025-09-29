use thiserror::Error;

use paragraph_breaker::Error as ParagraphError;
use pdf_helper::Error as PDFError;
use usvg::Error as USVGError;

//TODO: flesh this out more
#[derive(Error, Debug)]
pub enum Error {
    #[error("The ID {0} not found in library")]
    LibraryValueNotFound(String),
    #[error("The ID {0} not found in project")]
    ProjectValueNotFound(String),
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
}
