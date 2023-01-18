mod pixel_fetcher;

use std::{collections::VecDeque, sync::mpsc::Sender};

use eframe::epaint::ColorImage;
use egui_extras::RetainedImage;

use self::pixel_fetcher::{Pixel, PixelFetcher};

use super::{
    lcd_controller::{self, PPUMode},
    GBCState,
};

const GBC_RESOLUTION_X: u8 = 160;
const GBC_RESOLUTION_Y: u8 = 144;
const IMG_BUFFER_SIZE: usize = GBC_RESOLUTION_X as usize * GBC_RESOLUTION_Y as usize * 3;

pub struct Renderer {
    img_publisher: Sender<RetainedImage>,
    // Flat RGB values for each pixel
    img_buffer: [u8; IMG_BUFFER_SIZE],
    // FIFO of pixels to draw. Refilled by pixel fetcher
    bg_fifo: VecDeque<Pixel>,
    obj_fifo: VecDeque<Pixel>,
    obj_slots: [u8; 10],
    pixel_fetcher: PixelFetcher,
    lcd_x: u8,
    lcd_y: u8,
}

impl Renderer {
    pub fn new(img_publisher: Sender<RetainedImage>) -> Self {
        Self {
            img_publisher,
            img_buffer: [0xFF; IMG_BUFFER_SIZE],
            bg_fifo: VecDeque::with_capacity(8),
            obj_fifo: VecDeque::with_capacity(8),
            obj_slots: [0; 10],
            pixel_fetcher: PixelFetcher::new(),
            lcd_x: 0,
            lcd_y: 0,
        }
    }
}

pub fn tick(state: &mut GBCState) {
    let status_reg = lcd_controller::get_lcd_status_register(state);
    let ctrl_reg = lcd_controller::get_lcd_control_register(state);

    match status_reg.ppu_mode {
        PPUMode::OAMScan => {}
        PPUMode::Drawing => {
            pixel_fetcher::tick(state, &ctrl_reg);
            draw(state);

            if state.render_engine.lcd_x == GBC_RESOLUTION_X {
                if state.render_engine.lcd_y == GBC_RESOLUTION_Y {
                    // Finish frame
                    publish_image(state);
                } else {
                    // Finish scanline
                }
                state.render_engine.lcd_x = 0;
            }
        }
        PPUMode::HBlank | PPUMode::VBlank => {
            // Do Nothing
        }
    }
}

fn draw(state: &mut GBCState) {
    let px = state.render_engine.bg_fifo.pop_front();
    if let None = px {
        return;
    }
    let px = px.unwrap();
    state.render_engine.lcd_x += 1;
}

fn publish_image(state: &GBCState) {
    let image = ColorImage::from_rgb(
        [GBC_RESOLUTION_X.into(), GBC_RESOLUTION_Y.into()],
        &state.render_engine.img_buffer,
    );
    let texture = RetainedImage::from_color_image("GBC frame", image);
    state.render_engine.img_publisher.send(texture).ok();
}
