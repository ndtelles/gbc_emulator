mod color_value;
mod pixel_fetcher;

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    thread, time,
};

use eframe::epaint::ColorImage;
use egui_extras::RetainedImage;
use tracing::{debug, debug_span, trace};

use crate::util::combine_high_low;

use self::pixel_fetcher::{Pixel, PixelFetcher};

use super::{
    lcd_controller::{self, PPUMode},
    virtual_memory, GBCState,
};

const FRAME_PERIOD: time::Duration = time::Duration::from_micros(16_666);
const GBC_RESOLUTION_X: u8 = 160;
const GBC_RESOLUTION_Y: u8 = 144;
const IMG_BUFFER_SIZE: usize = GBC_RESOLUTION_X as usize * GBC_RESOLUTION_Y as usize * 3;

const BYTES_PER_PALETTE: u8 = 8;
const BYTES_PER_PALETTE_COLOR: u8 = 2;

pub struct Renderer {
    // The current frame being displayed
    display_buffer: Arc<Mutex<RetainedImage>>,
    // Allows render enginer to redraw gui when publishing a frame
    gui_ctx: eframe::egui::Context,
    // Flat RGB values for each pixel
    // The current frame being drawn
    working_frame_buffer: [u8; IMG_BUFFER_SIZE],
    // FIFO of pixels to draw. Refilled by pixel fetcher
    bg_fifo: VecDeque<Pixel>,
    obj_fifo: VecDeque<Pixel>,
    obj_slots: [u8; 10],
    pixel_fetcher: PixelFetcher,
    lcd_x: u8,
    lcd_y: u8,
    last_frame_time: time::Instant,
}

impl Renderer {
    pub fn new(display_buffer: Arc<Mutex<RetainedImage>>, gui_ctx: eframe::egui::Context) -> Self {
        Self {
            display_buffer,
            gui_ctx,
            working_frame_buffer: [0xFF; IMG_BUFFER_SIZE],
            bg_fifo: VecDeque::with_capacity(8),
            obj_fifo: VecDeque::with_capacity(8),
            obj_slots: [0; 10],
            pixel_fetcher: PixelFetcher::new(),
            lcd_x: 0,
            lcd_y: 0,
            last_frame_time: time::Instant::now(),
        }
    }
}

pub fn tick(state: &mut GBCState) {
    let status_reg = lcd_controller::get_lcd_status_register(state);

    match status_reg.ppu_mode {
        PPUMode::OAMScan => {}
        PPUMode::Drawing => {
            let span = debug_span!(
                "Render Engine Draw",
                x = state.render_engine.lcd_x,
                y = state.render_engine.lcd_y
            )
            .entered();

            let ctrl_reg = lcd_controller::get_lcd_control_register(state);

            pixel_fetcher::tick(state, &ctrl_reg);
            draw(state);

            if state.render_engine.lcd_x == GBC_RESOLUTION_X {
                // Scanline drawing complete
                state.render_engine.lcd_x = 0;
                state.render_engine.lcd_y += 1;
                // Reset pixel fetcher
                state.render_engine.pixel_fetcher = PixelFetcher::new();

                lcd_controller::update_ppu_mode(state, PPUMode::HBlank);

                if state.render_engine.lcd_y == GBC_RESOLUTION_Y {
                    // Frame drawing complete
                    state.render_engine.lcd_y = 0;
                    publish_frame(state);
                    debug!("Frame published");
                }
            }

            span.exit();
        }
        PPUMode::HBlank | PPUMode::VBlank => {}
    }
}

fn draw(state: &mut GBCState) {
    let pixel = state.render_engine.bg_fifo.pop_front();
    if let None = pixel {
        return;
    }
    let pixel = pixel.unwrap();
    let rgb = pixel_to_rgb(state, &pixel);
    let buffer_idx = ((state.render_engine.lcd_y as usize * GBC_RESOLUTION_X as usize)
        + state.render_engine.lcd_x as usize)
        * 3;
    let buffer_slice = &mut state.render_engine.working_frame_buffer[buffer_idx..(buffer_idx + 3)];
    buffer_slice.copy_from_slice(&rgb);

    state.render_engine.lcd_x += 1;
}

fn publish_frame(state: &mut GBCState) {
    let image = ColorImage::from_rgb(
        [GBC_RESOLUTION_X.into(), GBC_RESOLUTION_Y.into()],
        &state.render_engine.working_frame_buffer,
    );
    let texture = RetainedImage::from_color_image("GBC frame", image);

    // debug!("{:?}", virtual_memory::read_bytes(state, 0xFF69, 0x0400));
    // debug!("{:?}", virtual_memory::borrow_palette_mem(state));

    // Sleep until frame is needed
    let time_since_last_frame = state.render_engine.last_frame_time.elapsed();
    thread::sleep(FRAME_PERIOD.saturating_sub(time_since_last_frame));
    state.render_engine.last_frame_time = time::Instant::now();

    let mut display_buffer = state.render_engine.display_buffer.lock().unwrap();
    *display_buffer = texture;

    state.render_engine.gui_ctx.request_repaint();
}

fn pixel_to_rgb(state: &GBCState, pixel: &Pixel) -> [u8; 3] {
    let palettes = virtual_memory::borrow_palette_mem(state);
    let palette_idx =
        (pixel.palette * BYTES_PER_PALETTE) + (pixel.color_idx * BYTES_PER_PALETTE_COLOR);
    let low = palettes[palette_idx as usize];
    let high = palettes[palette_idx as usize + 1];
    let rgb555 = combine_high_low(high, low);
    let r5 = (rgb555 & 0x1F) as u8;
    let g5 = ((rgb555 >> 5) & 0x1F) as u8;
    let b5 = ((rgb555 >> 10) & 0x1F) as u8;
    color_value::rgb555_to_rgb888(&[r5, g5, b5])
}
