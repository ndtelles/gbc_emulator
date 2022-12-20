use crate::gbc::cpu::register::Register;

// Sources for reading 8 bit data
pub enum OPSrc {
    Register(Register),
    RegisterAsPointer(Register),
    RegisterPairAsPointer(Register, Register),
    PCImmediate,
    PCImmediateAsPointer,
    PCImmediateAsPointer16,
}

// Destinations for writing 8 bit data
pub enum OPDest {
    Register(Register),
    RegisterAsPointer(Register),
    RegisterPairAsPointer(Register, Register),
    PCImmediateAsPointer,
    PCImmediateAsPointer16,
}

// Sources for reading 16 bit data
pub enum OPSrc16 {
    RegisterPair(Register, Register),
    PCImmediate16,
}

// Destinations for writing 16 bit data
pub enum OPDest16 {
    RegisterPair(Register, Register),
    StackPointerRegister,
}

pub enum OperationType {
    LD(OPDest, OPSrc),
    LDAndIncrementSrc(OPDest, OPSrc),
    LDAndDecrementSrc(OPDest, OPSrc),
    LDAndIncrementDest(OPDest, OPSrc),
    LDAndDecrementDest(OPDest, OPSrc),
    LD16(OPDest16, OPSrc16),
    PUSH(OPSrc16),
    POP(OPDest16),
}

pub struct Operation {
    pub op: OperationType,
    pub cycles: u16,
}

pub fn parse_opcode(opcode: u8) -> Operation {
    match opcode {
        0x40 => Operation {
            op: OperationType::LD(OPDest::Register(Register::B), OPSrc::Register(Register::B)),
            cycles: 1,
        },
        0x41 => Operation {
            op: OperationType::LD(OPDest::Register(Register::B), OPSrc::Register(Register::C)),
            cycles: 1,
        },
        0x42 => Operation {
            op: OperationType::LD(OPDest::Register(Register::B), OPSrc::Register(Register::D)),
            cycles: 1,
        },
        0x43 => Operation {
            op: OperationType::LD(OPDest::Register(Register::B), OPSrc::Register(Register::E)),
            cycles: 1,
        },
        0x44 => Operation {
            op: OperationType::LD(OPDest::Register(Register::B), OPSrc::Register(Register::H)),
            cycles: 1,
        },
        0x45 => Operation {
            op: OperationType::LD(OPDest::Register(Register::B), OPSrc::Register(Register::L)),
            cycles: 1,
        },
        0x47 => Operation {
            op: OperationType::LD(OPDest::Register(Register::B), OPSrc::Register(Register::A)),
            cycles: 1,
        },
        0x48 => Operation {
            op: OperationType::LD(OPDest::Register(Register::C), OPSrc::Register(Register::B)),
            cycles: 1,
        },
        0x49 => Operation {
            op: OperationType::LD(OPDest::Register(Register::C), OPSrc::Register(Register::C)),
            cycles: 1,
        },
        0x4A => Operation {
            op: OperationType::LD(OPDest::Register(Register::C), OPSrc::Register(Register::D)),
            cycles: 1,
        },
        0x4B => Operation {
            op: OperationType::LD(OPDest::Register(Register::C), OPSrc::Register(Register::E)),
            cycles: 1,
        },
        0x4C => Operation {
            op: OperationType::LD(OPDest::Register(Register::C), OPSrc::Register(Register::H)),
            cycles: 1,
        },
        0x4D => Operation {
            op: OperationType::LD(OPDest::Register(Register::C), OPSrc::Register(Register::L)),
            cycles: 1,
        },
        0x4F => Operation {
            op: OperationType::LD(OPDest::Register(Register::C), OPSrc::Register(Register::A)),
            cycles: 1,
        },
        0x50 => Operation {
            op: OperationType::LD(OPDest::Register(Register::D), OPSrc::Register(Register::B)),
            cycles: 1,
        },
        0x51 => Operation {
            op: OperationType::LD(OPDest::Register(Register::D), OPSrc::Register(Register::C)),
            cycles: 1,
        },
        0x52 => Operation {
            op: OperationType::LD(OPDest::Register(Register::D), OPSrc::Register(Register::D)),
            cycles: 1,
        },
        0x53 => Operation {
            op: OperationType::LD(OPDest::Register(Register::D), OPSrc::Register(Register::E)),
            cycles: 1,
        },
        0x54 => Operation {
            op: OperationType::LD(OPDest::Register(Register::D), OPSrc::Register(Register::H)),
            cycles: 1,
        },
        0x55 => Operation {
            op: OperationType::LD(OPDest::Register(Register::D), OPSrc::Register(Register::L)),
            cycles: 1,
        },
        0x57 => Operation {
            op: OperationType::LD(OPDest::Register(Register::D), OPSrc::Register(Register::A)),
            cycles: 1,
        },
        0x58 => Operation {
            op: OperationType::LD(OPDest::Register(Register::E), OPSrc::Register(Register::B)),
            cycles: 1,
        },
        0x59 => Operation {
            op: OperationType::LD(OPDest::Register(Register::E), OPSrc::Register(Register::C)),
            cycles: 1,
        },
        0x5A => Operation {
            op: OperationType::LD(OPDest::Register(Register::E), OPSrc::Register(Register::D)),
            cycles: 1,
        },
        0x5B => Operation {
            op: OperationType::LD(OPDest::Register(Register::E), OPSrc::Register(Register::E)),
            cycles: 1,
        },
        0x5C => Operation {
            op: OperationType::LD(OPDest::Register(Register::E), OPSrc::Register(Register::H)),
            cycles: 1,
        },
        0x5D => Operation {
            op: OperationType::LD(OPDest::Register(Register::E), OPSrc::Register(Register::L)),
            cycles: 1,
        },
        0x5F => Operation {
            op: OperationType::LD(OPDest::Register(Register::E), OPSrc::Register(Register::A)),
            cycles: 1,
        },
        0x60 => Operation {
            op: OperationType::LD(OPDest::Register(Register::H), OPSrc::Register(Register::B)),
            cycles: 1,
        },
        0x61 => Operation {
            op: OperationType::LD(OPDest::Register(Register::H), OPSrc::Register(Register::C)),
            cycles: 1,
        },
        0x62 => Operation {
            op: OperationType::LD(OPDest::Register(Register::H), OPSrc::Register(Register::D)),
            cycles: 1,
        },
        0x63 => Operation {
            op: OperationType::LD(OPDest::Register(Register::H), OPSrc::Register(Register::E)),
            cycles: 1,
        },
        0x64 => Operation {
            op: OperationType::LD(OPDest::Register(Register::H), OPSrc::Register(Register::H)),
            cycles: 1,
        },
        0x65 => Operation {
            op: OperationType::LD(OPDest::Register(Register::H), OPSrc::Register(Register::L)),
            cycles: 1,
        },
        0x67 => Operation {
            op: OperationType::LD(OPDest::Register(Register::H), OPSrc::Register(Register::A)),
            cycles: 1,
        },
        0x68 => Operation {
            op: OperationType::LD(OPDest::Register(Register::L), OPSrc::Register(Register::B)),
            cycles: 1,
        },
        0x69 => Operation {
            op: OperationType::LD(OPDest::Register(Register::L), OPSrc::Register(Register::C)),
            cycles: 1,
        },
        0x6A => Operation {
            op: OperationType::LD(OPDest::Register(Register::L), OPSrc::Register(Register::D)),
            cycles: 1,
        },
        0x6B => Operation {
            op: OperationType::LD(OPDest::Register(Register::L), OPSrc::Register(Register::E)),
            cycles: 1,
        },
        0x6C => Operation {
            op: OperationType::LD(OPDest::Register(Register::L), OPSrc::Register(Register::H)),
            cycles: 1,
        },
        0x6D => Operation {
            op: OperationType::LD(OPDest::Register(Register::L), OPSrc::Register(Register::L)),
            cycles: 1,
        },
        0x6F => Operation {
            op: OperationType::LD(OPDest::Register(Register::L), OPSrc::Register(Register::A)),
            cycles: 1,
        },
        0x78 => Operation {
            op: OperationType::LD(OPDest::Register(Register::A), OPSrc::Register(Register::B)),
            cycles: 1,
        },
        0x79 => Operation {
            op: OperationType::LD(OPDest::Register(Register::A), OPSrc::Register(Register::C)),
            cycles: 1,
        },
        0x7A => Operation {
            op: OperationType::LD(OPDest::Register(Register::A), OPSrc::Register(Register::D)),
            cycles: 1,
        },
        0x7B => Operation {
            op: OperationType::LD(OPDest::Register(Register::A), OPSrc::Register(Register::E)),
            cycles: 1,
        },
        0x7C => Operation {
            op: OperationType::LD(OPDest::Register(Register::A), OPSrc::Register(Register::H)),
            cycles: 1,
        },
        0x7D => Operation {
            op: OperationType::LD(OPDest::Register(Register::A), OPSrc::Register(Register::L)),
            cycles: 1,
        },
        0x7F => Operation {
            op: OperationType::LD(OPDest::Register(Register::A), OPSrc::Register(Register::A)),
            cycles: 1,
        },
        0x06 => Operation {
            op: OperationType::LD(OPDest::Register(Register::B), OPSrc::PCImmediate),
            cycles: 2,
        },
        0x0E => Operation {
            op: OperationType::LD(OPDest::Register(Register::C), OPSrc::PCImmediate),
            cycles: 2,
        },
        0x16 => Operation {
            op: OperationType::LD(OPDest::Register(Register::D), OPSrc::PCImmediate),
            cycles: 2,
        },
        0x1E => Operation {
            op: OperationType::LD(OPDest::Register(Register::E), OPSrc::PCImmediate),
            cycles: 2,
        },
        0x26 => Operation {
            op: OperationType::LD(OPDest::Register(Register::H), OPSrc::PCImmediate),
            cycles: 2,
        },
        0x2E => Operation {
            op: OperationType::LD(OPDest::Register(Register::L), OPSrc::PCImmediate),
            cycles: 2,
        },
        0x3E => Operation {
            op: OperationType::LD(OPDest::Register(Register::A), OPSrc::PCImmediate),
            cycles: 2,
        },
        0x46 => Operation {
            op: OperationType::LD(
                OPDest::Register(Register::B),
                OPSrc::RegisterPairAsPointer(Register::H, Register::L),
            ),
            cycles: 2,
        },
        0x4E => Operation {
            op: OperationType::LD(
                OPDest::Register(Register::C),
                OPSrc::RegisterPairAsPointer(Register::H, Register::L),
            ),
            cycles: 2,
        },
        0x56 => Operation {
            op: OperationType::LD(
                OPDest::Register(Register::D),
                OPSrc::RegisterPairAsPointer(Register::H, Register::L),
            ),
            cycles: 2,
        },
        0x5E => Operation {
            op: OperationType::LD(
                OPDest::Register(Register::E),
                OPSrc::RegisterPairAsPointer(Register::H, Register::L),
            ),
            cycles: 2,
        },
        0x66 => Operation {
            op: OperationType::LD(
                OPDest::Register(Register::H),
                OPSrc::RegisterPairAsPointer(Register::H, Register::L),
            ),
            cycles: 2,
        },
        0x6E => Operation {
            op: OperationType::LD(
                OPDest::Register(Register::L),
                OPSrc::RegisterPairAsPointer(Register::H, Register::L),
            ),
            cycles: 2,
        },
        0x7E => Operation {
            op: OperationType::LD(
                OPDest::Register(Register::A),
                OPSrc::RegisterPairAsPointer(Register::H, Register::L),
            ),
            cycles: 2,
        },
        0x0A => Operation {
            op: OperationType::LD(
                OPDest::Register(Register::A),
                OPSrc::RegisterPairAsPointer(Register::B, Register::C),
            ),
            cycles: 2,
        },
        0x1A => Operation {
            op: OperationType::LD(
                OPDest::Register(Register::A),
                OPSrc::RegisterPairAsPointer(Register::D, Register::E),
            ),
            cycles: 2,
        },
        0x70 => Operation {
            op: OperationType::LD(
                OPDest::RegisterPairAsPointer(Register::H, Register::L),
                OPSrc::Register(Register::B),
            ),
            cycles: 2,
        },
        0x71 => Operation {
            op: OperationType::LD(
                OPDest::RegisterPairAsPointer(Register::H, Register::L),
                OPSrc::Register(Register::C),
            ),
            cycles: 2,
        },
        0x72 => Operation {
            op: OperationType::LD(
                OPDest::RegisterPairAsPointer(Register::H, Register::L),
                OPSrc::Register(Register::D),
            ),
            cycles: 2,
        },
        0x73 => Operation {
            op: OperationType::LD(
                OPDest::RegisterPairAsPointer(Register::H, Register::L),
                OPSrc::Register(Register::E),
            ),
            cycles: 2,
        },
        0x74 => Operation {
            op: OperationType::LD(
                OPDest::RegisterPairAsPointer(Register::H, Register::L),
                OPSrc::Register(Register::H),
            ),
            cycles: 2,
        },
        0x75 => Operation {
            op: OperationType::LD(
                OPDest::RegisterPairAsPointer(Register::H, Register::L),
                OPSrc::Register(Register::L),
            ),
            cycles: 2,
        },
        0x77 => Operation {
            op: OperationType::LD(
                OPDest::RegisterPairAsPointer(Register::H, Register::L),
                OPSrc::Register(Register::A),
            ),
            cycles: 2,
        },
        0x02 => Operation {
            op: OperationType::LD(
                OPDest::RegisterPairAsPointer(Register::B, Register::C),
                OPSrc::Register(Register::A),
            ),
            cycles: 2,
        },
        0x12 => Operation {
            op: OperationType::LD(
                OPDest::RegisterPairAsPointer(Register::D, Register::E),
                OPSrc::Register(Register::A),
            ),
            cycles: 2,
        },
        0x36 => Operation {
            op: OperationType::LD(
                OPDest::RegisterPairAsPointer(Register::H, Register::L),
                OPSrc::PCImmediate,
            ),
            cycles: 3,
        },
        0xF2 => Operation {
            op: OperationType::LD(
                OPDest::Register(Register::A),
                OPSrc::RegisterAsPointer(Register::C),
            ),
            cycles: 2,
        },
        0xE2 => Operation {
            op: OperationType::LD(
                OPDest::RegisterAsPointer(Register::C),
                OPSrc::Register(Register::A),
            ),
            cycles: 2,
        },
        0xF0 => Operation {
            op: OperationType::LD(OPDest::Register(Register::A), OPSrc::PCImmediateAsPointer),
            cycles: 3,
        },
        0xE0 => Operation {
            op: OperationType::LD(OPDest::PCImmediateAsPointer, OPSrc::Register(Register::A)),
            cycles: 3,
        },
        0xFA => Operation {
            op: OperationType::LD(OPDest::Register(Register::A), OPSrc::PCImmediateAsPointer16),
            cycles: 4,
        },
        0xEA => Operation {
            op: OperationType::LD(OPDest::PCImmediateAsPointer16, OPSrc::Register(Register::A)),
            cycles: 4,
        },
        0x2A => Operation {
            op: OperationType::LDAndIncrementSrc(
                OPDest::Register(Register::A),
                OPSrc::RegisterPairAsPointer(Register::H, Register::L),
            ),
            cycles: 2,
        },
        0x3A => Operation {
            op: OperationType::LDAndDecrementSrc(
                OPDest::Register(Register::A),
                OPSrc::RegisterPairAsPointer(Register::H, Register::L),
            ),
            cycles: 2,
        },
        0x22 => Operation {
            op: OperationType::LDAndIncrementDest(
                OPDest::RegisterPairAsPointer(Register::H, Register::L),
                OPSrc::Register(Register::A),
            ),
            cycles: 2,
        },
        0x32 => Operation {
            op: OperationType::LDAndDecrementDest(
                OPDest::RegisterPairAsPointer(Register::H, Register::L),
                OPSrc::Register(Register::A),
            ),
            cycles: 2,
        },
        0x01 => Operation {
            op: OperationType::LD16(
                OPDest16::RegisterPair(Register::B, Register::C),
                OPSrc16::PCImmediate16,
            ),
            cycles: 3,
        },
        0x11 => Operation {
            op: OperationType::LD16(
                OPDest16::RegisterPair(Register::D, Register::E),
                OPSrc16::PCImmediate16,
            ),
            cycles: 3,
        },
        0x21 => Operation {
            op: OperationType::LD16(
                OPDest16::RegisterPair(Register::H, Register::L),
                OPSrc16::PCImmediate16,
            ),
            cycles: 3,
        },
        0x31 => Operation {
            op: OperationType::LD16(OPDest16::StackPointerRegister, OPSrc16::PCImmediate16),
            cycles: 3,
        },
        0xF9 => Operation {
            op: OperationType::LD16(
                OPDest16::StackPointerRegister,
                OPSrc16::RegisterPair(Register::H, Register::L),
            ),
            cycles: 2,
        },
        0xC5 => Operation {
            op: OperationType::PUSH(OPSrc16::RegisterPair(Register::B, Register::C)),
            cycles: 4,
        },
        0xD5 => Operation {
            op: OperationType::PUSH(OPSrc16::RegisterPair(Register::D, Register::E)),
            cycles: 4,
        },
        0xE5 => Operation {
            op: OperationType::PUSH(OPSrc16::RegisterPair(Register::H, Register::L)),
            cycles: 4,
        },
        0xF5 => Operation {
            op: OperationType::PUSH(OPSrc16::RegisterPair(Register::A, Register::F)),
            cycles: 4,
        },
        0xC1 => Operation {
            op: OperationType::POP(OPDest16::RegisterPair(Register::B, Register::C)),
            cycles: 3,
        },
        0xD1 => Operation {
            op: OperationType::POP(OPDest16::RegisterPair(Register::D, Register::E)),
            cycles: 3,
        },
        0xE1 => Operation {
            op: OperationType::POP(OPDest16::RegisterPair(Register::H, Register::L)),
            cycles: 3,
        },
        0xF1 => Operation {
            op: OperationType::POP(OPDest16::RegisterPair(Register::A, Register::F)),
            cycles: 3,
        },
        _ => panic!(),
    }
}
