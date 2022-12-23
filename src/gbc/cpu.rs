mod instruction;
mod instruction_impl;
mod register;

use bitmaps::Bitmap;

use self::instruction::map_instruction;
use self::register::{RegisterMap, RegisterMapMethods, Register};
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
    fn fetch(&mut self, mem: &VirtualMemory) -> u8 {
        let data = mem.read(self.pc);
        self.pc += 1;
        data
    }

    // Fetch next 16 bits (little endian) at program counter. Return as big endian
    fn fetch_16(&mut self, mem: &VirtualMemory) -> u16 {
        self.fetch(mem) as u16 | ((self.fetch(mem) as u16) << 8)
    }

    fn set_flags(&mut self, cy: bool, h: bool, n: bool, z: bool) {
        let mut flags = Bitmap::<8>::new();
        flags.set(7, cy);
        flags.set(6, h);
        flags.set(5, n);
        flags.set(4, z);
        self.registers.write(Register::F, flags.into_value());
    }

    pub fn execute(&mut self, mem: &mut VirtualMemory) {
        let instruction = self.fetch(mem);
        let instruction_impl = map_instruction(instruction);
        instruction_impl(self, mem);
    }
}
