// Memory areas boundaries in contigous order. Would use ranges but const ranges can't be used in rust for pattern matching :(
const PRG_ROM_ADDR: u16 = 0x0000;
const PRG_ROM_ADDR_END: u16 = 0x7FFF;

const VRAM_ADDR: u16 = 0x8000;
const VRAM_ADDR_END: u16 = 0x9FFF;

const WORK_RAM_FIXED_ADDR: u16 = 0xC000;
const WORK_RAM_FIXED_ADDR_END: u16 = 0xCFFF;

const WORK_RAM_SWITCHABLE_ADDR: u16 = 0xD000;
const WORK_RAM_SWITCHABLE_ADDR_END: u16 = 0xDFFF;

const REGISTERS_ADDR: u16 = 0xFF00;
const REGISTERS_ADDR_END: u16 = 0xFF7F;

/**
 * Special registers
 */
const WORK_RAM_BANK_REGISTER: u16 = 0xFF70;
const VRAM_BANK_REGISTER: u16 = 0xFF4F;

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
    rom_data: MemoryArea,
    vram_data: MemoryArea,
    work_ram_fixed_data: MemoryArea,
    work_ram_switchable_data: MemoryArea,
    registers_data: MemoryArea,
}

impl VirtualMemory {
    pub fn new() -> Self {
        Self {
            rom_data: MemoryArea::new(
                PRG_ROM_ADDR,
                PRG_ROM_ADDR_END,
                1,
                MemoryPermission::ReadOnly,
            ),
            vram_data: MemoryArea::new(VRAM_ADDR, VRAM_ADDR_END, 2, MemoryPermission::ReadAndWrite),
            work_ram_fixed_data: MemoryArea::new(
                WORK_RAM_FIXED_ADDR,
                WORK_RAM_FIXED_ADDR_END,
                1,
                MemoryPermission::ReadAndWrite,
            ),
            work_ram_switchable_data: MemoryArea::new(
                WORK_RAM_SWITCHABLE_ADDR,
                WORK_RAM_SWITCHABLE_ADDR_END,
                7,
                MemoryPermission::ReadAndWrite,
            ),
            registers_data: MemoryArea::new(
                REGISTERS_ADDR,
                REGISTERS_ADDR_END,
                1,
                MemoryPermission::ReadAndWrite,
            ),
        }
    }

    // Map virtual memory address to actual memory. Returns a reference to the memory area and
    // the currently active bank
    fn map_memory(&self, addr: u16) -> (&MemoryArea, usize) {
        match addr {
            PRG_ROM_ADDR..=PRG_ROM_ADDR_END => (&self.rom_data, 0),
            VRAM_ADDR..=VRAM_ADDR_END => {
                let bank = self.get_vram_bank();
                (&self.vram_data, bank)
            }
            WORK_RAM_FIXED_ADDR..=WORK_RAM_FIXED_ADDR_END => (&self.work_ram_fixed_data, 0),
            WORK_RAM_SWITCHABLE_ADDR..=WORK_RAM_SWITCHABLE_ADDR_END => {
                let bank = self.get_work_ram_bank();
                (&self.work_ram_switchable_data, bank)
            }
            REGISTERS_ADDR..=REGISTERS_ADDR_END => (&self.registers_data, 0),
            _ => panic!(),
        }
    }

    fn map_memory_mut(&mut self, addr: u16) -> (&mut MemoryArea, usize) {
        match addr {
            PRG_ROM_ADDR..=PRG_ROM_ADDR_END => (&mut self.rom_data, 0),
            VRAM_ADDR..=VRAM_ADDR_END => {
                let bank = self.get_vram_bank();
                (&mut self.vram_data, bank)
            }
            WORK_RAM_FIXED_ADDR..=WORK_RAM_FIXED_ADDR_END => (&mut self.work_ram_fixed_data, 0),
            WORK_RAM_SWITCHABLE_ADDR..=WORK_RAM_SWITCHABLE_ADDR_END => {
                let bank = self.get_work_ram_bank();
                (&mut self.work_ram_switchable_data, bank)
            }
            REGISTERS_ADDR..=REGISTERS_ADDR_END => (&mut self.registers_data, 0),
            _ => panic!(),
        }
    }

    fn get_work_ram_bank(&self) -> usize {
        // First 3 bits hold the flags
        let bank_reg = self.read(WORK_RAM_BANK_REGISTER) & 0x07;
        // Both 0 and 1 mean the first bank
        bank_reg.saturating_sub(1).into()
    }

    fn get_vram_bank(&self) -> usize {
        (self.read(VRAM_BANK_REGISTER) & 0x01).into()
    }

    pub fn read(&self, addr: u16) -> u8 {
        let (memory, bank) = self.map_memory(addr);
        // TODO: Bank swtiching
        memory.read(addr, bank)
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        let (memory, bank) = self.map_memory_mut(addr);
        // TODO: Bank swtiching
        memory.write(addr, bank, val);
    }
}
