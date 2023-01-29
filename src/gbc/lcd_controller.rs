use int_enum::IntEnum;
use tracing::{trace, debug_span};

use crate::{
    util::index_bits, gbc::dma_controller,
};

use super::{
    virtual_memory,
    GBCState,
};

const LCD_CONTROL_REGISTER: u16 = 0xFF40;
pub const LCD_STATUS_REGISTER: u16 = 0xFF41;
const SCROLL_Y_REGISTER: u16 = 0xFF42;
const SCROLL_X_REGISTER: u16 = 0xFF43;
pub const LCD_Y_COORDINATE_REGISTER: u16 = 0xFF44;
pub const LY_COMPARE_REGISTER: u16 = 0xFF45;

const CYCLES_PER_SCANLINE: u16 = 114;
const CYCLES_BEFORE_DRAWING: u16 = 20;
const VERTICAL_BLANK_BEGIN_CYCLE: u16 = 16416;

#[repr(u8)]
#[derive(Clone, Copy, IntEnum)]
pub enum TileMapArea {
    Map0 = 0,
    Map1 = 1,
}

#[repr(u8)]
#[derive(Clone, Copy, IntEnum)]
pub enum TileDataArea {
    Lower = 0,
    Upper = 1,
}

#[repr(u8)]
#[derive(Clone, Copy, IntEnum)]
pub enum VRAMBank {
    Bank0 = 0,
    Bank1 = 1,
}

pub struct LCDControl {
    pub lcd_enable: bool,
    pub window_tile_map_area: bool,
    pub window_enable: bool,
    pub bg_and_window_tile_data_area: TileDataArea,
    pub bg_tile_map_area: TileMapArea,
    pub obj_size: bool,
    pub obj_enable: bool,
    pub bg_and_window_priority: bool,
}

impl From<u8> for LCDControl {
    fn from(val: u8) -> Self {
        Self {
            lcd_enable: index_bits(val, 7),
            window_tile_map_area: index_bits(val, 6),
            window_enable: index_bits(val, 5),
            bg_and_window_tile_data_area: TileDataArea::from_int(index_bits(val, 4) as u8).unwrap(),
            bg_tile_map_area: TileMapArea::from_int(index_bits(val, 3) as u8).unwrap(),
            obj_size: index_bits(val, 2),
            obj_enable: index_bits(val, 1),
            bg_and_window_priority: index_bits(val, 0),
        }
    }
}

pub fn get_lcd_control_register(state: &GBCState) -> LCDControl {
    LCDControl::from(virtual_memory::read(state, LCD_CONTROL_REGISTER))
}

#[repr(u8)]
#[derive(Clone, Copy, IntEnum, PartialEq)]
pub enum PPUMode {
    HBlank = 0,
    VBlank = 1,
    OAMScan = 2,
    Drawing = 3,
}

// STAT register
pub struct LCDStatus {
    pub lyc_match_ly_interrupt_source: bool,
    pub oam_stat_interrupt_source: bool,
    pub vblank_interrupt_source: bool,
    pub hblank_interrupt_source: bool,
    pub lyc_match_ly: bool,
    pub ppu_mode: PPUMode,
}

impl From<u8> for LCDStatus {
    fn from(val: u8) -> Self {
        Self {
            lyc_match_ly_interrupt_source: index_bits(val, 6),
            oam_stat_interrupt_source: index_bits(val, 5),
            vblank_interrupt_source: index_bits(val, 4),
            hblank_interrupt_source: index_bits(val, 3),
            lyc_match_ly: index_bits(val, 2),
            ppu_mode: PPUMode::from_int(val & 0x03).unwrap(),
        }
    }
}

pub fn get_lcd_status_register(state: &GBCState) -> LCDStatus {
    LCDStatus::from(virtual_memory::read(state, LCD_STATUS_REGISTER))
}

pub fn get_scroll_y(state: &GBCState) -> u8 {
    virtual_memory::read(state, SCROLL_Y_REGISTER)
}

pub fn get_scroll_x(state: &GBCState) -> u8 {
    virtual_memory::read(state, SCROLL_X_REGISTER)
}

// Current horizontal line
pub fn get_lcd_y_coordinate(state: &GBCState) -> u8 {
    virtual_memory::read(state, LCD_Y_COORDINATE_REGISTER)
}

pub fn read_from_vram_bank(state: &mut GBCState, addr: u16, bank: VRAMBank) -> u8 {
    virtual_memory::read_override_bank(state, addr, bank.int_value().into())
}

pub fn tick(state: &mut GBCState) {
    let span = debug_span!("LCD Controller").entered();

    let scanline_idx = state.machine_cycle % CYCLES_PER_SCANLINE;
    if scanline_idx == 0 {
        // Beginning of scanline
        set_lcd_y_coordinate(state, (state.machine_cycle / CYCLES_PER_SCANLINE) as u8)
    }

    match scanline_idx {
        0 if state.machine_cycle == VERTICAL_BLANK_BEGIN_CYCLE => {
            update_ppu_mode(state, PPUMode::VBlank)
        }
        0 if state.machine_cycle < VERTICAL_BLANK_BEGIN_CYCLE => {
            update_ppu_mode(state, PPUMode::OAMScan)
        }
        CYCLES_BEFORE_DRAWING if state.machine_cycle < VERTICAL_BLANK_BEGIN_CYCLE => {
            update_ppu_mode(state, PPUMode::Drawing);
        }
        _ => {}
    };

    span.exit();
}

pub fn update_ppu_mode(state: &mut GBCState, new_mode: PPUMode) {
    trace!("PPU Mode updated to {}", new_mode.int_value());
    let mut val = virtual_memory::read(state, LCD_STATUS_REGISTER);
    val = (val & 0xFC) | new_mode.int_value();
    virtual_memory::write_without_triggers(state, LCD_STATUS_REGISTER, val);

    if let PPUMode::HBlank = new_mode {
        // Trigger hblank DMA transfer
        dma_controller::process_hblank_transfer(state);
    }
}

/**
 * Update comparison of lyc and ly in STAT register
 */
pub fn update_lyc_match_ly_check(state: &mut GBCState) {
    let ly = virtual_memory::read(state, LCD_Y_COORDINATE_REGISTER);
    let lyc = virtual_memory::read(state, LY_COMPARE_REGISTER);
    let mut stat = virtual_memory::read(state, LCD_STATUS_REGISTER);
    let matches = ly == lyc;
    stat = (stat & 0xFB) | ((matches as u8) << 2);
    virtual_memory::write(state, LCD_STATUS_REGISTER, stat);
}

// Should only be set internally by lcd controller
fn set_lcd_y_coordinate(state: &mut GBCState, y: u8) {
    trace!("LCD Y set to {}", y);
    // Write will trigger update_lyc_match_ly_check call from vm
    virtual_memory::write(state, LCD_Y_COORDINATE_REGISTER, y);
}
