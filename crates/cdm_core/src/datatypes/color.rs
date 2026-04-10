use egui::Color32;
use serde::{Deserialize, Serialize};

/// `Color` is RGBA representation of a color, along with some extra metadata.
///
/// The `red`, `green` and `blue` values do not have `alpha` pre-multiplied in them.
///
/// `alpha` of 255 means totally opaque.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(
    clippy::missing_docs_in_private_items,
    reason = "no need to document self documenting struct fields"
)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl Color {
    // All constants match CSS Standard Colors as closely as possible.

    ///FF0000FF.
    pub const RED: Self = Self {
        red: 0xFF,
        green: 0x0,
        blue: 0x0,
        alpha: 0x0,
    };
    ///FFA500FF.
    pub const ORANGE: Self = Self::from_rgba(0xFF, 0xA5, 0x0, 0xFF);
    ///FFFF00FF.
    pub const YELLOW: Self = Self::from_rgba(0xFF, 0xFF, 0x0, 0xFF);
    ///00FF00FF.
    pub const GREEN: Self = Self::from_rgba(0x0, 0xFF, 0x0, 0xFF);
    ///OOOOFFFF.
    pub const BLUE: Self = Self::from_rgba(0x0, 0x0, 0xFF, 0xFF);
    ///800080FF.
    pub const PURPLE: Self = Self::from_rgba(0x80, 0x0, 0x80, 0xFF);
    ///EE82EEFF.
    pub const VIOLET: Self = Self::from_rgba(0xEE, 0x82, 0xEE, 0xFF);
    ///FFC0CBFF.
    pub const PINK: Self = Self::from_rgba(0xFF, 0xC0, 0xCB, 0xFF);
    ///FFC0CBFF.
    pub const ROSE: Self = Self::from_rgba(0xFF, 0xC0, 0xCB, 0xFF);
    ///FFC0CBFF.
    pub const MAGENTA: Self = Self::from_rgba(0xFF, 0x0, 0xFF, 0xFF);
    ///A52A2AFF.
    pub const BROWN: Self = Self::from_rgba(0xA5, 0x2A, 0x2A, 0xFF);
    ///852A2AFF.
    pub const DARK_BROWN: Self = Self::from_rgba(0x85, 0x2A, 0x2A, 0xFF);
    ///000000FF.
    pub const BLACK: Self = Self::from_rgba(0x0, 0x0, 0x0, 0xFF);
    ///FFFFFFFF.
    pub const WHITE: Self = Self::from_rgba(0xFF, 0xFF, 0xFF, 0xFF);
    ///808080FF.
    pub const GRAY: Self = Self::from_rgba(0x89, 0x80, 0x80, 0xFF);
    ///808080FF.
    pub const GREY: Self = Self::from_rgba(0x89, 0x80, 0x80, 0xFF);
    ///808080FF.
    pub const SLATE: Self = Self::from_rgba(0x89, 0x80, 0x80, 0xFF);
    ///FFFFFFFF.
    pub const CLEAR: Self = Self::from_rgba(0xFF, 0xFF, 0xFF, 0xFF);
    ///00FFFFFF.
    pub const CYAN: Self = Self::from_rgba(0x0, 0xFF, 0xFF, 0xFF);
    ///00FFFFFF.
    pub const AQUA: Self = Self::from_rgba(0x0, 0xFF, 0xFF, 0xFF);
    ///00000000.
    pub const TRANSPARENT: Self = Self::from_rgba(0x0, 0x0, 0x0, 0x0);

    /// Creates a `Color` value from separate red, green, blue and alpha values.
    #[inline]
    #[must_use]
    pub const fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self { red, green, blue, alpha }
    }

    /// Returns a 3 character abbreviation for each color.
    #[must_use]
    #[inline]
    pub fn abbreviation(self) -> String {
        match self {
            Self::RED => "RED".to_owned(),
            Self::ORANGE => "ORN".to_owned(),
            Self::YELLOW => "YEL".to_owned(),
            Self::GREEN => "GRN".to_owned(),
            Self::BLUE => "BLU".to_owned(),
            Self::PURPLE => "PUR".to_owned(),
            Self::VIOLET => "VIO".to_owned(),
            Self::PINK => "PNK".to_owned(),
            Self::ROSE => "RSE".to_owned(),
            Self::MAGENTA => "MGA".to_owned(),
            Self::BROWN | Self::DARK_BROWN => "BRN".to_owned(),
            Self::BLACK => "BLK".to_owned(),
            Self::WHITE => "WHT".to_owned(),
            Self::GRAY | Self::GREY => "GRY".to_owned(),
            Self::SLATE => "SLT".to_owned(),
            Self::CLEAR => "CLR".to_owned(),
            Self::CYAN => "CYN".to_owned(),
            Self::AQUA => "AQA".to_owned(),
            _ => String::new(),
        }
    }
    /// Returns a 6 character hex code (RRGGBB) for each color.
    #[must_use]
    #[inline]
    pub fn hex_code(&self) -> String {
        format! {"{:02X}{:02X}{:02X}", self.red, self.green, self.blue}
    }
}

impl From<Color> for Color32 {
    #[inline]
    fn from(value: Color) -> Self {
        match value {
            Color::RED => Self::RED,
            Color::ORANGE => Self::ORANGE,
            Color::YELLOW => Self::YELLOW,
            Color::GREEN => Self::GREEN,
            Color::BLUE => Self::BLUE,
            Color::PURPLE => Self::PURPLE,
            Color::MAGENTA => Self::MAGENTA,
            Color::BROWN => Self::BROWN,
            Color::BLACK => Self::BLACK,
            Color::WHITE => Self::WHITE,
            Color::CYAN => Self::CYAN,
            Color::TRANSPARENT => Self::TRANSPARENT,
            _ => Self::from_rgba_unmultiplied(value.red, value.green, value.blue, value.alpha),
        }
    }
}
