mod op_code;
mod register;

use self::op_code::{parse_opcode, OPDest, OPDest16, OPSrc, OPSrc16, Operation, OperationType};
use self::register::{Register, RegisterPair, RegisterPairData};
use super::memory::{VirtualMemory, PROGRAM_START_ADDR};

use enum_map::{enum_map, EnumMap};

pub struct CPU {
    registers: EnumMap<RegisterPair, RegisterPairData>,
    pc: u16,
    sp: u16,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: enum_map! {
                RegisterPair::AF => RegisterPairData {high: 0x00, low: 0x00},
                RegisterPair::BC => RegisterPairData {high: 0x00, low: 0x00},
                RegisterPair::DE => RegisterPairData {high: 0x00, low: 0x00},
                RegisterPair::HL => RegisterPairData {high: 0x00, low: 0x00},
            },
            // Start of user program
            pc: PROGRAM_START_ADDR,
            // End of stack RAM (stack starts at end)
            sp: 0xFFFE,
        }
    }

    fn read_register(&self, register: Register) -> u8 {
        *RegisterPair::get_individual_register(&self.registers, register)
    }

    fn write_register(&mut self, register: Register, val: u8) {
        *RegisterPair::get_individual_register_mut(&mut self.registers, register) = val;
    }

    fn read_register_pair(&self, pair: RegisterPair) -> u16 {
        self.registers[pair].read()
    }

    // Write Big Endian value to register pair
    fn write_register_pair(&mut self, pair: RegisterPair, val: u16) {
        self.registers[pair].write(val);
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
        let opcode = self.fetch(mem);
        let Operation { op, cycles } = parse_opcode(opcode);
        match op {
            // Load source into destination
            OperationType::LD(dest, src) => self.ld(mem, dest, src),
            // Load source into destination and increment the register pair used for src
            OperationType::LDAndIncrementSrc(dest, src) => {
                if let OPSrc::RegisterPairAsPointer(reg_pair) = src {
                    self.ld(mem, dest, src);
                    let new_hl = u16::wrapping_add(self.read_register_pair(reg_pair), 1);
                    self.write_register_pair(reg_pair, new_hl);
                } else {
                    panic!("Operation only supported for register pair as pointer!")
                }
            }
            // Load source into destination and decrement the register pair used for src
            OperationType::LDAndDecrementSrc(dest, src) => {
                if let OPSrc::RegisterPairAsPointer(reg_pair) = src {
                    self.ld(mem, dest, src);
                    let new_hl = u16::wrapping_sub(self.read_register_pair(reg_pair), 1);
                    self.write_register_pair(reg_pair, new_hl);
                } else {
                    panic!("Operation only supported for register pair as pointer!")
                }
            }
            // Load source into destination and increment the register pair used for dest
            OperationType::LDAndIncrementDest(dest, src) => {
                if let OPDest::RegisterPairAsPointer(reg_pair) = dest {
                    self.ld(mem, dest, src);
                    let new_hl = u16::wrapping_add(self.read_register_pair(reg_pair), 1);
                    self.write_register_pair(reg_pair, new_hl);
                } else {
                    panic!("Operation only supported for register pair as pointer!")
                }
            }
            // Load source into destination and decrement the register pair used for dest
            OperationType::LDAndDecrementDest(dest, src) => {
                if let OPDest::RegisterPairAsPointer(reg_pair) = dest {
                    self.ld(mem, dest, src);
                    let new_hl = u16::wrapping_sub(self.read_register_pair(reg_pair), 1);
                    self.write_register_pair(reg_pair, new_hl);
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
            OPSrc::Register(reg) => self.read_register(reg),
            OPSrc::RegisterAsPointer(reg) => {
                let addr = 0xFF00 | self.read_register(reg) as u16;
                mem.read(addr)
            }
            OPSrc::RegisterPairAsPointer(reg_pair) => {
                let addr = self.read_register_pair(reg_pair);
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

    fn write_op_dest(&mut self, mem: &mut VirtualMemory, dest: OPDest, val: u8) {
        match dest {
            OPDest::Register(reg) => self.write_register(reg, val),
            OPDest::RegisterAsPointer(reg) => {
                let addr = 0xFF00 | self.read_register(reg) as u16;
                mem.write(addr, val);
            }
            OPDest::RegisterPairAsPointer(reg_pair) => {
                let addr = self.read_register_pair(reg_pair);
                mem.write(addr, val);
            }
            OPDest::PCImmediateAsPointer => {
                let addr = 0xFF00 | self.fetch(mem) as u16;
                mem.write(addr, val);
            }
            OPDest::PCImmediateAsPointer16 => {
                let addr = self.fetch_16(mem);
                mem.write(addr, val);
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
            OPSrc16::RegisterPair(reg_pair) => self.read_register_pair(reg_pair),
            OPSrc16::PCImmediate16 => self.fetch_16(mem),
        }
    }

    fn write_op_dest_16(&mut self, dest: OPDest16, value: u16) {
        match dest {
            OPDest16::RegisterPair(reg_pair) => self.write_register_pair(reg_pair, value),
            OPDest16::StackPointerRegister => self.sp = value,
        }
    }
}
