use eframe::egui::{self, Context, Ui};

use crate::App;

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            self.gbc_ui(ctx, ui);
        });
    }
}

impl App {
    fn gbc_ui(&mut self, ctx: &Context, ui: &mut Ui) {
        match self.gbc {
            Some(ref mut gbc) => {
                let display_buffer = gbc.display_buffer.lock().unwrap();
                ui.add(egui::Image::new(
                    display_buffer.texture_id(ctx),
                    display_buffer.size_vec2(),
                ));
            }
            None => {
                if ui.button("Load ROM").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("GB(C) ROM", &["gbc", "gb"])
                        .pick_file()
                    {
                        self.spawn_gbc(path, ctx);
                    }
                }
            }
        }
    }
}
