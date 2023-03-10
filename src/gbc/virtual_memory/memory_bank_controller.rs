use std::cmp::max;

use color_eyre::eyre::{bail, ensure, Result};
use enum_map::EnumMap;
use int_enum::IntEnum;

use crate::util::combine_high_low;

use super::{
    memory_area::{MemoryArea, MemoryAreaName, MemoryPermission},
    EXT_RAM_SIZE_ADDR, ROM_SIZE_ADDR,
};

const CARTRIDGE_TYPE_ADDR: u16 = 0x0147;

#[repr(u8)]
#[derive(Clone, Copy, IntEnum)]
enum BankSelectMode {
    UpperROM = 0,
    RAM = 1,
}

pub trait MBC {
    fn write_register(
        &mut self,
        mem_areas: &mut EnumMap<MemoryAreaName, MemoryArea>,
        addr: u16,
        val: u8,
    );
}

struct NoMBC {}
impl MBC for NoMBC {
    fn write_register(
        &mut self,
        _mem_areas: &mut EnumMap<MemoryAreaName, MemoryArea>,
        _addr: u16,
        _val: u8,
    ) {
        // Do nothing
    }
}

struct MBC1 {
    // 5 bit bank select. Both 0x0 and 0x1 map to the first (non-fixed) bank
    rom_bank_select: u8,
    // 2 bits of either upper bank select for ROM or bank select for RAM
    ram_or_upper_rom_bank_select: u8,
    // Whether the 2 bit bank select controls ROM or RAM addressing
    bank_mode: BankSelectMode,
}
impl MBC1 {
    pub fn new() -> Self {
        Self {
            rom_bank_select: 1,
            ram_or_upper_rom_bank_select: 0,
            bank_mode: BankSelectMode::UpperROM,
        }
    }
}
impl MBC for MBC1 {
    fn write_register(
        &mut self,
        mem_areas: &mut EnumMap<MemoryAreaName, MemoryArea>,
        addr: u16,
        val: u8,
    ) {
        match addr {
            // RAM Enable. Any value with 0xA in its lower 4 bits enables RAM
            0x0000..=0x1FFF => match val & 0x0F == 0x0A {
                false => {
                    mem_areas[MemoryAreaName::ExternalRam].set_permission(MemoryPermission::None)
                }
                true => mem_areas[MemoryAreaName::ExternalRam]
                    .set_permission(MemoryPermission::ReadAndWrite),
            },
            0x2000..=0x3FFF => self.rom_bank_select = val & 0x1F,
            0x4000..=0x5FFF => self.ram_or_upper_rom_bank_select = val & 0x03,
            0x6000..=0x7FFF => self.bank_mode = BankSelectMode::from_int(val & 0x01).unwrap(),
            _ => {}
        }

        let (rom_bank, ram_bank) = match self.bank_mode {
            BankSelectMode::UpperROM => {
                // MBC1 can not select ROM bank 0
                let rom_bank = max(
                    (self.ram_or_upper_rom_bank_select << 5) | self.rom_bank_select,
                    1,
                );
                (rom_bank, 0)
            }
            BankSelectMode::RAM => (
                // MBC1 can not select ROM bank 0
                max(self.rom_bank_select, 1),
                self.ram_or_upper_rom_bank_select,
            ),
        };
        mem_areas[MemoryAreaName::PrgRomBanked].set_active_bank(rom_bank.into());
        mem_areas[MemoryAreaName::ExternalRam].set_active_bank(ram_bank.into());
    }
}

struct MBC5 {
    rom_bank_select_low: u8,
    // Upper 1 bit of rom bank select
    rom_bank_select_high: u8,
    ram_bank_select: u8,
}
impl MBC5 {
    pub fn new() -> Self {
        Self {
            rom_bank_select_low: 1,
            rom_bank_select_high: 0,
            ram_bank_select: 0,
        }
    }
}
impl MBC for MBC5 {
    fn write_register(
        &mut self,
        mem_areas: &mut EnumMap<MemoryAreaName, MemoryArea>,
        addr: u16,
        val: u8,
    ) {
        match addr {
            // RAM Enable. Any value with 0xA in its lower 4 bits enables RAM
            0x0000..=0x1FFF => match val & 0x0F == 0x0A {
                false => {
                    mem_areas[MemoryAreaName::ExternalRam].set_permission(MemoryPermission::None)
                }
                true => mem_areas[MemoryAreaName::ExternalRam]
                    .set_permission(MemoryPermission::ReadAndWrite),
            },
            0x2000..=0x2FFF => self.rom_bank_select_low = val,
            0x3000..=0x3FFF => self.rom_bank_select_high = val & 0x01,
            0x4000..=0x5FFF => self.ram_bank_select = val & 0x0F,
            _ => {}
        }
        let rom_bank = combine_high_low(self.rom_bank_select_high, self.rom_bank_select_low).into();

        mem_areas[MemoryAreaName::PrgRomBanked].set_active_bank(rom_bank);
        mem_areas[MemoryAreaName::ExternalRam].set_active_bank(self.ram_bank_select.into());
    }
}

pub(super) fn build_mbc(rom_data: &Vec<u8>) -> Result<Box<dyn MBC>> {
    let code = rom_data[CARTRIDGE_TYPE_ADDR as usize];
    let mbc: Box<dyn MBC> = match code {
        0x00 => Box::new(NoMBC {}),
        0x01..=0x03 => Box::new(MBC1::new()),
        0x19..=0x1E => Box::new(MBC5::new()),
        _ => bail!("Unimplemented or invalid cartridge type code {:#04x}", code),
    };
    Ok(mbc)
}

pub(super) fn get_num_ext_ram_banks(rom_data: &Vec<u8>) -> Result<usize> {
    let code = rom_data[EXT_RAM_SIZE_ADDR as usize];
    let num = match code {
        0x00 => 0,
        0x02 => 1,
        0x03 => 4,
        0x04 => 16,
        0x05 => 8,
        _ => bail!("Invalid external RAM size code {:#04x}", code),
    };
    Ok(num)
}

pub(super) fn get_num_rom_banks(rom_data: &Vec<u8>) -> Result<usize> {
    let code = rom_data[ROM_SIZE_ADDR as usize];
    ensure!(code <= 0x08, "Invalid ROM size code {:#04x}", code);
    Ok(0x2 << code)
}
