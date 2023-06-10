use paragraph_breaker::Error as ParagraphError;
use pdf_helper::Error as PDFError;
use usvg::Error as USVGError;

use std::io;

/// `Error` is the general list of all errors that are triggered in this library
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Errors relating to linebreaking
    ParagraphBreaking(String),
    /// Errors relating to input / output
    IOError(String),
    /// Errors relating to PDF creation and export
    PDFError(String),
    /// Errors resulting from the USVG library
    USVGError(String),
    /// Errors relating to file handling
    FileError(String),
}

impl std::error::Error for Error {}

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

impl From<ParagraphError> for Error {
    fn from(e: ParagraphError) -> Self {
        Error::ParagraphBreaking(format!("{e}"))
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IOError(format!("{e}"))
    }
}

impl From<PDFError> for Error {
    fn from(e: PDFError) -> Self {
        Error::PDFError(format!("{e}"))
    }
}

impl From<USVGError> for Error {
    fn from(e: USVGError) -> Self {
        Error::USVGError(format!("{e}"))
    }
}

impl From<lopdf::Error> for Error {
    fn from(e: lopdf::Error) -> Self {
        match e {
            lopdf::Error::ContentDecode
            | lopdf::Error::DictKey
            | lopdf::Error::Header
            | lopdf::Error::ObjectIdMismatch
            | lopdf::Error::ObjectNotFound
            | lopdf::Error::Offset(..)
            | lopdf::Error::PageNumberNotFound(..)
            | lopdf::Error::Parse { .. }
            | lopdf::Error::ReferenceLimit
            | lopdf::Error::BracketLimit
            | lopdf::Error::Trailer
            | lopdf::Error::Type
            | lopdf::Error::UTF8
            | lopdf::Error::Syntax(..)
            | lopdf::Error::Xref(..)
            | lopdf::Error::Invalid(..)
            | lopdf::Error::NoOutlines
            | lopdf::Error::Decryption(..) => Error::PDFError(format!("{e}")),
            lopdf::Error::IO(..) => Error::IOError(format!("{e}")),
        }
    }
}
