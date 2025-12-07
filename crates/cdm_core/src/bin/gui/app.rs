use miniquad::{self as mq, TouchPhase};

/// Main GUI app
pub struct Cdm {
    /// Egui Miniquad bindings
    egui_mq: egui_miniquad::EguiMq,
    /// rendering context
    // TODO: try to remove box and dyn here
    mq_ctx: Box<dyn mq::RenderingBackend>,
}

impl Cdm {
    /// Create new app
    pub fn new() -> Self {
        let mut mq_ctx = mq::window::new_rendering_backend();
        Self {
            egui_mq: egui_miniquad::EguiMq::new(&mut *mq_ctx),
            mq_ctx,
        }
    }
}

impl mq::EventHandler for Cdm {
    fn update(&mut self) {}
    fn draw(&mut self) {
        // red, green, blue, alpha, depth, stencil
        // TODO: test if I need this call
        self.mq_ctx.clear(Some((1.0, 1.0, 1.0, 1.0)), None, None);
        self.mq_ctx
            .begin_default_pass(mq::PassAction::clear_color(0.0, 0.0, 0.0, 1.0));
        self.mq_ctx.end_render_pass();
        let dpi_scale = mq::window::dpi_scale();

        // This is where all the egui code goes
        self.egui_mq.run(&mut *self.mq_ctx, |_mq_ctx, egui_ctx| {
            egui::Window::new("Main Window").show(egui_ctx, |ui| {
                egui::widgets::global_theme_preference_switch(ui);
            });
        });
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
