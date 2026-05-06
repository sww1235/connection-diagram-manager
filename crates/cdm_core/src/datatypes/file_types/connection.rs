use serde::{Deserialize, Serialize};

use crate::datatypes::project_types::connection::Type;

/// `Connection` is the source code representation of the on-disk file format for an in-memory
/// [`Connection`](crate::datatypes::project_types::connection::Connection).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Connection {
    /// ID of one end of connection.
    pub end1: Type,
    /// ID of other end of connection.
    pub end2: Type,
}
