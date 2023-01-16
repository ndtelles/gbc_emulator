mod memory_area;
mod memory_bank_controller;

use std::{borrow::Cow, cmp::min};

use enum_map::{enum_map, EnumMap};

use crate::gbc::virtual_memory::memory_area::MemoryPermission;

use self::{
    memory_area::{MemoryArea, MemoryAreaName},
    memory_bank_controller::{get_num_ext_ram_banks, get_num_rom_banks, MBC},
};

use super::{dma_controller, GBCState};

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
const OAM_DMA_REGISTER: u16 = 0xFF46;
pub const VRAM_DMA_REGISTER: u16 = 0xFF55;

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
    pub fn new(rom_data: Vec<u8>) -> Self {
        let num_rom_banks = get_num_rom_banks(&rom_data);
        let num_ext_ram_banks = get_num_ext_ram_banks(&rom_data);
        let mbc = memory_bank_controller::build_mbc(&rom_data);

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
        vm
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
        IO_REGISTERS_ADDR..=IO_REGISTERS_ADDR_END => MemoryAreaName::IORegisters,
        HIGH_RAM_ADDR..=HIGH_RAM_ADDR_END => MemoryAreaName::HighRam,
        IE_REGISTER_ADDR => MemoryAreaName::IERegister,
        // Invalid address areas
        0xE000..=0xFDFF | 0xFEA0..=0xFEFF => unimplemented!(),
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
        _ => {}
    };
}

pub fn read(state: &GBCState, addr: u16) -> u8 {
    let area = map_memory(addr);
    state.mem.areas[area].read(addr)
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
    write_without_triggers(state, addr, val);
    handle_write_triggered_events(state, addr, val);
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
