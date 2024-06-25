use std::fmt;
use std::path::PathBuf;

use pdf_helper::{paper::PaperSize, scale::ScalingFactor, Margins, PDFDocument, PDFPage};

use crate::datatypes::internal_types::{location::Location, Library, Project};

use log::{trace, warn};

//TODO: add page templates with proper borders and titleblocks

/// `pdf_all_locations` generates a pdf for the entire project
pub fn pdf_all_locations(_project: &Project, _library: &Library, _page_size: PaperSize) {
    // first thing is to layout all locations
    //
    // then layout all pathways between locations
    //
    // then layout all equipment in each location
}

/// `pdf_one_location` generates one PDF page with one location in the project
///
/// # Arguments
///
/// * `project` - the `Project` that contains this location
/// * `reference_location` - the `Location` that will be rendered as a PDF
/// * `page_size` - the target page size of the PDF file
/// * `margins` - the margin sizes of the PDF page
/// * `scale` - optional - specifies the scale of the rendered objects relative to their full size,
/// represented as a:b.
/// For example, 1:2 would double the size of the object on the page, relative to its actual size,
/// and 2:1 would half the size of the object
///
/// # Errors
///
/// will Error if the location doesn't fit on page at specified scale
pub fn pdf_one_location(
    project: &Project,
    reference_location: &Location,
    margins: Margins,
    page_size: PaperSize,
    scale: Option<ScalingFactor>,
    config_font_paths: Vec<PathBuf>,
) -> Result<(), Error> {
    let mut pdf = PDFDocument::new(page_size, config_font_paths)?;
    pdf.push_page(None, margins);
    render_location(project, reference_location, scale, &mut pdf.pages[0])?;
    Ok(())
}
//TODO: autoscale boolean?
/// `render_location` generates one PDF page with one location in the project
///
/// # Arguments
///
/// * `project` - the `Project` that contains this location
/// * `reference_location` - the `Location` that will be rendered as a PDF
/// * `scale` - optional - specifies the scale of the rendered objects relative to their full size,
/// represented as `.0`:`.1`.
/// For example, 1:2 would double the size of the object on the page, relative to its actual size,
/// and 2:1 would half the size of the object. This is equal scaling in both X and Y direction.
///
/// # Errors
///
/// will Error if the location doesn't fit on page at specified scale or if there is no equipment
/// at the specified location.
pub fn render_location(
    project: &Project,
    reference_location: &Location,
    scale: Option<ScalingFactor>,
    pdf_page: &mut PDFPage,
) -> Result<(), Error> {
    // layout all equipment in location

    if project.equipment.is_empty() {
        return Err(Error::PDFCreationError(
            "no equipment in location".to_string(),
        ));
    }
    let page_width = pdf_page.page_size.size().0;
    let page_height = pdf_page.page_size.size().1;
    // check if location will fit within page at 1:1 scale
    #[allow(clippy::arithmetic_side_effects)]
    let location_default_scale_fit = {
        reference_location.location_type.borrow().width
            < (page_width - pdf_page.margins.left - pdf_page.margins.right)
            && reference_location.location_type.borrow().height
                < (page_height - pdf_page.margins.top - pdf_page.margins.bottom)
    };

    // check if location will fit on page at specified scale
    #[allow(clippy::arithmetic_side_effects)]
    let location_scale_fit = {
        (reference_location.location_type.borrow().width * f64::from(scale.unwrap_or_default().a)
            / f64::from(scale.unwrap_or_default().b))
            < (page_width - pdf_page.margins.left - pdf_page.margins.right)
            && (reference_location.location_type.borrow().height
                * f64::from(scale.unwrap_or_default().a)
                / f64::from(scale.unwrap_or_default().b))
                < (page_height - pdf_page.margins.top - pdf_page.margins.bottom)
    };

    if location_default_scale_fit {
        if scale.is_some() {
            //TODO: change this to info
            warn!("location fits within page at 1:1 scale. Scale does not need to be specified");
        }
    } else if location_scale_fit {
        trace!(
            "location fits within page at {} scale",
            scale.unwrap_or_default()
        );
    } else {
        return Err(Error::LayoutError(format!(
            "Location {} did not fit on Page Size {} at scale: {}",
            reference_location.id,
            pdf_page.page_size,
            scale.unwrap_or_default(),
        )));
    }
    // loop through all equipment in project and render
    for equipment in project.equipment.values() {
        // select equipment that is within location
        if equipment.borrow().location.borrow().id == reference_location.id {
            let svg_text = equipment
                .borrow()
                .equip_type
                .borrow()
                .visual_rep()
                .to_string();
            let x = equipment.borrow().sub_location.x;
            let y = equipment.borrow().sub_location.y;
            pdf_page.add_svg(svg_text.as_str(), x, y, scale)?;
        }
    }
    //TODO: now need to figure out which equipment is connected to which other equipment in the
    //same location and draw those connections and pathways
    //
    //Then draw connections off page gated behind a boolean

    Ok(())
}

/// `Error` is the list of errors that can occur in `PDFGeneration`
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Error resulting from layout or rendering
    LayoutError(String),
    /// Error resulting durin PDF Creation
    PDFCreationError(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::LayoutError(ref e) => write!(f, "Layout Error: {e}"),
            Error::PDFCreationError(ref e) => write!(f, "PDF Creation Error: {e}"),
        }
    }
}
impl From<pdf_helper::Error> for Error {
    fn from(e: pdf_helper::Error) -> Self {
        Error::PDFCreationError(format!("{e}"))
    }
}
