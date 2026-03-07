use std::path::PathBuf;

use log::{info, trace};
use num_rational::Rational64;
use pdf_helper::{Margins, PDFDocument, PDFPage, paper::PaperSize};

use crate::{
    datatypes::{
        library_types::Library,
        project_types::{
            Project,
            connection::Type as ConnectionType,
            enclosure::{Enclosure, MountPoint},
        },
    },
    error::{ConnectionError, Error, LibraryError, PDFError, ProjectError},
};

//TODO: add page templates with proper borders and titleblocks
//TODO: instead of page templates, make a configurable page border/titleblock
//
//
//TODO: add function that lays out all locations on one page

/// `pdf_all_the_things` generates a pdf for the entire project.
#[expect(unused_variables, reason = "function not finished yet")]
#[inline(never)]
pub fn pdf_all_the_things(project: &Project, library: &Library, page_size: PaperSize) {
    // first thing is to layout all enclosures
    //
    // then layout all pathways between enclosures
    //
    // then layout all equipment in each enclosure
}
/// `pdf_one_enclosure` generates one PDF page with one enclosure in the project.
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
///   to its actual size, and 2:1 would half the size of the object.
///
/// # Errors
///
/// will Error if the enclosure doesn't fit on page at specified scale.
#[inline(never)]
#[expect(clippy::result_large_err, reason = "Don't want to have to split up error::Error ")]
pub fn pdf_one_enclosure(
    project: &Project,
    library: &Library,
    enclosure: &Enclosure,
    margins: Margins,
    page_size: PaperSize,
    scale: Rational64,
    config_font_paths: Vec<PathBuf>,
    render_connections: bool,
) -> Result<(), Error> {
    let mut pdf = PDFDocument::new(page_size, config_font_paths)?;
    pdf.push_page(None, margins);
    #[expect(clippy::indexing_slicing, reason = "We are inserting a page the previous line")]
    render_enclosure(project, library, enclosure, scale, &mut pdf.pages[0], render_connections)?;
    Ok(())
}
//TODO: autoscale boolean?
/// `render_enclosure` generates one PDF page with one enclosure in the project.
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
/// * `pdf_page` - a particular page within a pdf that this enclosure will be rendered on.
/// * `render_connections` - if connections with enclosure should be rendered.
///
/// # Errors
///
/// Will Error if the location doesn't fit on page at specified scale or if there is no equipment
/// at the specified location.
///
/// Will Error if any of the data in `project` or `library` is incorrect.
///
/// # Panics
///
/// Will panic if key is not found for value in enclosures hashmap.
#[expect(clippy::too_many_lines, reason = "hard to split up this function")]
//TODO: revisit the lint below
#[expect(clippy::pattern_type_mismatch, reason = "revisit this")]
#[expect(clippy::result_large_err, reason = "Don't want to have to split up error::Error ")]
#[expect(unused_variables, reason = "function not finished yet")]
#[inline(never)]
pub fn render_enclosure(
    project: &Project,
    library: &Library,
    enclosure: &Enclosure,
    scale: Rational64,
    pdf_page: &mut PDFPage,
    render_connections: bool,
) -> Result<(), Error> {
    // layout all equipment in location

    if project.equipment.is_empty() {
        return Err(PDFError::PDFCreationError("no equipment in location".to_owned()).into());
    }
    let page_width = pdf_page.page_size.size().0;
    let page_height = pdf_page.page_size.size().1;
    #[expect(
        clippy::expect_used,
        reason = "if the key of the value can't be found when searching by value, with both in the map, something went \
                  seriously wrong"
    )]
    let enclosure_id = project
        .enclosures
        .iter()
        .find_map(|(key, val)| (val == enclosure).then_some(key))
        .expect("Enclosure ID not found in map when searching by value. Something went seriously wrong.");
    let enclosure_type = library
        .enclosure_types
        .get(&enclosure.enclosure_type)
        .ok_or(LibraryError::ValueNotFound {
            id: enclosure.enclosure_type.clone(),
            found_in: enclosure_id.clone(),
            library_type: "Enclosure Type".to_owned(),
        })?;
    let enclosure_type_dimensions = enclosure_type.dimensions.clone();
    // check if location will fit within page at 1:1 scale
    #[expect(clippy::arithmetic_side_effects, reason = "no good way around this")]
    let enclosure_default_scale_fit = {
        enclosure_type_dimensions.width.value < (page_width - pdf_page.margins.left - pdf_page.margins.right)
            && enclosure_type_dimensions.height.value < (page_height - pdf_page.margins.top - pdf_page.margins.bottom)
    };

    // check if location will fit on page at specified scale
    #[expect(clippy::arithmetic_side_effects, reason = "no good way around this")]
    let enclosure_scale_fit = {
        (enclosure_type_dimensions.width.value * scale) < (page_width - pdf_page.margins.left - pdf_page.margins.right)
            && (enclosure_type_dimensions.height.value * scale) < (page_height - pdf_page.margins.top - pdf_page.margins.bottom)
    };

    if enclosure_default_scale_fit {
        info!("location fits within page at 1:1 scale.");
    } else if enclosure_scale_fit {
        trace!("location fits within page at {scale} scale");
    } else {
        return Err(PDFError::LayoutError(format!(
            "Enclosure {} did not fit on Page Size {} at scale: {}",
            enclosure_id, pdf_page.page_size, scale,
        ))
        .into());
    }
    // loop through all mounting_points in Enclosure and render them if they are mounting rails
    let mut mounting_rails_rendered = Vec::new();
    for (mount_id, mount_point) in &enclosure.mount_points {
        if let MountPoint::MountingRail { x, y, mounting_rail, .. } = mount_point
            && !mounting_rails_rendered.contains(mount_id)
        // only want to render each rail once
        {
            pdf_page.add_svg(
                project
                    .mounting_rails
                    .get(mounting_rail)
                    .ok_or(ProjectError::ValueNotFound {
                        id: mounting_rail.clone(),
                        found_in: enclosure_id.clone(),
                        project_type: "MountingRail".to_owned(),
                    })?
                    .vis_rep(library)?
                    .get_tree()?,
                x.value,
                y.value,
                scale,
            )?;
            // only want to render each rail once
            mounting_rails_rendered.push(mount_id.clone());
        }
    }

    // loop through all equipment in project and render if in enclosure
    for (equipment_id, equipment) in &project.equipment {
        // select equipment that is within enclosure
        if let Some(equip_enclosure_id) = equipment.enclosure.clone()
            && equip_enclosure_id == *enclosure_id
        {
            let equipment_type = library
                .equipment_types
                .get(&equipment.equipment_type)
                .ok_or(LibraryError::ValueNotFound {
                    id: equipment.equipment_type.clone(),
                    found_in: equipment_id.clone(),
                    library_type: "Equipment_type".to_owned(),
                })?;
            let equipment_mount_point_id = equipment.mount_point.clone().ok_or(ProjectError::DataMissing {
                id: equipment_id.clone(),
                found_in: equipment_id.clone(),
                project_type: "Equipment".to_owned(),
                data_missing: "mount point".to_owned(),
            })?;
            let equipment_mount_point =
                enclosure
                    .mount_points
                    .get(&equipment_mount_point_id)
                    .ok_or(ProjectError::DataMissing {
                        id: enclosure_id.clone(),
                        found_in: equipment_id.clone(),
                        project_type: "Enclosure".to_owned(),
                        data_missing: format!("mount_point with id {equipment_mount_point_id}").to_owned(),
                    })?;
            match equipment_mount_point {
                MountPoint::CoordinatePair { x, y } => {
                    //pdf_page.add_svg(equipment_type.visual_rep().get_tree(), x.value, y.value,
                    // scale)?;
                }
                MountPoint::MountingRail { distance, x, y, .. } => {
                    //#[expect(clippy::arithmetic_side_effects, reason = "no good way around this")]
                    //pdf_page.add_svg(
                    //    equipment_type.visual_rep().get_tree(),
                    //    x.value,
                    //    y.value + distance.value,
                    //    scale,
                    //)?;
                }
            }
        }
    }

    if render_connections {
        //TODO: fix this
        for (idx, connection) in project.connections.iter().enumerate() {
            match &connection.end1 {
                ConnectionType::Wire(outer_wire_id) if project.wires.contains_key(outer_wire_id) => match &connection.end2 {
                    ConnectionType::Wire(inner_wire_id) if project.wires.contains_key(inner_wire_id) => {
                        return Err(Error::from(ProjectError::from(ConnectionError::SameType {
                            end1: outer_wire_id.clone(),
                            end2: inner_wire_id.clone(),
                            project_file: connection.contained_datafile_path.clone(),
                            message: Some(
                                "Wires cannot be directly connected. Use an interposing terminal strip or connector".to_owned(),
                            ),
                        })));
                    }
                    ConnectionType::Cable(inner_cable_id) if project.cables.contains_key(inner_cable_id) => {
                        return Err(Error::from(ProjectError::from(ConnectionError::Invalid {
                            end1: outer_wire_id.clone(),
                            end2: inner_cable_id.clone(),
                            project_file: connection.contained_datafile_path.clone(),
                            reason: "Wires and cables cannot be directly connected. Use an interposing terminal strip or \
                                     connector"
                                .to_owned(),
                        })));
                    }
                    ConnectionType::TermCable(inner_term_cable_id) if project.term_cables.contains_key(inner_term_cable_id) => {
                        return Err(Error::from(ProjectError::from(ConnectionError::Invalid {
                            end1: outer_wire_id.clone(),
                            end2: inner_term_cable_id.clone(),
                            project_file: connection.contained_datafile_path.clone(),
                            reason: "Wires and term cables cannot be directly connected. use an interposing connector".to_owned(),
                        })));
                    }
                    ConnectionType::Equipment(inner_equipment_id) if project.equipment.contains_key(inner_equipment_id) => {
                        let equipment =
                            project
                                .equipment
                                .get(inner_equipment_id)
                                .ok_or(Error::from(ProjectError::ValueNotFound {
                                    id: inner_equipment_id.clone(),
                                    found_in: format!("connection: {idx}"),
                                    project_type: "Equipment".to_owned(),
                                }))?;
                        if let Some(equip_enclosure) = equipment.enclosure.clone()
                            && equip_enclosure == *enclosure_id
                        {}
                    }
                    ConnectionType::TerminalStrip(inner_terminal_strip_id)
                        if project.terminal_strips.contains_key(inner_terminal_strip_id) => {}
                    ConnectionType::Connector(inner_connector_id) if project.connectors.contains_key(inner_connector_id) => {}
                    ConnectionType::Wire(inner_wire_id) => {
                        return Err(Error::from(ProjectError::ValueNotFound {
                            id: inner_wire_id.clone(),
                            found_in: format!("connection: {idx}"),
                            project_type: "Wire".to_owned(),
                        }));
                    }
                    ConnectionType::Cable(inner_cable_id) => {
                        return Err(Error::from(ProjectError::ValueNotFound {
                            id: inner_cable_id.clone(),
                            found_in: format!("connection: {idx}"),
                            project_type: "Cable".to_owned(),
                        }));
                    }
                    ConnectionType::TermCable(inner_term_cable_id) => {
                        return Err(Error::from(ProjectError::ValueNotFound {
                            id: inner_term_cable_id.clone(),
                            found_in: format!("connection: {idx}"),
                            project_type: "TermCable".to_owned(),
                        }));
                    }
                    ConnectionType::Equipment(inner_equipment_id) => {
                        return Err(Error::from(ProjectError::ValueNotFound {
                            id: inner_equipment_id.clone(),
                            found_in: format!("connection: {idx}"),
                            project_type: "Equipment".to_owned(),
                        }));
                    }
                    ConnectionType::TerminalStrip(inner_terminal_strip_id) => {
                        return Err(Error::from(ProjectError::ValueNotFound {
                            id: inner_terminal_strip_id.clone(),
                            found_in: format!("connection: {idx}"),
                            project_type: "Terminal Strip".to_owned(),
                        }));
                    }
                    ConnectionType::Connector(inner_connector_id) => {
                        return Err(Error::from(ProjectError::ValueNotFound {
                            id: inner_connector_id.clone(),
                            found_in: format!("connection: {idx}"),
                            project_type: "Connector".to_owned(),
                        }));
                    }
                },
                ConnectionType::Cable(outer_cable_id) if project.term_cables.contains_key(outer_cable_id) => match &connection
                    .end2
                {
                    ConnectionType::Wire(inner_wire_id) if project.wires.contains_key(inner_wire_id) => {
                        return Err(Error::from(ProjectError::from(ConnectionError::Invalid {
                            end1: outer_cable_id.clone(),
                            end2: inner_wire_id.clone(),
                            project_file: connection.contained_datafile_path.clone(),
                            reason: "Wires and cables cannot be directly connected. Use an interposing terminal strip or \
                                     connector"
                                .to_owned(),
                        })));
                    }
                    ConnectionType::Cable(inner_cable_id) if project.cables.contains_key(inner_cable_id) => {
                        return Err(Error::from(ProjectError::from(ConnectionError::SameType {
                            end1: outer_cable_id.clone(),
                            end2: inner_cable_id.clone(),
                            project_file: connection.contained_datafile_path.clone(),
                            message: Some(
                                "Cables cannot be directly connected. Use an interposing terminal strip or connector".to_owned(),
                            ),
                        })));
                    }
                    ConnectionType::TermCable(inner_term_cable_id) if project.term_cables.contains_key(inner_term_cable_id) => {
                        return Err(Error::from(ProjectError::from(ConnectionError::Invalid {
                            end1: outer_cable_id.clone(),
                            end2: inner_term_cable_id.clone(),
                            project_file: connection.contained_datafile_path.clone(),
                            reason: "Cables and term cables cannot be directly connected. use an interposing connector"
                                .to_owned(),
                        })));
                    }
                    ConnectionType::Equipment(inner_equipment_id) if project.equipment.contains_key(inner_equipment_id) => {}
                    ConnectionType::TerminalStrip(inner_terminal_strip_id)
                        if project.terminal_strips.contains_key(inner_terminal_strip_id) => {}
                    ConnectionType::Connector(inner_connector_id) if project.connectors.contains_key(inner_connector_id) => {}
                    ConnectionType::Wire(inner_wire_id) => {
                        return Err(Error::from(ProjectError::ValueNotFound {
                            id: inner_wire_id.clone(),
                            found_in: format!("connection: {idx}"),
                            project_type: "Wire".to_owned(),
                        }));
                    }
                    ConnectionType::Cable(inner_cable_id) => {
                        return Err(Error::from(ProjectError::ValueNotFound {
                            id: inner_cable_id.clone(),
                            found_in: format!("connection: {idx}"),
                            project_type: "Cable".to_owned(),
                        }));
                    }
                    ConnectionType::TermCable(inner_term_cable_id) => {
                        return Err(Error::from(ProjectError::ValueNotFound {
                            id: inner_term_cable_id.clone(),
                            found_in: format!("connection: {idx}"),
                            project_type: "TermCable".to_owned(),
                        }));
                    }
                    ConnectionType::Equipment(inner_equipment_id) => {
                        return Err(Error::from(ProjectError::ValueNotFound {
                            id: inner_equipment_id.clone(),
                            found_in: format!("connection: {idx}"),
                            project_type: "Equipment".to_owned(),
                        }));
                    }
                    ConnectionType::TerminalStrip(inner_terminal_strip_id) => {
                        return Err(Error::from(ProjectError::ValueNotFound {
                            id: inner_terminal_strip_id.clone(),
                            found_in: format!("connection: {idx}"),
                            project_type: "Terminal Strip".to_owned(),
                        }));
                    }
                    ConnectionType::Connector(inner_connector_id) => {
                        return Err(Error::from(ProjectError::ValueNotFound {
                            id: inner_connector_id.clone(),
                            found_in: format!("connection: {idx}"),
                            project_type: "Connector".to_owned(),
                        }));
                    }
                },
                ConnectionType::TermCable(outer_term_cable_id) if project.term_cables.contains_key(outer_term_cable_id) => {
                    match &connection.end2 {
                        ConnectionType::Wire(inner_wire_id) if project.wires.contains_key(inner_wire_id) => {
                            return Err(Error::from(ProjectError::from(ConnectionError::Invalid {
                                end1: outer_term_cable_id.clone(),
                                end2: inner_wire_id.clone(),
                                project_file: connection.contained_datafile_path.clone(),
                                reason: "Wires and term cables cannot be directly connected. use an interposing connector"
                                    .to_owned(),
                            })));
                        }
                        ConnectionType::Cable(inner_cable_id) if project.cables.contains_key(inner_cable_id) => {
                            return Err(Error::from(ProjectError::from(ConnectionError::Invalid {
                                end1: outer_term_cable_id.clone(),
                                end2: inner_cable_id.clone(),
                                project_file: connection.contained_datafile_path.clone(),
                                reason: "Cables and term cables cannot be directly connected. use an interposing connector"
                                    .to_owned(),
                            })));
                        }
                        ConnectionType::TermCable(inner_term_cable_id)
                            if project.term_cables.contains_key(inner_term_cable_id) => {}
                        ConnectionType::Equipment(inner_equipment_id) if project.equipment.contains_key(inner_equipment_id) => {}
                        ConnectionType::TerminalStrip(inner_terminal_strip_id)
                            if project.terminal_strips.contains_key(inner_terminal_strip_id) => {}
                        ConnectionType::Connector(inner_connector_id) if project.connectors.contains_key(inner_connector_id) => {}
                        ConnectionType::Wire(inner_wire_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_wire_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Wire".to_owned(),
                            }));
                        }
                        ConnectionType::Cable(inner_cable_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_cable_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Cable".to_owned(),
                            }));
                        }
                        ConnectionType::TermCable(inner_term_cable_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_term_cable_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "TermCable".to_owned(),
                            }));
                        }
                        ConnectionType::Equipment(inner_equipment_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_equipment_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Equipment".to_owned(),
                            }));
                        }
                        ConnectionType::TerminalStrip(inner_terminal_strip_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_terminal_strip_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Terminal Strip".to_owned(),
                            }));
                        }
                        ConnectionType::Connector(inner_connector_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_connector_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Connector".to_owned(),
                            }));
                        }
                    }
                }
                ConnectionType::Equipment(outer_equipment_id) if project.equipment.contains_key(outer_equipment_id) => {
                    match &connection.end2 {
                        ConnectionType::Wire(inner_wire_id) if project.wires.contains_key(inner_wire_id) => {}
                        ConnectionType::Cable(inner_cable_id) if project.cables.contains_key(inner_cable_id) => {}
                        ConnectionType::TermCable(inner_term_cable_id)
                            if project.term_cables.contains_key(inner_term_cable_id) => {}
                        ConnectionType::Equipment(inner_equipment_id) if project.equipment.contains_key(inner_equipment_id) => {
                            return Err(Error::from(ProjectError::from(ConnectionError::SameType {
                                end1: outer_equipment_id.clone(),
                                end2: inner_equipment_id.clone(),
                                project_file: connection.contained_datafile_path.clone(),
                                message: Some(
                                    "Equipment cannot be directly connected. Use an interposing wire, cable or term cable"
                                        .to_owned(),
                                ),
                            })));
                        }
                        ConnectionType::TerminalStrip(inner_terminal_strip_id)
                            if project.terminal_strips.contains_key(inner_terminal_strip_id) =>
                        {
                            return Err(Error::from(ProjectError::from(ConnectionError::Invalid {
                                end1: outer_equipment_id.clone(),
                                end2: inner_terminal_strip_id.clone(),
                                project_file: connection.contained_datafile_path.clone(),
                                reason: "Equipment and Terminal Strips cannot be directly connected. use an wire, cable or term \
                                         cable"
                                    .to_owned(),
                            })));
                        }
                        ConnectionType::Connector(inner_connector_id) if project.connectors.contains_key(inner_connector_id) => {}
                        ConnectionType::Wire(inner_wire_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_wire_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Wire".to_owned(),
                            }));
                        }
                        ConnectionType::Cable(inner_cable_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_cable_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Cable".to_owned(),
                            }));
                        }
                        ConnectionType::TermCable(inner_term_cable_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_term_cable_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "TermCable".to_owned(),
                            }));
                        }
                        ConnectionType::Equipment(inner_equipment_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_equipment_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Equipment".to_owned(),
                            }));
                        }
                        ConnectionType::TerminalStrip(inner_terminal_strip_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_terminal_strip_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Terminal Strip".to_owned(),
                            }));
                        }
                        ConnectionType::Connector(inner_connector_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_connector_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Connector".to_owned(),
                            }));
                        }
                    }
                }
                ConnectionType::TerminalStrip(outer_terminal_strip_id)
                    if project.terminal_strips.contains_key(outer_terminal_strip_id) =>
                {
                    match &connection.end2 {
                        ConnectionType::Wire(inner_wire_id) if project.wires.contains_key(inner_wire_id) => {}
                        ConnectionType::Cable(inner_cable_id) if project.cables.contains_key(inner_cable_id) => {}
                        ConnectionType::TermCable(inner_term_cable_id)
                            if project.term_cables.contains_key(inner_term_cable_id) => {}
                        ConnectionType::Equipment(inner_equipment_id) if project.equipment.contains_key(inner_equipment_id) => {
                            return Err(Error::from(ProjectError::from(ConnectionError::Invalid {
                                end1: outer_terminal_strip_id.clone(),
                                end2: inner_equipment_id.clone(),
                                project_file: connection.contained_datafile_path.clone(),
                                reason: "Equipment and Terminal Strips cannot be directly connected. use an wire, cable or term \
                                         cable"
                                    .to_owned(),
                            })));
                        }
                        ConnectionType::TerminalStrip(inner_terminal_strip_id)
                            if project.terminal_strips.contains_key(inner_terminal_strip_id) =>
                        {
                            return Err(Error::from(ProjectError::from(ConnectionError::SameType {
                                end1: outer_terminal_strip_id.clone(),
                                end2: inner_terminal_strip_id.clone(),
                                project_file: connection.contained_datafile_path.clone(),
                                message: Some(
                                    "Terminal Strips cannot be directly connected. Use an interposing wire, cable or term cable"
                                        .to_owned(),
                                ),
                            })));
                        }
                        ConnectionType::Connector(inner_connector_id) if project.connectors.contains_key(inner_connector_id) => {}
                        ConnectionType::Wire(inner_wire_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_wire_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Wire".to_owned(),
                            }));
                        }
                        ConnectionType::Cable(inner_cable_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_cable_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Cable".to_owned(),
                            }));
                        }
                        ConnectionType::TermCable(inner_term_cable_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_term_cable_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "TermCable".to_owned(),
                            }));
                        }
                        ConnectionType::Equipment(inner_equipment_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_equipment_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Equipment".to_owned(),
                            }));
                        }
                        ConnectionType::TerminalStrip(inner_terminal_strip_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_terminal_strip_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Terminal Strip".to_owned(),
                            }));
                        }
                        ConnectionType::Connector(inner_connector_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_connector_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Connector".to_owned(),
                            }));
                        }
                    }
                }
                ConnectionType::Connector(outer_connector_id) if project.connectors.contains_key(outer_connector_id) => {
                    match &connection.end2 {
                        ConnectionType::Wire(inner_wire_id) if project.wires.contains_key(inner_wire_id) => {}
                        ConnectionType::Cable(inner_cable_id) if project.cables.contains_key(inner_cable_id) => {}
                        ConnectionType::TermCable(inner_term_cable_id)
                            if project.term_cables.contains_key(inner_term_cable_id) => {}
                        ConnectionType::Equipment(inner_equipment_id) if project.equipment.contains_key(inner_equipment_id) => {}
                        ConnectionType::TerminalStrip(inner_terminal_strip_id)
                            if project.terminal_strips.contains_key(inner_terminal_strip_id) => {}
                        ConnectionType::Connector(inner_connector_id) if project.connectors.contains_key(inner_connector_id) => {}
                        ConnectionType::Wire(inner_wire_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_wire_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Wire".to_owned(),
                            }));
                        }
                        ConnectionType::Cable(inner_cable_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_cable_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Cable".to_owned(),
                            }));
                        }
                        ConnectionType::TermCable(inner_term_cable_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_term_cable_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "TermCable".to_owned(),
                            }));
                        }
                        ConnectionType::Equipment(inner_equipment_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_equipment_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Equipment".to_owned(),
                            }));
                        }
                        ConnectionType::TerminalStrip(inner_terminal_strip_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_terminal_strip_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Terminal Strip".to_owned(),
                            }));
                        }
                        ConnectionType::Connector(inner_connector_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_connector_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Connector".to_owned(),
                            }));
                        }
                    }
                }
                ConnectionType::Wire(outer_wire_id) => {
                    return Err(Error::from(ProjectError::ValueNotFound {
                        id: outer_wire_id.clone(),
                        found_in: format!("connection: {idx}"),
                        project_type: "Wire".to_owned(),
                    }));
                }
                ConnectionType::Cable(outer_cable_id) => {
                    return Err(Error::from(ProjectError::ValueNotFound {
                        id: outer_cable_id.clone(),
                        found_in: format!("connection: {idx}"),
                        project_type: "Cable".to_owned(),
                    }));
                }
                ConnectionType::TermCable(outer_term_cable_id) => {
                    return Err(Error::from(ProjectError::ValueNotFound {
                        id: outer_term_cable_id.clone(),
                        found_in: format!("connection: {idx}"),
                        project_type: "TermCable".to_owned(),
                    }));
                }
                ConnectionType::Equipment(outer_equipment_id) => {
                    return Err(Error::from(ProjectError::ValueNotFound {
                        id: outer_equipment_id.clone(),
                        found_in: format!("connection: {idx}"),
                        project_type: "Equipment".to_owned(),
                    }));
                }
                ConnectionType::TerminalStrip(outer_terminal_strip_id) => {
                    return Err(Error::from(ProjectError::ValueNotFound {
                        id: outer_terminal_strip_id.clone(),
                        found_in: format!("connection: {idx}"),
                        project_type: "Terminal Strip".to_owned(),
                    }));
                }
                ConnectionType::Connector(outer_connector_id) => {
                    return Err(Error::from(ProjectError::ValueNotFound {
                        id: outer_connector_id.clone(),
                        found_in: format!("connection: {idx}"),
                        project_type: "Connector".to_owned(),
                    }));
                }
            }
        }
    }
    //TODO: now need to figure out which equipment is connected to which other equipment in the
    //same location and draw those connections and pathways
    //
    //Then draw connections off page gated behind a boolean

    Ok(())
}

/// `render_enclosure_schematic_ladder` generates one PDF page with the schematic for one enclosure
/// in the project.
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
/// * `pdf_page` - a particular page within a pdf that this enclosure will be rendered on.
/// * `render_connections` - if connections with enclosure should be rendered.
///
/// # Errors
///
/// Will Error if the location doesn't fit on page at specified scale or if there is no equipment
/// at the specified location.
///
/// Will Error if any of the data in `project` or `library` is incorrect.
///
/// # Panics
///
/// Will panic if key is not found for value in enclosures hashmap.
#[expect(clippy::too_many_lines, reason = "hard to split up this function")]
//TODO: revisit the lint below
#[expect(clippy::pattern_type_mismatch, reason = "revisit this")]
#[inline(never)]
#[expect(clippy::result_large_err, reason = "Don't want to have to split up error::Error ")]
#[expect(unused_variables, reason = "function not finished yet")]
pub fn render_enclosure_schematic_ladder(
    project: &Project,
    library: &Library,
    enclosure: &Enclosure,
    scale: Rational64,
    pdf_page: &mut PDFPage,
    render_connections: bool,
) -> Result<(), Error> {
    // layout all equipment in location

    if project.equipment.is_empty() {
        return Err(PDFError::PDFCreationError("no equipment in location".to_owned()).into());
    }
    let page_width = pdf_page.page_size.size().0;
    let page_height = pdf_page.page_size.size().1;
    #[expect(
        clippy::expect_used,
        reason = "if the key of the value can't be found when searching by value, with both in the map, something went \
                  seriously wrong"
    )]
    let enclosure_id = project
        .enclosures
        .iter()
        .find_map(|(key, val)| (val == enclosure).then_some(key))
        .expect("Enclosure ID not found in map when searching by value. Something went seriously wrong.");
    let enclosure_type = library
        .enclosure_types
        .get(&enclosure.enclosure_type)
        .ok_or(LibraryError::ValueNotFound {
            id: enclosure.enclosure_type.clone(),
            found_in: enclosure_id.clone(),
            library_type: "Enclosure Type".to_owned(),
        })?;
    let enclosure_type_dimensions = enclosure_type.dimensions.clone();
    // check if location will fit within page at 1:1 scale
    #[expect(clippy::arithmetic_side_effects, reason = "no good way around this")]
    let enclosure_default_scale_fit = {
        enclosure_type_dimensions.width.value < (page_width - pdf_page.margins.left - pdf_page.margins.right)
            && enclosure_type_dimensions.height.value < (page_height - pdf_page.margins.top - pdf_page.margins.bottom)
    };

    // check if location will fit on page at specified scale
    #[expect(clippy::arithmetic_side_effects, reason = "no good way around this")]
    let enclosure_scale_fit = {
        (enclosure_type_dimensions.width.value * scale) < (page_width - pdf_page.margins.left - pdf_page.margins.right)
            && (enclosure_type_dimensions.height.value * scale) < (page_height - pdf_page.margins.top - pdf_page.margins.bottom)
    };

    if enclosure_default_scale_fit {
        info!("location fits within page at 1:1 scale.");
    } else if enclosure_scale_fit {
        trace!("location fits within page at {scale} scale");
    } else {
        return Err(PDFError::LayoutError(format!(
            "Enclosure {} did not fit on Page Size {} at scale: {}",
            enclosure_id, pdf_page.page_size, scale,
        ))
        .into());
    }
    // loop through all mounting_points in Enclosure and render them if they are mounting rails
    let mut mounting_rails_rendered = Vec::new();
    for (mount_id, mount_point) in &enclosure.mount_points {
        if let MountPoint::MountingRail { x, y, mounting_rail, .. } = mount_point
            && !mounting_rails_rendered.contains(mount_id)
        // only want to render each rail once
        {
            pdf_page.add_svg(
                project
                    .mounting_rails
                    .get(mounting_rail)
                    .ok_or(ProjectError::ValueNotFound {
                        id: mounting_rail.clone(),
                        found_in: enclosure_id.clone(),
                        project_type: "MountingRail".to_owned(),
                    })?
                    .vis_rep(library)?
                    .get_tree()?,
                x.value,
                y.value,
                scale,
            )?;
            // only want to render each rail once
            mounting_rails_rendered.push(mount_id.clone());
        }
    }

    // loop through all equipment in project and render if in enclosure
    for (equipment_id, equipment) in &project.equipment {
        // select equipment that is within enclosure
        if let Some(equip_enclosure_id) = equipment.enclosure.clone()
            && equip_enclosure_id == *enclosure_id
        {
            let equipment_type = library
                .equipment_types
                .get(&equipment.equipment_type)
                .ok_or(LibraryError::ValueNotFound {
                    id: equipment.equipment_type.clone(),
                    found_in: equipment_id.clone(),
                    library_type: "Equipment_type".to_owned(),
                })?;
            let equipment_mount_point_id = equipment.mount_point.clone().ok_or(ProjectError::DataMissing {
                id: equipment_id.clone(),
                found_in: equipment_id.clone(),
                project_type: "Equipment".to_owned(),
                data_missing: "mount point".to_owned(),
            })?;
            let equipment_mount_point =
                enclosure
                    .mount_points
                    .get(&equipment_mount_point_id)
                    .ok_or(ProjectError::DataMissing {
                        id: enclosure_id.clone(),
                        found_in: equipment_id.clone(),
                        project_type: "Enclosure".to_owned(),
                        data_missing: format!("mount_point with id {equipment_mount_point_id}").to_owned(),
                    })?;
            match equipment_mount_point {
                MountPoint::CoordinatePair { x, y } => {
                    //pdf_page.add_svg(equipment_type.visual_rep().get_tree(), x.value, y.value,
                    // scale)?;
                }
                MountPoint::MountingRail { distance, x, y, .. } => {
                    //#[expect(clippy::arithmetic_side_effects, reason = "no good way around this")]
                    //pdf_page.add_svg(
                    //    equipment_type.visual_rep().get_tree(),
                    //    x.value,
                    //    y.value + distance.value,
                    //    scale,
                    //)?;
                }
            }
        }
    }

    if render_connections {
        //TODO: fix this
        for (idx, connection) in project.connections.iter().enumerate() {
            match &connection.end1 {
                ConnectionType::Wire(outer_wire_id) if project.wires.contains_key(outer_wire_id) => match &connection.end2 {
                    ConnectionType::Wire(inner_wire_id) if project.wires.contains_key(inner_wire_id) => {
                        return Err(Error::from(ProjectError::from(ConnectionError::SameType {
                            end1: outer_wire_id.clone(),
                            end2: inner_wire_id.clone(),
                            project_file: connection.contained_datafile_path.clone(),
                            message: Some(
                                "Wires cannot be directly connected. Use an interposing terminal strip or connector".to_owned(),
                            ),
                        })));
                    }
                    ConnectionType::Cable(inner_cable_id) if project.cables.contains_key(inner_cable_id) => {
                        return Err(Error::from(ProjectError::from(ConnectionError::Invalid {
                            end1: outer_wire_id.clone(),
                            end2: inner_cable_id.clone(),
                            project_file: connection.contained_datafile_path.clone(),
                            reason: "Wires and cables cannot be directly connected. Use an interposing terminal strip or \
                                     connector"
                                .to_owned(),
                        })));
                    }
                    ConnectionType::TermCable(inner_term_cable_id) if project.term_cables.contains_key(inner_term_cable_id) => {
                        return Err(Error::from(ProjectError::from(ConnectionError::Invalid {
                            end1: outer_wire_id.clone(),
                            end2: inner_term_cable_id.clone(),
                            project_file: connection.contained_datafile_path.clone(),
                            reason: "Wires and term cables cannot be directly connected. use an interposing connector".to_owned(),
                        })));
                    }
                    ConnectionType::Equipment(inner_equipment_id) if project.equipment.contains_key(inner_equipment_id) => {
                        let equipment =
                            project
                                .equipment
                                .get(inner_equipment_id)
                                .ok_or(Error::from(ProjectError::ValueNotFound {
                                    id: inner_equipment_id.clone(),
                                    found_in: format!("connection: {idx}"),
                                    project_type: "Equipment".to_owned(),
                                }))?;
                        if let Some(equip_enclosure) = equipment.enclosure.clone()
                            && equip_enclosure == *enclosure_id
                        {}
                    }
                    ConnectionType::TerminalStrip(inner_terminal_strip_id)
                        if project.terminal_strips.contains_key(inner_terminal_strip_id) => {}
                    ConnectionType::Connector(inner_connector_id) if project.connectors.contains_key(inner_connector_id) => {}
                    ConnectionType::Wire(inner_wire_id) => {
                        return Err(Error::from(ProjectError::ValueNotFound {
                            id: inner_wire_id.clone(),
                            found_in: format!("connection: {idx}"),
                            project_type: "Wire".to_owned(),
                        }));
                    }
                    ConnectionType::Cable(inner_cable_id) => {
                        return Err(Error::from(ProjectError::ValueNotFound {
                            id: inner_cable_id.clone(),
                            found_in: format!("connection: {idx}"),
                            project_type: "Cable".to_owned(),
                        }));
                    }
                    ConnectionType::TermCable(inner_term_cable_id) => {
                        return Err(Error::from(ProjectError::ValueNotFound {
                            id: inner_term_cable_id.clone(),
                            found_in: format!("connection: {idx}"),
                            project_type: "TermCable".to_owned(),
                        }));
                    }
                    ConnectionType::Equipment(inner_equipment_id) => {
                        return Err(Error::from(ProjectError::ValueNotFound {
                            id: inner_equipment_id.clone(),
                            found_in: format!("connection: {idx}"),
                            project_type: "Equipment".to_owned(),
                        }));
                    }
                    ConnectionType::TerminalStrip(inner_terminal_strip_id) => {
                        return Err(Error::from(ProjectError::ValueNotFound {
                            id: inner_terminal_strip_id.clone(),
                            found_in: format!("connection: {idx}"),
                            project_type: "Terminal Strip".to_owned(),
                        }));
                    }
                    ConnectionType::Connector(inner_connector_id) => {
                        return Err(Error::from(ProjectError::ValueNotFound {
                            id: inner_connector_id.clone(),
                            found_in: format!("connection: {idx}"),
                            project_type: "Connector".to_owned(),
                        }));
                    }
                },
                ConnectionType::Cable(outer_cable_id) if project.term_cables.contains_key(outer_cable_id) => match &connection
                    .end2
                {
                    ConnectionType::Wire(inner_wire_id) if project.wires.contains_key(inner_wire_id) => {
                        return Err(Error::from(ProjectError::from(ConnectionError::Invalid {
                            end1: outer_cable_id.clone(),
                            end2: inner_wire_id.clone(),
                            project_file: connection.contained_datafile_path.clone(),
                            reason: "Wires and cables cannot be directly connected. Use an interposing terminal strip or \
                                     connector"
                                .to_owned(),
                        })));
                    }
                    ConnectionType::Cable(inner_cable_id) if project.cables.contains_key(inner_cable_id) => {
                        return Err(Error::from(ProjectError::from(ConnectionError::SameType {
                            end1: outer_cable_id.clone(),
                            end2: inner_cable_id.clone(),
                            project_file: connection.contained_datafile_path.clone(),
                            message: Some(
                                "Cables cannot be directly connected. Use an interposing terminal strip or connector".to_owned(),
                            ),
                        })));
                    }
                    ConnectionType::TermCable(inner_term_cable_id) if project.term_cables.contains_key(inner_term_cable_id) => {
                        return Err(Error::from(ProjectError::from(ConnectionError::Invalid {
                            end1: outer_cable_id.clone(),
                            end2: inner_term_cable_id.clone(),
                            project_file: connection.contained_datafile_path.clone(),
                            reason: "Cables and term cables cannot be directly connected. use an interposing connector"
                                .to_owned(),
                        })));
                    }
                    ConnectionType::Equipment(inner_equipment_id) if project.equipment.contains_key(inner_equipment_id) => {}
                    ConnectionType::TerminalStrip(inner_terminal_strip_id)
                        if project.terminal_strips.contains_key(inner_terminal_strip_id) => {}
                    ConnectionType::Connector(inner_connector_id) if project.connectors.contains_key(inner_connector_id) => {}
                    ConnectionType::Wire(inner_wire_id) => {
                        return Err(Error::from(ProjectError::ValueNotFound {
                            id: inner_wire_id.clone(),
                            found_in: format!("connection: {idx}"),
                            project_type: "Wire".to_owned(),
                        }));
                    }
                    ConnectionType::Cable(inner_cable_id) => {
                        return Err(Error::from(ProjectError::ValueNotFound {
                            id: inner_cable_id.clone(),
                            found_in: format!("connection: {idx}"),
                            project_type: "Cable".to_owned(),
                        }));
                    }
                    ConnectionType::TermCable(inner_term_cable_id) => {
                        return Err(Error::from(ProjectError::ValueNotFound {
                            id: inner_term_cable_id.clone(),
                            found_in: format!("connection: {idx}"),
                            project_type: "TermCable".to_owned(),
                        }));
                    }
                    ConnectionType::Equipment(inner_equipment_id) => {
                        return Err(Error::from(ProjectError::ValueNotFound {
                            id: inner_equipment_id.clone(),
                            found_in: format!("connection: {idx}"),
                            project_type: "Equipment".to_owned(),
                        }));
                    }
                    ConnectionType::TerminalStrip(inner_terminal_strip_id) => {
                        return Err(Error::from(ProjectError::ValueNotFound {
                            id: inner_terminal_strip_id.clone(),
                            found_in: format!("connection: {idx}"),
                            project_type: "Terminal Strip".to_owned(),
                        }));
                    }
                    ConnectionType::Connector(inner_connector_id) => {
                        return Err(Error::from(ProjectError::ValueNotFound {
                            id: inner_connector_id.clone(),
                            found_in: format!("connection: {idx}"),
                            project_type: "Connector".to_owned(),
                        }));
                    }
                },
                ConnectionType::TermCable(outer_term_cable_id) if project.term_cables.contains_key(outer_term_cable_id) => {
                    match &connection.end2 {
                        ConnectionType::Wire(inner_wire_id) if project.wires.contains_key(inner_wire_id) => {
                            return Err(Error::from(ProjectError::from(ConnectionError::Invalid {
                                end1: outer_term_cable_id.clone(),
                                end2: inner_wire_id.clone(),
                                project_file: connection.contained_datafile_path.clone(),
                                reason: "Wires and term cables cannot be directly connected. use an interposing connector"
                                    .to_owned(),
                            })));
                        }
                        ConnectionType::Cable(inner_cable_id) if project.cables.contains_key(inner_cable_id) => {
                            return Err(Error::from(ProjectError::from(ConnectionError::Invalid {
                                end1: outer_term_cable_id.clone(),
                                end2: inner_cable_id.clone(),
                                project_file: connection.contained_datafile_path.clone(),
                                reason: "Cables and term cables cannot be directly connected. use an interposing connector"
                                    .to_owned(),
                            })));
                        }
                        ConnectionType::TermCable(inner_term_cable_id)
                            if project.term_cables.contains_key(inner_term_cable_id) => {}
                        ConnectionType::Equipment(inner_equipment_id) if project.equipment.contains_key(inner_equipment_id) => {}
                        ConnectionType::TerminalStrip(inner_terminal_strip_id)
                            if project.terminal_strips.contains_key(inner_terminal_strip_id) => {}
                        ConnectionType::Connector(inner_connector_id) if project.connectors.contains_key(inner_connector_id) => {}
                        ConnectionType::Wire(inner_wire_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_wire_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Wire".to_owned(),
                            }));
                        }
                        ConnectionType::Cable(inner_cable_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_cable_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Cable".to_owned(),
                            }));
                        }
                        ConnectionType::TermCable(inner_term_cable_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_term_cable_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "TermCable".to_owned(),
                            }));
                        }
                        ConnectionType::Equipment(inner_equipment_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_equipment_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Equipment".to_owned(),
                            }));
                        }
                        ConnectionType::TerminalStrip(inner_terminal_strip_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_terminal_strip_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Terminal Strip".to_owned(),
                            }));
                        }
                        ConnectionType::Connector(inner_connector_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_connector_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Connector".to_owned(),
                            }));
                        }
                    }
                }
                ConnectionType::Equipment(outer_equipment_id) if project.equipment.contains_key(outer_equipment_id) => {
                    match &connection.end2 {
                        ConnectionType::Wire(inner_wire_id) if project.wires.contains_key(inner_wire_id) => {}
                        ConnectionType::Cable(inner_cable_id) if project.cables.contains_key(inner_cable_id) => {}
                        ConnectionType::TermCable(inner_term_cable_id)
                            if project.term_cables.contains_key(inner_term_cable_id) => {}
                        ConnectionType::Equipment(inner_equipment_id) if project.equipment.contains_key(inner_equipment_id) => {
                            return Err(Error::from(ProjectError::from(ConnectionError::SameType {
                                end1: outer_equipment_id.clone(),
                                end2: inner_equipment_id.clone(),
                                project_file: connection.contained_datafile_path.clone(),
                                message: Some(
                                    "Equipment cannot be directly connected. Use an interposing wire, cable or term cable"
                                        .to_owned(),
                                ),
                            })));
                        }
                        ConnectionType::TerminalStrip(inner_terminal_strip_id)
                            if project.terminal_strips.contains_key(inner_terminal_strip_id) =>
                        {
                            return Err(Error::from(ProjectError::from(ConnectionError::Invalid {
                                end1: outer_equipment_id.clone(),
                                end2: inner_terminal_strip_id.clone(),
                                project_file: connection.contained_datafile_path.clone(),
                                reason: "Equipment and Terminal Strips cannot be directly connected. use an wire, cable or term \
                                         cable"
                                    .to_owned(),
                            })));
                        }
                        ConnectionType::Connector(inner_connector_id) if project.connectors.contains_key(inner_connector_id) => {}
                        ConnectionType::Wire(inner_wire_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_wire_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Wire".to_owned(),
                            }));
                        }
                        ConnectionType::Cable(inner_cable_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_cable_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Cable".to_owned(),
                            }));
                        }
                        ConnectionType::TermCable(inner_term_cable_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_term_cable_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "TermCable".to_owned(),
                            }));
                        }
                        ConnectionType::Equipment(inner_equipment_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_equipment_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Equipment".to_owned(),
                            }));
                        }
                        ConnectionType::TerminalStrip(inner_terminal_strip_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_terminal_strip_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Terminal Strip".to_owned(),
                            }));
                        }
                        ConnectionType::Connector(inner_connector_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_connector_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Connector".to_owned(),
                            }));
                        }
                    }
                }
                ConnectionType::TerminalStrip(outer_terminal_strip_id)
                    if project.terminal_strips.contains_key(outer_terminal_strip_id) =>
                {
                    match &connection.end2 {
                        ConnectionType::Wire(inner_wire_id) if project.wires.contains_key(inner_wire_id) => {}
                        ConnectionType::Cable(inner_cable_id) if project.cables.contains_key(inner_cable_id) => {}
                        ConnectionType::TermCable(inner_term_cable_id)
                            if project.term_cables.contains_key(inner_term_cable_id) => {}
                        ConnectionType::Equipment(inner_equipment_id) if project.equipment.contains_key(inner_equipment_id) => {
                            return Err(Error::from(ProjectError::from(ConnectionError::Invalid {
                                end1: outer_terminal_strip_id.clone(),
                                end2: inner_equipment_id.clone(),
                                project_file: connection.contained_datafile_path.clone(),
                                reason: "Equipment and Terminal Strips cannot be directly connected. use an wire, cable or term \
                                         cable"
                                    .to_owned(),
                            })));
                        }
                        ConnectionType::TerminalStrip(inner_terminal_strip_id)
                            if project.terminal_strips.contains_key(inner_terminal_strip_id) =>
                        {
                            return Err(Error::from(ProjectError::from(ConnectionError::SameType {
                                end1: outer_terminal_strip_id.clone(),
                                end2: inner_terminal_strip_id.clone(),
                                project_file: connection.contained_datafile_path.clone(),
                                message: Some(
                                    "Terminal Strips cannot be directly connected. Use an interposing wire, cable or term cable"
                                        .to_owned(),
                                ),
                            })));
                        }
                        ConnectionType::Connector(inner_connector_id) if project.connectors.contains_key(inner_connector_id) => {}
                        ConnectionType::Wire(inner_wire_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_wire_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Wire".to_owned(),
                            }));
                        }
                        ConnectionType::Cable(inner_cable_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_cable_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Cable".to_owned(),
                            }));
                        }
                        ConnectionType::TermCable(inner_term_cable_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_term_cable_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "TermCable".to_owned(),
                            }));
                        }
                        ConnectionType::Equipment(inner_equipment_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_equipment_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Equipment".to_owned(),
                            }));
                        }
                        ConnectionType::TerminalStrip(inner_terminal_strip_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_terminal_strip_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Terminal Strip".to_owned(),
                            }));
                        }
                        ConnectionType::Connector(inner_connector_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_connector_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Connector".to_owned(),
                            }));
                        }
                    }
                }
                ConnectionType::Connector(outer_connector_id) if project.connectors.contains_key(outer_connector_id) => {
                    match &connection.end2 {
                        ConnectionType::Wire(inner_wire_id) if project.wires.contains_key(inner_wire_id) => {}
                        ConnectionType::Cable(inner_cable_id) if project.cables.contains_key(inner_cable_id) => {}
                        ConnectionType::TermCable(inner_term_cable_id)
                            if project.term_cables.contains_key(inner_term_cable_id) => {}
                        ConnectionType::Equipment(inner_equipment_id) if project.equipment.contains_key(inner_equipment_id) => {}
                        ConnectionType::TerminalStrip(inner_terminal_strip_id)
                            if project.terminal_strips.contains_key(inner_terminal_strip_id) => {}
                        ConnectionType::Connector(inner_connector_id) if project.connectors.contains_key(inner_connector_id) => {}
                        ConnectionType::Wire(inner_wire_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_wire_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Wire".to_owned(),
                            }));
                        }
                        ConnectionType::Cable(inner_cable_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_cable_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Cable".to_owned(),
                            }));
                        }
                        ConnectionType::TermCable(inner_term_cable_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_term_cable_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "TermCable".to_owned(),
                            }));
                        }
                        ConnectionType::Equipment(inner_equipment_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_equipment_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Equipment".to_owned(),
                            }));
                        }
                        ConnectionType::TerminalStrip(inner_terminal_strip_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_terminal_strip_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Terminal Strip".to_owned(),
                            }));
                        }
                        ConnectionType::Connector(inner_connector_id) => {
                            return Err(Error::from(ProjectError::ValueNotFound {
                                id: inner_connector_id.clone(),
                                found_in: format!("connection: {idx}"),
                                project_type: "Connector".to_owned(),
                            }));
                        }
                    }
                }
                ConnectionType::Wire(outer_wire_id) => {
                    return Err(Error::from(ProjectError::ValueNotFound {
                        id: outer_wire_id.clone(),
                        found_in: format!("connection: {idx}"),
                        project_type: "Wire".to_owned(),
                    }));
                }
                ConnectionType::Cable(outer_cable_id) => {
                    return Err(Error::from(ProjectError::ValueNotFound {
                        id: outer_cable_id.clone(),
                        found_in: format!("connection: {idx}"),
                        project_type: "Cable".to_owned(),
                    }));
                }
                ConnectionType::TermCable(outer_term_cable_id) => {
                    return Err(Error::from(ProjectError::ValueNotFound {
                        id: outer_term_cable_id.clone(),
                        found_in: format!("connection: {idx}"),
                        project_type: "TermCable".to_owned(),
                    }));
                }
                ConnectionType::Equipment(outer_equipment_id) => {
                    return Err(Error::from(ProjectError::ValueNotFound {
                        id: outer_equipment_id.clone(),
                        found_in: format!("connection: {idx}"),
                        project_type: "Equipment".to_owned(),
                    }));
                }
                ConnectionType::TerminalStrip(outer_terminal_strip_id) => {
                    return Err(Error::from(ProjectError::ValueNotFound {
                        id: outer_terminal_strip_id.clone(),
                        found_in: format!("connection: {idx}"),
                        project_type: "Terminal Strip".to_owned(),
                    }));
                }
                ConnectionType::Connector(outer_connector_id) => {
                    return Err(Error::from(ProjectError::ValueNotFound {
                        id: outer_connector_id.clone(),
                        found_in: format!("connection: {idx}"),
                        project_type: "Connector".to_owned(),
                    }));
                }
            }
        }
    }
    //TODO: now need to figure out which equipment is connected to which other equipment in the
    //same location and draw those connections and pathways
    //
    //Then draw connections off page gated behind a boolean

    Ok(())
}

/// `schematic_ladder` generates the schematic of the entire project and tries to place it on
/// pdf pages of the specified size.
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
/// * `pdf_page` - a particular page within a pdf that this enclosure will be rendered on.
/// * `render_connections` - if connections with enclosure should be rendered.
///
/// # Errors
///
/// Will Error if the location doesn't fit on page at specified scale or if there is no equipment
/// at the specified location.
///
/// Will Error if any of the data in `project` or `library` is incorrect.
///
/// # Panics
///
/// Will panic if key is not found for value in enclosures hashmap.
#[expect(clippy::result_large_err, reason = "Don't want to have to split up error::Error ")]
#[expect(unused_variables, reason = "function not finished yet")]
#[inline(never)]
pub fn render_schematic_ladder(
    project: &Project,
    library: &Library,
    page_size: PaperSize,
    scale: Rational64,
) -> Result<(), Error> {
    //    // layout all equipment in location
    //
    //    if project.equipment.is_empty() {
    //        return Err(PDFGenerationError::PDFCreationError("no equipment in
    // location".to_owned()).into());    }
    //    let page_width = pdf_page.page_size.size().0;
    //    let page_height = pdf_page.page_size.size().1;
    //    #[expect(
    //        clippy::expect_used,
    //        reason = "if the key of the value can't be found when searching by value, with both in the
    // map, something went \                  seriously wrong"
    //    )]
    //    let enclosure_id = project
    //        .enclosures
    //        .iter()
    //        .find_map(|(key, val)| (val == enclosure).then_some(key))
    //        .expect("Enclosure ID not found in map when searching by value. Something went seriously
    // wrong.");    let enclosure_type = library
    //        .enclosure_types
    //        .get(&enclosure.enclosure_type)
    //        .ok_or(LibraryError::ValueNotFound {
    //            id: enclosure.enclosure_type.clone(),
    //            library_type: "Enclosure Type".to_owned(),
    //        })?;
    //    let enclosure_type_dimensions = enclosure_type.dimensions.clone();
    //    // check if location will fit within page at 1:1 scale
    //    #[expect(clippy::arithmetic_side_effects, reason = "no good way around this")]
    //    let enclosure_default_scale_fit = {
    //        enclosure_type_dimensions.width.value < (page_width - pdf_page.margins.left -
    // pdf_page.margins.right)            && enclosure_type_dimensions.height.value < (page_height -
    // pdf_page.margins.top - pdf_page.margins.bottom)    };
    //
    //    // check if location will fit on page at specified scale
    //    #[expect(clippy::arithmetic_side_effects, reason = "no good way around this")]
    //    let enclosure_scale_fit = {
    //        (enclosure_type_dimensions.width.value * scale) < (page_width - pdf_page.margins.left -
    // pdf_page.margins.right)            && (enclosure_type_dimensions.height.value * scale) <
    // (page_height - pdf_page.margins.top - pdf_page.margins.bottom)    };
    //
    //    if enclosure_default_scale_fit {
    //        info!("location fits within page at 1:1 scale.");
    //    } else if enclosure_scale_fit {
    //        trace!("location fits within page at {scale} scale");
    //    } else {
    //        return Err(PDFGenerationError::LayoutError(format!(
    //            "Enclosure {} did not fit on Page Size {} at scale: {}",
    //            enclosure_id, pdf_page.page_size, scale,
    //        ))
    //        .into());
    //    }
    //    // loop through all mounting_points in Enclosure and render them if they are mounting rails
    //    let mut mounting_rails_rendered = Vec::new();
    //    for (mount_id, mount_point) in &enclosure.mount_points {
    //        if let MountPoint::MountingRail { x, y, mounting_rail, .. } = mount_point
    //            // only want to render each rail once
    //            && !mounting_rails_rendered.contains(mount_id)
    //        {
    //            pdf_page.add_svg(
    //                project
    //                    .mounting_rails
    //                    .get(mounting_rail)
    //                    .ok_or(ProjectError::ValueNotFound {
    //                        id: mounting_rail.clone(),
    //                        project_type: "MountingRail".to_owned(),
    //                    })?
    //                    .vis_rep(library)?
    //                    .get_tree(),
    //                x.value,
    //                y.value,
    //                scale,
    //            )?;
    //            // only want to render each rail once
    //            mounting_rails_rendered.push(mount_id.clone());
    //        }
    //    }
    //
    //    // loop through all equipment in project and render if in enclosure
    //    for (equipment_id, equipment) in &project.equipment {
    //        // select equipment that is within enclosure
    //        if let Some(equip_enclosure_id) = equipment.enclosure.clone()
    //            && equip_enclosure_id == *enclosure_id
    //        {
    //            let equipment_type = library
    //                .equipment_types
    //                .get(&equipment.equipment_type)
    //                .ok_or(LibraryError::ValueNotFound {
    //                    id: equipment.equipment_type.clone(),
    //                    library_type: "Equipment_type".to_owned(),
    //                })?;
    //            let equipment_mount_point_id =
    // equipment.mount_point.clone().ok_or(ProjectError::DataMissing {                id:
    // equipment_id.clone(),                project_type: "Equipment".to_owned(),
    //                data_missing: "mount point".to_owned(),
    //            })?;
    //            let equipment_mount_point =
    //                enclosure
    //                    .mount_points
    //                    .get(&equipment_mount_point_id)
    //                    .ok_or(ProjectError::DataMissing {
    //                        id: enclosure_id.clone(),
    //                        project_type: "Enclosure".to_owned(),
    //                        data_missing: format!("mount_point with id
    // {equipment_mount_point_id}").to_owned(),                    })?;
    //            match equipment_mount_point {
    //                MountPoint::CoordinatePair { x, y } => {
    //                    //pdf_page.add_svg(equipment_type.visual_rep().get_tree(), x.value, y.value,
    //                    // scale)?;
    //                }
    //                MountPoint::MountingRail { distance, x, y, .. } => {
    //                    //#[expect(clippy::arithmetic_side_effects, reason = "no good way around
    // this")]                    //pdf_page.add_svg(
    //                    //    equipment_type.visual_rep().get_tree(),
    //                    //    x.value,
    //                    //    y.value + distance.value,
    //                    //    scale,
    //                    //)?;
    //                }
    //            }
    //        }
    //    }
    //
    //    if render_connections {
    //        //TODO: fix this
    //        for connection in &project.connections {
    //            match &connection.end1 {
    //                ConnectionType::Wire(outer_wire_id) if project.wires.contains_key(outer_wire_id)
    // => match &connection.end2 {                    ConnectionType::Wire(inner_wire_id) if
    // project.wires.contains_key(inner_wire_id) => {                        return
    // Err(Error::from(ProjectError::from(ConnectionError::SameType {
    // end1: outer_wire_id.clone(),                            end2: inner_wire_id.clone(),
    //                            project_file: connection.contained_datafile_path.clone(),
    //                            message: Some(
    //                                "Wires cannot be directly connected. Use an interposing terminal
    // strip or connector".to_owned(),                            ),
    //                        })));
    //                    }
    //                    ConnectionType::Cable(inner_cable_id) if
    // project.cables.contains_key(inner_cable_id) => {                        return
    // Err(Error::from(ProjectError::from(ConnectionError::Invalid {
    // end1: outer_wire_id.clone(),                            end2: inner_cable_id.clone(),
    //                            project_file: connection.contained_datafile_path.clone(),
    //                            reason: "Wires and cables cannot be directly connected. Use an
    // interposing terminal strip or \                                     connector"
    //                                .to_owned(),
    //                        })));
    //                    }
    //                    ConnectionType::TermCable(inner_term_cable_id) if
    // project.term_cables.contains_key(inner_term_cable_id) => {                        return
    // Err(Error::from(ProjectError::from(ConnectionError::Invalid {
    // end1: outer_wire_id.clone(),                            end2: inner_term_cable_id.clone(),
    //                            project_file: connection.contained_datafile_path.clone(),
    //                            reason: "Wires and term cables cannot be directly connected. use an
    // interposing connector".to_owned(),                        })));
    //                    }
    //                    ConnectionType::Equipment(inner_equipment_id) if
    // project.equipment.contains_key(inner_equipment_id) => {                        let equipment
    // =                            project
    //                                .equipment
    //                                .get(inner_equipment_id)
    //                                .ok_or(Error::from(ProjectError::ValueNotFound {
    //                                    id: inner_equipment_id.clone(),
    //                                    project_type: "Equipment".to_owned(),
    //                                }))?;
    //                        if let Some(equip_enclosure) = equipment.enclosure.clone()
    //                            && equip_enclosure == *enclosure_id
    //                        {}
    //                    }
    //                    ConnectionType::TerminalStrip(inner_terminal_strip_id)
    //                        if project.terminal_strips.contains_key(inner_terminal_strip_id) => {}
    //                    ConnectionType::Connector(inner_connector_id) if
    // project.connectors.contains_key(inner_connector_id) => {}
    // ConnectionType::Wire(inner_wire_id) => {                        return
    // Err(Error::from(ProjectError::ValueNotFound {                            id:
    // inner_wire_id.clone(),                            project_type: "Wire".to_owned(),
    //                        }));
    //                    }
    //                    ConnectionType::Cable(inner_cable_id) => {
    //                        return Err(Error::from(ProjectError::ValueNotFound {
    //                            id: inner_cable_id.clone(),
    //                            project_type: "Cable".to_owned(),
    //                        }));
    //                    }
    //                    ConnectionType::TermCable(inner_term_cable_id) => {
    //                        return Err(Error::from(ProjectError::ValueNotFound {
    //                            id: inner_term_cable_id.clone(),
    //                            project_type: "TermCable".to_owned(),
    //                        }));
    //                    }
    //                    ConnectionType::Equipment(inner_equipment_id) => {
    //                        return Err(Error::from(ProjectError::ValueNotFound {
    //                            id: inner_equipment_id.clone(),
    //                            project_type: "Equipment".to_owned(),
    //                        }));
    //                    }
    //                    ConnectionType::TerminalStrip(inner_terminal_strip_id) => {
    //                        return Err(Error::from(ProjectError::ValueNotFound {
    //                            id: inner_terminal_strip_id.clone(),
    //                            project_type: "Terminal Strip".to_owned(),
    //                        }));
    //                    }
    //                    ConnectionType::Connector(inner_connector_id) => {
    //                        return Err(Error::from(ProjectError::ValueNotFound {
    //                            id: inner_connector_id.clone(),
    //                            project_type: "Connector".to_owned(),
    //                        }));
    //                    }
    //                },
    //                ConnectionType::Cable(outer_cable_id) if
    // project.term_cables.contains_key(outer_cable_id) => match &connection
    // .end2                {
    //                    ConnectionType::Wire(inner_wire_id) if
    // project.wires.contains_key(inner_wire_id) => {                        return
    // Err(Error::from(ProjectError::from(ConnectionError::Invalid {
    // end1: outer_cable_id.clone(),                            end2: inner_wire_id.clone(),
    //                            project_file: connection.contained_datafile_path.clone(),
    //                            reason: "Wires and cables cannot be directly connected. Use an
    // interposing terminal strip or \                                     connector"
    //                                .to_owned(),
    //                        })));
    //                    }
    //                    ConnectionType::Cable(inner_cable_id) if
    // project.cables.contains_key(inner_cable_id) => {                        return
    // Err(Error::from(ProjectError::from(ConnectionError::SameType {
    // end1: outer_cable_id.clone(),                            end2: inner_cable_id.clone(),
    //                            project_file: connection.contained_datafile_path.clone(),
    //                            message: Some(
    //                                "Cables cannot be directly connected. Use an interposing terminal
    // strip or connector".to_owned(),                            ),
    //                        })));
    //                    }
    //                    ConnectionType::TermCable(inner_term_cable_id) if
    // project.term_cables.contains_key(inner_term_cable_id) => {                        return
    // Err(Error::from(ProjectError::from(ConnectionError::Invalid {
    // end1: outer_cable_id.clone(),                            end2: inner_term_cable_id.clone(),
    //                            project_file: connection.contained_datafile_path.clone(),
    //                            reason: "Cables and term cables cannot be directly connected. use an
    // interposing connector"                                .to_owned(),
    //                        })));
    //                    }
    //                    ConnectionType::Equipment(inner_equipment_id) if
    // project.equipment.contains_key(inner_equipment_id) => {}
    // ConnectionType::TerminalStrip(inner_terminal_strip_id)                        if
    // project.terminal_strips.contains_key(inner_terminal_strip_id) => {}
    // ConnectionType::Connector(inner_connector_id) if
    // project.connectors.contains_key(inner_connector_id) => {}
    // ConnectionType::Wire(inner_wire_id) => {                        return
    // Err(Error::from(ProjectError::ValueNotFound {                            id:
    // inner_wire_id.clone(),                            project_type: "Wire".to_owned(),
    //                        }));
    //                    }
    //                    ConnectionType::Cable(inner_cable_id) => {
    //                        return Err(Error::from(ProjectError::ValueNotFound {
    //                            id: inner_cable_id.clone(),
    //                            project_type: "Cable".to_owned(),
    //                        }));
    //                    }
    //                    ConnectionType::TermCable(inner_term_cable_id) => {
    //                        return Err(Error::from(ProjectError::ValueNotFound {
    //                            id: inner_term_cable_id.clone(),
    //                            project_type: "TermCable".to_owned(),
    //                        }));
    //                    }
    //                    ConnectionType::Equipment(inner_equipment_id) => {
    //                        return Err(Error::from(ProjectError::ValueNotFound {
    //                            id: inner_equipment_id.clone(),
    //                            project_type: "Equipment".to_owned(),
    //                        }));
    //                    }
    //                    ConnectionType::TerminalStrip(inner_terminal_strip_id) => {
    //                        return Err(Error::from(ProjectError::ValueNotFound {
    //                            id: inner_terminal_strip_id.clone(),
    //                            project_type: "Terminal Strip".to_owned(),
    //                        }));
    //                    }
    //                    ConnectionType::Connector(inner_connector_id) => {
    //                        return Err(Error::from(ProjectError::ValueNotFound {
    //                            id: inner_connector_id.clone(),
    //                            project_type: "Connector".to_owned(),
    //                        }));
    //                    }
    //                },
    //                ConnectionType::TermCable(outer_term_cable_id) if
    // project.term_cables.contains_key(outer_term_cable_id) => {                    match
    // &connection.end2 {                        ConnectionType::Wire(inner_wire_id) if
    // project.wires.contains_key(inner_wire_id) => {                            return
    // Err(Error::from(ProjectError::from(ConnectionError::Invalid {
    // end1: outer_term_cable_id.clone(),                                end2:
    // inner_wire_id.clone(),                                project_file:
    // connection.contained_datafile_path.clone(),                                reason: "Wires and
    // term cables cannot be directly connected. use an interposing connector"
    // .to_owned(),                            })));
    //                        }
    //                        ConnectionType::Cable(inner_cable_id) if
    // project.cables.contains_key(inner_cable_id) => {                            return
    // Err(Error::from(ProjectError::from(ConnectionError::Invalid {
    // end1: outer_term_cable_id.clone(),                                end2:
    // inner_cable_id.clone(),                                project_file:
    // connection.contained_datafile_path.clone(),                                reason: "Cables
    // and term cables cannot be directly connected. use an interposing connector"
    // .to_owned(),                            })));
    //                        }
    //                        ConnectionType::TermCable(inner_term_cable_id)
    //                            if project.term_cables.contains_key(inner_term_cable_id) => {}
    //                        ConnectionType::Equipment(inner_equipment_id) if
    // project.equipment.contains_key(inner_equipment_id) => {}
    // ConnectionType::TerminalStrip(inner_terminal_strip_id)                            if
    // project.terminal_strips.contains_key(inner_terminal_strip_id) => {}
    // ConnectionType::Connector(inner_connector_id) if
    // project.connectors.contains_key(inner_connector_id) => {}
    // ConnectionType::Wire(inner_wire_id) => {                            return
    // Err(Error::from(ProjectError::ValueNotFound {                                id:
    // inner_wire_id.clone(),                                project_type: "Wire".to_owned(),
    //                            }));
    //                        }
    //                        ConnectionType::Cable(inner_cable_id) => {
    //                            return Err(Error::from(ProjectError::ValueNotFound {
    //                                id: inner_cable_id.clone(),
    //                                project_type: "Cable".to_owned(),
    //                            }));
    //                        }
    //                        ConnectionType::TermCable(inner_term_cable_id) => {
    //                            return Err(Error::from(ProjectError::ValueNotFound {
    //                                id: inner_term_cable_id.clone(),
    //                                project_type: "TermCable".to_owned(),
    //                            }));
    //                        }
    //                        ConnectionType::Equipment(inner_equipment_id) => {
    //                            return Err(Error::from(ProjectError::ValueNotFound {
    //                                id: inner_equipment_id.clone(),
    //                                project_type: "Equipment".to_owned(),
    //                            }));
    //                        }
    //                        ConnectionType::TerminalStrip(inner_terminal_strip_id) => {
    //                            return Err(Error::from(ProjectError::ValueNotFound {
    //                                id: inner_terminal_strip_id.clone(),
    //                                project_type: "Terminal Strip".to_owned(),
    //                            }));
    //                        }
    //                        ConnectionType::Connector(inner_connector_id) => {
    //                            return Err(Error::from(ProjectError::ValueNotFound {
    //                                id: inner_connector_id.clone(),
    //                                project_type: "Connector".to_owned(),
    //                            }));
    //                        }
    //                    }
    //                }
    //                ConnectionType::Equipment(outer_equipment_id) if
    // project.equipment.contains_key(outer_equipment_id) => {                    match
    // &connection.end2 {                        ConnectionType::Wire(inner_wire_id) if
    // project.wires.contains_key(inner_wire_id) => {}
    // ConnectionType::Cable(inner_cable_id) if project.cables.contains_key(inner_cable_id) => {}
    //                        ConnectionType::TermCable(inner_term_cable_id)
    //                            if project.term_cables.contains_key(inner_term_cable_id) => {}
    //                        ConnectionType::Equipment(inner_equipment_id) if
    // project.equipment.contains_key(inner_equipment_id) => {                            return
    // Err(Error::from(ProjectError::from(ConnectionError::SameType {
    // end1: outer_equipment_id.clone(),                                end2:
    // inner_equipment_id.clone(),                                project_file:
    // connection.contained_datafile_path.clone(),                                message: Some(
    //                                    "Equipment cannot be directly connected. Use an interposing
    // wire, cable or term cable"                                        .to_owned(),
    //                                ),
    //                            })));
    //                        }
    //                        ConnectionType::TerminalStrip(inner_terminal_strip_id)
    //                            if project.terminal_strips.contains_key(inner_terminal_strip_id) =>
    //                        {
    //                            return Err(Error::from(ProjectError::from(ConnectionError::Invalid {
    //                                end1: outer_equipment_id.clone(),
    //                                end2: inner_terminal_strip_id.clone(),
    //                                project_file: connection.contained_datafile_path.clone(),
    //                                reason: "Equipment and Terminal Strips cannot be directly
    // connected. use an wire, cable or term \                                         cable"
    //                                    .to_owned(),
    //                            })));
    //                        }
    //                        ConnectionType::Connector(inner_connector_id) if
    // project.connectors.contains_key(inner_connector_id) => {}
    // ConnectionType::Wire(inner_wire_id) => {                            return
    // Err(Error::from(ProjectError::ValueNotFound {                                id:
    // inner_wire_id.clone(),                                project_type: "Wire".to_owned(),
    //                            }));
    //                        }
    //                        ConnectionType::Cable(inner_cable_id) => {
    //                            return Err(Error::from(ProjectError::ValueNotFound {
    //                                id: inner_cable_id.clone(),
    //                                project_type: "Cable".to_owned(),
    //                            }));
    //                        }
    //                        ConnectionType::TermCable(inner_term_cable_id) => {
    //                            return Err(Error::from(ProjectError::ValueNotFound {
    //                                id: inner_term_cable_id.clone(),
    //                                project_type: "TermCable".to_owned(),
    //                            }));
    //                        }
    //                        ConnectionType::Equipment(inner_equipment_id) => {
    //                            return Err(Error::from(ProjectError::ValueNotFound {
    //                                id: inner_equipment_id.clone(),
    //                                project_type: "Equipment".to_owned(),
    //                            }));
    //                        }
    //                        ConnectionType::TerminalStrip(inner_terminal_strip_id) => {
    //                            return Err(Error::from(ProjectError::ValueNotFound {
    //                                id: inner_terminal_strip_id.clone(),
    //                                project_type: "Terminal Strip".to_owned(),
    //                            }));
    //                        }
    //                        ConnectionType::Connector(inner_connector_id) => {
    //                            return Err(Error::from(ProjectError::ValueNotFound {
    //                                id: inner_connector_id.clone(),
    //                                project_type: "Connector".to_owned(),
    //                            }));
    //                        }
    //                    }
    //                }
    //                ConnectionType::TerminalStrip(outer_terminal_strip_id)
    //                    if project.terminal_strips.contains_key(outer_terminal_strip_id) =>
    //                {
    //                    match &connection.end2 {
    //                        ConnectionType::Wire(inner_wire_id) if
    // project.wires.contains_key(inner_wire_id) => {}
    // ConnectionType::Cable(inner_cable_id) if project.cables.contains_key(inner_cable_id) => {}
    //                        ConnectionType::TermCable(inner_term_cable_id)
    //                            if project.term_cables.contains_key(inner_term_cable_id) => {}
    //                        ConnectionType::Equipment(inner_equipment_id) if
    // project.equipment.contains_key(inner_equipment_id) => {                            return
    // Err(Error::from(ProjectError::from(ConnectionError::Invalid {
    // end1: outer_terminal_strip_id.clone(),                                end2:
    // inner_equipment_id.clone(),                                project_file:
    // connection.contained_datafile_path.clone(),                                reason: "Equipment
    // and Terminal Strips cannot be directly connected. use an wire, cable or term \
    // cable"                                    .to_owned(),
    //                            })));
    //                        }
    //                        ConnectionType::TerminalStrip(inner_terminal_strip_id)
    //                            if project.terminal_strips.contains_key(inner_terminal_strip_id) =>
    //                        {
    //                            return Err(Error::from(ProjectError::from(ConnectionError::SameType {
    //                                end1: outer_terminal_strip_id.clone(),
    //                                end2: inner_terminal_strip_id.clone(),
    //                                project_file: connection.contained_datafile_path.clone(),
    //                                message: Some(
    //                                    "Terminal Strips cannot be directly connected. Use an
    // interposing wire, cable or term cable"                                        .to_owned(),
    //                                ),
    //                            })));
    //                        }
    //                        ConnectionType::Connector(inner_connector_id) if
    // project.connectors.contains_key(inner_connector_id) => {}
    // ConnectionType::Wire(inner_wire_id) => {                            return
    // Err(Error::from(ProjectError::ValueNotFound {                                id:
    // inner_wire_id.clone(),                                project_type: "Wire".to_owned(),
    //                            }));
    //                        }
    //                        ConnectionType::Cable(inner_cable_id) => {
    //                            return Err(Error::from(ProjectError::ValueNotFound {
    //                                id: inner_cable_id.clone(),
    //                                project_type: "Cable".to_owned(),
    //                            }));
    //                        }
    //                        ConnectionType::TermCable(inner_term_cable_id) => {
    //                            return Err(Error::from(ProjectError::ValueNotFound {
    //                                id: inner_term_cable_id.clone(),
    //                                project_type: "TermCable".to_owned(),
    //                            }));
    //                        }
    //                        ConnectionType::Equipment(inner_equipment_id) => {
    //                            return Err(Error::from(ProjectError::ValueNotFound {
    //                                id: inner_equipment_id.clone(),
    //                                project_type: "Equipment".to_owned(),
    //                            }));
    //                        }
    //                        ConnectionType::TerminalStrip(inner_terminal_strip_id) => {
    //                            return Err(Error::from(ProjectError::ValueNotFound {
    //                                id: inner_terminal_strip_id.clone(),
    //                                project_type: "Terminal Strip".to_owned(),
    //                            }));
    //                        }
    //                        ConnectionType::Connector(inner_connector_id) => {
    //                            return Err(Error::from(ProjectError::ValueNotFound {
    //                                id: inner_connector_id.clone(),
    //                                project_type: "Connector".to_owned(),
    //                            }));
    //                        }
    //                    }
    //                }
    //                ConnectionType::Connector(outer_connector_id) if
    // project.connectors.contains_key(outer_connector_id) => {                    match
    // &connection.end2 {                        ConnectionType::Wire(inner_wire_id) if
    // project.wires.contains_key(inner_wire_id) => {}
    // ConnectionType::Cable(inner_cable_id) if project.cables.contains_key(inner_cable_id) => {}
    //                        ConnectionType::TermCable(inner_term_cable_id)
    //                            if project.term_cables.contains_key(inner_term_cable_id) => {}
    //                        ConnectionType::Equipment(inner_equipment_id) if
    // project.equipment.contains_key(inner_equipment_id) => {}
    // ConnectionType::TerminalStrip(inner_terminal_strip_id)                            if
    // project.terminal_strips.contains_key(inner_terminal_strip_id) => {}
    // ConnectionType::Connector(inner_connector_id) if
    // project.connectors.contains_key(inner_connector_id) => {}
    // ConnectionType::Wire(inner_wire_id) => {                            return
    // Err(Error::from(ProjectError::ValueNotFound {                                id:
    // inner_wire_id.clone(),                                project_type: "Wire".to_owned(),
    //                            }));
    //                        }
    //                        ConnectionType::Cable(inner_cable_id) => {
    //                            return Err(Error::from(ProjectError::ValueNotFound {
    //                                id: inner_cable_id.clone(),
    //                                project_type: "Cable".to_owned(),
    //                            }));
    //                        }
    //                        ConnectionType::TermCable(inner_term_cable_id) => {
    //                            return Err(Error::from(ProjectError::ValueNotFound {
    //                                id: inner_term_cable_id.clone(),
    //                                project_type: "TermCable".to_owned(),
    //                            }));
    //                        }
    //                        ConnectionType::Equipment(inner_equipment_id) => {
    //                            return Err(Error::from(ProjectError::ValueNotFound {
    //                                id: inner_equipment_id.clone(),
    //                                project_type: "Equipment".to_owned(),
    //                            }));
    //                        }
    //                        ConnectionType::TerminalStrip(inner_terminal_strip_id) => {
    //                            return Err(Error::from(ProjectError::ValueNotFound {
    //                                id: inner_terminal_strip_id.clone(),
    //                                project_type: "Terminal Strip".to_owned(),
    //                            }));
    //                        }
    //                        ConnectionType::Connector(inner_connector_id) => {
    //                            return Err(Error::from(ProjectError::ValueNotFound {
    //                                id: inner_connector_id.clone(),
    //                                project_type: "Connector".to_owned(),
    //                            }));
    //                        }
    //                    }
    //                }
    //                ConnectionType::Wire(outer_wire_id) => {
    //                    return Err(Error::from(ProjectError::ValueNotFound {
    //                        id: outer_wire_id.clone(),
    //                        project_type: "Wire".to_owned(),
    //                    }));
    //                }
    //                ConnectionType::Cable(outer_cable_id) => {
    //                    return Err(Error::from(ProjectError::ValueNotFound {
    //                        id: outer_cable_id.clone(),
    //                        project_type: "Cable".to_owned(),
    //                    }));
    //                }
    //                ConnectionType::TermCable(outer_term_cable_id) => {
    //                    return Err(Error::from(ProjectError::ValueNotFound {
    //                        id: outer_term_cable_id.clone(),
    //                        project_type: "TermCable".to_owned(),
    //                    }));
    //                }
    //                ConnectionType::Equipment(outer_equipment_id) => {
    //                    return Err(Error::from(ProjectError::ValueNotFound {
    //                        id: outer_equipment_id.clone(),
    //                        project_type: "Equipment".to_owned(),
    //                    }));
    //                }
    //                ConnectionType::TerminalStrip(outer_terminal_strip_id) => {
    //                    return Err(Error::from(ProjectError::ValueNotFound {
    //                        id: outer_terminal_strip_id.clone(),
    //                        project_type: "Terminal Strip".to_owned(),
    //                    }));
    //                }
    //                ConnectionType::Connector(outer_connector_id) => {
    //                    return Err(Error::from(ProjectError::ValueNotFound {
    //                        id: outer_connector_id.clone(),
    //                        project_type: "Connector".to_owned(),
    //                    }));
    //                }
    //            }
    //        }
    //    }
    //    //TODO: now need to figure out which equipment is connected to which other equipment in the
    //    //same location and draw those connections and pathways
    //    //
    //    //Then draw connections off page gated behind a boolean

    Ok(())
}
