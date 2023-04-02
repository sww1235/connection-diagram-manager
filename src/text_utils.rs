// Heavily inspired by: https://github.com/fizzbucket/bookbinder/blob/d0c5773e1716987b7f454bc346317cae40e7bf4c/bookbinder_epub/src/text_splitting.rs

use super::pdf_helper::fonts::{FontMetric, FontMetrics};

use paragraph_breaker::{total_fit, Item};

use unicode_normalization::UnicodeNormalization;

use dimensioned::{f64prefixes::MILLI, ucum, Dimensionless};

/// `size_text` returns the text width of a string based on the provided font metrics.
/// `point_size` is whatever units the font is specified in.
/// For example, in pdf files, user space is in units of 72 points: 1 inch,
/// so 1 userspace unit is 1/72 inch. This would result in a `point_size` value of
/// (1.0 / (72.0 * 1000.0)) * ucum::IN_US
fn size_text(
    text: String,
    font_data: FontMetrics,
    font_size: u32,
    point_size: ucum::Meter<f64>,
) -> ucum::Meter<f64> {
    //https://www.reddit.com/r/rust/comments/dmy9rz/how_convert_unicode_accents_convert_%C3%A9_to_e_and/
    let simple: String = text
        // get rid of beginning and ending whitespace
        .trim()
        // splits unicode characters into normalized form canonical decomposition
        // this means that the base letter and diacritical marks, etc are split up into 2 separate
        // unicode code points.
        .nfd()
        // filter out extra crap on letters
        .filter(|x: &char| {
            !((*x >= '\u{0300}' && *x <= '\u{036F}') // combining diacritical marks
                || (*x >= '\u{1AB0}' && *x <= '\u{1AFF}') // combining diacritical marks extended
                || (*x >= '\u{1DC0}' && *x <= '\u{1DFF}') // combining diacritical marks supplement
                || (*x >= '\u{FE20}' && *x <= '\u{FE2F}')) // combining half marks
        })
        .collect();

    let mut chars_width = 0.0 * ucum::IN_US;
    // in theory, simple now only consists of UTF-8 single characters,
    // so can use str.chars() instead of fancy unicode parsing
    for grapheme in simple.as_str().chars() {
        if font_data.contains_key(&(grapheme as i64)) {
            chars_width +=
                font_data[&(grapheme as i64)].width as f64 * (1.0 / (72.0 * 1000.0)) * ucum::IN_US;
        } else {
            //TODO: return error here
            panic! {concat!{"grapheme {} not found in font {}. ",
            "Please select an alternate font with better ",
            "unicode support."}, grapheme, font.base_font}
        }
    }
    chars_width
}

pub fn to_lines(
    text: String,
    font_data: FontMetrics,
    font_size: u32,
    textbox_width: ucum::Meter<f64>,
) -> Vec<String> {
    let mut lines = Vec::new();
    let text_width = size_text(text, font_data, font_size);
    let mm_width = *(text_width * MILLI / (ucum::M)).value() as i32;

    let item = [Item::Box {
        //https://github.com/paholg/dimensioned/issues/35#issuecomment-363774108
        width: mm_width,
        data: text,
    }];

    let threshold = 8.0;
    let looseness = 0;
    let breakpoints = total_fit(&item, &[mm_width], threshold, looseness);

    lines
}

fn greedy_fallback(
    text: String,
    font_data: FontMetrics,
    font_size: u32,
    text_width: ucum::Meter<f64>,
) -> Vec<String> {
    let mut lines = Vec::new();

    let mut current_line = Vec::new();

    let mut current_line_width = 0.0 * ucum::M; // any unit will do here
}
