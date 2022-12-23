mod gbc;
mod util;

use crate::gbc::GBC;

fn main() {
    let mut gbc = GBC::new();
    gbc.run();
}
