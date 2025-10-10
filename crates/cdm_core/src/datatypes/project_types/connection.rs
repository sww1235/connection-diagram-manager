use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// `Connection` represents a connection between two different elements.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Connection {
    /// Dot joined IDs of one end of connection
    pub end1: String,
    /// Dot joined IDs of other end of connection
    pub end2: String,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}
