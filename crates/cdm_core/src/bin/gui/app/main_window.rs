//use std::path::Path;

use cdm_core::{
    config::ApplicationConfig,
    datatypes::{library_types::Library, project_types::Project},
    traits::SchematicRepresentation as _,
};
use egui::{
    Area,
    Id,
    Sense,
    Window,
    containers::menu,
    layers::Order,
    viewport,
    widgets,
    widgets::{Image, ImageSource},
};
use log::{debug, trace};
use num_traits::cast::FromPrimitive as _;

use crate::app::AppState;

#[expect(
    clippy::shadow_unrelated,
    reason = "ui and other variables keep getting passed into closures"
)]
/// Main window rendering code.
pub fn main_window(
    egui_ctx: &egui::Context,
    app_config: &ApplicationConfig,
    app_state: &mut AppState,
    project_data: &Project,
    library_data: &Library,
) {
    let main_window_id = Id::new("root");
    Window::new("Main Window")
        .id(main_window_id)
        .open(&mut app_state.main_window_state)
        .default_width(f32::from_i32(app_config.graphics_config.starting_window_width).unwrap_or(1024.0))
        .default_height(f32::from_i32(app_config.graphics_config.starting_window_height).unwrap_or(1024.0))
        .resizable(true)
        .show(egui_ctx, |ui| {
            widgets::global_theme_preference_switch(ui);

            main_menu(ui);

            //FIXME: area_rect() is not filtering out title bar.
            //https://github.com/emilk/egui/issues/7836

            if let Some(rect) = ui.memory(|memory| memory.area_rect(main_window_id)) {
                for (id, equipment) in &project_data.equipment {
                    trace! {"ID: {id}, Equipment: {equipment:#?}"};
                    //TODO: instead of expect() just load image error placeholder and log
                    let (symbol, uri) = equipment.schematic_symbol();
                    let svg_data = symbol.visual_representation.get_data().into_bytes();
                    let sense_settings = Sense::DRAG & Sense::FOCUSABLE;

                    //TODO: set sensible max_height() and max_width() here (maybe the size of the
                    //window rectangle or something?

                    let image = Image::new(ImageSource::Bytes {
                        uri: uri.into(),
                        bytes: svg_data.into(),
                    })
                    .sense(sense_settings)
                    .fit_to_original_size(app_state.symbol_scale_factor);
                    Area::new(Id::new(id))
                        .movable(true)
                        .order(Order::Foreground)
                        .constrain_to(rect)
                        .show(egui_ctx, |ui| {
                            ui.add(image);
                        });
                }
            }
            ui.allocate_space(ui.available_size());
        });
}

//NOTE: this relies on the changes in not-fl3/egui-miniquad#84.
//Using git dependancy for now.
/// `main_menu` creates the menu bar for `main_window`.
#[expect(clippy::shadow_reuse, reason = "ui is being passed down closure chains")]
fn main_menu(ui: &mut egui::Ui) {
    //TODO: set style and config using .style() and .config()
    menu::MenuBar::new().ui(ui, |ui| {
        // menu_button is creating a submenu
        ui.menu_button("File", |ui| {
            // ui.button creates a button in that submenu
            if ui.button("Quit").clicked() {
                //TODO: fix quit button
                debug! {"quit menu button clicked"};
                ui.ctx().send_viewport_cmd(viewport::ViewportCommand::Close);
            }
        });
    });
}
