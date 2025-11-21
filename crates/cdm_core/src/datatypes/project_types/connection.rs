use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// `Connection` represents a connection between two different elements.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::partial_pub_fields)]
pub struct Connection {
    /// Dot joined IDs of one end of connection
    pub end1: String,
    /// root type of end1
    pub end1_type: String,
    /// Dot joined IDs of other end of connection
    pub end2: String,
    /// root type of end2
    pub end2_type: String,
    /// datafile the struct instance was read in from
    #[serde(skip)]
    pub(crate) contained_datafile_path: PathBuf,
}
