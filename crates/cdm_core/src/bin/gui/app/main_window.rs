use cdm_core::{
    config::ApplicationConfig,
    datatypes::{
        library_types::Library,
        project_types::{
            Project,
            connection::{End, EndDesignation, InnerConnection},
        },
        schematic_connector::{AsConnector as _, SchematicConnector as _, TypeFlag as SCType},
        schematic_symbol::SchematicRepresentation as _,
    },
};
use egui::{
    CursorIcon,
    Id,
    Rect,
    Theme,
    containers::{
        Window,
        menu,
        panel::{CentralPanel, Panel},
    },
    epaint::emath::GuiRounding as _,
    style::Visuals,
};
use log::{debug, error, trace};
use num_traits::cast::FromPrimitive as _;

use crate::app::{AppState, Commands};

#[expect(clippy::shadow_reuse, reason = "ui and other variables keep getting passed into closures")]
#[expect(clippy::too_many_lines, reason = "UI function")]
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

    //TODO: provide configuration option for showing interaction rect around objects when hovered.
    let main_window_is_open = &mut app_state.main_window_state.is_open;
    Window::new("Main Window")
        .id(main_window_id)
        .open(main_window_is_open)
        .movable(false)
        .default_width(f32::from_i32(app_config.graphics_config.starting_window_width).unwrap_or(1024.0))
        .default_height(f32::from_i32(app_config.graphics_config.starting_window_height).unwrap_or(1024.0))
        .resizable(true)
        .show(egui_ctx, |ui| {
            Panel::top(top_menu_id).show(ui, |ui| {
                main_menu(ui, &mut app_state.commands);
            });

            Panel::left(left_sidebar_id).show(ui, |ui| {
                //TODO: add list of locations/physical locations/enclosures? here as a tree
                //view and make selecting them filter what objects you see.
                ui.label("This is the sidebar");
            });

            CentralPanel::default().show(ui, |ui| {
                let panel_rect = ui.max_rect();
                let min_rect_position = panel_rect.left_top();
                let max_rect_position = panel_rect.right_bottom();

                for (id, equipment) in &mut project_data.equipment {
                    //trace! {"ID: {id}, Equipment: {equipment:#?}"};
                    equipment.update_symbol_scale(app_state.symbol_scale_factor);
                    //trace! {"Equipment connections: {:?}", equipment.schematic_symbol().connections};
                    trace!("pre_rendered position: {}", equipment.schematic_symbol().position);
                    //TODO: revisit scaling here. Provide a method to return size based on scale,
                    //instead of doing the math all over the place.
                    let symbol_size = equipment.schematic_symbol().scaled_size();

                    #[expect(clippy::arithmetic_side_effects, reason = "/shrug")]
                    let max_symbol_rect_position = max_rect_position - symbol_size;
                    let symbol_position = equipment
                        .symbol_position()
                        .clamp(min_rect_position, max_symbol_rect_position)
                        .round_ui();
                    //trace! {"min_postion: {symbol_position}"}
                    let rect = Rect::from_min_size(symbol_position, symbol_size);
                    //trace!("rect: {rect:?}");
                    equipment.set_symbol_position(symbol_position.clamp(min_rect_position, max_symbol_rect_position).round_ui());
                    let response = ui.place(rect, equipment.schematic_symbol_mut());
                    // from https://github.com/emilk/egui/discussions/1926#discussioncomment-3414942
                    //
                    //trace!("{response:?}");

                    // equipment.connections stores index of all connections

                    //TODO: embed connection_id inside cable/core and figure out a better way
                    //of looking this up rather than looping every frame.
                    #[expect(clippy::panic, reason = "using instead of expect(format!())")]
                    for (connection_id, end_designation) in &equipment.connections {
                        let connection = project_data.connections.get(connection_id).unwrap_or_else(|| {
                            panic!(
                                "Connection in equipment {}: {connection_id:?} not found in project connections",
                                equipment.identifier
                            )
                        });
                        trace! {
                        "connection rendering loop: {connection_id:?} -- {end_designation}"}

                        match &connection.connection {
                            InnerConnection::Cable { cable_id, core_id } => {
                                let cable = project_data.cables.get_mut(cable_id).unwrap_or_else(|| {
                                    panic!(
                                        "Cable {cable_id} referenced in connection {connection_id:?} of equipment {}: \
                                         {connection_id:?} not found in project.",
                                        equipment.identifier
                                    )
                                });

                                let core = cable
                                    .cores_mut()
                                    .get_mut(core_id)
                                    .unwrap_or_else(|| panic!("Core {core_id} not found in cable {cable_id}"));

                                match end_designation {
                                    EndDesignation::End1 => {
                                        // Only care about equipment here.
                                        if let End::Equipment { connection_point_id, .. } = &connection.end1 {
                                            let connection_offset =
                                                equipment.connection_point_offset(connection_point_id).unwrap_or_else(|_| {
                                                    panic! {
                                                    "Connection point {connection_point_id} not \
                                                    found in schematic symbol for equipment {}",
                                                    equipment.identifier}
                                                });
                                            trace! {"end1 position {} of cable {cable_id} -- {core_id}", core.end1_position()}

                                            trace! {"setting end1 position of cable {cable_id} -- {core_id}"}
                                            //TODO: change this to a move_symbol_position() function rather than doing
                                            //the delta math here
                                            #[expect(clippy::arithmetic_side_effects, reason = "/shrug")]
                                            core.set_end1_position((equipment.symbol_position() + connection_offset).round_ui());
                                        }
                                    }

                                    EndDesignation::End2 => {
                                        // Only care about equipment here.
                                        if let End::Equipment { connection_point_id, .. } = &connection.end2 {
                                            let connection_offset =
                                                equipment.connection_point_offset(connection_point_id).unwrap_or_else(|_| {
                                                    panic! {
                                                    "Connection point {connection_point_id} not \
                                                    found in schematic symbol for equipment {}",
                                                    equipment.identifier}
                                                });

                                            trace! {"end2 position {} of cable {cable_id} -- {core_id}", core.end2_position()}

                                            trace! {"setting end2 position of cable {cable_id} -- {core_id}"}
                                            //TODO: change this to a move_symbol_position() function rather than doing
                                            //the delta math here
                                            #[expect(clippy::arithmetic_side_effects, reason = "/shrug")]
                                            core.set_end2_position((equipment.symbol_position() + connection_offset).round_ui());
                                        }
                                    }
                                }
                            }
                            InnerConnection::TermCable { cable_id, core_id } => {}
                            _ => {
                                panic! {"InnerConnection type not implemented."}
                            }
                        }
                    }
                    //response.paint_debug_info();

                    if response.hovered() {
                        // This should be CursorIcon::Grab but it is not implemented yet.
                        // See https://github.com/not-fl3/miniquad/issues/171#issuecomment-773394249

                        ui.output_mut(|output| output.cursor_icon = CursorIcon::PointingHand);
                    }

                    if response.dragged() {
                        // This should be CursorIcon::Grabbing but it is not implemented yet.
                        // See https://github.com/not-fl3/miniquad/issues/171#issuecomment-773394249

                        ui.output_mut(|output| output.cursor_icon = CursorIcon::Move);
                        trace!("symbol dragged");

                        //TODO: add optional hover text. See lines 614-621 of drag_value.rs from egui.

                        //TODO: change this to a move_symbol_position() function rather than doing
                        //the delta math here
                        #[expect(clippy::arithmetic_side_effects, reason = "/shrug")]
                        equipment.set_symbol_position(
                            (symbol_position + response.drag_delta())
                                .clamp(min_rect_position, max_symbol_rect_position)
                                .round_ui(),
                        );
                    }
                    trace!("post_rendered position: {}", equipment.schematic_symbol().position);
                }
                //TODO: look for crossed SchematicConnectors and insert wire jumps. Add config
                //option for this.

                for (cable_id, cable) in &mut project_data.cables {
                    //trace! {"ID: {id}, Cable: {cable:#?}"};

                    //TODO: figure out how to get this value out of project config.
                    let connector_type = Some(SCType::RightAngle);
                    #[expect(clippy::wildcard_enum_match_arm, reason = "returns unimplemented error")]
                    #[expect(clippy::unnecessary_literal_unwrap, reason = "testing porpoises")]
                    #[expect(clippy::arithmetic_side_effects, reason = "/shrug")]
                    match connector_type.unwrap_or_default() {
                        SCType::RightAngle => {
                            for (core_id, core) in cable.cores_mut() {
                                trace! {"attempting to render cable {cable_id} -- {core_id}"}
                                let core_old_midpoint = core.connector().midpoint();
                                core.connector_mut()
                                    .set_midpoint(core_old_midpoint.clamp(min_rect_position, max_rect_position).round_ui());
                                let response = ui.place(core.connector().bounding_rect(), core.connector_mut());
                                response.paint_debug_info();
                                trace! {"core_response: {response:?}"}
                                // This should be CursorIcon::Grab but it is not implemented yet.
                                // See https://github.com/not-fl3/miniquad/issues/171#issuecomment-773394249
                                if response.hovered() {
                                    ui.output_mut(|output| output.cursor_icon = CursorIcon::PointingHand);
                                }
                                if response.dragged() {
                                    // This should be CursorIcon::Grabbing but it is not implemented yet.
                                    // See https://github.com/not-fl3/miniquad/issues/171#issuecomment-773394249

                                    ui.output_mut(|output| output.cursor_icon = CursorIcon::Move);
                                    trace!("connector for core {core_id} dragged");

                                    //TODO: add optional hover text. See lines 614-621 of drag_value.rs from egui.

                                    // TODO: need to figure out if the junction point is being dragged
                                    let midpoint = core.connector().midpoint();

                                    core.connector_mut().set_midpoint(
                                        (midpoint + response.drag_delta())
                                            .clamp(min_rect_position, max_rect_position)
                                            .round_ui(),
                                    );
                                }
                            }
                        }
                        _ => {
                            error!("Straight connector not implemented yet");
                        }
                    }
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
