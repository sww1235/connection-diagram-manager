use serde::{Deserialize, Serialize};

/// `Color` is a standardized list of colors supported in the application.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[expect(
    missing_docs,
    clippy::exhaustive_enums,
    reason = "no need to document self documenting enum variants"
)]
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
    /// Used for custom colors. Only the `hex_code` is required.
    RGB {
        /// 6 character HEX code. RRGGBB.
        hex_code: String,
        /// 3 character abbreviation for the custom color.
        abbreviation: String,
        /// name / color code for a color standard that this custom color represents.
        color_standard: Option<String>,
    },
}

impl Color {
    /// Returns a 3 character abbreviation for each color.
    #[must_use]
    #[inline]
    pub fn abbreviation(&self) -> String {
        match self {
            Self::Red => "RED".to_owned(),
            Self::Orange => "ORN".to_owned(),
            Self::Yellow => "YEL".to_owned(),
            Self::Green => "GRN".to_owned(),
            Self::Blue => "BLU".to_owned(),
            Self::Purple => "PUR".to_owned(),
            Self::Violet => "VIO".to_owned(),
            Self::Pink => "PNK".to_owned(),
            Self::Rose => "RSE".to_owned(),
            Self::Brown => "BRN".to_owned(),
            Self::Black => "BLK".to_owned(),
            Self::White => "WHT".to_owned(),
            Self::Gray | Self::Grey => "GRY".to_owned(),
            Self::Slate => "SLT".to_owned(),
            Self::Clear => "CLR".to_owned(),
            Self::Cyan => "CYN".to_owned(),
            Self::Aqua => "AQA".to_owned(),
            Self::RGB { abbreviation, .. } => abbreviation.clone(),
        }
    }
    /// Returns a 6 character hex code (RRGGBB) for each color.
    #[must_use]
    #[inline]
    pub fn hex_code(&self) -> String {
        match &self {
            Self::Red => "FF0000".to_owned(),
            Self::Orange => "FF5100".to_owned(),
            Self::Yellow => "FFFF00".to_owned(),
            Self::Green => "00FF00".to_owned(),
            Self::Blue => "0000FF".to_owned(),
            Self::Purple => "6700FF".to_owned(),
            Self::Violet => "EE82EE".to_owned(),
            Self::Pink | Self::Rose => "FFE4E1".to_owned(),
            Self::Brown => "8B4513".to_owned(),
            Self::Black => "000000".to_owned(),
            Self::White | Self::Clear => "FFFFFF".to_owned(),
            Self::Gray | Self::Grey | Self::Slate => "808080".to_owned(),
            Self::Cyan | Self::Aqua => "00FFFF".to_owned(),
            Self::RGB { hex_code, .. } => hex_code.clone(),
        }
    }
}
