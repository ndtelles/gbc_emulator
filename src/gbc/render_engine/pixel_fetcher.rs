use std::collections::VecDeque;

use int_enum::IntEnum;
use tracing::debug;

use crate::{
    gbc::{
        lcd_controller::{self, LCDControl, TileDataArea, TileMapArea, VRAMBank},
        GBCState, GBC,
    },
    util::index_bits,
};

use super::GBC_RESOLUTION_X;

const TILE_MAP_0_ADDR: u16 = 0x9800;
const TILE_MAP_1_ADDR: u16 = 0x9C00;

const TILE_DATA_LOWER_ADDR: u16 = 0x8000;
const TILE_DATA_UPPER_ADDR: u16 = 0x9000;

const BYTES_PER_TILE: u16 = 16;
const BYTES_PER_TILE_LINE: u16 = 2;

enum PixelFetcherState {
    FetchTileID,
    FetchTileRowLow {
        tile_id: u8,
        tile_attr: TileAttributes,
        fetching_window: bool,
    },
    FetchTileRowHigh {
        tile_row_low: u8,
        addr: u16,
        tile_attr: TileAttributes,
    },
    PixelsReady {
        pixels: [Pixel; 8],
    },
    FinishedScanline,
}
impl PixelFetcherState {
    pub fn initial_state() -> PixelFetcherState {
        PixelFetcherState::FetchTileID
    }
}

#[derive(Clone, Copy)]
struct TileAttributes {
    vertical_flip: bool,
    horizontal_flip: bool,
    vram_bank: VRAMBank,
    palette: u8,
}
impl From<u8> for TileAttributes {
    fn from(val: u8) -> Self {
        Self {
            // TODO BG-to-OAM priority bit 7
            vertical_flip: index_bits(val, 6),
            horizontal_flip: index_bits(val, 5),
            // Bit 4 is not used
            vram_bank: VRAMBank::from_int(index_bits(val, 3) as u8).unwrap(),
            palette: val & 0x07,
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct Pixel {
    pub color_idx: u8,
    pub palette: u8,
    // sprite_priority: u8,
    // background_priority: bool,
}

pub(super) struct PixelFetcher {
    state: PixelFetcherState,
    // Current display X coordinate we are fetching for
    fetching_x: u8,
}
impl PixelFetcher {
    pub fn new() -> Self {
        Self {
            state: PixelFetcherState::initial_state(),
            fetching_x: 0,
        }
    }
}

pub(super) fn tick(state: &mut GBCState, ctrl_reg: &LCDControl) {
    match state.render_engine.pixel_fetcher.state {
        PixelFetcherState::FetchTileID => {
            lcd_controller::maybe_trigger_window_x_requirement(
                state,
                state.render_engine.pixel_fetcher.fetching_x,
            );

            let fetching_window = should_fetch_window(state, ctrl_reg);

            let (tile_id, tile_attr) = get_bg_tile_id_and_attr(state, ctrl_reg, fetching_window);
            state.render_engine.pixel_fetcher.state = PixelFetcherState::FetchTileRowLow {
                tile_id,
                tile_attr,
                fetching_window,
            };
        }
        PixelFetcherState::FetchTileRowLow {
            tile_id,
            tile_attr,
            fetching_window,
        } => {
            let addr = get_bg_tile_data_addr(state, tile_id, &tile_attr, ctrl_reg, fetching_window);
            let tile_row_low = get_bg_tile_data(state, addr, &tile_attr);
            state.render_engine.pixel_fetcher.state = PixelFetcherState::FetchTileRowHigh {
                tile_row_low,
                tile_attr,
                addr: addr + 1,
            };
        }
        PixelFetcherState::FetchTileRowHigh {
            tile_row_low,
            tile_attr,
            addr,
        } => {
            let tile_row_high = get_bg_tile_data(state, addr, &tile_attr);
            let pixels = build_pixels_from_tile_row(tile_row_high, tile_row_low, &tile_attr);
            state.render_engine.pixel_fetcher.state = PixelFetcherState::PixelsReady { pixels };
        }
        PixelFetcherState::PixelsReady { ref pixels } => {
            if !state.render_engine.bg_fifo.is_empty() {
                return;
            }

            let mut num_fetched_pixels = 0;
            if state.render_engine.pixel_fetcher.fetching_x == 0 {
                // Remove tile pixels outside of scroll range at beginning of scanline
                let pixels_to_skip = lcd_controller::get_scroll_x(state) % 8;
                state
                    .render_engine
                    .bg_fifo
                    .extend(pixels.iter().skip(pixels_to_skip.into()).copied());
                num_fetched_pixels = 8 - pixels_to_skip;
            } else if state.render_engine.pixel_fetcher.fetching_x >= GBC_RESOLUTION_X - 8 {
                // Remove tile pixels outside of scroll range at end of scanline
                let pixels_to_include = 8 - (lcd_controller::get_scroll_x(state) % 8);
                state
                    .render_engine
                    .bg_fifo
                    .extend(pixels.iter().take(pixels_to_include.into()).copied());
                num_fetched_pixels = pixels_to_include;
            } else {
                state.render_engine.bg_fifo.extend(pixels.iter().copied());
                num_fetched_pixels = 8;
            }

            let next_x = state.render_engine.pixel_fetcher.fetching_x + num_fetched_pixels;
            if next_x >= GBC_RESOLUTION_X {
                state.render_engine.pixel_fetcher.state = PixelFetcherState::FinishedScanline;
                state.render_engine.pixel_fetcher.fetching_x = 0;
            } else {
                state.render_engine.pixel_fetcher.state = PixelFetcherState::FetchTileID;
                state.render_engine.pixel_fetcher.fetching_x = next_x;
            }
        }
        PixelFetcherState::FinishedScanline => {
            // Do nothing
        }
    }
}

fn should_fetch_window(state: &GBCState, ctrl_reg: &LCDControl) -> bool {
    state.lcd_ctrl.window_x_triggered && state.lcd_ctrl.window_y_triggered && ctrl_reg.window_enable
}

/**
 * Use the current X and Y coordinate to fetch the current tile id from
 * the tile map
 */
fn get_bg_tile_id_and_attr(
    state: &mut GBCState,
    ctrl_reg: &LCDControl,
    fetching_window: bool,
) -> (u8, TileAttributes) {
    let map_area = if fetching_window {
        ctrl_reg.window_tile_map_area
    } else {
        ctrl_reg.bg_tile_map_area
    };
    let tile_map_base_addr = match map_area {
        TileMapArea::Map0 => TILE_MAP_0_ADDR,
        TileMapArea::Map1 => TILE_MAP_1_ADDR,
    };

    let (x_coordinate, y_coordinate) = if fetching_window {
        let x = state.render_engine.pixel_fetcher.fetching_x;
        let lcd_y = lcd_controller::get_lcd_y_coordinate(state);
        let y = lcd_controller::get_window_y_coordinate(state).saturating_sub(lcd_y);
        (x, y)
    } else {
        let scroll_x = lcd_controller::get_scroll_x(state);
        // Purposely wrap around after 256 pixels (max u8 size)
        let x = state
            .render_engine
            .pixel_fetcher
            .fetching_x
            .wrapping_add(scroll_x);

        let lcd_y = lcd_controller::get_lcd_y_coordinate(state);
        let scroll_y = lcd_controller::get_scroll_y(state);
        // Purposely wrap around after 256 pixels (max u8 size)
        let y = lcd_y.wrapping_add(scroll_y);
        (x, y)
    };

    // Each tile in the 32x32 tile map corresponds to 8x8 pixels
    let x_tile_map_idx = (x_coordinate / 8) as u16;
    let y_tile_map_idx = (y_coordinate / 8) as u16;

    let tile_map_addr = tile_map_base_addr + (32 * y_tile_map_idx) + x_tile_map_idx;

    let tile_id = lcd_controller::read_from_vram_bank(state, tile_map_addr, VRAMBank::Bank0);
    let tile_attr = lcd_controller::read_from_vram_bank(state, tile_map_addr, VRAMBank::Bank1);
    (tile_id, TileAttributes::from(tile_attr))
}

fn get_bg_tile_data_addr(
    state: &GBCState,
    tile_id: u8,
    tile_attr: &TileAttributes,
    ctrl_reg: &LCDControl,
    fetching_window: bool,
) -> u16 {
    let lcd_y = lcd_controller::get_lcd_y_coordinate(state);
    let y_coordinate = if fetching_window {
        lcd_controller::get_window_y_coordinate(state).saturating_sub(lcd_y)
    } else {
        let scroll_y = lcd_controller::get_scroll_y(state);
        // Purposely wrap around after 256 pixels (max u8 size)
        lcd_y.wrapping_add(scroll_y)
    };

    let mut tile_addr = match ctrl_reg.bg_and_window_tile_data_area {
        TileDataArea::Lower => TILE_DATA_LOWER_ADDR + (BYTES_PER_TILE * tile_id as u16),
        TileDataArea::Upper => {
            // When querying from uppper data area tile id is treated as a signed int
            let tile_idx = BYTES_PER_TILE as i16 * (tile_id as i8 as i16);
            match tile_idx.is_negative() {
                false => TILE_DATA_UPPER_ADDR + tile_idx as u16,
                true => TILE_DATA_UPPER_ADDR - ((tile_idx * -1) as u16),
            }
        }
    };

    // Get the offset for the specific tile line
    let mut y_offset = (y_coordinate as u16 % 8) * BYTES_PER_TILE_LINE;
    if tile_attr.vertical_flip {
        // Flip the 3 bits that are y_offset to read the tile backwards
        y_offset = (!y_offset) & 0x0E;
    }
    tile_addr |= y_offset;
    tile_addr
}

fn get_bg_tile_data(state: &mut GBCState, tile_addr: u16, attr: &TileAttributes) -> u8 {
    let mut tile_row_half = lcd_controller::read_from_vram_bank(state, tile_addr, attr.vram_bank);
    if attr.horizontal_flip {
        tile_row_half = tile_row_half.reverse_bits();
    }
    tile_row_half
}

fn build_pixels_from_tile_row(row_high: u8, row_low: u8, attr: &TileAttributes) -> [Pixel; 8] {
    core::array::from_fn(|i| {
        let color_idx = (index_bits(row_high, 7 - i) as u8) << 1 | index_bits(row_low, 7 - i) as u8;
        Pixel {
            color_idx,
            palette: attr.palette,
        }
    })
}
