mod op_code;
mod register;

use self::op_code::{parse_opcode, OPDest, OPDest16, OPSrc, OPSrc16, Operation, OperationType};
use self::register::Register;
use super::memory::{VirtualMemory, PROGRAM_START_ADDR};

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
        let Operation { op, cycles } = parse_opcode(opcode);
        match op {
            // Load source into destination
            OperationType::LD(dest, src) => self.ld(mem, dest, src),
            // Load source into destination and increment the register pair used for src
            OperationType::LDAndIncrementSrc(dest, src) => {
                if let OPSrc::RegisterPairAsPointer(high, low) = src {
                    self.ld(mem, dest, src);
                    let new_hl = u16::wrapping_add(self.read_register_pair(high, low), 1);
                    self.write_register_pair(high, low, new_hl);
                } else {
                    panic!("Operation only supported for register pair as pointer!")
                }
            }
            // Load source into destination and decrement the register pair used for src
            OperationType::LDAndDecrementSrc(dest, src) => {
                if let OPSrc::RegisterPairAsPointer(high, low) = src {
                    self.ld(mem, dest, src);
                    let new_hl = u16::wrapping_sub(self.read_register_pair(high, low), 1);
                    self.write_register_pair(high, low, new_hl);
                } else {
                    panic!("Operation only supported for register pair as pointer!")
                }
            }
            // Load source into destination and increment the register pair used for dest
            OperationType::LDAndIncrementDest(dest, src) => {
                if let OPDest::RegisterPairAsPointer(high, low) = dest {
                    self.ld(mem, dest, src);
                    let new_hl = u16::wrapping_add(self.read_register_pair(high, low), 1);
                    self.write_register_pair(high, low, new_hl);
                } else {
                    panic!("Operation only supported for register pair as pointer!")
                }
            }
            // Load source into destination and decrement the register pair used for dest
            OperationType::LDAndDecrementDest(dest, src) => {
                if let OPDest::RegisterPairAsPointer(high, low) = dest {
                    self.ld(mem, dest, src);
                    let new_hl = u16::wrapping_sub(self.read_register_pair(high, low), 1);
                    self.write_register_pair(high, low, new_hl);
                } else {
                    panic!("Operation only supported for register pair as pointer!")
                }
            }
            // Load 16 bit source into destination
            OperationType::LD16(dest, src) => self.ld_16(mem, dest, src),
            // Push src onto the stack
            OperationType::PUSH(src) => {
                let val = self.read_op_src_16(mem, src);
                self.sp -= 1;
                mem.write(self.sp, (val >> 8) as u8);
                self.sp -= 1;
                mem.write(self.sp, val as u8);
            }
            // Pop the stack
            OperationType::POP(dest) => {
                let val_low = mem.read(self.sp);
                self.sp += 1;
                let val_high = mem.read(self.sp);
                self.sp += 1;
                let val = (val_high as u16) << 8 | val_low as u16;
                self.write_op_dest_16(dest, val);
            }
        };
    }

    /*
     * 8-bit transfer instructions
     */
    fn ld(&mut self, mem: &mut VirtualMemory, dest: OPDest, src: OPSrc) {
        let value = self.read_op_src(mem, src);
        self.write_op_dest(mem, dest, value);
    }

    fn read_op_src(&mut self, mem: &VirtualMemory, src: OPSrc) -> u8 {
        match src {
            OPSrc::Register(reg) => self.registers[reg],
            OPSrc::RegisterAsPointer(reg) => {
                let addr = 0xFF00 | self.registers[reg] as u16;
                mem.read(addr)
            }
            OPSrc::RegisterPairAsPointer(high, low) => {
                let addr = self.read_register_pair(high, low);
                mem.read(addr)
            }
            OPSrc::PCImmediate => self.fetch(mem),
            OPSrc::PCImmediateAsPointer => {
                let addr = 0xFF00 | self.fetch(mem) as u16;
                mem.read(addr)
            }
            OPSrc::PCImmediateAsPointer16 => {
                let addr = self.fetch_16(mem);
                mem.read(addr)
            }
        }
    }

    fn write_op_dest(&mut self, mem: &mut VirtualMemory, dest: OPDest, value: u8) {
        match dest {
            OPDest::Register(reg) => self.registers[reg] = value,
            OPDest::RegisterAsPointer(reg) => {
                let addr = 0xFF00 | self.registers[reg] as u16;
                mem.write(addr, value);
            }
            OPDest::RegisterPairAsPointer(high, low) => {
                let addr = self.read_register_pair(high, low);
                mem.write(addr, value);
            }
            OPDest::PCImmediateAsPointer => {
                let addr = 0xFF00 | self.fetch(mem) as u16;
                mem.write(addr, value);
            }
            OPDest::PCImmediateAsPointer16 => {
                let addr = self.fetch_16(mem);
                mem.write(addr, value);
            }
        }
    }

    /*
     * 16-bit transfer instructions
     */
    fn ld_16(&mut self, mem: &VirtualMemory, dest: OPDest16, src: OPSrc16) {
        let value = self.read_op_src_16(mem, src);
        self.write_op_dest_16(dest, value);
    }

    fn read_op_src_16(&mut self, mem: &VirtualMemory, src: OPSrc16) -> u16 {
        match src {
            OPSrc16::RegisterPair(high, low) => self.read_register_pair(high, low),
            OPSrc16::PCImmediate16 => self.fetch_16(mem),
        }
    }

    fn write_op_dest_16(&mut self, dest: OPDest16, value: u16) {
        match dest {
            OPDest16::RegisterPair(high, low) => self.write_register_pair(high, low, value),
            OPDest16::StackPointerRegister => self.sp = value,
        }
    }
}
