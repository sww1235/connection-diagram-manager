//use std::path::Path;

use cdm_core::{
    config::ApplicationConfig,
    datatypes::{
        library_types::Library,
        project_types::{Project, connection::Type},
        schematic_symbol::SchematicSymbol,
    },
    traits::SchematicRepresentation as _,
};
use egui::{
    CursorIcon,
    Id,
    Pos2,
    Rect,
    Sense,
    Theme,
    Vec2,
    containers::{
        Area,
        Window,
        menu,
        panel::{CentralPanel, SidePanel, TopBottomPanel},
    },
    layers::Order,
    style::Visuals,
    widgets::{Image, ImageSource},
};
use log::{debug, trace};
use num_traits::cast::FromPrimitive as _;

use crate::app::{AppState, Commands};

#[expect(
    clippy::shadow_unrelated,
    reason = "ui and other variables keep getting passed into closures"
)]
#[expect(clippy::shadow_reuse, reason = "ui and other variables keep getting passed into closures")]
/// Main window rendering code.
pub fn main_window(
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
                //let rect = ui.max_rect();
                //TODO: somehow update fields on either equipment or equipment.symbol with the
                //actual screen coordinates of the connections
                //
                //Maybe store in appstate?
                for (id, equipment) in &mut project_data.equipment {
                    //trace! {"ID: {id}, Equipment: {equipment:#?}"};
                    equipment.update_symbol_scale(app_state.symbol_scale_factor);
                    //TODO: update Vec2::new() with actual current size of symbols
                    trace!("rendered position: {}", equipment.schematic_symbol().position);
                    let rect = Rect::from_min_size(equipment.schematic_symbol().position, Vec2::new(1000.0, 1000.0));
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
                        //trace!("position before: {}", self.position);
                        //self.position += response.drag_delta();
                        //trace!("position after: {}", self.position);

                        //TODO: update symbol connections here as well?
                        equipment.set_symbol_position(equipment.schematic_symbol().position + response.drag_delta());
                    }
                }

                //TODO: Maintain a map of images/areas in appstate along with their IDs and the ID
                //of the equipment.
                for (id, wire) in &project_data.wires {
                    let mut end1 = Pos2::ZERO;
                    let mut end2 = Pos2::ZERO;
                    for connection in &project_data.connections {
                        match &connection.end1 {
                            Type::Wire { wire_id } if wire_id == id => {
                                match &connection.end2 {
                                    // TODO: wire-wire, wire-cable and wire-term_cable connections
                                    // not defined yet.
                                    Type::Wire { .. } | Type::Cable { .. } | Type::TermCable { .. } => {
                                        //todo!();
                                    }

                                    Type::Equipment {
                                        equipment_id,
                                        connection_point_id,
                                    } => {
                                        let equipment = project_data.equipment.get(equipment_id).unwrap();
                                        //let (_, symbol) = equipment.schematic_symbol();
                                    }

                                    _ => {}
                                }
                            }

                            _ => {}
                        }
                    }

                    trace! {"ID: {id}, Wire: {wire:#?}"};
                    let panel_painter = ui.painter();
                    egui_ctx.memory(|memory| for area in memory.areas().visible_layer_ids() {});
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
