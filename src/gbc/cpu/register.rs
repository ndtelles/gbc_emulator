use enum_map::{enum_map, Enum, EnumMap};

pub type RegisterMap = EnumMap<Register, u8>;

pub trait RegisterMapMethods {
    fn new() -> Self;
    fn read(&self, register: Register) -> u8;
    fn write(&mut self, register: Register, val: u8);
    fn read_pair(&self, pair: RegisterPair) -> u16;
    fn write_pair(&mut self, pair: RegisterPair, val: u16);
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
}

fn map_register_pair_to_register(pair: RegisterPair) -> (Register, Register) {
    match pair {
        RegisterPair::AF => (Register::A, Register::F),
        RegisterPair::BC => (Register::B, Register::C),
        RegisterPair::DE => (Register::D, Register::E),
        RegisterPair::HL => (Register::H, Register::L),
    }
}

#[derive(Enum)]
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

pub enum RegisterPair {
    AF,
    BC,
    DE,
    HL,
}
