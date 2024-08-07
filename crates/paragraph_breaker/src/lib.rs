//https://github.com/jaroslov/knuth-plass-thoughts/blob/master/plass.md

use dimensioned::{ucum, MapUnsafe};

use log::trace;

#[derive(Debug, Default)]
/// `ParagraphWord` represents one word in a paragraph
struct ParagraphWord {
    /// index of first character in word
    first: Option<usize>,
    /// index of last character in word
    last: Option<usize>,
    /// index of next word
    next: Option<usize>,
    /// word breaking score
    score: Option<ucum::Meter<f64>>,
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::arithmetic_side_effects)]
/// A simplified implementation of the Knuth-Plass algorithm, as found
/// [here](https://github.com/jaroslov/knuth-plass-thoughts/blob/master/plass.cpp)
/// and converted into rust.
///
/// # Errors
///
/// Can error due to failure to split words into lines
pub fn to_lines(
    text: &str,
    font_data: &rustybuzz::Face,
    font_size: u32,
    textbox_width: ucum::Meter<f64>,
    text_direction: rustybuzz::Direction,
    text_language: rustybuzz::Language,
) -> Result<(Vec<String>, rustybuzz::GlyphBuffer), Error> {
    if text.is_empty() {
        return Err(Error::EmptyString);
    }
    let mut buffer = rustybuzz::UnicodeBuffer::new();
    buffer.push_str(text);

    buffer.set_direction(text_direction);
    buffer.set_language(text_language.clone());
    // see harfbuzz documentation for what this means
    buffer.set_cluster_level(rustybuzz::BufferClusterLevel::MonotoneCharacters);
    trace! {"{:?}", &buffer};
    // see harfbuzz documentation for what this means
    let features: Vec<rustybuzz::Feature> = Vec::new();

    let glyph_buffer = rustybuzz::shape(font_data, &features, buffer);
    if glyph_buffer.is_empty() {
        return Err(Error::EmptyString);
    }
    // calculate approx space width
    let mut space_buffer = rustybuzz::UnicodeBuffer::new();
    space_buffer.push_str(" ");
    space_buffer.set_direction(text_direction);
    space_buffer.set_language(text_language);
    // see harfbuzz documentation for what this means
    space_buffer.set_cluster_level(rustybuzz::BufferClusterLevel::MonotoneCharacters);
    let space_buffer = rustybuzz::shape(font_data, &features, space_buffer);
    if space_buffer.is_empty() {
        return Err(Error::SpaceFailedToShape);
    }

    // https://github.com/RazrFalcon/rustybuzz/issues/33#issuecomment-784716703
    let units_per_em: u32 = u32::try_from(font_data.units_per_em()).unwrap_or(u32::MIN);
    trace! {"units_per_em: {}", units_per_em};

    let point = (1.0_f64 / 72.0_f64) * ucum::IN_US;
    //https://stackoverflow.com/a/68387730/3342767
    let em_width = f64::from(font_size) * point;

    let space_width =
        f64::from(space_buffer.glyph_positions()[0].x_advance) * em_width / f64::from(units_per_em);

    let mut words = text_to_words(text)?;

    let max_width = textbox_width;
    let ideal_width = textbox_width - (textbox_width * 0.1_f64);

    line_break_internal(
        &mut words,
        &glyph_buffer,
        em_width,
        space_width,
        units_per_em,
        0,
        ideal_width,
        max_width,
    )?;
    Ok((to_lines_internal(&words, text), glyph_buffer))
}
#[allow(clippy::too_many_arguments)]
#[allow(clippy::arithmetic_side_effects)]
#[allow(clippy::shadow_unrelated)]
/// `lineBreakInternal` scores each `ParagraphWord` for breaking possibilities
fn line_break_internal(
    words: &mut [ParagraphWord],
    shaped_text: &rustybuzz::GlyphBuffer,
    em_width: ucum::Meter<f64>,
    space_width: ucum::Meter<f64>,
    units_per_em: u32,
    current_word_index: usize,
    ideal_width: ucum::Meter<f64>,
    max_width: ucum::Meter<f64>,
) -> Result<(), Error> {
    let glyph_infos = shaped_text.glyph_infos();
    let glyph_positions = shaped_text.glyph_positions();

    let mut next_word_index = current_word_index + 1;

    let mut line_length = 0.0_f64 * ucum::M; // any unit will do here
    let first = words[current_word_index].first.unwrap_or(0);
    let last = words[current_word_index].last.unwrap_or(0);
    // current line length is length of first word.
    for glyph in &glyph_positions[first..last] {
        line_length += f64::from(glyph.x_advance) * em_width / f64::from(units_per_em);
    }
    // the best score is current line length, squared
    let mut best_score = ideal_width - line_length;
    // I think this works?
    best_score = best_score.map_unsafe(|v| v * v);
    // best tail is current break
    let mut best_tail = next_word_index;

    // scan down word list looking for better entries
    while next_word_index < words.len() {
        // get width of new potential word
        let mut word_width = 0.0_f64 * ucum::M; // any unit will do here

        let first = words[next_word_index].first.unwrap_or(0);
        let last = words[next_word_index].last.unwrap_or(0);
        for glyph in &glyph_positions[first..last] {
            word_width += f64::from(glyph.x_advance) * em_width / f64::from(units_per_em);
        }
        // if the new word will make the line too long, stop
        if (line_length + word_width) >= max_width {
            break;
        }
        // compute a new line length score
        let mut line_score = ideal_width - (line_length + word_width);
        // I think this works?
        line_score = line_score.map_unsafe(|v| v * v);
        // add a word and a space to the current line
        line_length += word_width + space_width; // need to figure out space width

        // if we haven't solved the subproblem at this i
        // potential line break, go ahead and do so now.
        if words[next_word_index].score.is_none() {
            line_break_internal(
                words,
                shaped_text,
                em_width,
                space_width,
                units_per_em,
                next_word_index,
                ideal_width,
                max_width,
            )?;
        }

        // is this new line_score better than current best_score
        if (line_score + words[next_word_index].score.unwrap_or(0.0_f64 * ucum::M)) < best_score {
            // update to this new score
            best_score = line_score + words[next_word_index].score.unwrap_or(0.0_f64 * ucum::M);
            // track the new tail
            best_tail = next_word_index;
        }
        // Look at the next word
        next_word_index += 1;
    }
    // write down the best score and tail for this subproblem
    words[current_word_index].score = Some(best_score);
    words[current_word_index].next = Some(best_tail);

    // the last word of the paragraph doesn't contribute to the score
    if (words[current_word_index].next.unwrap_or(0) + 1) == words.len() {
        words[current_word_index].score = Some(0.0_f64 * ucum::M);
    }
    Ok(())
}

#[allow(clippy::arithmetic_side_effects)]
/// `text_to_words` splits a utf8 string into an array of [`ParagraphWord`]s
fn text_to_words(text: &str) -> Result<Vec<ParagraphWord>, Error> {
    let mut index = 0;
    let mut words = Vec::new();

    while index < text.len() {
        // find next (first) non whitespace char index in string
        while index < text.len()
            && text
                .chars()
                .nth(index)
                .ok_or(Error::CharacterNotFound)?
                .is_whitespace()
        {
            index += 1;
        }
        let start = index;
        // find ending index of word
        while index < text.len()
            && !text
                .chars()
                .nth(index)
                .ok_or(Error::CharacterNotFound)?
                .is_whitespace()
        {
            index += 1;
        }
        if start < index {
            words.push(ParagraphWord {
                first: Some(start),
                last: Some(index),
                next: None,
                score: None,
            });
        }
    }
    words.push(ParagraphWord {
        first: None,
        last: None,
        next: None,
        score: Some(0.0_f64 * ucum::M),
    });
    Ok(words)
}
#[allow(clippy::similar_names)]
#[allow(clippy::arithmetic_side_effects)]
/// `toLines` takes in a vector of `ParagraphWord`s and converts them to a vector of strings
/// of the correct lengths.
fn to_lines_internal(words: &[ParagraphWord], text: &str) -> Vec<String> {
    let mut index = 0;
    let mut output = Vec::new();

    while index < words.len() {
        let next = words[index].next.unwrap_or(0);
        let mut line = String::new();

        {
            let internal_index = index;
            while (internal_index <= next) && (internal_index + 1 < words.len()) {
                let first = match words[internal_index].first {
                    None => break,
                    Some(first) => first,
                };
                let last = match words[internal_index].last {
                    None => break,
                    Some(last) => last,
                };
                if last - first == 0 {
                    break;
                }
                if internal_index == index {
                    line += "";
                } else {
                    line += " ";
                }
                //TODO: add in check with glyph_info to see if it doesn't like breaking there.
                line += &text[words[internal_index].first.unwrap_or(0)
                    ..words[internal_index].last.unwrap_or(0)];
            }
            output.push(line);
            index = words[index].next.unwrap_or(0);
        }
    }
    output
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::arithmetic_side_effects)]
/// `greedy_break`
fn greedy_break(
    words: &mut [ParagraphWord],
    shaped_text: &rustybuzz::GlyphBuffer,
    em_width: ucum::Meter<f64>,
    space_width: ucum::Meter<f64>,
    units_per_em: u16,
    ideal_width: ucum::Meter<f64>,
    max_width: ucum::Meter<f64>,
) -> Result<(), Error> {
    let glyph_positions = shaped_text.glyph_positions();

    let mut line_length = 0.0_f64 * ucum::M; // any unit will do here

    let mut line_next = 0;

    let mut internal_index = 0;

    while internal_index < words.len() {
        let mut word_width = 0.0_f64 * ucum::M;
        let first = words[internal_index].first.unwrap_or(0);
        let last = words[internal_index].last.unwrap_or(0);
        for glyph in &glyph_positions[first..last] {
            word_width += f64::from(glyph.x_advance) * em_width / f64::from(units_per_em);
        }
        if (line_length + word_width + space_width) >= ideal_width {
            words[line_next].next = Some(internal_index - 1);
            line_next = internal_index - 1;
            line_length = 0.0_f64 * ucum::M;
        }
        line_length += word_width + space_width; // This should work
        if line_length < max_width {
            return Err(Error::LineLengthTooLong);
        }

        internal_index += 1;
    }
    words[line_next].next = Some(words.len() + 1);
    words[words.len()].next = Some(words.len() + 1);
    Ok(())
}

#[derive(Debug)]
#[non_exhaustive]
/// list of errors for this library
pub enum Error {
    /// Line length too long
    LineLengthTooLong,
    /// Character not found
    CharacterNotFound,
    /// Empty string
    EmptyString,
    /// Space character failed to shape
    SpaceFailedToShape,
}
impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::LineLengthTooLong => write!(f, "Line Length is too long"),
            Error::CharacterNotFound => write!(f, "Character Not Found"),
            Error::EmptyString => write!(f, "Empty String"),
            Error::SpaceFailedToShape => write!(f, "Space Failed To Shape"),
        }
    }
}
