mod instruction;
mod instruction_impl;
mod register;

use self::instruction::map_instruction;
use self::register::{RegisterMap, RegisterMapMethods};
use super::memory::{VirtualMemory, PROGRAM_START_ADDR};

pub struct CPU {
    registers: RegisterMap,
    pc: u16,
    sp: u16,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: RegisterMap::new(),
            // Start of user program
            pc: PROGRAM_START_ADDR,
            // End of stack RAM (stack starts at end)
            sp: 0xFFFE,
        }
    }

    // Fetch next 8 bits at program counter
    fn fetch_and_incr_pc(&mut self, mem: &VirtualMemory) -> u8 {
        let data = mem.read(self.pc);
        self.pc += 1;
        data
    }

    // Fetch next 16 bits (little endian) at program counter. Return as big endian
    fn fetch_and_incr_pc_16(&mut self, mem: &VirtualMemory) -> u16 {
        self.fetch_and_incr_pc(mem) as u16 | ((self.fetch_and_incr_pc(mem) as u16) << 8)
    }

    pub fn execute(&mut self, mem: &mut VirtualMemory) {
        let instruction = self.fetch_and_incr_pc(mem);
        let instruction_impl = map_instruction(instruction);
        instruction_impl(self, mem);
    }
}
