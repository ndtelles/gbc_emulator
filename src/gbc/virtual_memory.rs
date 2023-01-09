use std::{borrow::Cow, cmp::min};

use enum_map::{enum_map, Enum, EnumMap};

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
const VRAM_BANK_REGISTER: u16 = 0xFF4F;
const OAM_DMA_REGISTER: u16 = 0xFF46;
pub const VRAM_DMA_REGISTER: u16 = 0xFF55;
const RAM_ENABLE_REGISTER: u16 = 0x0000;
const RAM_ENABLE_REGISTER_END: u16 = 0x1FFF;
const ROM_BANK_REGISTER: u16 = 0x2000;
const ROM_BANK_REGISTER_END: u16 = 0x3FFF;

/**
 * ROM Data Addresses
 */
const ROM_SIZE_ADDR: u16 = 0x0148;

#[derive(Enum)]
enum MemoryAreaName {
    PrgRomFixed,
    PrgRomBanked,
    Vram,
    ExternalRam,
    WorkRamFixed,
    WorkRamBanked,
    Oam,
    IORegisters,
    HighRam,
    IERegister,
}

enum MemoryPermission {
    None,
    ReadOnly,
    ReadAndWrite,
}

/**
 * MemoryArea reprsents the physical memory (including multiple banks)
 * between two virtual addresses
 */
struct MemoryArea {
    start_addr: u16,
    end_addr: u16,
    bank_size: usize,
    num_banks: usize,
    active_bank: usize,
    permission: MemoryPermission,
    data: Vec<u8>,
}

impl MemoryArea {
    pub fn new(
        start_addr: u16,
        end_addr: u16,
        num_banks: usize,
        permission: MemoryPermission,
    ) -> Self {
        debug_assert!(end_addr >= start_addr);
        let bank_size = (end_addr - start_addr + 1) as usize;
        Self {
            start_addr,
            end_addr,
            bank_size,
            num_banks,
            active_bank: 0,
            permission,
            data: vec![0x00; bank_size * num_banks],
        }
    }

    // Convert the u16 virtual address to an index in the data vec
    fn virtual_address_to_data_index(&self, addr: u16) -> usize {
        debug_assert!(self.active_bank < self.num_banks);
        (addr - self.start_addr) as usize + (self.bank_size * self.active_bank)
    }

    pub fn read(&self, addr: u16) -> u8 {
        if let MemoryPermission::None = self.permission {
            return 0xFF;
        }
        let idx = self.virtual_address_to_data_index(addr);
        self.data[idx]
    }

    // Return cow which lets us return either a borrowed or owned value
    pub fn read_bytes(&self, addr: u16, length_bytes: usize) -> Cow<[u8]> {
        if let MemoryPermission::None = self.permission {
            return Cow::from(vec![0xFF; length_bytes]);
        }
        let idx = self.virtual_address_to_data_index(addr);
        let end_idx = idx + length_bytes;
        Cow::from(&self.data[idx..end_idx])
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        if let MemoryPermission::ReadOnly | MemoryPermission::None = self.permission {
            return;
        }
        let idx = self.virtual_address_to_data_index(addr);
        self.data[idx] = val;
    }

    pub fn write_bytes(&mut self, addr: u16, vals: &[u8]) {
        if let MemoryPermission::ReadOnly | MemoryPermission::None = self.permission {
            return;
        }
        let idx = self.virtual_address_to_data_index(addr);
        let end_idx = idx + vals.len();
        self.data.splice(idx..end_idx, vals.iter().cloned());
    }

    pub fn fill_from_src(&mut self, src: &[u8]) {
        assert!(self.data.len() == src.len());
        self.data = src.to_vec();
    }
}

pub struct VirtualMemory {
    areas: EnumMap<MemoryAreaName, MemoryArea>,
}

impl VirtualMemory {
    pub fn new(rom_data: Vec<u8>) -> Self {
        let rom_size_indicator = rom_data[ROM_SIZE_ADDR as usize];
        assert!(rom_size_indicator <= 0x08);
        let num_rom_banks = (0x2 << rom_size_indicator) - 1;

        let mut vm = Self {
            areas: enum_map! {
                MemoryAreaName::PrgRomFixed => MemoryArea::new(
                    PRG_ROM_FIXED_ADDR,
                    PRG_ROM_FIXED_ADDR_END,
                    1,
                    MemoryPermission::ReadOnly,
                ),
                MemoryAreaName::PrgRomBanked => MemoryArea::new(
                    PRG_ROM_BANKED_ADDR,
                    PRG_ROM_BANKED_ADDR_END,
                    num_rom_banks,
                    MemoryPermission::ReadOnly,
                ),
                MemoryAreaName::Vram => MemoryArea::new(VRAM_ADDR, VRAM_ADDR_END, 2, MemoryPermission::ReadAndWrite),
                // @TODO: Build out external ram
                MemoryAreaName::ExternalRam => MemoryArea::new(EXTERNAL_RAM_ADDR, EXTERNAL_RAM_ADDR_END, 1, MemoryPermission::None),
                MemoryAreaName::WorkRamFixed =>MemoryArea::new(
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
        let (fixed_rom_data, banked_rom_data) = rom_data.split_at(PRG_ROM_BANKED_ADDR.into());
        vm.areas[MemoryAreaName::PrgRomFixed].fill_from_src(fixed_rom_data);
        vm.areas[MemoryAreaName::PrgRomBanked].fill_from_src(banked_rom_data);
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
        RAM_ENABLE_REGISTER..=RAM_ENABLE_REGISTER_END => match val & 0x0F == 0x0A {
            false => {
                state.mem.areas[MemoryAreaName::ExternalRam].permission = MemoryPermission::None
            }
            true => {
                state.mem.areas[MemoryAreaName::ExternalRam].permission =
                    MemoryPermission::ReadAndWrite
            }
        },
        ROM_BANK_REGISTER..=ROM_BANK_REGISTER_END => todo!(),
        WORK_RAM_BANK_REGISTER => {
            // First 3 bits hold the flags. Both 0 and 1 mean the first bank
            let new_bank = (val & 0x07).saturating_sub(1).into();
            state.mem.areas[MemoryAreaName::WorkRamBanked].active_bank = new_bank;
        }
        VRAM_BANK_REGISTER => {
            let new_bank = (val & 0x01).into();
            state.mem.areas[MemoryAreaName::Vram].active_bank = new_bank;
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
    let area_read_len = (area.end_addr - addr + 1).into();
    let bytes_to_read = min(area_read_len, length_bytes);

    let mut result = area.read_bytes(addr, bytes_to_read);

    // We may have to read across multiple memory areas
    if length_bytes > bytes_to_read {
        let more = read_bytes(
            state,
            area.end_addr.wrapping_add(1),
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
    let area = &mut state.mem.areas[map_memory(addr)];
    // How many bytes can we actually write to this memory area
    let area_write_len = (area.end_addr - addr + 1).into();
    let bytes_to_write = min(area_write_len, vals.len());
    let (vals, rest) = vals.split_at(bytes_to_write);
    area.write_bytes(addr, vals);

    // We may have to write across multiple memory areas
    if !rest.is_empty() {
        let next_addr = area.end_addr.wrapping_add(1);
        write_bytes(state, next_addr, rest);
    }
}
