use std::path::PathBuf;

use log::{info, trace};
use num_rational::Rational64;
use pdf_helper::{Margins, PDFDocument, PDFPage, paper::PaperSize};

use crate::{
    datatypes::{
        library_types::Library,
        project_types::{
            Project,
            enclosure::{Enclosure, MountPoint},
        },
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
/// `pdf_one_enclosure` generates one PDF page with one enclosure in the project
///
/// # Arguments
///
/// * `project` - the `Project` that contains this enclosure
/// * `library` - the `Library` that contains reference data for this project
/// * `enclosure` - the `Enclosure` that will be rendered as a PDF
/// * `page_size` - the target page size of the PDF file
/// * `margins` - the margin sizes of the PDF page
/// * `scale` - optional - specifies the scale of the rendered objects relative to their full size,
///   represented as a:b. For example, 1:2 would double the size of the object on the page, relative
///   to its actual size, and 2:1 would half the size of the object
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
    scale: Rational64,
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
///   represented as `a`:`b`. For example, 1:2 would double the size of the object on the page,
///   relative to its actual size, and 2:1 would half the size of the object. This is equal scaling
///   in both X and Y direction.
///
/// # Errors
///
/// will Error if the location doesn't fit on page at specified scale or if there is no equipment
/// at the specified location.
///
/// # Panics
///
/// Will panic if key is not found for value in enclosures hashmap.
pub fn render_enclosure(
    project: &Project,
    library: &Library,
    enclosure: &Enclosure,
    scale: Rational64,
    pdf_page: &mut PDFPage,
) -> Result<(), Error> {
    // layout all equipment in location

    if project.equipment.is_empty() {
        return Err(PDFGenerationError::PDFCreationError("no equipment in location".to_string()).into());
    }
    let page_width = pdf_page.page_size.size().0;
    let page_height = pdf_page.page_size.size().1;
    #[expect(clippy::expect_used)]
    let enclosure_id = project
        .enclosures
        .iter()
        .find_map(|(key, val)| (val == enclosure).then_some(key))
        .expect("Enclosure ID not found in hashmap when searching by value. Something went seriously wrong.");
    let enclosure_type = library
        .enclosure_types
        .get(&enclosure.enclosure_type)
        .ok_or(Error::LibraryValueNotFound {
            id: enclosure.enclosure_type.clone(),
            library_type: "Enclosure Type".to_string(),
        })?;
    let enclosure_type_dimensions = enclosure_type.dimensions.clone();
    // check if location will fit within page at 1:1 scale
    #[expect(clippy::arithmetic_side_effects)]
    let enclosure_default_scale_fit = {
        enclosure_type_dimensions.width.value < (page_width - pdf_page.margins.left - pdf_page.margins.right)
            && enclosure_type_dimensions.height.value < (page_height - pdf_page.margins.top - pdf_page.margins.bottom)
    };

    // check if location will fit on page at specified scale
    #[expect(clippy::arithmetic_side_effects)]
    let enclosure_scale_fit = {
        (enclosure_type_dimensions.width.value * scale) < (page_width - pdf_page.margins.left - pdf_page.margins.right)
            && (enclosure_type_dimensions.height.value * scale) < (page_height - pdf_page.margins.top - pdf_page.margins.bottom)
    };

    if enclosure_default_scale_fit {
        info!("location fits within page at 1:1 scale.");
    } else if enclosure_scale_fit {
        trace!("location fits within page at {scale} scale");
    } else {
        return Err(PDFGenerationError::LayoutError(format!(
            "Enclosure {} did not fit on Page Size {} at scale: {}",
            enclosure_id, pdf_page.page_size, scale,
        ))
        .into());
    }
    // loop through all mounting_points in Enclosure and render them if they are mounting rails
    let mut mounting_rails_rendered = Vec::new();
    for (mount_id, mount_point) in &enclosure.mount_points {
        if let MountPoint::MountingRail { x, y, mounting_rail, .. } = mount_point && !mounting_rails_rendered.contains(mount_id) {
            pdf_page.add_svg(
                project
                    .mounting_rails
                    .get(mounting_rail)
                    .ok_or(Error::ProjectValueNotFound {
                        id: mounting_rail.clone(),
                        project_type: "MountingRail".to_string(),
                    })?
                    .vis_rep()?
                    .get_tree(),
                x.value,
                y.value,
                scale,
            )?;
            mounting_rails_rendered.push(mount_id.clone());
        }
    }

    // loop through all equipment in project and render
    let mut equipment_ids_in_location = Vec::new();
    for (equipment_id, equipment) in &project.equipment {
        // select equipment that is within enclosure
        if let Some(equip_enclosure_id) = equipment.enclosure.clone()
            && equip_enclosure_id == *enclosure_id
        {
            let equipment_type = library
                .equipment_types
                .get(&equipment.equipment_type)
                .ok_or(Error::LibraryValueNotFound {
                    id: equipment.equipment_type.clone(),
                    library_type: "Equipment_type".to_string(),
                })?;
            let equipment_mount_point_id = equipment.mount_point.clone().ok_or(Error::ProjectDataMissing {
                id: equipment_id.clone(),
                project_type: "Equipment".to_string(),
                data_missing: "mount point".to_string(),
            })?;
            let equipment_mount_point =
                enclosure
                    .mount_points
                    .get(&equipment_mount_point_id)
                    .ok_or(Error::ProjectDataMissing {
                        id: enclosure_id.clone(),
                        project_type: "Enclosure".to_string(),
                        data_missing: format!("mount_point with id {equipment_mount_point_id}").to_string(),
                    })?;
            match equipment_mount_point {
                MountPoint::CoordinatePair { x, y } => {
                    pdf_page.add_svg(equipment_type.visual_rep().get_tree(), x.value, y.value, scale)?;
                }
                MountPoint::MountingRail { distance, x, y, .. } => {
                    #[expect(clippy::arithmetic_side_effects)]
                    pdf_page.add_svg(
                        equipment_type.visual_rep().get_tree(),
                        x.value,
                        y.value + distance.value,
                        scale,
                    )?;
                }
                _ => {
                    unimplemented!()
                }
            }
            equipment_ids_in_location.push(equipment_id.clone());
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
