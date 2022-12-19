use crate::gbc::cpu::register::Register;

pub enum Operation {
    LdRegToReg {
        dest: Register,
        src: Register,
    },
    LdImmediateDataToReg {
        dest: Register,
    },
    LdRegPairAddrToReg {
        dest: Register,
        src: (Register, Register),
    },
    LdRegToRegPairAddr {
        dest: (Register, Register),
        src: Register,
    },
    LdImmediateDataToHLAddr,
    LdRegCAddrToRegA,
    LdRegAToRegCAddr,
    LdImmediateAddrToRegA,
    LdRegAToImmediateAddr,
    LdImmediate16BitAddrToRegA,
    LdRegAToImmediate16BitAddr,
    LdHLAddrToRegAAndIncrement,
    LdHLAddrToRegAAndDecrement,
    LdRegAToHLAddrAndIncrement,
    LdRegAToHLAddrAndDecrement,
    LdImmediate16BitDataToRegPair {
        dest: (Register, Register),
    },
    LdImmediate16BitDataToSP,
    LdRegHLToSP,
    PushRegPairToStack {
        src: (Register, Register),
    },
    PopStackToRegPair {
        dest: (Register, Register),
    },
}

pub fn parse_opcode(opcode: u8) -> Operation {
    match opcode {
        0x40 => Operation::LdRegToReg {
            dest: Register::B,
            src: Register::B,
        },
        0x41 => Operation::LdRegToReg {
            dest: Register::B,
            src: Register::C,
        },
        0x42 => Operation::LdRegToReg {
            dest: Register::B,
            src: Register::D,
        },
        0x43 => Operation::LdRegToReg {
            dest: Register::B,
            src: Register::E,
        },
        0x44 => Operation::LdRegToReg {
            dest: Register::B,
            src: Register::H,
        },
        0x45 => Operation::LdRegToReg {
            dest: Register::B,
            src: Register::L,
        },
        0x47 => Operation::LdRegToReg {
            dest: Register::B,
            src: Register::A,
        },
        0x48 => Operation::LdRegToReg {
            dest: Register::C,
            src: Register::B,
        },
        0x49 => Operation::LdRegToReg {
            dest: Register::C,
            src: Register::C,
        },
        0x4A => Operation::LdRegToReg {
            dest: Register::C,
            src: Register::D,
        },
        0x4B => Operation::LdRegToReg {
            dest: Register::C,
            src: Register::E,
        },
        0x4C => Operation::LdRegToReg {
            dest: Register::C,
            src: Register::H,
        },
        0x4D => Operation::LdRegToReg {
            dest: Register::C,
            src: Register::L,
        },
        0x4F => Operation::LdRegToReg {
            dest: Register::C,
            src: Register::A,
        },
        0x50 => Operation::LdRegToReg {
            dest: Register::D,
            src: Register::B,
        },
        0x51 => Operation::LdRegToReg {
            dest: Register::D,
            src: Register::C,
        },
        0x52 => Operation::LdRegToReg {
            dest: Register::D,
            src: Register::D,
        },
        0x53 => Operation::LdRegToReg {
            dest: Register::D,
            src: Register::E,
        },
        0x54 => Operation::LdRegToReg {
            dest: Register::D,
            src: Register::H,
        },
        0x55 => Operation::LdRegToReg {
            dest: Register::D,
            src: Register::L,
        },
        0x57 => Operation::LdRegToReg {
            dest: Register::D,
            src: Register::A,
        },
        0x58 => Operation::LdRegToReg {
            dest: Register::E,
            src: Register::B,
        },
        0x59 => Operation::LdRegToReg {
            dest: Register::E,
            src: Register::C,
        },
        0x5A => Operation::LdRegToReg {
            dest: Register::E,
            src: Register::D,
        },
        0x5B => Operation::LdRegToReg {
            dest: Register::E,
            src: Register::E,
        },
        0x5C => Operation::LdRegToReg {
            dest: Register::E,
            src: Register::H,
        },
        0x5D => Operation::LdRegToReg {
            dest: Register::E,
            src: Register::L,
        },
        0x5F => Operation::LdRegToReg {
            dest: Register::E,
            src: Register::A,
        },
        0x60 => Operation::LdRegToReg {
            dest: Register::H,
            src: Register::B,
        },
        0x61 => Operation::LdRegToReg {
            dest: Register::H,
            src: Register::C,
        },
        0x62 => Operation::LdRegToReg {
            dest: Register::H,
            src: Register::D,
        },
        0x63 => Operation::LdRegToReg {
            dest: Register::H,
            src: Register::E,
        },
        0x64 => Operation::LdRegToReg {
            dest: Register::H,
            src: Register::H,
        },
        0x65 => Operation::LdRegToReg {
            dest: Register::H,
            src: Register::L,
        },
        0x67 => Operation::LdRegToReg {
            dest: Register::H,
            src: Register::A,
        },
        0x68 => Operation::LdRegToReg {
            dest: Register::L,
            src: Register::B,
        },
        0x69 => Operation::LdRegToReg {
            dest: Register::L,
            src: Register::C,
        },
        0x6A => Operation::LdRegToReg {
            dest: Register::L,
            src: Register::D,
        },
        0x6B => Operation::LdRegToReg {
            dest: Register::L,
            src: Register::E,
        },
        0x6C => Operation::LdRegToReg {
            dest: Register::L,
            src: Register::H,
        },
        0x6D => Operation::LdRegToReg {
            dest: Register::L,
            src: Register::L,
        },
        0x6F => Operation::LdRegToReg {
            dest: Register::L,
            src: Register::A,
        },
        0x78 => Operation::LdRegToReg {
            dest: Register::A,
            src: Register::B,
        },
        0x79 => Operation::LdRegToReg {
            dest: Register::A,
            src: Register::C,
        },
        0x7A => Operation::LdRegToReg {
            dest: Register::A,
            src: Register::D,
        },
        0x7B => Operation::LdRegToReg {
            dest: Register::A,
            src: Register::E,
        },
        0x7C => Operation::LdRegToReg {
            dest: Register::A,
            src: Register::H,
        },
        0x7D => Operation::LdRegToReg {
            dest: Register::A,
            src: Register::L,
        },
        0x7F => Operation::LdRegToReg {
            dest: Register::A,
            src: Register::A,
        },
        0x06 => Operation::LdImmediateDataToReg { dest: Register::B },
        0x0E => Operation::LdImmediateDataToReg { dest: Register::C },
        0x16 => Operation::LdImmediateDataToReg { dest: Register::D },
        0x1E => Operation::LdImmediateDataToReg { dest: Register::E },
        0x26 => Operation::LdImmediateDataToReg { dest: Register::H },
        0x2E => Operation::LdImmediateDataToReg { dest: Register::L },
        0x3E => Operation::LdImmediateDataToReg { dest: Register::A },
        0x46 => Operation::LdRegPairAddrToReg {
            dest: Register::B,
            src: (Register::H, Register::L),
        },
        0x4E => Operation::LdRegPairAddrToReg {
            dest: Register::C,
            src: (Register::H, Register::L),
        },
        0x56 => Operation::LdRegPairAddrToReg {
            dest: Register::D,
            src: (Register::H, Register::L),
        },
        0x5E => Operation::LdRegPairAddrToReg {
            dest: Register::E,
            src: (Register::H, Register::L),
        },
        0x66 => Operation::LdRegPairAddrToReg {
            dest: Register::H,
            src: (Register::H, Register::L),
        },
        0x6E => Operation::LdRegPairAddrToReg {
            dest: Register::L,
            src: (Register::H, Register::L),
        },
        0x7E => Operation::LdRegPairAddrToReg {
            dest: Register::A,
            src: (Register::H, Register::L),
        },
        0x0A => Operation::LdRegPairAddrToReg {
            dest: Register::A,
            src: (Register::B, Register::C),
        },
        0x1A => Operation::LdRegPairAddrToReg {
            dest: Register::A,
            src: (Register::D, Register::E),
        },
        0x70 => Operation::LdRegToRegPairAddr {
            dest: (Register::H, Register::L),
            src: Register::B,
        },
        0x71 => Operation::LdRegToRegPairAddr {
            dest: (Register::H, Register::L),
            src: Register::C,
        },
        0x72 => Operation::LdRegToRegPairAddr {
            dest: (Register::H, Register::L),
            src: Register::D,
        },
        0x73 => Operation::LdRegToRegPairAddr {
            dest: (Register::H, Register::L),
            src: Register::E,
        },
        0x74 => Operation::LdRegToRegPairAddr {
            dest: (Register::H, Register::L),
            src: Register::H,
        },
        0x75 => Operation::LdRegToRegPairAddr {
            dest: (Register::H, Register::L),
            src: Register::L,
        },
        0x77 => Operation::LdRegToRegPairAddr {
            dest: (Register::H, Register::L),
            src: Register::A,
        },
        0x02 => Operation::LdRegToRegPairAddr {
            dest: (Register::B, Register::C),
            src: Register::A,
        },
        0x12 => Operation::LdRegToRegPairAddr {
            dest: (Register::D, Register::E),
            src: Register::A,
        },
        0x36 => Operation::LdImmediateDataToHLAddr,
        0xF2 => Operation::LdRegCAddrToRegA,
        0xE2 => Operation::LdRegAToRegCAddr,
        0xF0 => Operation::LdImmediateAddrToRegA,
        0xE0 => Operation::LdRegAToImmediateAddr,
        0xFA => Operation::LdImmediate16BitAddrToRegA,
        0xEA => Operation::LdRegAToImmediate16BitAddr,
        0x2A => Operation::LdHLAddrToRegAAndIncrement,
        0x3A => Operation::LdHLAddrToRegAAndDecrement,
        0x22 => Operation::LdRegAToHLAddrAndIncrement,
        0x32 => Operation::LdRegAToHLAddrAndDecrement,
        0x01 => Operation::LdImmediate16BitDataToRegPair {
            dest: (Register::B, Register::C),
        },
        0x11 => Operation::LdImmediate16BitDataToRegPair {
            dest: (Register::D, Register::E),
        },
        0x21 => Operation::LdImmediate16BitDataToRegPair {
            dest: (Register::H, Register::L),
        },
        0x31 => Operation::LdImmediate16BitDataToSP,
        0xF9 => Operation::LdRegHLToSP,
        0xC5 => Operation::PushRegPairToStack {
            src: (Register::B, Register::C),
        },
        0xD5 => Operation::PushRegPairToStack {
            src: (Register::D, Register::E),
        },
        0xE5 => Operation::PushRegPairToStack {
            src: (Register::H, Register::L),
        },
        0xF5 => Operation::PushRegPairToStack {
            src: (Register::A, Register::F),
        },
        0xC1 => Operation::PopStackToRegPair {
            dest: (Register::B, Register::C),
        },
        0xD1 => Operation::PopStackToRegPair {
            dest: (Register::D, Register::E),
        },
        0xE1 => Operation::PopStackToRegPair {
            dest: (Register::H, Register::L),
        },
        0xF1 => Operation::PopStackToRegPair {
            dest: (Register::A, Register::F),
        },
        _ => panic!(),
    }
}
