use core::cmp::Ordering;

use cdm_core::{
    config::ApplicationConfig,
    datatypes::{
        library_types::Library,
        project_types::{
            Project,
            connection::{Connection, Type},
        },
    },
    traits::SchematicRepresentation as _,
};
use egui::{
    Color32,
    CursorIcon,
    Id,
    Pos2,
    Rect,
    Stroke,
    Theme,
    Vec2,
    containers::{
        Window,
        menu,
        panel::{CentralPanel, SidePanel, TopBottomPanel},
    },
    epaint::emath::GuiRounding as _,
    style::Visuals,
};
use log::{debug, trace, warn};
use num_traits::cast::FromPrimitive as _;

use crate::app::{AppState, Commands};

#[expect(clippy::shadow_reuse, reason = "ui and other variables keep getting passed into closures")]
#[expect(clippy::too_many_lines, reason = "Its a UI function")]
/// Main window rendering code.
pub(crate) fn main_window(
    egui_ctx: &egui::Context,
    app_config: &ApplicationConfig,
    app_state: &mut AppState,
    project_data: &mut Project,
    library_data: &Library,
) {
    let main_window_id = Id::new("root");
    let top_menu_id = Id::new("root-top-menu");
    let left_sidebar_id = Id::new("root-left-sidebar");
    //let central_panel_id = Id::new("central-panel");

    let main_window_is_open = &mut app_state.main_window_state.is_open;
    Window::new("Main Window")
        .id(main_window_id)
        .open(main_window_is_open)
        .default_width(f32::from_i32(app_config.graphics_config.starting_window_width).unwrap_or(1024.0))
        .default_height(f32::from_i32(app_config.graphics_config.starting_window_height).unwrap_or(1024.0))
        .resizable(true)
        .show(egui_ctx, |ui| {
            TopBottomPanel::top(top_menu_id).show_inside(ui, |ui| {
                main_menu(ui, &mut app_state.commands);
            });

            SidePanel::left(left_sidebar_id).show_inside(ui, |ui| {
                //TODO: add list of locations/physical locations/enclosures? here as a tree
                //view and make selecting them filter what objects you see.
                ui.label("This is the sidebar");
            });

            CentralPanel::default().show_inside(ui, |ui| {
                let panel_rect = ui.max_rect();

                for (id, equipment) in &mut project_data.equipment {
                    //trace! {"ID: {id}, Equipment: {equipment:#?}"};
                    equipment.update_symbol_scale(app_state.symbol_scale_factor);
                    //trace! {"Equipment connections: {:?}", equipment.schematic_symbol().connections};
                    trace!("pre_rendered position: {}", equipment.schematic_symbol().position);
                    //TODO: revisit scaling here. Provide a method to return size based on scale,
                    //instead of doing the math all over the place.
                    let symbol_size = equipment.schematic_symbol().scaled_size();
                    // want the larger of the two values, to be the minimum top_left corner. It is
                    // confusing.
                    let min_rect_position = panel_rect.left_top();
                    #[expect(clippy::arithmetic_side_effects, reason = "/shrug")]
                    let max_rect_position = panel_rect.right_bottom() - symbol_size;
                    let rect_position = equipment
                        .schematic_symbol()
                        .position
                        .clamp(min_rect_position, max_rect_position)
                        .round_ui();
                    trace! {"min_postion: {rect_position}"}
                    let rect = Rect::from_min_size(rect_position, symbol_size);
                    trace!("rect: {rect:?}");
                    //trace!("{:?}", equipment.schematic_symbol());
                    let response = ui.place(rect, &mut equipment.schematic_symbol());
                    // from https://github.com/emilk/egui/discussions/1926#discussioncomment-3414942
                    //
                    //trace!("{response:?}");
                    if response.hovered() {
                        // This should be CursorIcon::Grab but it is not implemented yet. See https://github.com/not-fl3/miniquad/issues/171#issuecomment-773394249
                        ui.output_mut(|output| output.cursor_icon = CursorIcon::PointingHand);
                    }

                    if response.dragged() {
                        // This should be CursorIcon::Grabbing but it is not implemented yet. See https://github.com/not-fl3/miniquad/issues/171#issuecomment-773394249
                        ui.output_mut(|output| output.cursor_icon = CursorIcon::Move);
                        trace!("dragged");

                        //TODO: add optional hover text. See lines 614-621 of drag_value.rs from egui.

                        #[expect(clippy::arithmetic_side_effects, reason = "/shrug")]
                        equipment.set_symbol_position(
                            (rect_position + response.drag_delta())
                                .clamp(min_rect_position, max_rect_position)
                                .round_ui(),
                        );
                    }
                    trace!("post_rendered position: {}", equipment.schematic_symbol().position);
                }

                for (id, wire) in &project_data.wires {
                    //TODO: constrain end1/end2 to ui area, and to their respective endpoint
                    //positions. Look at percentage math
                    let mut end1 = Pos2::ZERO;
                    let mut end2 = Pos2::ZERO;
                    let mut wire_connections: Vec<Connection> = Vec::new();

                    //TODO:
                    //
                    //First need to find all connections that reference this wire
                    //
                    //If more than 2, bird strike, log it, and pick 2 at random?
                    //
                    //Then set end 1 and end 2 based on those two connections

                    for connection in &project_data.connections {
                        if connection.end1 == connection.end2 {
                            warn! {"connection: {connection:?} has both ends assigned to the same id."};
                            break;
                        }
                        if let Type::Wire { wire_id } = &connection.end1
                            && wire_id == id
                        {
                            wire_connections.push(connection.clone());
                        }
                        if let Type::Wire { wire_id } = &connection.end2
                            && wire_id == id
                        {
                            wire_connections.push(connection.clone());
                        }
                    }
                    #[expect(clippy::match_same_arms, reason = "code not finished yet")]
                    #[expect(clippy::indexing_slicing, reason = "size of vec validated in outer match")]
                    match wire_connections.len().cmp(&2) {
                        Ordering::Equal => {
                            let wire_end_connections = [(&mut end1, &wire_connections[0]), (&mut end2, &wire_connections[1])];

                            for (end, connection) in wire_end_connections {
                                let connection_ends = [&connection.end1, &connection.end2];
                                for connection_end in connection_ends {
                                    #[expect(
                                        clippy::match_same_arms,
                                        reason = "Separating out some match elements makes the code read clearer"
                                    )]
                                    match connection_end {
                                        Type::Wire { wire_id } if wire_id == id => {
                                            //Do nothing.
                                        }
                                        // TODO: wire-wire, wire-cable and wire-term_cable connections
                                        // not defined yet.
                                        Type::Wire { .. } | Type::Cable { .. } | Type::TermCable { .. } => {
                                            //todo!();
                                        }

                                        #[expect(clippy::arithmetic_side_effects, reason = "deal with it")]
                                        Type::Equipment {
                                            equipment_id,
                                            connection_point_id,
                                        } => {
                                            trace! {"end1 equipment: {equipment_id}"};
                                            #[expect(clippy::get_unwrap, reason = "temporary for testing")]
                                            #[expect(clippy::unwrap_used, reason = "temporary for testing")]
                                            let equipment = project_data.equipment.get(equipment_id).unwrap();
                                            let symbol = equipment.schematic_symbol();
                                            #[expect(clippy::get_unwrap, reason = "temporary for testing")]
                                            #[expect(clippy::unwrap_used, reason = "temporary for testing")]
                                            let connection_point = symbol.connections.get(connection_point_id).unwrap();

                                            //TODO: move this to a method on SchematicSymbol
                                            //
                                            //TODO: change connection point to contain a Pos2?
                                            //
                                            //TODO: look at lerp (linear interpolation functions instead.
                                            let connection_point_offset = Vec2::from((
                                                symbol.scaled_size().x * (connection_point.x / 100.0),
                                                symbol.scaled_size().y * (connection_point.y / 100.0),
                                            ));
                                            trace! {"symbol scaled_size: {}", symbol.scaled_size()};
                                            trace! {"end1 connection_point: {connection_point:?}"};
                                            trace! {"end1 connection_point_offset: {connection_point_offset}"};
                                            *end = symbol.position
                                                + Vec2::from((
                                                    symbol.scaled_size().x * connection_point.x / 100.0,
                                                    symbol.scaled_size().y * connection_point.y / 100.0,
                                                ));
                                        }

                                        _ => {}
                                    }
                                }
                            }
                        }
                        Ordering::Greater => {

                            //TODO: log which connections were indicated on this wire and that there
                            //were greater than 2.
                        }
                        Ordering::Less => {

                            //TODO: log which connections were on this wire and that there were less
                            //than 2.
                        }
                    }

                    //trace! {"ID: {id}, Wire: {wire:#?}"};

                    trace! {"wire: {id} end1: {end1}"};
                    trace! {"wire: {id} end2: {end2}"};
                    let panel_painter = ui.painter();
                    let stroke = Stroke {
                        width: 4.0,
                        color: Color32::RED,
                    };

                    panel_painter.line_segment([end1, end2], stroke);
                }
            });
            ui.allocate_space(ui.available_size());
        });
}

//NOTE: this relies on the changes in not-fl3/egui-miniquad#84.
//Using git dependancy for now.
/// `main_menu` creates the menu bar for `main_window`.
#[expect(clippy::shadow_reuse, reason = "ui is being passed down closure chains")]
fn main_menu(ui: &mut egui::Ui, cmds: &mut Commands) {
    //TODO: set style and config using .style() and .config()
    menu::MenuBar::new().ui(ui, |ui| {
        // menu_button is creating a submenu
        //
        // File menu
        ui.menu_button("File", |ui| {
            // ui.button creates a button in that submenu
            if ui.button("Quit").clicked() {
                debug! {"quit menu button clicked"};
                cmds.quit_clicked = true;
            }
        });
        // Appearance menu
        ui.menu_button("Appearance", |ui| {
            if ui.button("Dark").clicked() {
                ui.ctx().set_visuals_of(Theme::Dark, Visuals::dark());
            }
            if ui.button("Light").clicked() {
                ui.ctx().set_visuals_of(Theme::Light, Visuals::light());
            }
        });
    });
}
