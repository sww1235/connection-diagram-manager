use serde::{Deserialize, Serialize};

use crate::{datatypes::library_types::Library, error::Error, traits};

/// `Connector` is an instance of a [`ConnectorType`](super::connector_type::ConnectorType)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Connector {
    /// `id` of connector
    pub id: String,
    /// The type of this connector instance
    pub connector_type: String,
}

impl traits::Connector for Connector {
    fn pin_count(&self, library: &Library) -> Result<u64, Error> {
        let connector_type = library
            .connector_types
            .get(&self.connector_type)
            .ok_or(Error::LibraryValueNotFound(self.connector_type.clone()))?;

        #[expect(clippy::unwrap_used)]
        // allowing unwrap as I want a panic here if this application
        // is used on a 128 bit architecture
        Ok(u64::try_from(connector_type.pins.len()).unwrap())
    }
}
