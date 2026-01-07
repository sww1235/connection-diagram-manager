//use std::path::Path;

use cdm_core::{
    config::ApplicationConfig,
    datatypes::{library_types::Library, project_types::Project},
    traits::SchematicRepresentation as _,
};
use egui::{Area, Id, Sense, Window, layers::Order, widgets};
use num_traits::cast::FromPrimitive as _;
//use usvg::{Options as ParseOptions, Tree, WriteOptions};

#[expect(clippy::shadow_unrelated, reason = "ui and other variables keep getting passed into closures")]
/// Main window rendering code
pub fn main_window(
    egui_ctx: &egui::Context,
    open: &mut bool,
    app_config: &ApplicationConfig,
    project_data: &Project,
    library_data: &Library,
) {
    let main_window_id = Id::new("root");
    Window::new("Main Window")
        .id(main_window_id)
        .open(open)
        .default_width(f32::from_i32(app_config.graphics_config.clone().unwrap_or_default().window_width).unwrap_or(800.0))
        .default_height(f32::from_i32(app_config.graphics_config.clone().unwrap_or_default().window_height).unwrap_or(600.0))
        //.min_width(800.0_f32)
        //.min_height(800.0_f32)
        .resizable(true)
        .show(egui_ctx, |ui| {
            widgets::global_theme_preference_switch(ui);
            //FIXME: area_rect() is not filtering out title bar. 
            //https://github.com/emilk/egui/issues/7836
            if let Some(rect) = ui.memory(|memory| memory.area_rect(main_window_id)) {
            for (id, equipment) in &project_data.equipment {
                //TODO: instead of expect() just load image error placeholder and log
                #[expect(clippy::panic, reason = "Error handling is hard in GUI code")]
                let (symbol, uri) = equipment
                    .schematic_symbol(library_data, None)
                    .unwrap_or_else(|_| panic!("schematic symbol not defined in library_data for equipment {id}"));
                let svg_data = symbol.into_bytes();
                let sense_settings = Sense::DRAG & Sense::FOCUSABLE;
                let max_height: f32 = 200.0;
                let max_width: f32 = 200.0;
                let image = egui::widgets::Image::from_bytes(uri, svg_data)
                    .sense(sense_settings)
                    .max_height(max_height)
                    .max_width(max_width);
                Area::new(Id::new(id)).movable(true).order(Order::Foreground).constrain_to(rect).show(egui_ctx, |ui| {
                    ui.add(image);
                });
            }}
            ui.allocate_space(ui.available_size());
        });
}

fn main_menu(ui: &mut egui::Ui) {
    //egui::MenuBar::new()
}
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
