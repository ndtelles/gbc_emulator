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
    fn fetch(&mut self, mem: &VirtualMemory) -> u8 {
        let data = mem.read(self.pc);
        self.pc += 1;
        data
    }

    // Fetch next 16 bits (little endian) at program counter. Return as big endian
    fn fetch_16(&mut self, mem: &VirtualMemory) -> u16 {
        self.fetch(mem) as u16 | ((self.fetch(mem) as u16) << 8)
    }

    pub fn execute(&mut self, mem: &mut VirtualMemory) {
        let instruction = self.fetch(mem);
        let instruction_impl = map_instruction(instruction);
        instruction_impl(self, mem);
        // match op {
        //     // Load source into destination
        //     OperationType::LD(dest, src) => self.ld(mem, dest, src),
        //     // Load source into destination and increment the register pair used for src
        //     OperationType::LDAndIncrementSrc(dest, src) => {
        //         if let OPSrc::RegisterPairAsPointer(reg_pair) = src {
        //             self.ld(mem, dest, src);
        //             let new_hl = self.read_register_pair(reg_pair).wrapping_add(1);
        //             self.write_register_pair(reg_pair, new_hl);
        //         } else {
        //             panic!("Operation only supported for register pair as pointer!")
        //         }
        //     }
        //     // Load source into destination and decrement the register pair used for src
        //     OperationType::LDAndDecrementSrc(dest, src) => {
        //         if let OPSrc::RegisterPairAsPointer(reg_pair) = src {
        //             self.ld(mem, dest, src);
        //             let new_hl = self.read_register_pair(reg_pair).wrapping_sub(1);
        //             self.write_register_pair(reg_pair, new_hl);
        //         } else {
        //             panic!("Operation only supported for register pair as pointer!")
        //         }
        //     }
        //     // Load source into destination and increment the register pair used for dest
        //     OperationType::LDAndIncrementDest(dest, src) => {
        //         if let OPDest::RegisterPairAsPointer(reg_pair) = dest {
        //             self.ld(mem, dest, src);
        //             let new_hl = self.read_register_pair(reg_pair).wrapping_add(1);
        //             self.write_register_pair(reg_pair, new_hl);
        //         } else {
        //             panic!("Operation only supported for register pair as pointer!")
        //         }
        //     }
        //     // Load source into destination and decrement the register pair used for dest
        //     OperationType::LDAndDecrementDest(dest, src) => {
        //         if let OPDest::RegisterPairAsPointer(reg_pair) = dest {
        //             self.ld(mem, dest, src);
        //             let new_hl = self.read_register_pair(reg_pair).wrapping_add(1);
        //             self.write_register_pair(reg_pair, new_hl);
        //         } else {
        //             panic!("Operation only supported for register pair as pointer!")
        //         }
        //     }
        //     // Load 16 bit source into destination
        //     OperationType::LD16(dest, src) => self.ld_16(mem, dest, src),
        //     // Push src onto the stack
        //     OperationType::PUSH(src) => {
        //         let val = self.read_op_src_16(mem, src);
        //         self.sp -= 1;
        //         mem.write(self.sp, (val >> 8) as u8);
        //         self.sp -= 1;
        //         mem.write(self.sp, val as u8);
        //     }
        //     // Pop the stack
        //     OperationType::POP(dest) => {
        //         let val_low = mem.read(self.sp);
        //         self.sp += 1;
        //         let val_high = mem.read(self.sp);
        //         self.sp += 1;
        //         let val = (val_high as u16) << 8 | val_low as u16;
        //         self.write_op_dest_16(dest, val);
        //     }
        //     // Add signed operand to Stack Pointer and store value in HL
        //     OperationType::LDHL => {
        //         // Be careful of data types and sign extensions in this operation!
        //         let operand = self.fetch(mem) as i8;
        //         let result = if (operand) >= 0 {
        //             // Operand is positive so we can directly add it as u16
        //             self.sp.wrapping_add(operand as u16)
        //         } else {
        //             // Sign extend operand to i16 then multiply by -1 to make it positive.
        //             // Finally convert positive value to u16.
        //             // We need to cast to i16 before multiply so value 128 doesn't overflow in i8
        //             let pos_operand = ((operand as i16) * -1) as u16;
        //             self.sp.wrapping_sub(pos_operand)
        //         };
        //         self.write_register_pair(RegisterPair::HL, result);
        //     }
        // };
    }
}
