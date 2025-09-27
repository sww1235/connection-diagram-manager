use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// `Connection` represents a connection between two different elements.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub end1: String,
    pub end2: String,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}
