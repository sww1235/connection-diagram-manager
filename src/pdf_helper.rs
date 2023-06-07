/// `paper` contains information about various
/// physical paper sizes.
pub mod paper;

use lopdf::content::{Content, Operation};
use lopdf::dictionary;
use lopdf::{Document, Object, Stream};

use dimensioned::{ucum, Dimensionless};

use log::warn;

use std::path::PathBuf;
use std::{io, str};

/// `PDFDocument` is a helper type to properly generate PDFs
/// It allows for easier tracking of default page size, available fonts
/// and allows for easier implementation of helper methods.
pub struct PDFDocument<'a> {
    /// default page size for PDF
    default_page_size: paper::PaperSize,
    /// Vector of fonts available to be used in a pdf.
    available_fonts: Vec<PDFFont<'a>>,
    /// all the pages in the PDF document
    pages: Vec<PDFPage>,
}
/// `PDFFont` contains information about a font.
pub struct PDFFont<'a> {
    /// full font name
    font_name: String,
    /// internal PDF font ID string
    font_id_str: String,
    /// internal pdf font object id
    font_id: Option<lopdf::ObjectId>,
    /// font data
    font_face: rustybuzz::Face<'a>,
    /// raw_font_data
    font_data: Vec<u8>,
}
/// `PDFPage` represents an individual page of a pdf file
pub struct PDFPage {
    operations: Vec<Operation>,
    page_size: paper::PaperSize,
    left_margin: ucum::Meter<f64>,
    right_margin: ucum::Meter<f64>,
    top_margin: ucum::Meter<f64>,
    bottom_margin: ucum::Meter<f64>,
}
/// `PDFTextRenderMode` is an enumeration
/// of defined text rendering modes in pdf documents
pub enum PDFTextRenderMode {
    /// Normal Text, colored with current non-stroking color
    Fill,
    /// Outline text, outlined with current stroking color
    Stroke,
    /// Outline text, outlined with current stroking color
    /// and filled with current non-stroking color
    FillStroke,
    /// Don't render text, invisible
    Blank,
    /// Fill text and add to path for clipping
    /// see section 9.3.6 of the PDF standard.
    FillForClipping,
    /// Stroke text and add to path for clipping
    StrokeForClipping,
}

impl PDFTextRenderMode {
    fn value(&self) -> u16 {
        match self {
            PDFTextRenderMode::Fill => 0,
            PDFTextRenderMode::Stroke => 1,
            PDFTextRenderMode::FillStroke => 2,
            PDFTextRenderMode::Blank => 3,
            PDFTextRenderMode::FillForClipping => 4,
            PDFTextRenderMode::StrokeForClipping => 5,
        }
    }
}

impl PDFPage {
    /// `add_text` writes text into a page of a pdf at a specified position.
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
        text: &str,
        font_size: u32,
        font: &PDFFont,
        line_spacing: u32,
        text_width: ucum::Meter<f64>,
        x_pos: ucum::Meter<f64>,
        y_pos: ucum::Meter<f64>,
        text_direction: rustybuzz::Direction,
        text_language: rustybuzz::Language,
        text_render_mode: PDFTextRenderMode,
    ) -> io::Result<()> {
        use super::paragraph_breaking::to_lines;
        let (lines, glyphs) = to_lines(
            text,
            &font.font_face,
            font_size,
            text_width,
            text_direction,
            text_language,
        )?;

        let num_lines = lines.len();

        // then check to see if text starts inside page boundaries.
        let current_page_size = self.page_size.size();

        if x_pos > current_page_size.0
            || x_pos < 0.0 * ucum::IN_US
            || y_pos > current_page_size.1
            || y_pos < 0.0 * ucum::IN_US
        {
            panic! {concat!{"Position of text X: {}, Y: {}, ",
            "is outside page boundaries. Please fix this"},
            current_page_size.0, current_page_size.1}
        }
        if x_pos > current_page_size.0 - self.right_margin
            || x_pos < 0.0 * ucum::IN_US + self.left_margin
            || y_pos > current_page_size.1 - self.top_margin
            || y_pos < 0.0 * ucum::IN_US + self.bottom_margin
        {
            warn! {concat!{"Position of text X: {}, Y: {}, ",
            "is outside page margin boundaries. ",
            "Please fix this if unintentional"},
            current_page_size.0, current_page_size.1}
        }

        // BT begins a text element. it takes no operands
        self.operations.push(Operation::new("BT", vec![]));

        // save/push current graphics state
        self.operations.push(Operation::new("q", vec![]));
        // Tf specifies the font and font size. Font scaling is complicated in PDFs. Reference
        // the reference for more info.
        // The into() methods are defined based on their paired .from() methods (this
        // functionality is built into rust), and are converting the provided values into
        // An enum that represents the basic object types in PDF documents.
        //
        // font_id_str is the font reference string in the resources dictionary
        self.operations.push(Operation::new(
            "Tf",
            vec![font.font_id_str.clone().into(), font_size.into()],
        ));
        // Td adjusts the translation components of the text matrix. When used for the first
        // time after BT, it sets the initial text position on the page.
        // Note: PDF documents have Y=0 at the bottom. Thus 600 to print text near the top.
        // This is setting the position of the first line
        // TODO: convert into user space units here and use value rather than value unsafe
        self.operations.push(Operation::new(
            "Td",
            vec![x_pos.value_unsafe.into(), y_pos.value_unsafe.into()],
        ));

        // sets rendering mode of text
        self.operations
            .push(Operation::new("Tr", vec![text_render_mode.value().into()]));

        if num_lines == 1 {
            // Tj prints a string literal to the page.
            self.operations
                .push(Operation::new("Tj", vec![Object::string_literal(text)]));
        } else {
            // set leading / line gap
            self.operations
                .push(Operation::new("TL", vec![font.font_face.line_gap().into()]));
            // push first line
            self.operations.push(Operation::new(
                "Tj",
                vec![Object::string_literal(lines[1].as_str())],
            ));
            // push remaining lines
            for line in &lines[2..] {
                // `'` operation moves to next line using value specified by TL for spacing
                // and shows a line of text
                self.operations.push(Operation::new(
                    "'",
                    vec![Object::string_literal(line.as_str())],
                ));
            }
        }
        // restore/pop old graphics state
        self.operations.push(Operation::new("Q", vec![]));
        // ET ends the text element
        self.operations.push(Operation::new("ET", vec![]));
        Ok(())
    }

    /// `add_svg` parses an SVG image, converts it into pdf graphics operators,
    /// and inserts it into the referenced pdf document at the specified page
    pub fn add_svg() {}
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
        let font_data_owned = font_data.to_owned().leak();
        //TODO: maybe error out if font_file is a collection
        let face_index = font_index.unwrap_or(0);

        let font_face = Face::from_slice(font_data_owned, face_index).ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "font failed to parse",
        ))?;

        // 0 indexed, 4 is the table row that contains the full name of the font
        // https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6name.html
        // the .get function returns a byte array which needs to be converted into a string and
        // then processed.
        let font_name_data = font_face.names().get(4).ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "font failed to parse",
        ))?;
        let font_name = String::from_utf8_lossy(font_name_data.name)
            .to_mut()
            .replace(char::REPLACEMENT_CHARACTER, "");
        let font_id_str = format! {"F{}", self.available_fonts.len()+1};
        self.available_fonts.push(PDFFont {
            font_name,
            font_id_str,
            font_id: None,
            font_face,
            font_data,
        });

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
                page_size: {
                    if let Some(page_size) = page_size {
                        page_size
                    } else {
                        self.default_page_size
                    }
                },
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
            page_size: {
                if let Some(page_size) = page_size {
                    page_size
                } else {
                    self.default_page_size
                }
            },
            left_margin,
            right_margin,
            top_margin,
            bottom_margin,
        });
    }
    /// `write` sets up and writes a pdf file.
    pub fn write(
        &mut self,
        out_path: PathBuf,
        file_name: PathBuf,
        compress: bool,
    ) -> io::Result<()> {
        let pdf_version = "1.7";
        let mut doc = Document::with_version(pdf_version);
        // default userspace units in PDF are digital printers points
        let pdf_point = (1.0 / 72.0) * ucum::IN_US;

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

        for font in &mut self.available_fonts {
            //https://github.com/J-F-Liu/lopdf/issues/219#issuecomment-1464928118

            // create a stream object of font data
            let font_stream = Stream::new(dictionary! {}, font.font_data.clone());
            let font_stream_id = doc.add_object(font_stream);
            // create a font descriptor dictionary object
            let font_descriptor_dict = dictionary! {
                "Type" => "FontDescriptor",
                "FontName" => font.font_name.as_str(),
                "Flags" => {
                    // font_descriptor_flag is a unsigned 32 bit integer containing bitflags
                    // that specify various characteristics of the font.
                    // Bits are numbered from 1 (low-order) to 32 (high-order)
                    // All non-specified flags are reserved and shall be set to 0
                    //
                    // | Bit position | Name        |
                    // | ------------ | ----------- |
                    // | 1            | FixedPitch  |
                    // | 2            | Serif       |
                    // | 3            | Symbolic    |
                    // | 4            | Script      |
                    // | 6            | NonSymbolic |
                    // | 7            | Italic      |
                    // | 17           | AllCap      |
                    // | 18           | SmallCap    |
                    // | 19           | ForceBold   |
                    //
                    // From table 123 of the PDF1.7 specification pdf.
                    let mut font_descriptor_flag: u32 = 0;
                    if font.font_face.is_monospaced(){font_descriptor_flag += 2_u32.pow(0); }
                    //if font.font_face.
                    //if
                    //if
                    if font.font_face.is_italic() || font.font_face.is_oblique() {font_descriptor_flag += 2_u32.pow(6);}
                    font_descriptor_flag

                },
                // this is a PDF rectangle
                "FontBBox" => vec![
                    font.font_face.global_bounding_box().x_min.into(),
                    font.font_face.global_bounding_box().y_min.into(),
                    font.font_face.global_bounding_box().x_max.into(),
                    font.font_face.global_bounding_box().y_max.into(),
                ],
                "ItalicAngle" => font.font_face.italic_angle().unwrap_or(0.0),
                "Ascent" => font.font_face.ascender(),
                "Decent" => font.font_face.descender(),
                "Leading" => font.font_face.line_gap(),
                "CapHeight" => font.font_face.capital_height().unwrap_or(0),
                "XHeight" => font.font_face.x_height().unwrap_or(0),
                // Defaulted to 80. No way to pull this out of font files. Seems to not truely be
                // required by most PDF readers
                // https://stackoverflow.com/a/35543715/3342767
                // https://www.truetype-typography.com/ttqa_1998.htm
                "StemV" => 80,
                "FontFile2" => font_stream_id,


            };

            let font_descriptor_id = doc.add_object(font_descriptor_dict);
            // The encoding entry in a font dictionary is optional as long as the font has built in
            // encoding.
            // Left here for reference
            //let encoding = dictionary! {
            //    "Type" => "Encoding",
            //    // the baseEncoding value is as specified in the PDF standard
            //    "BaseEncoding" => "StandardEncoding",
            //};

            // create a font dictionary object
            let font_dict = dictionary! {
                "Type" => "Font",
                "Subtype" => "TrueType",
                "BaseFont" => font.font_name.as_str(),
                //TODO:
                //"FirstChar" => ,
                //"LastChar" => ,
                //"Widths" => ,
                "FontDescriptor" => font_descriptor_id,
                "Length1" => font.font_data.clone().len() as i32,
            };
            font.font_id = Some(doc.add_object(font_dict));
        }

        // font dictionaries need to be added into resource dictionaries
        // in order to be used.
        // Resource dictionaries can contain more than just fonts,
        // but normally just contains fonts
        // Only one resource dictionary is allowed per page tree root
        let resources_id = doc.add_object(dictionary! {
            // fonts are actually triplely nested dictionaries. Fun!
            "Font" =>  {
                let mut temp_dict = lopdf::Dictionary::new();
                for font in &self.available_fonts{

                    temp_dict.set(font.font_id_str.as_str(), font.font_id.unwrap());
            }
                temp_dict
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
            // need to be manually malnanipulated. It contains keys such as
            // Length, Filter, DecodeParams, etc
            //
            // content is a stream of encoded content data.
            let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));

            // Page is a page object dictionary dictionary that represents one page of a PDF file.
            let page_id = doc.add_object(dictionary! {
                "Type" => "Page",
                "Parent" => pages_id, // root of page tree
                "Contents" => content_id,
                // a rectangle that defines the boundaries of the physical or digital media. This is the
                // "Page Size"
                "MediaBox" => vec![
                    0.into(),
                    0.into(),
                    (*(page.page_size.size().0/pdf_point).value()).into(),
                    (*(page.page_size.size().1/pdf_point).value()).into()
                ],
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

        let page_count: u32 = self.pages.len().try_into().unwrap();

        let pages = dictionary! {
            // Type of dictionary
            "Type" => "Pages",
            // Vector of page IDs in document.
            "Kids" => page_ids,
            // Page count
            "Count" => page_count,
            // ID of resources, defined earlier
            "Resources" => resources_id,
            // a rectangle that defines the boundaries of the physical or digital media. This is the
            // "Page Size"
            // The mediabox in the root page node is the default page size
            "MediaBox" => vec![
                0.into(),
                0.into(),
                (*(self.default_page_size.size().0/pdf_point).value()).into(),
                (*(self.default_page_size.size().1/pdf_point).value()).into()
            ],
        };

        // using insert() here, instead of add_object() since the id is already known.
        doc.objects.insert(pages_id, Object::Dictionary(pages));

        // Creating document catalog.
        // There are many more entries allowed in the catalog dictionary.
        let doc_catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Version" => pdf_version,
            "Pages" => pages_id,
        });

        // Root key in trailer is set here to ID of document catalog,
        // remainder of trailer is set during doc.save().
        doc.trailer.set("Root", doc_catalog_id);

        let file_path = out_path.join(file_name);
        if compress {
            doc.compress();
        }
        doc.save(file_path)?;
        Ok(())
    }
}