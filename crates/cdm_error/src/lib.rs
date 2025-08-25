//! CDM Error contains some error handling stuff that is used in the other crates in the workspace
//!
//!

use thiserror::Error;

use paragraph_breaker::Error as ParagraphError;
use pdf_helper::Error as PDFError;
use usvg::Error as USVGError;

/// `Error` is the general list of all errors that are triggered in this library
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    /// Errors relating to linebreaking
    ParagraphBreaking(#[from] ParagraphError),
    /// Errors relating to input / output
    IOError(#[from] std::io::Error),
    /// Errors relating to PDF creation and export
    PDFError(#[from] PDFError),
    /// Errors resulting from the USVG library
    USVGError(#[from] USVGError),
    /// Errors relating to file handling
    FileError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ParagraphBreaking(ref e) => write!(f, "Line Breaking failed: {e}"),
            Error::IOError(ref e) => write!(f, "IO error: {e}"),
            Error::PDFError(ref e) => write!(f, "PDF error: {e}"),
            Error::USVGError(ref e) => write!(f, "SVG error: {e}"),
            Error::FileError(ref e) => write!(f, "File error: {e}"),
        }
    }
}
