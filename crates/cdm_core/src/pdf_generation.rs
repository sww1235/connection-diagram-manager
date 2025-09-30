use std::path::PathBuf;

use log::{trace, warn};
use num_rational::Rational64;

use pdf_helper::{paper::PaperSize, scale::ScalingFactor, Margins, PDFDocument, PDFPage};

use crate::{
    datatypes::{
        library_types::Library,
        project_types::{enclosure::Enclosure, Project},
    },
    error::{Error, PDFGenerationError},
};

//TODO: add page templates with proper borders and titleblocks
//TODO: instead of page templates, make a configurable page border/titleblock
//
//
//TODO: add function that lays out all locations on one page

/// `pdf_all_the_things` generates a pdf for the entire project
#[expect(unused_variables)]
pub fn pdf_all_the_things(project: &Project, library: &Library, page_size: PaperSize) {
    // first thing is to layout all enclosures
    //
    // then layout all pathways between enclosures
    //
    // then layout all equipment in each enclosure
}
/// `pdf_one_enclosure` generates one PDF page with one location in the project
///
/// # Arguments
///
/// * `project` - the `Project` that contains this enclosure
/// * `library` - the `Library` that contains reference data for this project
/// * `enclosure` - the `Enclosure` that will be rendered as a PDF
/// * `page_size` - the target page size of the PDF file
/// * `margins` - the margin sizes of the PDF page
/// * `scale` - optional - specifies the scale of the rendered objects relative to their full size,
///   represented as a:b.
///   For example, 1:2 would double the size of the object on the page, relative to its actual size,
///   and 2:1 would half the size of the object
///
/// # Errors
///
/// will Error if the enclosure doesn't fit on page at specified scale
pub fn pdf_one_enclosure(
    project: &Project,
    library: &Library,
    enclosure: &Enclosure,
    margins: Margins,
    page_size: PaperSize,
    scale: Option<ScalingFactor>,
    config_font_paths: Vec<PathBuf>,
) -> Result<(), Error> {
    let mut pdf = PDFDocument::new(page_size, config_font_paths)?;
    pdf.push_page(None, margins);
    render_enclosure(project, library, enclosure, scale, &mut pdf.pages[0])?;
    Ok(())
}
//TODO: autoscale boolean?
/// `render_enclosure` generates one PDF page with one enclosure in the project
///
/// # Arguments
///
/// * `project` - the `Project` that contains this enclosure
/// * `library` - the `Library` that contains reference data for this project
/// * `enclosure` - the `Enclosure` that will be rendered as a PDF
/// * `scale` - optional - specifies the scale of the rendered objects relative to their full size,
///   represented as `a`:`b`.
///   For example, 1:2 would double the size of the object on the page, relative to its actual size,
///   and 2:1 would half the size of the object. This is equal scaling in both X and Y direction.
///
/// # Errors
///
/// will Error if the location doesn't fit on page at specified scale or if there is no equipment
/// at the specified location.
pub fn render_enclosure(
    project: &Project,
    library: &Library,
    enclosure: &Enclosure,
    scale: Option<ScalingFactor>,
    pdf_page: &mut PDFPage,
) -> Result<(), Error> {
    // layout all equipment in location

    if project.equipment.is_empty() {
        return Err(
            PDFGenerationError::PDFCreationError("no equipment in location".to_string()).into(),
        );
    }
    let page_width = pdf_page.page_size.size().0;
    let page_height = pdf_page.page_size.size().1;
    let enclosure_id = project.enclosures.iter().find_map(|(key, val)| if val == enclosure { Some(key) } else { None }).expect("Enclosure ID not found in hashmap when searching by value. Something went seriously wrong.");
    let (enclosure_type_id, enclosure_type) = library
        .enclosure_types
        .get_key_value(&enclosure.enclosure_type)
        .ok_or(Error::LibraryValueNotFound(
            enclosure.enclosure_type.clone(),
        ))?;
    let enclosure_type_dimensions =
        enclosure_type
            .dimensions
            .clone()
            .ok_or(Error::LibraryDataMissing {
                id: enclosure_type_id.clone(),
                data_missing: "dimensions".to_owned(),
            })?;
    // check if location will fit within page at 1:1 scale
    #[expect(clippy::arithmetic_side_effects)]
    let enclosure_default_scale_fit = {
        enclosure_type_dimensions.width.value
            < (page_width - pdf_page.margins.left - pdf_page.margins.right)
            && enclosure_type_dimensions.height.value
                < (page_height - pdf_page.margins.top - pdf_page.margins.bottom)
    };

    // check if location will fit on page at specified scale
    #[expect(clippy::arithmetic_side_effects)]
    let enclosure_scale_fit = {
        (enclosure_type_dimensions.width.value
            * Rational64::new(
                scale.unwrap_or_default().a.into(),
                scale.unwrap_or_default().b.into(),
            ))
            < (page_width - pdf_page.margins.left - pdf_page.margins.right)
            && (enclosure_type_dimensions.height.value
                * Rational64::new(
                    scale.unwrap_or_default().a.into(),
                    scale.unwrap_or_default().b.into(),
                ))
                < (page_height - pdf_page.margins.top - pdf_page.margins.bottom)
    };

    if enclosure_default_scale_fit {
        if scale.is_some() {
            warn!("location fits within page at 1:1 scale. Scale does not need to be specified");
        }
    } else if enclosure_scale_fit {
        trace!(
            "location fits within page at {} scale",
            scale.unwrap_or_default()
        );
    } else {
        return Err(PDFGenerationError::LayoutError(format!(
            "Enclosure {} did not fit on Page Size {} at scale: {}",
            enclosure_id,
            pdf_page.page_size,
            scale.unwrap_or_default(),
        ))
        .into());
    }
    // loop through all equipment in project and render

    //let mut equipment_ids_in_location = Vec::new();
    for (equipment_id, equipment) in project.equipment.iter() {
        // select equipment that is within location
        if let Some(equip_enclosure_id) = equipment.enclosure.clone()
            && equip_enclosure_id == *enclosure_id
        {
            let (equipment_type_id, equipment_type) = library
                .equipment_types
                .get_key_value(&equipment.equipment_type)
                .ok_or(Error::LibraryValueNotFound(
                    equipment.equipment_type.clone(),
                ))?;
            //TODO: fix this
            //let svg_text = equipment_type.visual_rep();
            //let x = equipment.mount_point.x;
            //let y = equipment.mount_point.y;
            //pdf_page.add_svg(svg_text.as_str(), x, y, scale)?;
            //equipment_ids_in_location.push(equipment.id.clone());
        }
    }

    //TODO: fix this
    for connection in &project.connections {
        //
    }
    //TODO: now need to figure out which equipment is connected to which other equipment in the
    //same location and draw those connections and pathways
    //
    //Then draw connections off page gated behind a boolean

    Ok(())
}
