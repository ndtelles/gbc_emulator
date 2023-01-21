mod memory_area;
mod memory_bank_controller;

use std::{borrow::Cow, cmp::min};

use color_eyre::eyre::Result;
use enum_map::{enum_map, EnumMap};
use tracing::{debug, debug_span, error, info_span, trace};

use crate::{gbc::virtual_memory::memory_area::MemoryPermission, util::index_bits};

use self::{
    memory_area::{MemoryArea, MemoryAreaName},
    memory_bank_controller::{get_num_ext_ram_banks, get_num_rom_banks, MBC},
};

use super::{
    dma_controller,
    lcd_controller::{self, LCD_STATUS_REGISTER, LCD_Y_COORDINATE_REGISTER},
    GBCState,
};

/**
 * Memory areas boundaries in contigous order. Would use ranges but
 * const ranges can't be used in rust for pattern matching :(
 */
const PRG_ROM_FIXED_ADDR: u16 = 0x0000;
const PRG_ROM_FIXED_ADDR_END: u16 = 0x3FFF;

const PRG_ROM_BANKED_ADDR: u16 = 0x4000;
const PRG_ROM_BANKED_ADDR_END: u16 = 0x7FFF;

const VRAM_ADDR: u16 = 0x8000;
const VRAM_ADDR_END: u16 = 0x9FFF;

const EXTERNAL_RAM_ADDR: u16 = 0xA000;
const EXTERNAL_RAM_ADDR_END: u16 = 0xBFFF;

const WORK_RAM_FIXED_ADDR: u16 = 0xC000;
const WORK_RAM_FIXED_ADDR_END: u16 = 0xCFFF;

const WORK_RAM_BANKED_ADDR: u16 = 0xD000;
const WORK_RAM_BANKED_ADDR_END: u16 = 0xDFFF;

pub const OAM_ADDR: u16 = 0xFE00;
const OAM_ADDR_END: u16 = 0xFE9F;

const BG_PALETTE_ADDR: u16 = 0xFF69;
const IO_REGISTERS_ADDR: u16 = 0xFF00;
const IO_REGISTERS_ADDR_END: u16 = 0xFF7F;

const HIGH_RAM_ADDR: u16 = 0xFF80;
const HIGH_RAM_ADDR_END: u16 = 0xFFFE;

const IE_REGISTER_ADDR: u16 = 0xFFFF;

/**
 * Registers that trigger behaviors when written to
 */
const WORK_RAM_BANK_REGISTER: u16 = 0xFF70;
pub const VRAM_BANK_REGISTER: u16 = 0xFF4F;
const LY_COMPARE_REGISTER: u16 = 0xFF45;
const OAM_DMA_REGISTER: u16 = 0xFF46;
pub const VRAM_DMA_REGISTER: u16 = 0xFF55;
const BG_PALETTE_INDEX_REGISTER: u16 = 0xFF68;

/**
 * ROM Data Addresses
 */
const ROM_SIZE_ADDR: u16 = 0x0148;
const EXT_RAM_SIZE_ADDR: u16 = 0x0149;

pub struct VirtualMemory {
    areas: EnumMap<MemoryAreaName, MemoryArea>,
    mbc: Box<dyn MBC>,
}

impl VirtualMemory {
    pub fn new(rom_data: Vec<u8>) -> Result<Self> {
        let num_rom_banks = get_num_rom_banks(&rom_data)?;
        let num_ext_ram_banks = get_num_ext_ram_banks(&rom_data)?;
        let mbc = memory_bank_controller::build_mbc(&rom_data)?;

        let mut vm = Self {
            mbc,
            areas: enum_map! {
                MemoryAreaName::PrgRomFixed => MemoryArea::new(
                    PRG_ROM_FIXED_ADDR,
                    PRG_ROM_FIXED_ADDR_END,
                    1,
                    MemoryPermission::ReadOnly,
                ),
                // Banked ROM. Bank 0 includes a duplicate of the fixed ROM data so that the
                // ROM data 0x0000..=0x3FFF can be selected by MBC5 banking at this address
                MemoryAreaName::PrgRomBanked => MemoryArea::new(
                    PRG_ROM_BANKED_ADDR,
                    PRG_ROM_BANKED_ADDR_END,
                    num_rom_banks,
                    MemoryPermission::ReadOnly,
                ),
                MemoryAreaName::Vram => MemoryArea::new(VRAM_ADDR, VRAM_ADDR_END, 2, MemoryPermission::ReadAndWrite),
                MemoryAreaName::ExternalRam => MemoryArea::new(EXTERNAL_RAM_ADDR, EXTERNAL_RAM_ADDR_END, num_ext_ram_banks, MemoryPermission::None),
                MemoryAreaName::WorkRamFixed => MemoryArea::new(
                    WORK_RAM_FIXED_ADDR,
                    WORK_RAM_FIXED_ADDR_END,
                    1,
                    MemoryPermission::ReadAndWrite,
                ),
                MemoryAreaName::WorkRamBanked => MemoryArea::new(
                    WORK_RAM_BANKED_ADDR,
                    WORK_RAM_BANKED_ADDR_END,
                    7,
                    MemoryPermission::ReadAndWrite,
                ),
                MemoryAreaName::Oam => MemoryArea::new(
                    OAM_ADDR,
                    OAM_ADDR_END,
                    1,
                    MemoryPermission::ReadAndWrite,
                ),
                // 64 Bytes of selectable palette memory
                MemoryAreaName::BGPalette => MemoryArea::new(BG_PALETTE_ADDR, BG_PALETTE_ADDR, 64, MemoryPermission::ReadAndWrite),
                MemoryAreaName::IORegisters => MemoryArea::new(
                    IO_REGISTERS_ADDR,
                    IO_REGISTERS_ADDR_END,
                    1,
                    MemoryPermission::ReadAndWrite,
                ),
                MemoryAreaName::HighRam => MemoryArea::new(
                    HIGH_RAM_ADDR,
                    HIGH_RAM_ADDR_END,
                    1,
                    MemoryPermission::ReadAndWrite,
                ),
                MemoryAreaName::IERegister => MemoryArea::new(
                    IE_REGISTER_ADDR,
                    IE_REGISTER_ADDR,
                    1,
                    MemoryPermission::ReadAndWrite,
                ),
            },
        };
        // Bank 0 contains a duplicate of the fixed bank for MBC5. Default to bank 1
        vm.areas[MemoryAreaName::PrgRomBanked].set_active_bank(1);
        vm.areas[MemoryAreaName::PrgRomFixed]
            .fill_from_src(&rom_data[..=PRG_ROM_FIXED_ADDR_END.into()]);
        vm.areas[MemoryAreaName::PrgRomBanked].fill_from_src(&rom_data);
        Ok(vm)
    }
}

fn map_memory(addr: u16) -> MemoryAreaName {
    match addr {
        PRG_ROM_FIXED_ADDR..=PRG_ROM_FIXED_ADDR_END => MemoryAreaName::PrgRomFixed,
        PRG_ROM_BANKED_ADDR..=PRG_ROM_BANKED_ADDR_END => MemoryAreaName::PrgRomBanked,
        VRAM_ADDR..=VRAM_ADDR_END => MemoryAreaName::Vram,
        EXTERNAL_RAM_ADDR..=EXTERNAL_RAM_ADDR_END => MemoryAreaName::ExternalRam,
        WORK_RAM_FIXED_ADDR..=WORK_RAM_FIXED_ADDR_END => MemoryAreaName::WorkRamFixed,
        WORK_RAM_BANKED_ADDR..=WORK_RAM_BANKED_ADDR_END => MemoryAreaName::WorkRamBanked,
        OAM_ADDR..=OAM_ADDR_END => MemoryAreaName::Oam,
        // BG Pallete must be specified before IO registers in this mapping!
        BG_PALETTE_ADDR => MemoryAreaName::BGPalette,
        IO_REGISTERS_ADDR..=IO_REGISTERS_ADDR_END => MemoryAreaName::IORegisters,
        HIGH_RAM_ADDR..=HIGH_RAM_ADDR_END => MemoryAreaName::HighRam,
        IE_REGISTER_ADDR => MemoryAreaName::IERegister,
        // Invalid address areas
        0xE000..=0xFDFF | 0xFEA0..=0xFEFF => {
            error!("Invalid memory area");
            unimplemented!();
        }
    }
}

/**
 * Preprocess value for some addresses
 */
fn preprocess_value(state: &GBCState, addr: u16, val: u8) -> u8 {
    match addr {
        LCD_STATUS_REGISTER => {
            // ROM program should not be able to set lower 3 bits
            let stat = state.mem.areas[MemoryAreaName::IORegisters].read(addr);
            (val & 0xF8) | (stat & 0x07)
        }
        _ => val,
    }
}

/**
 * Writing to some addresses trigger events such as setting bank registers or starting DMA.
 */
fn handle_write_triggered_events(state: &mut GBCState, addr: u16, val: u8) {
    match addr {
        // Writes to the memory bank controller
        PRG_ROM_FIXED_ADDR..=PRG_ROM_FIXED_ADDR_END
        | PRG_ROM_BANKED_ADDR..=PRG_ROM_BANKED_ADDR_END
        | EXTERNAL_RAM_ADDR..=EXTERNAL_RAM_ADDR_END => {
            state
                .mem
                .mbc
                .write_register(&mut state.mem.areas, addr, val);
        }
        WORK_RAM_BANK_REGISTER => {
            // First 3 bits hold the flags. Both 0 and 1 mean the first bank
            let active_bank = (val & 0x07).saturating_sub(1).into();
            state.mem.areas[MemoryAreaName::WorkRamBanked].set_active_bank(active_bank);
        }
        VRAM_BANK_REGISTER => {
            let active_bank = (val & 0x01).into();
            state.mem.areas[MemoryAreaName::Vram].set_active_bank(active_bank);
        }
        OAM_DMA_REGISTER => dma_controller::trigger_oam_transfer(state, val),
        VRAM_DMA_REGISTER => dma_controller::trigger_vram_transfer(state, val),
        // Update LY == LYC comparison
        LY_COMPARE_REGISTER | LCD_Y_COORDINATE_REGISTER => {
            lcd_controller::update_lyc_match_ly_check(state)
        }
        // Set the palette index
        BG_PALETTE_INDEX_REGISTER => {
            let palette_idx = (val & 0x3F).into();
            state.mem.areas[MemoryAreaName::BGPalette].set_active_bank(palette_idx);
        }
        // Maybe auto increment after writing to palette
        BG_PALETTE_ADDR => {
            let palette_reg_val = state.mem.areas[MemoryAreaName::IORegisters].read(addr);
            let auto_incr = index_bits(palette_reg_val, 7);
            if auto_incr {
                let curr_bank = state.mem.areas[MemoryAreaName::BGPalette].get_active_bank();
                let new_bank = (curr_bank + 1) % 64;
                state.mem.areas[MemoryAreaName::BGPalette].set_active_bank(new_bank);
            }
        }
        _ => {}
    };
}

pub fn read(state: &GBCState, addr: u16) -> u8 {
    let span = debug_span!("VM Read", addr = format!("{:#06x}", addr)).entered();

    let area = map_memory(addr);
    let read_val = state.mem.areas[area].read(addr);

    span.exit();
    read_val
}

pub fn read_bytes(state: &GBCState, addr: u16, length_bytes: usize) -> Cow<[u8]> {
    let area = &state.mem.areas[map_memory(addr)];

    // How many bytes can we actually read from this memory area
    let area_read_len = (area.get_end_addr() - addr + 1).into();
    let bytes_to_read = min(area_read_len, length_bytes);

    let mut result = area.read_bytes(addr, bytes_to_read);

    // We may have to read across multiple memory areas
    if length_bytes > bytes_to_read {
        let more = read_bytes(
            state,
            area.get_end_addr().wrapping_add(1),
            length_bytes - bytes_to_read,
        );
        result = Cow::from([result.as_ref(), more.as_ref()].concat());
    }
    result
}

pub fn write(state: &mut GBCState, addr: u16, val: u8) {
    let span = debug_span!(
        "VM Write",
        addr = format!("{:#06x}", addr),
        value = format!("{:#04x}", val)
    )
    .entered();

    let val = preprocess_value(state, addr, val);
    let area = map_memory(addr);
    state.mem.areas[area].write(addr, val);
    handle_write_triggered_events(state, addr, val);

    span.exit();
}

pub fn write_without_triggers(state: &mut GBCState, addr: u16, val: u8) {
    let area = map_memory(addr);
    state.mem.areas[area].write(addr, val);
}

pub fn write_bytes(state: &mut GBCState, addr: u16, vals: &[u8]) {
    let area_name = map_memory(addr);
    let area = &mut state.mem.areas[area_name];
    // How many bytes can we actually write to this memory area
    let area_write_len = (area.get_end_addr() - addr + 1).into();
    let bytes_to_write = min(area_write_len, vals.len());
    let (vals, rest) = vals.split_at(bytes_to_write);
    area.write_bytes(addr, vals);

    // We may have to write across multiple memory areas
    if !rest.is_empty() {
        let next_addr = area.get_end_addr().wrapping_add(1);
        write_bytes(state, next_addr, rest);
    }
}

pub fn borrow_palette_mem(state: &GBCState) -> &[u8] {
    state.mem.areas[MemoryAreaName::BGPalette].borrow_raw_data()
}
