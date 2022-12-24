use enum_map::{enum_map, Enum, EnumMap};

use crate::util::index_bitmap;

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
            Register::A => 0,
            Register::F => 0,
            Register::B => 0,
            Register::C => 0,
            Register::D => 0,
            Register::E => 0,
            Register::H => 0,
            Register::L => 0
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
        ((self.read(high) as u16) << 8) | self.read(low) as u16
    }

    fn write_pair(&mut self, pair: RegisterPair, val: u16) {
        let (high, low) = map_register_pair_to_register(pair);
        self.write(high, (val >> 8) as u8);
        self.write(low, val as u8);
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
            z: index_bitmap(val, 7),
            n: index_bitmap(val, 6),
            h: index_bitmap(val, 5),
            cy: index_bitmap(val, 4),
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
