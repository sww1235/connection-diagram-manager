use serde::{Deserialize, Serialize};

use crate::{
    datatypes::{library_types::Library, util_types::SymbolStyle},
    error::LibraryError,
    traits,
};

/// `Connector` is an instance of a [`ConnectorType`](super::connector_type::ConnectorType)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Connector {
    /// The type of this connector instance
    pub connector_type: String,
    /// Optional styling data for schematic symbol
    pub symbol_style: Option<SymbolStyle>,
}

impl traits::Connector for Connector {
    #[expect(clippy::unwrap_in_result)]
    fn pin_count(&self, library: &Library) -> Result<u64, LibraryError> {
        let connector_type = library
            .connector_types
            .get(&self.connector_type)
            .ok_or(LibraryError::ValueNotFound {
                id: self.connector_type.clone(),
                library_type: "Connector Type".to_string(),
            })?;

        #[expect(clippy::unwrap_used)]
        // allowing unwrap as I want a panic here if this application
        // is used on a 128 bit architecture
        Ok(u64::try_from(connector_type.pins.len()).unwrap())
    }
}
