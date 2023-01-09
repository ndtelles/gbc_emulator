mod gbc;
mod util;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use rfd::FileDialog;

use crate::gbc::GBC;

fn main() -> std::io::Result<()> {
    let path = FileDialog::new()
        .add_filter("GBC ROM", &["gbc"])
        .pick_file()
        .unwrap();
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut rom_data = Vec::new();
    buf_reader.read_to_end(&mut rom_data)?;
    let mut gbc = GBC::new(rom_data);
    gbc.run();
    Ok(())
}
