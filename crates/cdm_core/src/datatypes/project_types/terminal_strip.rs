use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    datatypes::{
        project_types::ProjectData,
        util_types::{IECCodes, PhysicalLocation, SymbolStyle, UserFields},
    },
    traits::FromFile,
};

/// `TerminalStrip` represents an individual terminal strip in a project.
///
/// A `TerminalStrip` is a collection or group of 1 or more terminal blocks.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::partial_pub_fields, reason = "contained_datafile_path is not part of public API")]
pub struct TerminalStrip {
    /// structured identifier of terminal strip.
    pub identifier: Option<String>,
    /// Optional description.
    pub description: Option<String>,
    /// containing enclosure.
    pub enclosure: Option<String>,
    /// mounting rail.
    pub mounting_rail: Option<String>,
    /// physical location of `TerminalStrip`.
    pub physical_location: Option<PhysicalLocation>,
    /// Fields for use with IEC project coding.
    pub iec_codes: Option<IECCodes>,
    /// User defined fields.
    pub user_fields: Option<UserFields>,
    /// terminals and accessories defined in terminal strip.
    pub elements: Vec<Element>,
    /// Jumpers in terminal strip.
    pub jumpers: Vec<Jumper>,
    /// datafile the struct instance was read in from.
    #[serde(skip)]
    pub(crate) contained_datafile_path: PathBuf,
}

/// `Element` represents one element of a terminal strip, be it terminal block or
/// accessory.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Element {
    ///Terminal Number. Used for ordering and identification
    ///If not specified, defaults to its index in the terminals Vec, plus 1.
    pub terminal_number: Option<u64>,
    /// structured identifier of terminal.
    pub identifier: Option<String>,
    /// Optional descriptive label.
    pub label: Option<String>,
    /// Accessories like component holders, fuses, etc for this terminal.
    pub accessories: Option<Vec<String>>,
    /// Optional styling data for schematic symbol.
    pub symbol_style: Option<SymbolStyle>,
    /// What the type of `Element` actually is.
    pub contained_type: TermAccy,
}

/// Enum allowing storage of either `TerminalType` or `TerminalStripAccessoryType` in `Terminal`.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
#[expect(missing_docs, reason = "self documenting enum")]
pub enum TermAccy {
    Terminal(String),
    Accessory(String),
}

/// `Jumper` represents a jumper instance within a terminal strip.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Jumper {
    /// Type of jumper.
    pub jumper_type: String,
    /// structured identifier of jumper.
    pub identifier: Option<String>,
    /// Optional descriptive label.
    pub label: Option<String>,
    /// Array of `terminal_number`s that this jumper makes connections to.
    pub jumper_connections: Vec<u64>,
    /// Optional styling data for schematic symbol.
    pub symbol_style: Option<SymbolStyle>,
}

impl FromFile for TerminalStrip {
    #[inline]
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
    #[inline]
    fn set_datafile(&mut self, datafile_path: &Path) {
        self.contained_datafile_path = datafile_path.to_path_buf();
    }
}

impl ProjectData for TerminalStrip {}
