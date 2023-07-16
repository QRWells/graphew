use egui::{Ui, Window};

pub struct AboutWindow;

impl AboutWindow {
    pub fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        Window::new("About Graphew")
            .open(open)
            .show(ctx, |ui| self.ui(ui));
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.label("Graphew");
        ui.label("Version 0.1.0");
        ui.label("Author: Emmanuel Odeke <>");
        ui.hyperlink_to("GitHub", "http://github.com");
    }
}
