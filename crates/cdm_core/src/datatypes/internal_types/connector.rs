use serde::{Deserialize, Serialize};

use cdm_traits::connector;

/// `Connector` is an instance of a [`ConnectorType`](super::connector_type::ConnectorType)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Connector {
    /// `id` of connector
    pub id: String,
    /// The type of this connector instance
    pub connector_type: String,
}

impl connector::Connector for Connector {
    fn pin_count(&self) -> u64 {
        #[expect(clippy::unwrap_used)]
        // allowing unwrap as I want a panic here if this application
        // is used on a 128 bit architecture
        u64::try_from(self.connector_type.borrow().pins.len()).unwrap()
    }
}
