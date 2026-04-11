use cdm_core::{
    config::ApplicationConfig,
    datatypes::{
        library_types::Library,
        project_types::Project,
        schematic_connector::{AsConnector as _, Type as SCType},
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
        panel::{CentralPanel, SidePanel, TopBottomPanel},
    },
    epaint::emath::GuiRounding as _,
    style::Visuals,
};
use log::{debug, error, trace, warn};
use num_traits::cast::FromPrimitive as _;

use crate::app::{AppState, Commands};

#[expect(clippy::shadow_reuse, reason = "ui and other variables keep getting passed into closures")]
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
                    trace! {"ID: {id}, Equipment: {equipment:#?}"};
                    equipment.update_symbol_scale(app_state.symbol_scale_factor);
                    trace! {"Equipment connections: {:?}", equipment.schematic_symbol().connections};
                    trace!("pre_rendered position: {}", equipment.schematic_symbol().position);
                    //TODO: revisit scaling here. Provide a method to return size based on scale,
                    //instead of doing the math all over the place.
                    let symbol_size = equipment.schematic_symbol().scaled_size();

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
                    equipment.set_symbol_position(rect_position.clamp(min_rect_position, max_rect_position).round_ui());
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
                        trace!("symbol dragged");

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
                    //trace! {"ID: {id}, Wire: {wire:#?}"};

                    //trace! {"wire: {id} end1: {}->{:?}", end1.0, end1.1};
                    //trace! {"wire: {id} end2: {}->{:?}", end2.0, end2.1};

                    //TODO: Finish this
                    //
                    //new connector
                    //
                    //set endpoints
                    //
                    //monitor response for drag

                    //TODO: figure out how to get this value out of project config.
                    let connector_type = Some(SCType::RightAngle);
                    #[expect(clippy::wildcard_enum_match_arm, reason = "returns unimplemented error")]
                    #[expect(clippy::unnecessary_literal_unwrap, reason = "testing porpoises")]
                    match connector_type.unwrap_or_default() {
                        SCType::RightAngle => {
                            let wire_connector = wire.as_connector(id.to_owned(), project_data);
                            if let Ok(mut wire_connector) = wire_connector {
                                let response = ui.place(wire_connector.containing_rect(), &mut wire_connector);
                                if response.hovered() {
                                    // This should be CursorIcon::Grab but it is not implemented yet. See https://github.com/not-fl3/miniquad/issues/171#issuecomment-773394249
                                    ui.output_mut(|output| output.cursor_icon = CursorIcon::PointingHand);
                                }
                                if response.dragged() {
                                    // This should be CursorIcon::Grabbing but it is not implemented yet. See https://github.com/not-fl3/miniquad/issues/171#issuecomment-773394249
                                    ui.output_mut(|output| output.cursor_icon = CursorIcon::Move);
                                    trace!("symbol dragged");

                                    //TODO: add optional hover text. See lines 614-621 of drag_value.rs from egui.

                                    //equipment.set_symbol_position(
                                    //    (rect_position + response.drag_delta())
                                    //        .clamp(min_rect_position, max_rect_position)
                                    //        .round_ui(),
                                    //);

                                    wire_connector.move_midpoint(response.drag_delta());
                                }
                            } else {
                                error!("{:?}", wire_connector.err());
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
