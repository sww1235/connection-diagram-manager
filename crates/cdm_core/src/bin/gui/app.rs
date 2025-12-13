use cdm_core::datatypes::{
    library_types::Library,
    project_types::{Config as ProjectConfig, Project},
};
use miniquad::{self as mq, TouchPhase, window as mqWindow};

mod main_window;

/// Main GUI app struct
pub struct App {
    /// Egui Miniquad bindings
    egui_mq: egui_miniquad::EguiMq,
    /// rendering context
    // TODO: try to remove box and dyn here
    mq_ctx: Box<dyn mq::RenderingBackend>,
    /// if main window is closed
    main_window_state: bool,
    /// if application has requested to quit
    quit_requested: bool,
    /// project data
    project_data: Project,
    /// library data
    library_data: Library,
    /// Project configuration
    project_config: ProjectConfig,
}

impl App {
    /// keyboard shortcut to quit app
    pub const QUIT_CMD: egui::KeyboardShortcut = egui::KeyboardShortcut {
        modifiers: egui::Modifiers {
            alt: false,
            ctrl: false,
            shift: false,
            mac_cmd: false,
            command: true,
        },
        logical_key: egui::Key::Q,
    };
    /// Create new app
    pub fn new(project_config: ProjectConfig, project_data: Project, library_data: Library) -> Self {
        let mut mq_ctx = mqWindow::new_rendering_backend();
        Self {
            egui_mq: egui_miniquad::EguiMq::new(&mut *mq_ctx),
            mq_ctx,
            main_window_state: true,
            quit_requested: false,
            project_data,
            library_data,
            project_config,
        }
    }
}

impl mq::EventHandler for App {
    fn update(&mut self) {}
    fn draw(&mut self) {
        use std::process;

        use egui::{Window, widgets};

        // red, green, blue, alpha, depth, stencil
        // TODO: test if I need this call
        self.mq_ctx.clear(Some((1.0, 1.0, 1.0, 1.0)), None, None);
        self.mq_ctx
            .begin_default_pass(mq::PassAction::clear_color(0.0, 0.0, 0.0, 1.0));
        self.mq_ctx.end_render_pass();
        let dpi_scale = mqWindow::dpi_scale();

        // This is where all the egui code goes
        self.egui_mq.run(&mut *self.mq_ctx, |_mq_ctx, egui_ctx| {
            egui_extras::install_image_loaders(egui_ctx);
            Window::new("Main Window")
                .open(&mut self.main_window_state)
                .show(egui_ctx, |ui| {
                    widgets::global_theme_preference_switch(ui);
                    ui.image(egui::include_image!(
                        "/home/toxicsauce/myprojects/connection-diagram-manager/resources/test/testproject/lib/SPST-Switch.svg"
                    ));
                });
            // input handler
            egui_ctx.input_mut(|input_state| {
                let keyboard_quit_request = input_state.consume_shortcut(&Self::QUIT_CMD);
                let window_quit_request = input_state.viewport().close_requested();
                self.quit_requested = keyboard_quit_request || window_quit_request;
            });

            // TODO: figure out a better way of exiting the app.
            // Investigate the code of egui_miniquad and minquad more to
            // see if things can be improved
            if self.quit_requested {
                //TODO: add checks here for unsaved files, prompt user if they want to close, etc
                process::exit(0);
            }
        });

        self.egui_mq.draw(&mut *self.mq_ctx);
        self.mq_ctx.commit_frame();
    }
    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.egui_mq.mouse_motion_event(x, y);
    }

    fn mouse_wheel_event(&mut self, dx: f32, dy: f32) {
        self.egui_mq.mouse_wheel_event(dx, dy);
    }

    fn mouse_button_down_event(&mut self, mb: mq::MouseButton, x: f32, y: f32) {
        self.egui_mq.mouse_button_down_event(mb, x, y);
    }

    fn mouse_button_up_event(&mut self, mb: mq::MouseButton, x: f32, y: f32) {
        self.egui_mq.mouse_button_up_event(mb, x, y);
    }

    fn char_event(&mut self, character: char, _keymods: mq::KeyMods, _repeat: bool) {
        self.egui_mq.char_event(character);
    }

    fn key_down_event(&mut self, keycode: mq::KeyCode, keymods: mq::KeyMods, _repeat: bool) {
        self.egui_mq.key_down_event(keycode, keymods);
    }

    fn key_up_event(&mut self, keycode: mq::KeyCode, keymods: mq::KeyMods) {
        self.egui_mq.key_up_event(keycode, keymods);
    }

    fn window_minimized_event(&mut self) {}

    fn window_restored_event(&mut self) {}

    fn touch_event(&mut self, phase: TouchPhase, _id: u64, x: f32, y: f32) {}

    fn quit_requested_event(&mut self) {}

    fn files_dropped_event(&mut self) {}

    fn resize_event(&mut self, width: f32, height: f32) {}

    fn raw_mouse_motion(&mut self, _dx: f32, _dy: f32) {}
}
