mod op_code;
mod register;

use crate::gbc::cpu::op_code::{parse_opcode, Operation};
use crate::gbc::cpu::register::Register;
use crate::gbc::memory::{VirtualMemory, PROGRAM_START_ADDR};
use enum_map::{enum_map, EnumMap};

pub struct CPU {
    registers: EnumMap<Register, u8>,
    pc: u16,
    sp: u16,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            // Efficient way of mapping registers on the stack
            registers: enum_map! {
                Register::A => 0,
                Register::F => 0,
                Register::B => 0,
                Register::C => 0,
                Register::D => 0,
                Register::E => 0,
                Register::H => 0,
                Register::L => 0
            },
            // Start of user program
            pc: PROGRAM_START_ADDR,
            // End of stack RAM (stack starts at end)
            sp: 0xFFFE,
        }
    }

    fn read_register_pair(&self, high: Register, low: Register) -> u16 {
        ((self.registers[high] as u16) << 8) | self.registers[low] as u16
    }

    // Write Big Endian value to register pair
    fn write_register_pair(&mut self, high: Register, low: Register, val: u16) {
        self.registers[high] = (val >> 8) as u8;
        self.registers[low] = val as u8;
    }

    // Fetch next 8 bits at program counter
    fn fetch(&mut self, mem: &VirtualMemory) -> u8 {
        let data = mem.read(self.pc);
        self.pc += 1;
        data
    }

    // Fetch next 16 bits (little endian) at program counter. Return as big endian
    fn fetch_16(&mut self, mem: &VirtualMemory) -> u16 {
        self.fetch(mem) as u16 | (self.fetch(mem) as u16) << 8
    }

    pub fn execute(&mut self, mem: &mut VirtualMemory) {
        let opcode = self.fetch(mem);

        match parse_opcode(opcode) {
            /*
             * 8-bit transfer instructions
             */
            // Load contents of one register to another
            Operation::LdRegToReg { dest, src } => {
                self.registers[dest] = self.registers[src];
            }
            // Load data at program counter to register
            Operation::LdImmediateDataToReg { dest } => {
                self.registers[dest] = self.fetch(mem);
            }
            // Load data at address denoted by register pairing to register
            Operation::LdRegPairAddrToReg { dest, src } => {
                let addr = self.read_register_pair(src.0, src.1);
                self.registers[dest] = mem.read(addr);
            }
            // Load register contents to address denoted by register pairing
            Operation::LdRegToRegPairAddr { dest, src } => {
                let addr = self.read_register_pair(dest.0, dest.1);
                mem.write(addr, self.registers[src]);
            }
            // Load data at program counter to address denoted by H & L register pairing
            Operation::LdImmediateDataToHLAddr => {
                let addr = self.read_register_pair(Register::H, Register::L);
                mem.write(addr, self.fetch(mem))
            }
            // Load data at address denoted by register C to register A
            Operation::LdRegCAddrToRegA => {
                let addr = 0xFF00 | self.registers[Register::C] as u16;
                self.registers[Register::A] = mem.read(addr);
            }
            // Load contents of register A to address denoted by register C
            Operation::LdRegAToRegCAddr => {
                let addr = 0xFF00 | self.registers[Register::C] as u16;
                mem.write(addr, self.registers[Register::A]);
            }
            // Load data at address denoted at program counter to register A
            Operation::LdImmediateAddrToRegA => {
                let addr = 0xFF00 | self.fetch(mem) as u16;
                self.registers[Register::A] = mem.read(addr);
            }
            // Load register A contents to address denoted at program counter
            Operation::LdRegAToImmediateAddr => {
                let addr = 0xFF00 | self.fetch(mem) as u16;
                mem.write(addr, self.registers[Register::A]);
            }
            // Load data at address denoted by 16bits at program counter to register A
            Operation::LdImmediate16BitAddrToRegA => {
                let addr = self.fetch_16(mem);
                self.registers[Register::A] = mem.read(addr);
            }
            // Load register A contents to address denoted by 16bits at program counter
            Operation::LdRegAToImmediate16BitAddr => {
                let addr = self.fetch_16(mem);
                mem.write(addr, self.registers[Register::A]);
            }
            // Load data at address denoted by H & L register pair to register A and increment HL
            Operation::LdHLAddrToRegAAndIncrement => {
                let addr = self.read_register_pair(Register::H, Register::L);
                self.registers[Register::A] = mem.read(addr);
                let new_hl = u16::wrapping_add(addr, 1);
                self.write_register_pair(Register::H, Register::L, new_hl);
            }
            // Load data at address denoted by H & L register pair to register A and decrement HL
            Operation::LdHLAddrToRegAAndDecrement => {
                let addr = self.read_register_pair(Register::H, Register::L);
                self.registers[Register::A] = mem.read(addr);
                let new_hl = u16::wrapping_sub(addr, 1);
                self.write_register_pair(Register::H, Register::L, new_hl);
            }
            // Load register A contents to address denoted by H & L register pair and increment HL
            Operation::LdRegAToHLAddrAndIncrement => {
                let addr = self.read_register_pair(Register::H, Register::L);
                mem.write(addr, self.registers[Register::A]);
                let new_hl = u16::wrapping_add(addr, 1);
                self.write_register_pair(Register::H, Register::L, new_hl);
            }
            // Load register A contents to address denoted by H & L register pair and decrement HL
            Operation::LdRegAToHLAddrAndDecrement => {
                let addr = self.read_register_pair(Register::H, Register::L);
                mem.write(addr, self.registers[Register::A]);
                let new_hl = u16::wrapping_sub(addr, 1);
                self.write_register_pair(Register::H, Register::L, new_hl);
            }

            /*
             * 16-bit transfer instructions
             */
            // Load 16bits at program counter to register
            Operation::LdImmediate16BitDataToRegPair { dest } => {
                let val = self.fetch_16(mem);
                self.write_register_pair(dest.0, dest.1, val);
            }
            // Load 16bits at program counter to Stack Pointer
            Operation::LdImmediate16BitDataToSP => {
                self.sp = self.fetch_16(mem);
            }
            // Load H & L register pair contents to Stack Pointer
            Operation::LdRegHLToSP => {
                self.sp = self.read_register_pair(Register::H, Register::L);
            }
            // Push register pair contents to stack
            Operation::PushRegPairToStack { src } => {
                let val = self.read_register_pair(src.0, src.1);
                let val_high = (val >> 8) as u8;
                let val_low = val as u8;
                self.sp -= 1;
                mem.write(self.sp, val_high);
                self.sp -= 1;
                mem.write(self.sp, val_low);
            }
            // Pop stack to register pair
            Operation::PopStackToRegPair { dest } => {
                let val_low = mem.read(self.sp);
                self.sp += 1;
                let val_high = mem.read(self.sp);
                self.sp += 1;
                self.registers[dest.0] = val_high;
                self.registers[dest.1] = val_low;
            }
        };
    }
}
