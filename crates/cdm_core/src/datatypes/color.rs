use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Violet,
    Pink,
    Rose,
    Brown,
    #[default]
    Black,
    White,
    Gray,
    Grey,
    Slate,
    Clear,
    Cyan,
    Aqua,
    RGB {
        hex_code: String,
        abbreviation: String,
        color_standard: Option<String>,
    },
}

impl Color {
    pub fn abbreviation(&self) -> String {
        match &self {
            Self::Red => "RED".to_string(),
            Self::Orange => "ORN".to_string(),
            Self::Yellow => "YEL".to_string(),
            Self::Green => "GRN".to_string(),
            Self::Blue => "BLU".to_string(),
            Self::Purple => "PUR".to_string(),
            Self::Violet => "VIO".to_string(),
            Self::Pink => "PNK".to_string(),
            Self::Rose => "RSE".to_string(),
            Self::Brown => "BRN".to_string(),
            Self::Black => "BLK".to_string(),
            Self::White => "WHT".to_string(),
            Self::Gray | Self::Grey => "GRY".to_string(),
            Self::Slate => "SLT".to_string(),
            Self::Clear => "CLR".to_string(),
            Self::Cyan => "CYN".to_string(),
            Self::Aqua => "AQA".to_string(),
            Self::RGB { abbreviation, .. } => abbreviation.clone(),
        }
    }
    pub fn hex_code(&self) -> String {
        match &self {
            Self::Red => "FF0000".to_string(),
            Self::Orange => "FF5100".to_string(),
            Self::Yellow => "FFFF00".to_string(),
            Self::Green => "00FF00".to_string(),
            Self::Blue => "0000FF".to_string(),
            Self::Purple => "6700FF".to_string(),
            Self::Violet => "EE82EE".to_string(),
            Self::Pink | Self::Rose => "FFE4E1".to_string(),
            Self::Brown => "8B4513".to_string(),
            Self::Black => "000000".to_string(),
            Self::White | Self::Clear => "FFFFFF".to_string(),
            Self::Gray | Self::Grey | Self::Slate => "808080".to_string(),
            Self::Cyan | Self::Aqua => "00FFFF".to_string(),
            Self::RGB { hex_code, .. } => hex_code.clone(),
        }
    }
}
