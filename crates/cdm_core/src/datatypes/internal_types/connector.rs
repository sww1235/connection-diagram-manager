use std::cell::RefCell;
use std::rc::Rc;

use cdm_traits::connector;

use super::connector_type::ConnectorType;

/// `Connector` is an instance of a [`ConnectorType`](super::connector_type::ConnectorType)
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Connector {
    /// `id` of connector
    pub id: String,
    /// The type of this connector instance
    pub connector_type: Rc<RefCell<ConnectorType>>,
}

impl connector::Connector for Connector {
    fn pin_count(&self) -> u64 {
        #[allow(clippy::unwrap_used)]
        // allowing unwrap as I want a panic here if this application
        // is used on a 128 bit architecture
        u64::try_from(self.connector_type.borrow().pins.len()).unwrap()
    }
}
