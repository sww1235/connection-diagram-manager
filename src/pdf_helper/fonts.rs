use std::collections::HashMap;
use std::io;

use serde::{Deserialize, Serialize};

/// `PDFFont` represents a particular font
pub struct PDFFont {
    /// Font Subtype
    pub subtype: PDFFontSubType,
    /// Base font name
    pub base_font: String,
    /// Font internal PDF resource name
    pub font_resource_name: String,
    /// Font Metric Data
    pub font_metrics: FontMetrics,
}
/// `PDFFontSubType` is the list of permissible font subtypes in a pdf document
/// Reference the PDF reference for more details
#[derive(Hash, PartialEq, Eq)]
pub enum PDFFontSubType {
    /// A composite font that is composed of glyphs from a decendant CIDFont
    Type0,
    /// A font that defines glyphs using Type1 font technology. These are normally simple fonts.
    Type1,
    /// A Multiple Master font, extension of Type1
    MMType1,
    /// A font that defines glyphs with streams of PDF graphics operators
    Type3,
    /// A TrueType font
    TrueType,
    /// A CIDFont whose glyph descriptions are based on Type1 font technology
    CIDFontType0,
    /// A CIDfont whose glyph descriptions are based on TrueType font technology
    CIDFontType2,
}

/// `FontMetric` contains font metric data for a specific letter.
#[derive(Serialize, Deserialize, Default, Debug, Hash, PartialEq, Eq)]
pub struct FontMetric {
    /// character code
    #[serde(rename = "charCode")]
    pub char_code: i64,
    /// glyph width
    pub width: u64,
    /// character name
    pub name: String,
}
/// `FontMetricsVec` contains font metric data for a particular font.
/// It is an intermediary type to make the serialization easier.
type FontMetricsVec = Vec<FontMetric>;

/// `Font Metrics contains font metric data for a particular font.
/// It is a hashmap for easier character lookup.
pub type FontMetrics = HashMap<i64, FontMetric>;

//TODO: look into actually parsing the AFM files rather than json. allows for shortcutting
//textwidth calcs if we know the text is monospaced.
//TODO: return error
pub(super) fn load_std_fonts() -> io::Result<HashMap<String, PDFFont>> {
    let mut std_font_strings: HashMap<&str, String> = HashMap::new();
    let mut output_fonts: HashMap<String, PDFFont> = HashMap::new();

    // because include_str! is not cross platform aware,
    // have all font files in same directory

    std_font_strings.insert("Courier", include_str!("Courier.json").to_string());
    std_font_strings.insert(
        "Courier-Bold",
        include_str!("Courier-Bold.json").to_string(),
    );
    std_font_strings.insert(
        "Courier-Oblique",
        include_str!("Courier-Oblique.json").to_string(),
    );
    std_font_strings.insert(
        "Courier-BoldOblique",
        include_str!("Courier-BoldOblique.json").to_string(),
    );
    std_font_strings.insert("Helvetica", include_str!("Helvetica.json").to_string());
    std_font_strings.insert(
        "Helvetica-Bold",
        include_str!("Helvetica-Bold.json").to_string(),
    );
    std_font_strings.insert(
        "Helvetica-Oblique",
        include_str!("Helvetica-Oblique.json").to_string(),
    );
    std_font_strings.insert(
        "Helvetica-BoldOblique",
        include_str!("Helvetica-BoldOblique.json").to_string(),
    );
    std_font_strings.insert("Times-Roman", include_str!("Times-Roman.json").to_string());
    std_font_strings.insert("Times-Bold", include_str!("Times-Bold.json").to_string());
    std_font_strings.insert(
        "Times-Italic",
        include_str!("Times-Italic.json").to_string(),
    );
    std_font_strings.insert(
        "Times-BoldItalic",
        include_str!("Times-BoldItalic.json").to_string(),
    );
    std_font_strings.insert("Symbol", include_str!("Symbol.json").to_string());
    std_font_strings.insert(
        "ZapfDingbats",
        include_str!("ZapfDingbats.json").to_string(),
    );

    for (font_resource_number, (font_name, font_metric_string)) in
        std_font_strings.into_iter().enumerate()
    {
        let font_resource_name = "F".to_string() + font_resource_number.to_string().as_str();
        output_fonts.insert(
            font_resource_name,
            PDFFont {
                subtype: PDFFontSubType::Type1,
                base_font: font_name.to_string(),
                font_resource_name,
                font_metrics: {
                    let output: HashMap<i64, FontMetric> = HashMap::new();

                    let font_vec: FontMetricsVec = serde_json::from_str(&font_metric_string)?;

                    for glyph in font_vec {
                        output.insert(glyph.char_code, glyph);
                    }
                    output
                },
            },
        );
    }

    Ok(output_fonts)
}
