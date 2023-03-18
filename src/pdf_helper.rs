pub mod paper;

use lopdf::content::{Content, Operation};
use lopdf::dictionary;
use lopdf::{Document, Object, Stream};

use dimensioned::{f64prefixes, ucum};

use std::io;
use std::path::PathBuf;

/// `PDFDocument` is a helper type to properly generate PDFs
/// It allows for easier tracking of default page size, available fonts
/// and allows for easier implementation of helper methods.
pub struct PDFDocument {
    document: Document,
    default_page_size: paper::PaperSize,
    available_fonts: Vec<String>,
    pages: Vec<PDFPage>,
}
/// `PDFPage` represents an individual page of a pdf file
struct PDFPage {
    operations: Vec<Operation>,
    page_size: Option<paper::PaperSize>,
}

impl PDFDocument {
    /// `add_text` writes text into a page of a pdf at a specified position
    pub fn add_text(
        &mut self,
        page_index: usize,
        text: String,
        font: String,
        font_size: u32,
        x_pos: u32,
        y_pos: u32,
    ) {
        // BT begins a text element. it takes no operands
        self.pages[page_index]
            .operations
            .push(Operation::new("BT", vec![]));
        // Tf specifies the font and font size. Font scaling is complicated in PDFs. Reference
        // the reference for more info.
        // The info() methods are defined based on their paired .from() methods (this
        // functionality is built into rust), and are converting the provided values into
        // An enum that represents the basic object types in PDF documents.
        self.pages[page_index]
            .operations
            .push(Operation::new("Tf", vec![font.into(), font_size.into()]));
        // Td adjusts the translation components of the text matrix. When used for the first
        // time after BT, it sets the initial text position on the page.
        // Note: PDF documents have Y=0 at the bottom. Thus 600 to print text near the top.
        self.pages[page_index]
            .operations
            .push(Operation::new("Td", vec![x_pos.into(), y_pos.into()]));
        // Tj prints a string literal to the page. By default, this is black text that is
        // filled in. There are other operators that can produce various textual effects and
        // colors
        self.pages[page_index]
            .operations
            .push(Operation::new("Tj", vec![Object::string_literal(text)]));
        // ET ends the text element
        self.pages[page_index]
            .operations
            .push(Operation::new("ET", vec![]));
    }

    /// `insert_page` inserts an empty PDFPage into self.pages vector
    /// at the specified page index which is zero indexed.
    /// This is a wrapper around vec.insert() so it follows the same rules.
    pub fn insert_page(&mut self, page_index: usize, page_size: Option<paper::PaperSize>) {
        self.pages.insert(
            page_index,
            PDFPage {
                operations: Vec::new(),
                page_size,
            },
        );
    }
    /// `push_page` inserts an empty PDFPage into self.pages vector
    /// at the end of the vector.
    /// This is a wrapper around vec.push() so it follows the same rules.
    pub fn push_page(&mut self, page_size: Option<paper::PaperSize>) {
        self.pages.push(PDFPage {
            operations: Vec::new(),
            page_size,
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
