use std::path::Path;

use cdm_core::config::ApplicationConfig;
use egui::{Window, widgets};
use num_traits::cast::FromPrimitive as _;
use url::Url;

/// Main window rendering code
pub fn main_window(egui_ctx: &egui::Context, open: &mut bool, app_config: &ApplicationConfig) {
    //
    Window::new("Main Window")
        .open(open)
        .default_width(f32::from_i32(app_config.graphics_config.clone().unwrap_or_default().window_width).unwrap_or(800.0))
        .default_height(f32::from_i32(app_config.graphics_config.clone().unwrap_or_default().window_height).unwrap_or(600.0))
        .show(egui_ctx, |ui| {
            widgets::global_theme_preference_switch(ui);
            load_svg(
                ui,
                Path::new(
                    "/home/toxicsauce/myprojects/connection-diagram-manager/resources/test/testproject/lib/SPST-Switch.svg",
                ),
            );
        });
}
//https://github.com/emilk/egui/pull/5732/files
/// load SVG image
fn load_svg(ui: &mut egui::Ui, path: &Path) {
    //TODO: fix error handling here
    let image_uri = Url::from_file_path(path.canonicalize().unwrap().to_str().unwrap()).unwrap();
    println!("{}", image_uri);
    let image = egui::widgets::Image::from_uri(image_uri.as_str());

    ui.add(image);
}
