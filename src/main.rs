mod gbc;
mod gui;
mod util;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

use color_eyre::eyre::{eyre, Result};
use egui_extras::RetainedImage;
use tracing::info_span;

use crate::gbc::GBC;

fn main() -> Result<()> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "GBC",
        native_options,
        Box::new(|_cc| Box::new(App::default())),
    )
    .map_err(|e| eyre!(e.to_string()))
}

struct App {
    gbc: Option<GBCThread>,
}
impl App {
    fn default() -> Self {
        Self { gbc: None }
    }

    fn spawn_gbc(&mut self, path: PathBuf) {
        let display_buffer = Arc::new(Mutex::new(RetainedImage::from_color_image(
            "start_frame",
            eframe::epaint::ColorImage::example(),
        )));
        let display_buffer_for_gbc_thread = Arc::clone(&display_buffer);

        let handle = thread::spawn(move || -> Result<()> {
            let span = info_span!("GBC Thread").entered();

            let file = File::open(path)?;
            let mut buf_reader = BufReader::new(file);
            let mut rom_data = Vec::new();
            buf_reader.read_to_end(&mut rom_data)?;
            let mut gbc = GBC::new(rom_data, display_buffer_for_gbc_thread)?;
            gbc.run();
            
            span.exit();
            Ok(())
        });

        self.gbc = Some(GBCThread {
            handle,
            display_buffer,
        });
    }
}

struct GBCThread {
    handle: JoinHandle<Result<()>>,
    display_buffer: Arc<Mutex<RetainedImage>>,
}
