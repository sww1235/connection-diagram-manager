/// `paper` contains information about various
/// physical paper sizes.
pub mod paper;

/// `scale` is a ratio for scaling objects during PDF rendering
pub mod scale;

use lopdf::content::{Content, Operation};
use lopdf::dictionary;
use lopdf::{Document, Object, Stream};

use dimensioned::{ucum, Dimensionless};

use log::warn;

use std::io;
use std::path::{Path, PathBuf};
use std::str;

use paragraph_breaker::Error as ParagraphError;
use usvg::Error as USVGError;

/// `PDFDocument` is a helper type to properly generate PDFs
/// It allows for easier tracking of default page size, available fonts
/// and allows for easier implementation of helper methods.
pub struct PDFDocument<'a> {
    /// default page size for PDF
    default_page_size: paper::PaperSize,
    /// Vector of fonts available to be used in a pdf.
    available_fonts: Vec<PDFFont<'a>>,
    /// all the pages in the PDF document
    pub pages: Vec<PDFPage>,
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
    /// The list of operations for the page
    operations: Vec<Operation>,
    /// The paper size of the page
    pub page_size: paper::PaperSize,
    pub margins: Margins,
}

/// `PDFTextRenderMode` is an enumeration
/// of defined text rendering modes in pdf documents
#[allow(clippy::exhaustive_enums)]
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

/// `Margins` represents the non-printing border around a PDF page, or piece of paper
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Margins {
    /// `top` margin
    pub top: ucum::Meter<f64>,
    /// `bottom` margin
    pub bottom: ucum::Meter<f64>,
    /// `left` margin
    pub left: ucum::Meter<f64>,
    /// `right` margin
    pub right: ucum::Meter<f64>,
}

impl PDFTextRenderMode {
    /// `value` returns the integer value of `PDFTextRenderMode` as needed in the internal PDF
    /// formatting
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
    /// * `text`: the text that will be added to the pdf page
    /// * `font`: a `PDFFont` object representing a font defined for use in a PDF document
    /// * `font_size`: Font size specified in points. This is used internally as a multiple
    ///     of the standard PDF user space unit size 1/72 inch
    /// * `line_spacing`: text line spacing in multiples of line height
    /// * `text width`: An optional parameter that defines the width of the text element. If
    ///     `None`, the width of the page minus the margins is used.
    /// * `x_pos`: horizontal starting position of text insertion, with 0 on left side of page
    ///     inside the margin
    /// * `y_pos`: vertical starting position of text insertion, with 0 on the bottom side of page,
    ///     inside the margin
    ///
    /// # Errors
    ///
    /// Can error if text position is incorrect, or if text fails to shape or split into lines
    #[allow(clippy::too_many_arguments)]
    pub fn add_text(
        &mut self,
        text: String,
        font_size: u32,
        font: &PDFFont,
        line_spacing: u32,
        text_width: ucum::Meter<f64>,
        x_pos: ucum::Meter<f64>,
        y_pos: ucum::Meter<f64>,
        text_direction: rustybuzz::Direction,
        text_language: rustybuzz::Language,
        text_render_mode: &PDFTextRenderMode,
    ) -> Result<(), Error> {
        use paragraph_breaker::to_lines;
        let (lines, glyphs) = to_lines(
            &text,
            &font.font_face,
            font_size,
            text_width,
            text_direction,
            text_language,
        )?;

        let num_lines = lines.len();

        // then check to see if text starts inside page boundaries.
        let current_page_size = self.page_size.size();

        #[allow(clippy::arithmetic_side_effects)]
        if x_pos > current_page_size.0
            || x_pos < 0.0_f64 * ucum::IN_US
            || y_pos > current_page_size.1
            || y_pos < 0.0_f64 * ucum::IN_US
        {
            return Err(Error::Other(format!(
                concat!(
                    "Position of text X: {}, Y: {}, ",
                    "is outside page boundaries. Please fix this"
                ),
                current_page_size.0, current_page_size.1,
            )));
        }
        #[allow(clippy::arithmetic_side_effects)]
        if x_pos > current_page_size.0 - self.margins.right
            || x_pos < 0.0_f64 * ucum::IN_US + self.margins.left
            || y_pos > current_page_size.1 - self.margins.top
            || y_pos < 0.0_f64 * ucum::IN_US + self.margins.bottom
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
        let x = *(((x_pos + self.margins.left) / PDFDocument::pdf_point()).value());
        let y = *(((y_pos + self.margins.bottom) / PDFDocument::pdf_point()).value());
        self.operations
            .push(Operation::new("Td", vec![x.into(), y.into()]));

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
                // TODO: add line spacing here
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

    //TODO: need to be able to specify location
    /// `add_svg` parses an SVG image, converts it into pdf graphics operators,
    /// and inserts it into the referenced pdf document at the specified page
    ///
    /// Heavily inspired by [resvg](https://github.com/RazrFalcon/resvg)
    ///
    /// # Arguments
    ///
    /// * `page_index`: The zero indexed page number that the text should be inserted into.
    /// * `text`: the text that will be added to the pdf page
    /// * `font`: a `PDFFont` object representing a font defined for use in a PDF document
    /// * `font_size`: Font size specified in points. This is used internally as a multiple
    ///     of the standard PDF user space unit size 1/72 inch
    /// * `line_spacing`: text line spacing in multiples of line height
    /// * `text width`: An optional parameter that defines the width of the text element. If
    ///     `None`, the width of the page minus the margins is used.
    /// * `x_pos`: horizontal starting position of text insertion, with 0 on left side of page
    /// * `y_pos`: vertical starting position of text insertion, with 0 on the bottom side of page
    ///
    /// # Errors
    ///
    /// May error due to malformed SVGs or other errors
    pub fn add_svg(
        &mut self,
        svg_string: &str,
        x_pos: ucum::Meter<f64>,
        y_pos: ucum::Meter<f64>,
        scale: Option<scale::ScalingFactor>,
    ) -> Result<(), Error> {
        use usvg::Tree;
        let parse_options = usvg::Options::default();
        let tree = Tree::from_str(svg_string, &parse_options)?;
        // TODO: either handle or forbid text nodes
        if tree.has_text_nodes() {
            return Err(Error::Other("Text Nodes in tree".to_string()));
        }
        let new_operations = loop_nodes(tree.root(), x_pos, y_pos, scale);
        self.operations.extend(new_operations);
        Ok(())
    }
}
/// `loop_nodes` loops over a SVG tree or subtree and outputs a vector of PDF operations
fn loop_nodes(
    parent: &usvg::Group,
    x_pos: ucum::Meter<f64>,
    y_pos: ucum::Meter<f64>,
    scale: Option<scale::ScalingFactor>,
) -> Vec<Operation> {
    let mut group_operations = Vec::new();
    for node in parent.children() {
        //TODO: investigate subroots
        let operation = match node {
            usvg::Node::Group(group) => loop_nodes(group, x_pos, y_pos, scale),
            usvg::Node::Path(ref path) => convert_path(path, x_pos, y_pos, scale),
            usvg::Node::Image(ref image) => convert_image(image, x_pos, y_pos, scale),
            usvg::Node::Text(_) => Vec::new(), // should already be converted into paths
        };
        group_operations.extend(operation);
    }
    group_operations
}

/// `convert_path` convets an SVG path element into a vector of PDF operations
/// `scale` parameter - optional - specifies the scale of the rendered objects relative to their full size,
/// represented as `a`:`b`.
/// For example, 1:2 would double the size of the object on the page, relative to its actual size,
/// and 2:1 would half the size of the object. This is equal scaling in both X and Y direction.

fn convert_path(
    path: &usvg::Path,
    x_pos: ucum::Meter<f64>,
    y_pos: ucum::Meter<f64>,
    scale: Option<scale::ScalingFactor>,
) -> Vec<Operation> {
    use usvg::tiny_skia_path::{PathSegment, Point};
    let mut new_operations = Vec::new();

    let int_scale: f32 = if let Some(scale) = scale {
        f32::from(scale.a / scale.b)
    } else {
        1.0
    };

    //TODO: determine if we need to start with m operation by looking at the first point in
    //data.points()
    let mut last_point = Point::zero();
    for segment in path.data().segments() {
        match segment {
            PathSegment::MoveTo(p) => {
                let mut scaled_p = p;
                scaled_p.x *= int_scale;
                scaled_p.y *= int_scale;
                last_point = scaled_p;
                // begin a new path (subpath in pdf language) by moving the current point to
                // coordinates (x,y)
                let x = *(((f64::from(scaled_p.x) * x_pos) / PDFDocument::pdf_point()).value());
                let y = *(((f64::from(scaled_p.y) * y_pos) / PDFDocument::pdf_point()).value());
                new_operations.push(Operation::new("m", vec![x.into(), y.into()]));
            }
            PathSegment::LineTo(p) => {
                let mut scaled_p = p;
                scaled_p.x *= int_scale;
                scaled_p.y *= int_scale;
                last_point = scaled_p;
                // append a straight line segment from current point to the point (x,y).
                new_operations.push(Operation::new(
                    "l",
                    vec![scaled_p.x.into(), scaled_p.y.into()],
                ));
            }
            // https://web.archive.org/web/20240625010856/https://www.reddit.com/r/AskComputerScience/comments/x0rrd2/does_applying_a_transformation_to_the_control/?rdt=60310
            // Scaling control points is ok, because the control points define the curve
            // p0 is control point, p1 is end point
            PathSegment::QuadTo(p0, p1) => {
                let mut scaled_p0 = p0;
                scaled_p0.x *= int_scale;
                scaled_p0.y *= int_scale;
                let mut scaled_p1 = p1;
                scaled_p1.x *= int_scale;
                scaled_p1.y *= int_scale;
                // create psuedo control points for cubic from quadratic.
                // Formuala from https://stackoverflow.com/a/3162732/3342767

                // begining control point
                #[allow(clippy::arithmetic_side_effects)]
                let cp1 = last_point
                    + Point::from_xy(
                        (2.0 / 3.0) * (scaled_p0 - last_point).x,
                        (2.0 / 3.0) * (scaled_p0 - last_point).y,
                    );
                // end control point
                #[allow(clippy::arithmetic_side_effects)]
                let cp2 = scaled_p1
                    + Point::from_xy(
                        (2.0 / 3.0) * (scaled_p0 - scaled_p1).x,
                        (2.0 / 3.0) * (scaled_p0 - scaled_p1).y,
                    );
                last_point = scaled_p1;
                // append a cubic bezier curve to current path.
                // Last 2 points are end point,
                // First 2 points are begining control point
                // Second 2 points are end control points
                new_operations.push(Operation::new(
                    "c",
                    vec![
                        cp1.x.into(),
                        cp1.y.into(),
                        cp2.x.into(),
                        cp2.y.into(),
                        scaled_p1.x.into(),
                        scaled_p1.y.into(),
                    ],
                ));
            }
            // p0 is begining control point, p1 is end control point, p2 is end point
            PathSegment::CubicTo(p0, p1, p2) => {
                let mut scaled_p0 = p0;
                scaled_p0.x *= int_scale;
                scaled_p0.y *= int_scale;
                let mut scaled_p1 = p1;
                scaled_p1.x *= int_scale;
                scaled_p1.y *= int_scale;
                let mut scaled_p2 = p2;
                scaled_p2.x *= int_scale;
                scaled_p2.y *= int_scale;
                last_point = scaled_p2;
                // append a cubic bezier curve to current path.
                // Last 2 points are end point,
                // First 2 points are begining control point
                // Second 2 points are end control points
                new_operations.push(Operation::new(
                    "c",
                    vec![
                        scaled_p0.x.into(),
                        scaled_p0.y.into(),
                        scaled_p1.x.into(),
                        scaled_p1.y.into(),
                        scaled_p2.x.into(),
                        scaled_p2.y.into(),
                    ],
                ));
            }
            PathSegment::Close => {
                // close current path by appending a straight line segment from current point to
                // starting point of path
                new_operations.push(Operation::new("h", vec![]));
            }
        }
    }
    new_operations
}

/// `convert_image` converts an embedded image in an SVG into a vector of PDF operations.
///
/// Currently ignores all embedded images other than SVGs.
fn convert_image(
    image: &usvg::Image,
    x_pos: ucum::Meter<f64>,
    y_pos: ucum::Meter<f64>,
    scale: Option<scale::ScalingFactor>,
) -> Vec<Operation> {
    use usvg::ImageKind;
    match &image.kind() {
        ImageKind::JPEG(_) => {
            // only vector graphic elements allowed
            warn! {"svg should not contain images or image tags, ignoring"};
            Vec::new()
        }
        ImageKind::PNG(_) => {
            // only vector graphic elements allowed
            warn! {"svg should not contain images or image tags, ignoring"};
            Vec::new()
        }
        ImageKind::GIF(_) => {
            // only vector graphic elements allowed
            warn! {"svg should not contain images or image tags, ignoring"};
            Vec::new()
        }
        ImageKind::SVG(tree) => loop_nodes(tree.root(), x_pos, y_pos, scale),
    }
}

impl<'a> PDFDocument<'a> {
    pub fn pdf_point() -> ucum::Meter<f64> {
        // default userspace units in PDF are digital printers points
        //#[allow(clippy::arithmetic_side_effects)]
        (1.0_f64 / 72.0_f64) * ucum::IN_US
    }

    /// `new` returns a set up and initialized `PDFDocument`
    /// with the specified font paths pre-populated and default page size,
    /// as specified via parameter
    ///
    /// # Errors
    ///
    /// Will error if loading configuration fonts fails
    pub fn new(
        default_page_size: paper::PaperSize,
        font_paths: Vec<PathBuf>,
    ) -> Result<Self, Error> {
        let mut output = Self {
            default_page_size,
            available_fonts: Vec::new(),
            pages: Vec::new(),
        };
        output.load_cfg_fonts(font_paths)?;
        Ok(output)
    }

    /// `empty` returns a mostly empty `PDFDocument`. It does not include default fonts
    /// and has the `default_page_size` set to A4.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            default_page_size: paper::PaperSize::A4,
            available_fonts: Vec::new(),
            pages: Vec::new(),
        }
    }

    /// `load_ttf_font` parses the specified TTF font via `rustybuzz` and `ttf_parser` and makes it
    /// available in the specified PDF document.
    ///
    /// # Errors
    ///
    /// Will produce errors if reading font data fails, or if a font fails to parse
    pub fn load_ttf_font(
        &mut self,
        font_file: PathBuf,
        font_index: Option<u32>,
    ) -> Result<(), Error> {
        use rustybuzz::Face;

        let font_data = std::fs::read(font_file)?;
        let font_data_owned = font_data.clone().leak();
        //TODO: maybe error out if font_file is a collection
        let face_index = font_index.unwrap_or(0);

        let font_face = Face::from_slice(font_data_owned, face_index)
            .ok_or(Error::FontLoading("font failed to parse".to_string()))?;

        // 0 indexed, 4 is the table row that contains the full name of the font
        // https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6name.html
        // the .get function returns a byte array which needs to be converted into a string and
        // then processed.
        let font_name_data = font_face
            .names()
            .get(4)
            .ok_or(Error::FontLoading("font failed to parse".to_string()))?;
        let font_name = String::from_utf8_lossy(font_name_data.name)
            .to_mut()
            .replace(char::REPLACEMENT_CHARACTER, "");
        let font_id_str = format! {"F{}", self.available_fonts.len()};
        self.available_fonts.push(PDFFont {
            font_name,
            font_id_str,
            font_id: None,
            font_face,
            font_data,
        });

        Ok(())
    }

    //TODO: need to add this to the config file, also this may not need to be public
    /// `load_cfg_font` loads the TTF font specified in the configuration file
    /// via `rustybuzz` and `ttf_parser` and makes it
    /// available in the specified PDF document.
    ///
    /// # Errors
    ///
    /// Will error if fonts are not specified in config file, or if a font fails to load.
    pub fn load_cfg_fonts(&mut self, font_paths: Vec<PathBuf>) -> Result<(), Error> {
        if font_paths.is_empty() {
            return Err(Error::FontLoading(
                "No Fonts specified in configuration file".to_string(),
            ));
        }
        for path in font_paths {
            // load default font_index
            self.load_ttf_font(path, Some(0))?;
        }
        Ok(())
    }

    /// `insert_page` inserts an empty `PDFPage` into self.pages vector
    /// at the specified page index which is zero indexed.
    /// This is a wrapper around vec.insert() so it follows the same rules.
    pub fn insert_page(
        &mut self,
        page_index: usize,
        page_size: Option<paper::PaperSize>,
        margins: Margins,
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
                margins,
            },
        );
    }
    /// `push_page` inserts an empty `PDFPage` into self.pages vector
    /// at the end of the vector.
    /// This is a wrapper around vec.push() so it follows the same rules.
    pub fn push_page(&mut self, page_size: Option<paper::PaperSize>, margins: Margins) {
        self.pages.push(PDFPage {
            operations: Vec::new(),
            page_size: {
                if let Some(page_size) = page_size {
                    page_size
                } else {
                    self.default_page_size
                }
            },
            margins,
        });
    }
    /// `write` sets up and writes a pdf file.
    ///
    /// # Errors
    ///
    /// Saving the file can error with [`std::io`] errors
    #[allow(clippy::too_many_lines)]
    pub fn write(
        &mut self,
        out_path: &Path,
        file_name: &Path,
        compress: bool,
    ) -> Result<(), Error> {
        let pdf_version = "1.7";
        let mut doc = Document::with_version(pdf_version);

        // Object IDs are used for cross referencing in PDF documents. `lopdf` helps keep track of them
        // for us. They are simple integers.
        // Calls to `doc.new_object_id` and `doc.add_object` return an object id

        // pages is the root node of the page tree
        let root_id = doc.new_object_id();

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
                    #[allow(clippy::arithmetic_side_effects)]
                    if font.font_face.is_monospaced(){font_descriptor_flag += 2_u32.pow(0); }
                    //if font.font_face.
                    //if
                    //if
                    #[allow(clippy::arithmetic_side_effects)]
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
                "StemV" => 80_u16,
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
                "Length1" => u32::try_from(font.font_data.clone().len()).unwrap_or(u32::MAX),
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
                    temp_dict.set(font.font_id_str.as_str(),
                        font.font_id.ok_or(Error::FontLoading("Font Object ID not set".to_string()))?);
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
            // TODO: return errors here
            let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode()?));

            // Page is a page object dictionary dictionary that represents one page of a PDF file.
            let page_id = doc.add_object(dictionary! {
                "Type" => "Page",
                "Parent" => root_id, // root of page tree
                "Contents" => content_id,
                // a rectangle that defines the boundaries of the physical or digital media. This is the
                // "Page Size"
                "MediaBox" => vec![
                    0_u16.into(),
                    0_u16.into(),
                    #[allow(clippy::arithmetic_side_effects)]
                    (*(page.page_size.size().0/Self::pdf_point()).value()).into(),
                    #[allow(clippy::arithmetic_side_effects)]
                    (*(page.page_size.size().1/Self::pdf_point()).value()).into()
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

        let page_count: u32 = self.pages.len().try_into().unwrap_or(u32::MAX);

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
                0_u16.into(),
                0_u16.into(),
                #[allow(clippy::arithmetic_side_effects)]
                (*(self.default_page_size.size().0/Self::pdf_point()).value()).into(),
                #[allow(clippy::arithmetic_side_effects)]
                (*(self.default_page_size.size().1/Self::pdf_point()).value()).into()
            ],
        };

        // using insert() here, instead of add_object() since the id is already known.
        doc.objects.insert(root_id, Object::Dictionary(pages));

        // Creating document catalog.
        // There are many more entries allowed in the catalog dictionary.
        let doc_catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Version" => pdf_version,
            "Pages" => root_id,
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

//TODO: switch these to contain the actual error types instead of strings, like cookbook crate
/// `Error` is the list of errors that can occur in `PDFHelper`
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Errors relating to linebreaking
    ParagraphBreaking(String),
    /// Errors relating to PDF creation and export
    PDFError(String),
    /// error in loading fonts
    FontLoading(String),
    /// error in parsing svg data
    SVGError(String),
    /// Errors from [`std::io`]
    IOError(String),
    /// Other errors
    Other(String),
}

impl std::error::Error for Error {}

#[allow(clippy::match_same_arms)]
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ParagraphBreaking(ref e) => write!(f, "Line Breaking failed: {e}"),
            Error::FontLoading(ref e) => write!(f, "Font Loading: {e}"),
            Error::PDFError(ref e) => write!(f, "PDF error: {e}"),
            Error::SVGError(ref e) => write!(f, "SVG: {e}"),
            Error::IOError(ref e) => write!(f, "IO error: {e}"),
            Error::Other(ref e) => write!(f, "{e}"),
        }
    }
}

impl From<ParagraphError> for Error {
    fn from(e: ParagraphError) -> Self {
        Error::ParagraphBreaking(format!("{e}"))
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IOError(format!("{e}"))
    }
}
impl From<USVGError> for Error {
    fn from(e: USVGError) -> Self {
        Error::SVGError(format!("{e}"))
    }
}
impl From<lopdf::Error> for Error {
    fn from(e: lopdf::Error) -> Self {
        match e {
            lopdf::Error::ContentDecode
            | lopdf::Error::DictKey
            | lopdf::Error::Header
            | lopdf::Error::ObjectIdMismatch
            | lopdf::Error::ObjectNotFound
            | lopdf::Error::Offset(..)
            | lopdf::Error::PageNumberNotFound(..)
            | lopdf::Error::Parse { .. }
            | lopdf::Error::ReferenceLimit
            | lopdf::Error::BracketLimit
            | lopdf::Error::Trailer
            | lopdf::Error::Type
            | lopdf::Error::UTF8
            | lopdf::Error::Syntax(..)
            | lopdf::Error::Xref(..)
            | lopdf::Error::Invalid(..)
            | lopdf::Error::NoOutlines
            | lopdf::Error::Decryption(..) => Error::PDFError(format!("{e}")),
            lopdf::Error::IO(..) => Error::IOError(format!("{e}")),
        }
    }
}
