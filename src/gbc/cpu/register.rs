use enum_map::{enum_map, Enum, EnumMap};

use crate::util::{index_bits, Bytes, combine_high_low};

pub type RegisterMap = EnumMap<Register, u8>;

pub trait RegisterMapMethods {
    fn new() -> Self;
    fn read(&self, register: Register) -> u8;
    fn write(&mut self, register: Register, val: u8);
    fn read_pair(&self, pair: RegisterPair) -> u16;
    fn write_pair(&mut self, pair: RegisterPair, val: u16);
    fn get_flags(&self) -> FlagRegister;
    fn set_flags(&mut self, flags: &FlagRegister);
}

impl RegisterMapMethods for RegisterMap {
    fn new() -> Self {
        enum_map! {
            // Initial value of 0x11 indicates this is CGB hardware
            Register::A => 0x11,
            Register::F => 0x00,
            Register::B => 0x00,
            Register::C => 0x00,
            Register::D => 0x00,
            Register::E => 0x00,
            Register::H => 0x00,
            Register::L => 0x00
        }
    }

    fn read(&self, register: Register) -> u8 {
        self[register]
    }

    fn write(&mut self, register: Register, val: u8) {
        self[register] = val;
    }

    fn read_pair(&self, pair: RegisterPair) -> u16 {
        let (high, low) = map_register_pair_to_register(pair);
        combine_high_low(self.read(high), self.read(low))
    }

    fn write_pair(&mut self, pair: RegisterPair, val: u16) {
        let (high, low) = map_register_pair_to_register(pair);
        self.write(high, val.high());
        self.write(low, val.low());
    }

    fn get_flags(&self) -> FlagRegister {
        FlagRegister::from(self.read(Register::F))
    }

    fn set_flags(&mut self, flags: &FlagRegister) {
        self.write(Register::F, flags.into());
    }
}

fn map_register_pair_to_register(pair: RegisterPair) -> (Register, Register) {
    match pair {
        RegisterPair::AF => (Register::A, Register::F),
        RegisterPair::BC => (Register::B, Register::C),
        RegisterPair::DE => (Register::D, Register::E),
        RegisterPair::HL => (Register::H, Register::L),
    }
}

#[derive(Clone, Copy, Enum)]
pub enum Register {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Clone, Copy)]
pub enum RegisterPair {
    AF,
    BC,
    DE,
    HL,
}

pub struct FlagRegister {
    pub z: bool,  // Set to 1 when the result of an operation is 0; otherwise reset
    pub n: bool, // Set to 1 following execution of the subtraction instruction, regardless of the result
    pub h: bool, // Set to 1 when an operation results in caryying from or borrowing to bit 3
    pub cy: bool, // Set to 1 when an operation results in carrying from or borrowing to bit 7
}

impl From<u8> for FlagRegister {
    fn from(val: u8) -> Self {
        Self {
            z: index_bits(val, 7),
            n: index_bits(val, 6),
            h: index_bits(val, 5),
            cy: index_bits(val, 4),
        }
    }
}

impl Into<u8> for &FlagRegister {
    fn into(self) -> u8 {
        let mut flags = 0x00;
        flags |= (self.z as u8) << 7;
        flags |= (self.n as u8) << 6;
        flags |= (self.h as u8) << 5;
        flags |= (self.cy as u8) << 4;
        flags
    }
}
