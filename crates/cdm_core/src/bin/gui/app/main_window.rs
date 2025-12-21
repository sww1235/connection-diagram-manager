use std::path::Path;

use cdm_core::config::ApplicationConfig;
use egui::{Window, widgets};
use num_traits::cast::FromPrimitive as _;
use url::Url;

use usvg::{Options as ParseOptions, Tree, WriteOptions};

/// Main window rendering code
pub fn main_window(egui_ctx: &egui::Context, open: &mut bool, app_config: &ApplicationConfig) {
    //
    Window::new("Main Window")
        .open(open)
        .default_width(f32::from_i32(app_config.graphics_config.clone().unwrap_or_default().window_width).unwrap_or(800.0))
        .default_height(f32::from_i32(app_config.graphics_config.clone().unwrap_or_default().window_height).unwrap_or(600.0))
        .show(egui_ctx, |ui| {
            widgets::global_theme_preference_switch(ui);
            load_svg_from_path(
                ui,
                Path::new(
                    "/home/toxicsauce/myprojects/connection-diagram-manager/resources/test/testproject/lib/SPST-Switch.svg",
                ),
            );
        });
}
//https://github.com/emilk/egui/pull/5732/files
/// load SVG image
fn load_svg_from_path(ui: &mut egui::Ui, path: &Path) {
    //TODO: fix error handling here
        let options = ParseOptions::default();
        let write_options = WriteOptions::default();
    let image_bytes = std::fs::read(path.canonicalize().expect("unable to canonicalize path")).expect("unable to read svg file");
    let tree = Tree::from_str(str::from_utf8(&image_bytes).expect("unable to parse iamge_bytes into valid utf8 str"), &options).expect("unable to parse utf str into usvg::Tree");
    let mut uri = "bytes://".to_owned();
        uri.push_str(path.to_str().expect("unable to convert path to string"));
    let image = egui::widgets::Image::from_bytes(uri, tree.to_string(&write_options).into_bytes());

    ui.add(image);
}
