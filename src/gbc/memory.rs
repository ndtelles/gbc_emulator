use enum_map::{enum_map, Enum, EnumMap};

// Memory areas boundaries in contigous order. Would use ranges but const ranges can't be used in rust for pattern matching :(
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

const OAM_ADDR: u16 = 0xFE00;
const OAM_ADDR_END: u16 = 0xFE9F;

const IO_REGISTERS_ADDR: u16 = 0xFF00;
const IO_REGISTERS_ADDR_END: u16 = 0xFF7F;

const HIGH_RAM_ADDR: u16 = 0xFF80;
const HIGH_RAM_ADDR_END: u16 = 0xFFFE;

const IE_REGISTER_ADDR: u16 = 0xFFFF;

/**
 * Special registers
 */
const WORK_RAM_BANK_REGISTER: u16 = 0xFF70;
const VRAM_BANK_REGISTER: u16 = 0xFF4F;

#[derive(Enum)]
pub enum MemoryAreaName {
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
    ReadOnly,
    ReadAndWrite,
}

/**
 * MemoryArea reprsents the physical memory (including multiple banks)
 * between two virtual addresses
 */
struct MemoryArea {
    start_addr: u16,
    bank_size: usize,
    num_banks: usize,
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
            bank_size,
            num_banks,
            permission,
            data: vec![0x00; bank_size * num_banks],
        }
    }

    // Convert the u16 virtual address to an index in the data vec
    fn virtual_address_to_data_index(&self, addr: u16, bank: usize) -> usize {
        debug_assert!(bank < self.num_banks);
        (addr - self.start_addr) as usize + (self.bank_size * bank)
    }

    pub fn read(&self, addr: u16, bank: usize) -> u8 {
        let idx = self.virtual_address_to_data_index(addr, bank);
        self.data[idx]
    }

    pub fn write(&mut self, addr: u16, bank: usize, val: u8) {
        if let MemoryPermission::ReadOnly = self.permission {
            // Read only memory is often written to to peform special hardware operations.
            // Don't actually write to the memory.
            return;
        }
        let idx = self.virtual_address_to_data_index(addr, bank);
        self.data[idx] = val;
    }
}

pub struct VirtualMemory {
    memory_areas: EnumMap<MemoryAreaName, MemoryArea>,
}

impl VirtualMemory {
    pub fn new() -> Self {
        Self {
            memory_areas: enum_map! {
                MemoryAreaName::PrgRomFixed => MemoryArea::new(
                    PRG_ROM_FIXED_ADDR,
                    PRG_ROM_FIXED_ADDR_END,
                    1,
                    MemoryPermission::ReadOnly,
                ),
                // @TODO: Imlement banking for Prg ROM
                MemoryAreaName::PrgRomBanked => MemoryArea::new(
                    PRG_ROM_BANKED_ADDR,
                    PRG_ROM_BANKED_ADDR_END,
                    1,
                    MemoryPermission::ReadOnly,
                ),
                MemoryAreaName::Vram => MemoryArea::new(VRAM_ADDR, VRAM_ADDR_END, 2, MemoryPermission::ReadAndWrite),
                // @TODO: Build out external ram
                MemoryAreaName::ExternalRam => MemoryArea::new(EXTERNAL_RAM_ADDR, EXTERNAL_RAM_ADDR_END, 1, MemoryPermission::ReadAndWrite),
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
        }
    }

    // Map virtual memory address to actual memory. Returns a reference to the memory area and
    // the currently active bank
    fn map_memory(&self, addr: u16) -> (MemoryAreaName, usize) {
        match addr {
            PRG_ROM_FIXED_ADDR..=PRG_ROM_FIXED_ADDR_END => (MemoryAreaName::PrgRomFixed, 0),
            PRG_ROM_BANKED_ADDR..=PRG_ROM_BANKED_ADDR_END => (MemoryAreaName::PrgRomBanked, 0),
            VRAM_ADDR..=VRAM_ADDR_END => {
                let bank = self.get_vram_bank();
                (MemoryAreaName::Vram, bank)
            }
            EXTERNAL_RAM_ADDR..=EXTERNAL_RAM_ADDR_END => (MemoryAreaName::ExternalRam, 0),
            WORK_RAM_FIXED_ADDR..=WORK_RAM_FIXED_ADDR_END => (MemoryAreaName::WorkRamFixed, 0),
            WORK_RAM_BANKED_ADDR..=WORK_RAM_BANKED_ADDR_END => {
                let bank = self.get_work_ram_bank();
                (MemoryAreaName::WorkRamBanked, bank)
            }
            OAM_ADDR..=OAM_ADDR_END => (MemoryAreaName::Oam, 0),
            IO_REGISTERS_ADDR..=IO_REGISTERS_ADDR_END => (MemoryAreaName::IORegisters, 0),
            HIGH_RAM_ADDR..=HIGH_RAM_ADDR_END => (MemoryAreaName::HighRam, 0),
            IE_REGISTER_ADDR => (MemoryAreaName::IERegister, 0),
            // Invalid address areas
            0xE000..=0xFDFF | 0xFEA0..=0xFEFF => unimplemented!(),
        }
    }

    fn get_work_ram_bank(&self) -> usize {
        let io_reg_mem = &self.memory_areas[MemoryAreaName::IORegisters];
        // First 3 bits hold the flags
        let bank_reg = io_reg_mem.read(WORK_RAM_BANK_REGISTER, 0) & 0x07;
        // Both 0 and 1 mean the first bank
        bank_reg.saturating_sub(1).into()
    }

    fn get_vram_bank(&self) -> usize {
        let io_reg_mem = &self.memory_areas[MemoryAreaName::IORegisters];
        (io_reg_mem.read(VRAM_BANK_REGISTER, 0) & 0x01).into()
    }

    pub fn read(&self, addr: u16) -> u8 {
        let (area, bank) = self.map_memory(addr);
        self.memory_areas[area].read(addr, bank)
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        let (area, bank) = self.map_memory(addr);
        self.memory_areas[area].write(addr, bank, val);
    }
}
