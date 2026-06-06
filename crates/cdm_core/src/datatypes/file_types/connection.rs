use serde::{Deserialize, Serialize};

/// `Connection` is the source code representation of the on-disk file format for an in-memory
/// [`Connection`](crate::datatypes::project_types::connection::Connection).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Connection {
    /// One end of connection.
    pub end1: End,
    /// The other end of connection.
    pub end2: End,
    /// What is connecting the two ends of the connection.
    pub connection: InnerConnection,
}

/// `End` is the source code representation of the on-disk file format for an in-memory
/// [`End`](crate::datatypes::project_types::connection::End).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum End {
    /// An [`Equipment`].
    Equipment {
        /// ID of [`Equipment`] instance in datafile.
        equipment_id: String,
        /// ID of connection point on equipment.
        connection_point_id: String,
    },
    /// A [`TerminalStrip`].
    TerminalStrip {
        /// ID of [`TerminalStrip`] in datafile.
        term_strip_id: String,
        /// ID or index of element within terminal strip.
        element_id: String,
    },
}

/// `InnerConnection` is the source code representation of the on-disk file format for an in-memory
/// [`InnerConnection`](crate::datatypes::project_types::connection::InnerConnection).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InnerConnection {
    /// A [`Cable`].
    Cable {
        /// ID of [`Cable`] instance in datafile.
        cable_id: String,
        /// ID of `CableCore` generated in `update_data_from_library()`.
        ///
        /// This is a `.` separated sequence of strings, starting with the ID of the outermost core
        /// in the `CableType` (All root cores in a `CableType` have a unique key).
        ///
        /// If the `CableCore` is `WireType`, then the ID will be the ID of the core in the
        /// `CableType`.
        ///
        /// If the `CableCore` is `CableType`, then the outer/super ID will be the ID of the core in the
        /// `CableType`. The IDs of the inner cores will be appended to the ID of the outer/super core
        /// with a `.`. This will recurse to as many cores as you need to define in a cable, or until
        /// the program runs out of memory.
        core_id: String,
    },
    //TODO: which cable core? Or should this be connector based?
    /// A [`TermCable`].
    TermCable {
        /// ID of [`TermCable`] instance in datafile.
        cable_id: String,
        /// ID of `CableCore` generated in `update_data_from_library()`.
        ///
        /// This is a `.` separated sequence of strings, starting with the ID of the outermost core
        /// in the `CableType` (All root cores in a `CableType` have a unique key).
        ///
        /// If the `CableCore` is `WireType`, then the ID will be the ID of the core in the
        /// `CableType`.
        ///
        /// If the `CableCore` is `CableType`, then the outer/super ID will be the ID of the core in the
        /// `CableType`. The IDs of the inner cores will be appended to the ID of the outer/super core
        /// with a `.`. This will recurse to as many cores as you need to define in a cable, or until
        /// the program runs out of memory.
        core_id: String,
    },
}
