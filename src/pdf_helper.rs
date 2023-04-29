/// `paper` contains information about various
/// physical paper sizes.
pub mod paper;

use lopdf::content::{Content, Operation};
use lopdf::dictionary;
use lopdf::{Document, Object, Stream};

use dimensioned::ucum;

use log::warn;

use std::path::PathBuf;
use std::{io, str};

/// `PDFDocument` is a helper type to properly generate PDFs
/// It allows for easier tracking of default page size, available fonts
/// and allows for easier implementation of helper methods.
pub struct PDFDocument<'a> {
    /// default page size for PDF
    default_page_size: paper::PaperSize,
    /// Hashmap of fonts available to be used in a pdf. String in tuple provides easy access to
    /// full font name
    available_fonts: Vec<(String, rustybuzz::Face<'a>)>,
    /// all the pages in the PDF document
    pages: Vec<PDFPage>,
}
/// `PDFPage` represents an individual page of a pdf file
struct PDFPage {
    operations: Vec<Operation>,
    page_size: Option<paper::PaperSize>,
    left_margin: ucum::Meter<f64>,
    right_margin: ucum::Meter<f64>,
    top_margin: ucum::Meter<f64>,
    bottom_margin: ucum::Meter<f64>,
}

impl PDFDocument<'_> {
    /// `new` returns a set up and initialized `PDFDocument`
    /// with the specified font paths pre-populated and default page size,
    /// as specified via parameter
    pub fn new(default_page_size: paper::PaperSize, font_paths: Vec<PathBuf>) -> io::Result<Self> {
        let mut output = Self {
            default_page_size,
            available_fonts: Vec::new(),
            pages: Vec::new(),
        };
        output.load_cfg_fonts(font_paths)?;
        Ok(output)
    }

    /// `empty` returns a mostly empty `PDFDocument`. It does not include default fonts
    /// and has the default_page_size set to A4.
    pub fn empty() -> Self {
        Self {
            default_page_size: paper::PaperSize::A4,
            available_fonts: Vec::new(),
            pages: Vec::new(),
        }
    }

    /// `load_ttf_font` parses the specified TTF font via `rustybuzz` and `ttf_parser` and makes it
    /// available in the specified PDF document.
    pub fn load_ttf_font(&mut self, font_file: PathBuf, font_index: Option<u32>) -> io::Result<()> {
        use rustybuzz::Face;

        let font_data = std::fs::read(font_file)?;
        let font_data = font_data.to_owned().leak();
        //TODO: maybe error out if font_file is a collection
        let face_index = font_index.unwrap_or(0);

        let face = Face::from_slice(font_data, face_index).ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "font failed to parse",
        ))?;

        // 0 indexed, 4 is the table row that contains the full name of the font
        // https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6name.html
        // the .get function returns a byte array which needs to be converted into a string and
        // then processed.
        let font_name_data = face.names().get(4).ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "font failed to parse",
        ))?;
        let font_name = String::from_utf8_lossy(font_name_data.name)
            .to_mut()
            .replace(char::REPLACEMENT_CHARACTER, "");
        self.available_fonts.push((font_name, face));

        Ok(())
    }

    //TODO: need to add this to the config file
    /// `load_cfg_font` loads the TTF font specified in the configuration file
    /// via `rustybuzz` and `ttf_parser` and makes it
    /// available in the specified PDF document.
    pub fn load_cfg_fonts(&mut self, font_paths: Vec<PathBuf>) -> io::Result<()> {
        if font_paths.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "No Fonts specified in configuration file",
            ));
        }
        for path in font_paths {
            // load default font_index
            self.load_ttf_font(path, Some(0))?;
        }
        Ok(())
    }

    /// `add_text` writes text into a page of a pdf at a specified position.
    /// In order to make this easier to implement for my use cases, for now,
    /// this only handles ascii text.
    ///
    /// # Arguments
    ///
    /// * `page_index`: The zero indexed page number that the text should be inserted into.
    /// * `text`: the text that will be added to the pdf page
    /// * `font`: a `PDFFont` object representing a font defined for use in a PDF document
    /// * `font_size`: Font size specified in points. This is used internally as a multiple
    /// of the standard PDF user space unit size 1/72 inch
    /// * `line_spacing`: text line spacing in multiples of line height
    /// * `text width`: An optional parameter that defines the width of the text element. If
    /// `None`, the width of the page minus the margins is used.
    /// * `x_pos`: horizontal starting position of text insertion, with 0 on left side of page
    /// * `y_pos`: vertical starting position of text insertion, with 0 on the bottom side of page
    #[allow(clippy::too_many_arguments)]
    pub fn add_text(
        &mut self,
        page_index: usize,
        text: &str,
        font_index: usize,
        font_size: u32,
        line_spacing: u32,
        text_width: ucum::Meter<f64>,
        x_pos: ucum::Meter<f64>,
        y_pos: ucum::Meter<f64>,
        text_direction: rustybuzz::Direction,
        text_language: rustybuzz::Language,
    ) -> io::Result<()> {
        use super::paragraph_breaking::to_lines;
        let (lines, glyphs) = to_lines(
            text,
            &self.available_fonts[font_index].1,
            font_size,
            text_width,
            text_direction,
            text_language,
        )?;

        let num_lines = lines.len();

        // then check to see if text starts inside page boundaries.
        let current_page_size = self.pages[page_index]
            .page_size
            .unwrap_or(self.default_page_size)
            .size();

        if x_pos > current_page_size.0
            || x_pos < 0.0 * ucum::IN_US
            || y_pos > current_page_size.1
            || y_pos < 0.0 * ucum::IN_US
        {
            panic! {concat!{"Position of text X: {}, Y: {}, ",
            "is outside page boundaries. Please fix this"},
            current_page_size.0, current_page_size.1}
        }
        if x_pos > current_page_size.0 - self.pages[page_index].right_margin
            || x_pos < 0.0 * ucum::IN_US + self.pages[page_index].left_margin
            || y_pos > current_page_size.1 - self.pages[page_index].top_margin
            || y_pos < 0.0 * ucum::IN_US + self.pages[page_index].bottom_margin
        {
            warn! {concat!{"Position of text X: {}, Y: {}, ",
            "is outside page margin boundaries. ",
            "Please fix this if unintentional"},
            current_page_size.0, current_page_size.1}
        }

        // BT begins a text element. it takes no operands
        self.pages[page_index]
            .operations
            .push(Operation::new("BT", vec![]));
        // Tf specifies the font and font size. Font scaling is complicated in PDFs. Reference
        // the reference for more info.
        // The into() methods are defined based on their paired .from() methods (this
        // functionality is built into rust), and are converting the provided values into
        // An enum that represents the basic object types in PDF documents.
        self.pages[page_index].operations.push(Operation::new(
            "Tf",
            vec![
                self.available_fonts[font_index].0.clone().into(),
                font_size.into(),
            ],
        ));
        // Td adjusts the translation components of the text matrix. When used for the first
        // time after BT, it sets the initial text position on the page.
        // Note: PDF documents have Y=0 at the bottom. Thus 600 to print text near the top.
        self.pages[page_index].operations.push(Operation::new(
            "Td",
            vec![x_pos.value_unsafe.into(), y_pos.value_unsafe.into()],
        ));
        if num_lines == 1 {
            // Tj prints a string literal to the page. By default, this is black text that is
            // filled in. There are other operators that can produce various textual effects and
            // colors
            self.pages[page_index]
                .operations
                .push(Operation::new("Tj", vec![Object::string_literal(text)]));
        } else {
            // push first line
            self.pages[page_index].operations.push(Operation::new(
                "Tj",
                vec![Object::string_literal(lines[1].as_str())],
            ));
            // push remaining lines
            for line in &lines[2..] {
                self.pages[page_index].operations.push(Operation::new(
                    "Tj",
                    vec![Object::string_literal(line.as_str())],
                ));
            }
        }
        // ET ends the text element
        self.pages[page_index]
            .operations
            .push(Operation::new("ET", vec![]));
        Ok(())
    }

    /// `insert_page` inserts an empty PDFPage into self.pages vector
    /// at the specified page index which is zero indexed.
    /// This is a wrapper around vec.insert() so it follows the same rules.
    pub fn insert_page(
        &mut self,
        page_index: usize,
        page_size: Option<paper::PaperSize>,
        left_margin: ucum::Meter<f64>,
        right_margin: ucum::Meter<f64>,
        top_margin: ucum::Meter<f64>,
        bottom_margin: ucum::Meter<f64>,
    ) {
        self.pages.insert(
            page_index,
            PDFPage {
                operations: Vec::new(),
                page_size,
                left_margin,
                right_margin,
                top_margin,
                bottom_margin,
            },
        );
    }
    /// `push_page` inserts an empty PDFPage into self.pages vector
    /// at the end of the vector.
    /// This is a wrapper around vec.push() so it follows the same rules.
    pub fn push_page(
        &mut self,
        page_size: Option<paper::PaperSize>,
        left_margin: ucum::Meter<f64>,
        right_margin: ucum::Meter<f64>,
        top_margin: ucum::Meter<f64>,
        bottom_margin: ucum::Meter<f64>,
    ) {
        self.pages.push(PDFPage {
            operations: Vec::new(),
            page_size,
            left_margin,
            right_margin,
            top_margin,
            bottom_margin,
        });
    }
    /// `write` sets up and writes a pdf file.
    pub fn write(&mut self, out_path: PathBuf, file_name: PathBuf) -> io::Result<()> {
        let mut doc = Document::with_version("1.7");

        // Object IDs are used for cross referencing in PDF documents. `lopdf` helps keep track of them
        // for us. They are simple integers.
        // Calls to `doc.new_object_id` and `doc.add_object` return an object id

        // pages is the root node of the page tree
        let pages_id = doc.new_object_id();

        // fonts are dictionaries. The type, subtype and basefont tags
        // are straight out of the PDF reference manual
        //
        // The dictionary macro is a helper that allows complex
        // key, value relationships to be represented in a simpler
        // visual manner, similar to a match statement.
        // Dictionary is linkedHashMap of byte vector, and object

        let font_id = doc.add_object(dictionary! {
            // type of dictionary
            "Type" => "Font",
            // type of font, type1 is simple postscript font
            "Subtype" => "Type1",
            // basefont is postscript name of font for type1 font.
            // See PDF reference document for more details
            "BaseFont" => "Courier",

        });

        // font dictionaries need to be added into resource dictionaries
        // in order to be used.
        // Resource dictionaries can contain more than just fonts,
        // but normally just contains fonts
        // Only one resource dictionary is allowed per page tree root
        let resources_id = doc.add_object(dictionary! {
            // fonts are actually triplely nested dictionaries. Fun!
            "Font" => dictionary! {
                // F1 is the font name used when writing text.
                // It must be unique in the document. It does not
                // have to be F1
                "F1" => font_id,
            }
        });

        let mut page_ids: Vec<Object> = Vec::new();
        // page is a vector of operations.
        for page in &self.pages {
            // Content is a wrapper struct around an operations struct that contains a vector of operations
            // The operations struct contains a vector of operations that match up with a particular PDF
            // operator and operands.
            // Reference the PDF reference for more details on these operators and operands.
            // Note, the operators and operands are specified in a reverse order than they
            // actually appear in the PDF file itself.
            let content = Content {
                operations: page.operations.clone(),
            };

            // Streams are a dictionary followed by a sequence of bytes. What that sequence of bytes
            // represents depends on context
            // The stream dictionary is set internally to lopdf and normally doesn't
            // need to be manually nanipulated. It contains keys such as
            // Length, Filter, DecodeParams, etc
            //
            // content is a stream of encoded content data.
            let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));

            // Page is a dictionary that represents one page of a PDF file.
            // It has a type, parent and contents
            let page_id = doc.add_object(dictionary! {
                "Type" => "Page",
                "Parent" => pages_id,
                "Contents" => content_id,
            });
            page_ids.push(page_id.into());
        }

        // Again, pages is the root of the page tree. The ID was already created
        // at the top of the page, since we needed it to assign to the parent element of the page
        // dictionary
        //
        // This is just the basic requirements for a page tree root object. There are also many
        // additional entries that can be added to the dictionary if needed. Some of these can also be
        // defined on the page dictionary itself, and not inherited from the page tree root.
        let pages = dictionary! {
            // Type of dictionary
            "Type" => "Pages",
            // Vector of page IDs in document. Normally would contain more than one ID and be produced
            // using a loop of some kind
            "Kids" => page_ids,
            // Page count
            "Count" => 1,
            // ID of resources, defined earlier
            "Resources" => resources_id,
            // a rectangle that defines the boundaries of the physical or digital media. This is the
            // "Page Size"
            "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
        };

        // using insert() here, instead of add_object() since the id is already known.
        doc.objects.insert(pages_id, Object::Dictionary(pages));

        // Creating document catalog.
        // There are many more entries allowed in the catalog dictionary.
        let doc_catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });

        // Root key in trailer is set here to ID of document catalog,
        // remainder of trailer is set during doc.save().
        doc.trailer.set("Root", doc_catalog_id);

        let file_path = out_path.join(file_name);
        //TODO: add flag for pdf compression
        doc.compress();
        doc.save(file_path)?;
        Ok(())
    }
}
