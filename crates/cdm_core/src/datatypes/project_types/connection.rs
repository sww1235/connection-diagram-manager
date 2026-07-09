use std::path::{Path, PathBuf};

use crate::{datatypes::file_types, traits::FromFile};

/// `Connection` represents a connection between two different elements.
#[derive(Debug, PartialEq, Clone)]
#[expect(clippy::partial_pub_fields, reason = "contained_datafile_path is not part of public API")]
pub struct Connection {
    /// One end of connection.
    pub end1: End,
    /// The other end of connection.
    pub end2: End,
    /// What is connecting the two ends of the connection.
    pub connection: InnerConnection,
    /// datafile the struct instance was read in from.
    pub(crate) contained_datafile_path: PathBuf,
}

impl From<file_types::connection::Connection> for Connection {
    #[inline]
    fn from(value: file_types::connection::Connection) -> Self {
        Self {
            end1: value.end1.into(),
            end2: value.end2.into(),
            connection: value.connection.into(),
            contained_datafile_path: PathBuf::new(),
        }
    }
}

/// `EndType` determines what each end of the connection is.
#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
pub enum End {
    /// An [`Equipment`].
    Equipment {
        /// ID of [`Equipment`] instance in [`Project`] hashmap.
        equipment_id: String,
        /// ID of connection point on equipment.
        connection_point_id: String,
    },
    /// A [`TerminalStrip`].
    TerminalStrip {
        /// ID of [`TerminalStrip`] in [`Project`] hashmap.
        term_strip_id: String,
        /// ID or index of element within terminal strip.
        element_id: String,
    },
}

impl From<file_types::connection::End> for End {
    #[inline]
    fn from(value: file_types::connection::End) -> Self {
        use file_types::connection::End;
        match value {
            End::Equipment {
                equipment_id,
                connection_point_id,
            } => Self::Equipment {
                equipment_id,
                connection_point_id,
            },
            End::TerminalStrip { term_strip_id, element_id } => Self::TerminalStrip { term_strip_id, element_id },
        }
    }
}

/// `ConnectionType` determines what is making the connection.
#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
#[expect(clippy::module_name_repetitions, reason = "still deciding on a better name")]
pub enum InnerConnection {
    ///// A [`Wire`].
    //Wire {
    //    /// ID of [`Wire`] instance in [`Project`] hashmap.
    //    wire_id: String,
    //},
    /// A [`Cable`].
    Cable {
        /// ID of [`Cable`] instance in [`Project`] hashmap.
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
        /// ID of [`TermCable`] instance in [`Project`] hashmap.
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

impl From<file_types::connection::InnerConnection> for InnerConnection {
    #[inline]
    fn from(value: file_types::connection::InnerConnection) -> Self {
        use file_types::connection::InnerConnection;
        match value {
            InnerConnection::Cable { cable_id, core_id } => Self::Cable { cable_id, core_id },
            InnerConnection::TermCable { cable_id, core_id } => Self::TermCable { cable_id, core_id },
        }
    }
}

impl FromFile for Connection {
    #[inline]
    fn datafile(&self) -> PathBuf {
        self.contained_datafile_path.clone()
    }
    #[inline]
    fn set_datafile(&mut self, datafile_path: &Path) {
        self.contained_datafile_path = datafile_path.to_path_buf();
    }
}

/// Used to tag connection id references if they refer to End1 or End2 to reduce the need for
/// additional match or if let statements.
#[derive(Debug, PartialEq, Clone)]
#[expect(clippy::exhaustive_enums, reason = "Can't have more than 2 ends of a linear item")]
pub enum EndDesignation {
    /// End 1.
    End1,
    /// End 2.
    End2,
}
