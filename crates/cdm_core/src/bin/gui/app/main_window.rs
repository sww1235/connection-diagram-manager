//use std::path::Path;

use cdm_core::{
    config::ApplicationConfig,
    datatypes::{library_types::Library, project_types::Project},
    traits::SchematicRepresentation as _,
};
use egui::{Area, Id, Sense, Window, layers::Order, widgets};
use num_traits::cast::FromPrimitive as _;

use log::trace;

use crate::app::AppState;
//use usvg::{Options as ParseOptions, Tree, WriteOptions};

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
            //FIXME: area_rect() is not filtering out title bar.
            //https://github.com/emilk/egui/issues/7836
            if let Some(rect) = ui.memory(|memory| memory.area_rect(main_window_id)) {
                for (id, equipment) in &project_data.equipment {
                    trace!{"ID: {id}, Equipment: {equipment:#?}"};
                    //TODO: instead of expect() just load image error placeholder and log
                    #[expect(clippy::panic, reason = "Error handling is hard in GUI code")]
                    let (mut symbol, uri) = equipment
                        .schematic_symbol(library_data, None)
                        .unwrap_or_else(|_| panic!("schematic symbol not defined in library_data for equipment {id}"));
                    #[expect(clippy::panic, reason = "Error handling is hard in GUI code")]
                    equipment
                        .update_symbol_data(library_data, &mut symbol)
                        .unwrap_or_else(|err| panic!("Failed to update fields in schematic symbol for equipment {id}. \n {err}"));
                    let svg_data = symbol.get_data().into_bytes();
                    let sense_settings = Sense::DRAG & Sense::FOCUSABLE;
                    let image = egui::widgets::Image::from_bytes(uri, svg_data)
                        .sense(sense_settings)
                        .max_height(app_state.schematic_symbol_height)
                        .max_width(app_state.schematic_symbol_width);
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

//TODO: finish once not-fl3/egui-miniquad#84 is fully released
//fn main_menu(ui: &mut egui::Ui) {
//TODO: set style and config using .style() and .config()
// egui::MenuBar::new().ui(ui, |ui|{
//
// })
//}
//https://github.com/emilk/egui/pull/5732/files
///// load SVG image
//fn load_svg_from_path(ui: &mut egui::Ui, path: &Path) {
//    //TODO: fix error handling here
//    let options = ParseOptions::default();
//    let write_options = WriteOptions::default();
//    let image_bytes = std::fs::read(path.canonicalize().expect("unable to canonicalize
// path")).expect("unable to read svg file");    let tree = Tree::from_str(
//        str::from_utf8(&image_bytes).expect("unable to parse iamge_bytes into valid utf8 str"),
//        &options,
//    )
//    .expect("unable to parse utf str into usvg::Tree");
//    let mut uri = "bytes://".to_owned();
//    uri.push_str(path.to_str().expect("unable to convert path to string"));
//    let image = egui::widgets::Image::from_bytes(uri,
// tree.to_string(&write_options).into_bytes());
//
//    ui.add(image);
//}
