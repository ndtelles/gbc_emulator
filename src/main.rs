mod gbc;
mod gui;
mod util;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;

use color_eyre::eyre::{eyre, Result};
use egui_extras::RetainedImage;

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
        let (tx_image, rx_image) = channel();

        let handle = thread::spawn(move || -> Result<()> {
            let file = File::open(path)?;
            let mut buf_reader = BufReader::new(file);
            let mut rom_data = Vec::new();
            buf_reader.read_to_end(&mut rom_data)?;
            let mut gbc = GBC::new(rom_data, tx_image)?;
            gbc.run();
            Ok(())
        });

        self.gbc = Some(GBCThread {
            handle,
            rx_image,
            latest_image: RetainedImage::from_color_image(
                "start_frame",
                eframe::epaint::ColorImage::example(),
            ),
        });
    }
}

struct GBCThread {
    handle: JoinHandle<Result<()>>,
    rx_image: Receiver<RetainedImage>,
    latest_image: RetainedImage,
}
impl GBCThread {
    fn pull_latest_image(&mut self) {
        if let Ok(img) = self.rx_image.try_recv() {
            self.latest_image = img;
        }
    }
}
