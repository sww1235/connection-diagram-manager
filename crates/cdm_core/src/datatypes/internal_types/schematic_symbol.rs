use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::datatypes::{color::Color, util_types::UserFields};

/// `SchematicSymbol` represents an instance of a SchematicSymbolType
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SchematicSymbol {
    /// type of schematic symbol
    pub symbol_type: String,
    /// color of symbol in schematic diagram
    pub symbol_color: Color,
    /// what object this symbol represents
    pub represented_object: SymbolType,
    /// User defined fields
    pub user_fields: Option<UserFields>,
    /// datafile the struct instance was read in from
    pub contained_datafile_path: PathBuf,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum SymbolType {
    Equipment(String),
    Terminal(String),
    Connector(String),
}
