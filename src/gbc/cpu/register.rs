use enum_map::{Enum, EnumMap};

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

pub struct RegisterPairData {
    pub high: u8,
    pub low: u8,
}

impl RegisterPairData {
    pub fn read(&self) -> u16 {
        ((self.high as u16) << 8) | self.low as u16
    }

    pub fn write(&mut self, val: u16) {
        self.high = (val >> 8) as u8;
        self.low = val as u8;
    }
}

#[derive(Clone, Copy, Enum)]
pub enum RegisterPair {
    AF,
    BC,
    DE,
    HL,
}

impl RegisterPair {
    // Map individual register to register pair value
    pub fn get_individual_register(
        reg_map: &EnumMap<RegisterPair, RegisterPairData>,
        register: Register,
    ) -> &u8 {
        match register {
            Register::A => &reg_map[RegisterPair::AF].high,
            Register::F => &reg_map[RegisterPair::AF].low,
            Register::B => &reg_map[RegisterPair::BC].high,
            Register::C => &reg_map[RegisterPair::BC].low,
            Register::D => &reg_map[RegisterPair::DE].high,
            Register::E => &reg_map[RegisterPair::DE].low,
            Register::H => &reg_map[RegisterPair::HL].high,
            Register::L => &reg_map[RegisterPair::HL].low,
        }
    }

    pub fn get_individual_register_mut(
        reg_map: &mut EnumMap<RegisterPair, RegisterPairData>,
        register: Register,
    ) -> &mut u8 {
        match register {
            Register::A => &mut reg_map[RegisterPair::AF].high,
            Register::F => &mut reg_map[RegisterPair::AF].low,
            Register::B => &mut reg_map[RegisterPair::BC].high,
            Register::C => &mut reg_map[RegisterPair::BC].low,
            Register::D => &mut reg_map[RegisterPair::DE].high,
            Register::E => &mut reg_map[RegisterPair::DE].low,
            Register::H => &mut reg_map[RegisterPair::HL].high,
            Register::L => &mut reg_map[RegisterPair::HL].low,
        }
    }
}
