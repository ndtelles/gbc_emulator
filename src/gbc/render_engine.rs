mod pixel_fetcher;

use std::collections::VecDeque;

use self::pixel_fetcher::{Pixel, PixelFetcher};

use super::{
    lcd_controller::{self, PPUMode},
    GBCState,
};

pub struct Renderer {
    bg_fifo: VecDeque<Pixel>,
    obj_fifo: VecDeque<Pixel>,
    obj_slots: [u8; 10],
    pixel_fetcher: PixelFetcher,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            bg_fifo: VecDeque::with_capacity(8),
            obj_fifo: VecDeque::with_capacity(8),
            obj_slots: [0; 10],
            pixel_fetcher: PixelFetcher::new(),
        }
    }
}

pub fn tick(state: &mut GBCState) {
    let status_reg = lcd_controller::get_lcd_status_register(state);
    let ctrl_reg = lcd_controller::get_lcd_control_register(state);
    pixel_fetcher::tick(state, &ctrl_reg);
    match status_reg.ppu_mode {
        PPUMode::OAMScan => {}
        PPUMode::Drawing => {}
        PPUMode::HBlank | PPUMode::VBlank => {
            // Do Nothing
        }
    }
}
