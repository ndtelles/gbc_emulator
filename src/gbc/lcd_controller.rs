use int_enum::IntEnum;

use crate::util::index_bits;

use super::{
    virtual_memory::{self, VRAM_BANK_REGISTER},
    GBCState,
};

const LCD_CONTROL_REGISTER: u16 = 0xFF40;
const LCD_STATUS_REGISTER: u16 = 0xFF41;
const SCROLL_Y_REGISTER: u16 = 0xFF42;
const SCROLL_X_REGISTER: u16 = 0xFF43;
const LCD_Y_COORDINATE_REGISTER: u16 = 0xFF44;

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
#[derive(Clone, Copy, IntEnum)]
pub enum PPUMode {
    HBlank = 0,
    VBlank = 1,
    OAMScan = 2,
    Drawing = 3,
}

// STAT register
pub struct LCDStatus {
    pub lyc_match_ly_interrupt_select: bool,
    pub oam_stat_interrupt_source: bool,
    pub vblank_interrupt_source: bool,
    pub hblank_interrupt_source: bool,
    pub lyc_match_ly: bool,
    pub ppu_mode: PPUMode,
}

impl From<u8> for LCDStatus {
    fn from(val: u8) -> Self {
        Self {
            lyc_match_ly_interrupt_select: index_bits(val, 6),
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

pub fn set_vram_bank(state: &mut GBCState, bank: VRAMBank) {
    virtual_memory::write(state, VRAM_BANK_REGISTER, bank as u8)
}

pub fn get_vram_bank(state: &GBCState) -> VRAMBank {
    VRAMBank::from_int(virtual_memory::read(state, VRAM_BANK_REGISTER) & 0x01).unwrap()
}

pub fn read_from_vram_bank(state: &mut GBCState, addr: u16, bank: VRAMBank) -> u8 {
    let original_bank_reg = get_vram_bank(state);
    set_vram_bank(state, bank);

    let data = virtual_memory::read(state, addr);

    // Set VRAM bank back. Not sure if this is necessary
    set_vram_bank(state, original_bank_reg);
    data
}
