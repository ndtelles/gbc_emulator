// Memory areas boundaries in contigous order. Would use ranges but const ranges can't be used in rust for pattern matching :(
const INTR_AND_RST_ADDR: u16 = 0x0000;
const INTR_AND_RST_ADDR_END: u16 = 0x00FF;
const ROM_ADDR: u16 = 0x0100;
const ROM_ADDR_END: u16 =  0x7FFF;
// const CHARACTER_DATA_ADDR: u16 = 0x8000;

pub const PROGRAM_START_ADDR: u16 = 0x0150;

pub struct VirtualMemory {
    intr_and_rst: Vec<u8>,
    rom_data: Vec<u8>,
    // character_data_banks: Vec<u8>,
}

impl VirtualMemory {
    pub fn new() -> Self {
        Self {
            intr_and_rst: vec![0x00; (INTR_AND_RST_ADDR_END - INTR_AND_RST_ADDR).into()],
            rom_data: vec![0x00; (ROM_ADDR_END - ROM_ADDR).into()],
            // character_data_banks: vec![0x00; CHARACTER_DATA_ADDR.len() * 2], // Two character data banks
        }
    }

    // Map virtual memory address to actual memory
    fn map_memory(&self, addr: u16) -> (&Vec<u8>, u16) {
        match addr {
            INTR_AND_RST_ADDR..=INTR_AND_RST_ADDR_END => (&self.intr_and_rst, INTR_AND_RST_ADDR),
            ROM_ADDR..=ROM_ADDR_END => (&self.rom_data, ROM_ADDR),
            _ => panic!()
        }
    }

    fn map_memory_mut(&mut self, addr: u16) -> (&mut Vec<u8>, u16) {
        match addr {
            INTR_AND_RST_ADDR..=INTR_AND_RST_ADDR_END => (&mut self.intr_and_rst, INTR_AND_RST_ADDR),
            ROM_ADDR..=ROM_ADDR_END => (&mut self.rom_data, ROM_ADDR),
            _ => panic!()
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        let (mem, offset) = self.map_memory(addr);
        mem[(addr - offset) as usize]
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        let (mem, offset) = self.map_memory_mut(addr);
        mem[(addr - offset) as usize] = val;
    }

}
