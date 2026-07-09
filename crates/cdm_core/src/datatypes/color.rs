use egui::Color32;
use serde::{Deserialize, Serialize};

/// `Color` is RGBA representation of a color, along with some extra metadata.
///
/// The `red`, `green` and `blue` values do not have `alpha` pre-multiplied in them.
///
/// `alpha` of 255 means totally opaque.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[expect(missing_docs, reason = "self documenting variants")]
#[non_exhaustive]
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
    Magenta,
    Brown,
    DarkBrown,
    Black,
    White,
    Gray,
    Grey,
    Slate,
    Clear,
    Cyan,
    Aqua,
    Transparent,
    Custom {
        red: u8,
        green: u8,
        blue: u8,
        alpha: u8,
        abbr: String,
    },
}

impl Color {
    /// Creates a `Color` value from separate red, green, blue and alpha values.
    #[inline]
    #[must_use]
    pub const fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self::Custom {
            red,
            green,
            blue,
            alpha,
            abbr: String::new(),
        }
    }

    /// Returns an RGBA tuple for each variant.
    #[inline]
    #[must_use]
    pub const fn to_rgba(&self) -> (u8, u8, u8, u8) {
        match self {
            //FF0000FF.
            Self::Red => (0xFF, 0x00, 0x00, 0xFF),
            //FFA500FF.
            Self::Orange => (0xFF, 0xA5, 0x00, 0xFF),
            //FFFF00FF.
            Self::Yellow => (0xFF, 0xFF, 0x00, 0xFF),
            //00FF00FF.
            Self::Green => (0x00, 0xFF, 0x00, 0xFF),
            //OOOOFFFF.
            Self::Blue => (0x00, 0x00, 0xFF, 0xFF),
            //800080FF.
            Self::Purple => (0x80, 0x00, 0x80, 0xFF),
            //EE82EEFF.
            Self::Violet => (0xEE, 0x82, 0xEE, 0xFF),
            //FFC0CBFF.
            Self::Pink | Self::Rose => (0xFF, 0xC0, 0xEE, 0xFF),
            //FF00FFFF.
            Self::Magenta => (0xFF, 0x00, 0xFF, 0xFF),
            //A52A2AFF.
            Self::Brown => (0xA5, 0x2A, 0x2A, 0xFF),
            //852A2AFF.
            Self::DarkBrown => (0x85, 0x2A, 0x2A, 0xFF),
            //000000FF.
            Self::Black => (0x00, 0x00, 0x00, 0xFF),
            //FFFFFFFF.
            Self::White => (0xFF, 0xFF, 0xFF, 0xFF),
            //808080FF.
            Self::Gray | Self::Grey | Self::Slate => (0x80, 0x80, 0x80, 0xFF),
            //FFFFFFFF.
            Self::Cyan | Self::Aqua => (0x00, 0xFF, 0xFF, 0xFF),
            //00000000.
            Self::Transparent | Self::Clear => (0x00, 0x00, 0x00, 0x00),
            Self::Custom {
                red, green, blue, alpha, ..
            } => (*red, *green, *blue, *alpha),
        }
    }

    /// Returns a 3 character abbreviation for each color.
    #[inline]
    #[must_use]
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
            Self::Magenta => "MGA".to_owned(),
            Self::Brown | Self::DarkBrown => "BRN".to_owned(),
            Self::Black => "BLK".to_owned(),
            Self::White => "WHT".to_owned(),
            Self::Gray | Self::Grey => "GRY".to_owned(),
            Self::Slate => "SLT".to_owned(),
            Self::Clear => "CLR".to_owned(),
            Self::Cyan => "CYN".to_owned(),
            Self::Aqua => "AQA".to_owned(),
            Self::Transparent => String::new(),
            Self::Custom { abbr, .. } => abbr.clone(),
        }
    }
    /// Returns a 6 character hex code (RRGGBB) for each color.
    #[must_use]
    #[inline]
    pub fn hex_code(&self) -> String {
        format! {"{:02X}{:02X}{:02X}", self.to_rgba().0, self.to_rgba().1, self.to_rgba().2}
    }
}

impl From<Color> for Color32 {
    #[inline]
    fn from(value: Color) -> Self {
        match value {
            Color::Red => Self::RED,
            Color::Orange => Self::ORANGE,
            Color::Yellow => Self::YELLOW,
            Color::Green => Self::GREEN,
            Color::Blue => Self::BLUE,
            Color::Purple => Self::PURPLE,
            Color::Magenta => Self::MAGENTA,
            Color::Brown => Self::BROWN,
            Color::Black => Self::BLACK,
            Color::White => Self::WHITE,
            Color::Cyan | Color::Aqua => Self::CYAN,
            Color::Transparent => Self::TRANSPARENT,
            Color::Violet | Color::Pink | Color::Rose | Color::DarkBrown | Color::Gray | Color::Grey | Color::Slate | Color::Clear => {
                let (red, green, blue, alpha) = value.to_rgba();
                Self::from_rgba_unmultiplied(red, green, blue, alpha)
            }

            Color::Custom {
                red, green, blue, alpha, ..
            } => Self::from_rgba_unmultiplied(red, green, blue, alpha),
        }
    }
}
