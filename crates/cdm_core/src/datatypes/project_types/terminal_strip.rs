use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    datatypes::util_types::{IECCodes, PhysicalLocation, UserFields},
    traits::FromFile,
};

/// `TerminalStrip` represents an individual terminal strip in a project.
///
/// A `TerminalStrip` is a collection or group of 1 or more terminal blocks
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::partial_pub_fields)]
pub struct TerminalStrip {
    /// structured identifier of terminal strip
    pub identifier: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// containing enclosure
    pub enclosure: Option<String>,
    /// mounting rail
    pub mounting_rail: Option<String>,
    /// physical location of `TerminalStrip`
    pub physical_location: Option<PhysicalLocation>,
    /// Fields for use with IEC project coding
    pub iec_codes: Option<IECCodes>,
    /// User defined fields
    pub user_fields: Option<UserFields>,
    /// terminals and accessories defined in terminal strip.
    pub terminals: Vec<Terminal>,
    /// Jumpers in terminal strip
    pub jumpers: Vec<Jumper>,
    /// datafile the struct instance was read in from
    #[serde(skip)]
    pub(super) contained_datafile_path: PathBuf,
}

/// `Terminal` represents one element of a terminal strip, be it terminal block or
/// accessory.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Terminal {
    ///Terminal Number. Used for ordering and identification
    ///If not specified, defaults to its index in the terminals Vec, one indexed.
    pub terminal_number: Option<u64>,
    /// structured identifier of terminal
    pub identifier: Option<String>,
    /// Optional descriptive label
    pub label: Option<String>,
    /// Accessories like component holders, fuses, etc for this terminal
    pub accessories: Option<Vec<String>>,
}

/// Enum allowing storage of either `TerminalType` or `TerminalStripAccessoryType` in `Terminal`
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
#[expect(missing_docs)]
pub enum TermAccy {
    Terminal(String),
    Accessory(String),
}

/// `Jumper` represents a jumper instance within a terminal strip
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Jumper {
    /// Type of jumper
    pub jumper_type: String,
    /// structured identifier of jumper
    pub identifier: Option<String>,
    /// Optional descriptive label
    pub label: Option<String>,
    /// Array of `terminal_number`s that this jumper makes connections to
    pub jumper_connections: Vec<u64>,
}

impl FromFile for TerminalStrip {
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
    fn set_datafile(&mut self, datafile_path: &Path) {
        self.contained_datafile_path = datafile_path.to_path_buf();
    }
}
