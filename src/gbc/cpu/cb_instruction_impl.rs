#![allow(non_snake_case)]

use crate::gbc::memory::VirtualMemory;

use super::{
    op_helpers::*,
    register::{Register, RegisterMapMethods, RegisterPair},
    CPU,
};

// RLC B
pub(super) fn instr_0xCB00(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RLC_reg(cpu, Register::B);
}

// RLC C
pub(super) fn instr_0xCB01(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RLC_reg(cpu, Register::C);
}

// RLC D
pub(super) fn instr_0xCB02(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RLC_reg(cpu, Register::D);
}

// RLC E
pub(super) fn instr_0xCB03(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RLC_reg(cpu, Register::E);
}

// RLC H
pub(super) fn instr_0xCB04(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RLC_reg(cpu, Register::H);
}

// RLC L
pub(super) fn instr_0xCB05(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RLC_reg(cpu, Register::L);
}

// RLC (HL)
pub(super) fn instr_0xCB06(cpu: &mut CPU, mem: &mut VirtualMemory) {
    let addr = cpu.registers.read_pair(RegisterPair::HL);
    let val = mem.read(addr);
    let result = op_RLC(cpu, val);
    mem.write(addr, result);
}

// RLC A
pub(super) fn instr_0xCB07(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RLC_reg(cpu, Register::A);
}

// RRC B
pub(super) fn instr_0xCB08(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RRC_reg(cpu, Register::B);
}

// RRC C
pub(super) fn instr_0xCB09(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RRC_reg(cpu, Register::C);
}

// RRC D
pub(super) fn instr_0xCB0A(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RRC_reg(cpu, Register::D);
}

// RRC E
pub(super) fn instr_0xCB0B(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RRC_reg(cpu, Register::E);
}

// RRC H
pub(super) fn instr_0xCB0C(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RRC_reg(cpu, Register::H);
}

// RRC L
pub(super) fn instr_0xCB0D(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RRC_reg(cpu, Register::L);
}

// RRC (HL)
pub(super) fn instr_0xCB0E(cpu: &mut CPU, mem: &mut VirtualMemory) {
    let addr = cpu.registers.read_pair(RegisterPair::HL);
    let val = mem.read(addr);
    let result = op_RRC(cpu, val);
    mem.write(addr, result);
}

// RRC A
pub(super) fn instr_0xCB0F(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RRC_reg(cpu, Register::A);
}

// RL B
pub(super) fn instr_0xCB10(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RL_reg(cpu, Register::B);
}

// RL C
pub(super) fn instr_0xCB11(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RL_reg(cpu, Register::C);
}

// RL D
pub(super) fn instr_0xCB12(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RL_reg(cpu, Register::D);
}

// RL E
pub(super) fn instr_0xCB13(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RL_reg(cpu, Register::E);
}

// RL H
pub(super) fn instr_0xCB14(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RL_reg(cpu, Register::H);
}

// RL L
pub(super) fn instr_0xCB15(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RL_reg(cpu, Register::L);
}

// RL (HL)
pub(super) fn instr_0xCB16(cpu: &mut CPU, mem: &mut VirtualMemory) {
    let addr = cpu.registers.read_pair(RegisterPair::HL);
    let val = mem.read(addr);
    let result = op_RL(cpu, val);
    mem.write(addr, result);
}

// RL A
pub(super) fn instr_0xCB17(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RL_reg(cpu, Register::A);
}

// RR B
pub(super) fn instr_0xCB18(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RR_reg(cpu, Register::B);
}

// RR C
pub(super) fn instr_0xCB19(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RR_reg(cpu, Register::C);
}

// RR D
pub(super) fn instr_0xCB1A(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RR_reg(cpu, Register::D);
}

// RR E
pub(super) fn instr_0xCB1B(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RR_reg(cpu, Register::E);
}

// RR H
pub(super) fn instr_0xCB1C(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RR_reg(cpu, Register::H);
}

// RR L
pub(super) fn instr_0xCB1D(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RR_reg(cpu, Register::L);
}

// RR (HL)
pub(super) fn instr_0xCB1E(cpu: &mut CPU, mem: &mut VirtualMemory) {
    let addr = cpu.registers.read_pair(RegisterPair::HL);
    let val = mem.read(addr);
    let result = op_RR(cpu, val);
    mem.write(addr, result);
}

// RR A
pub(super) fn instr_0xCB1F(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RR_reg(cpu, Register::A);
}

// SLA B
pub(super) fn instr_0xCB20(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SLA_reg(cpu, Register::B);
}

// SLA C
pub(super) fn instr_0xCB21(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SLA_reg(cpu, Register::C);
}

// SLA D
pub(super) fn instr_0xCB22(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SLA_reg(cpu, Register::D);
}

// SLA E
pub(super) fn instr_0xCB23(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SLA_reg(cpu, Register::E);
}

// SLA H
pub(super) fn instr_0xCB24(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SLA_reg(cpu, Register::H);
}

// SLA L
pub(super) fn instr_0xCB25(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SLA_reg(cpu, Register::L);
}

// SLA (HL)
pub(super) fn instr_0xCB26(cpu: &mut CPU, mem: &mut VirtualMemory) {
    let addr = cpu.registers.read_pair(RegisterPair::HL);
    let val = mem.read(addr);
    let result = op_SLA(cpu, val);
    mem.write(addr, result);
}

// SLA A
pub(super) fn instr_0xCB27(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SLA_reg(cpu, Register::A);
}

// SRA B
pub(super) fn instr_0xCB28(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SRA_reg(cpu, Register::B);
}

// SRA C
pub(super) fn instr_0xCB29(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SRA_reg(cpu, Register::C);
}

// SRA D
pub(super) fn instr_0xCB2A(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SRA_reg(cpu, Register::D);
}

// SRA E
pub(super) fn instr_0xCB2B(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SRA_reg(cpu, Register::E);
}

// SRA H
pub(super) fn instr_0xCB2C(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SRA_reg(cpu, Register::H);
}

// SRA L
pub(super) fn instr_0xCB2D(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SRA_reg(cpu, Register::L);
}

// SRA (HL)
pub(super) fn instr_0xCB2E(cpu: &mut CPU, mem: &mut VirtualMemory) {
    let addr = cpu.registers.read_pair(RegisterPair::HL);
    let val = mem.read(addr);
    let result = op_SRA(cpu, val);
    mem.write(addr, result);
}

// SRA A
pub(super) fn instr_0xCB2F(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SRA_reg(cpu, Register::A);
}

// SWAP B
pub(super) fn instr_0xCB30(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SWAP_reg(cpu, Register::B);
}

// SWAP C
pub(super) fn instr_0xCB31(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SWAP_reg(cpu, Register::C);
}

// SWAP D
pub(super) fn instr_0xCB32(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SWAP_reg(cpu, Register::D);
}

// SWAP E
pub(super) fn instr_0xCB33(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SWAP_reg(cpu, Register::E);
}

// SWAP H
pub(super) fn instr_0xCB34(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SWAP_reg(cpu, Register::H);
}

// SWAP L
pub(super) fn instr_0xCB35(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SWAP_reg(cpu, Register::L);
}

// SWAP (HL)
pub(super) fn instr_0xCB36(cpu: &mut CPU, mem: &mut VirtualMemory) {
    let addr = cpu.registers.read_pair(RegisterPair::HL);
    let val = mem.read(addr);
    let result = op_SWAP(cpu, val);
    mem.write(addr, result);
}

// SWAP A
pub(super) fn instr_0xCB37(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SWAP_reg(cpu, Register::A);
}

// SRL B
pub(super) fn instr_0xCB38(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SRL_reg(cpu, Register::B);
}

// SRL C
pub(super) fn instr_0xCB39(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SRL_reg(cpu, Register::C);
}

// SRL D
pub(super) fn instr_0xCB3A(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SRL_reg(cpu, Register::D);
}

// SRL E
pub(super) fn instr_0xCB3B(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SRL_reg(cpu, Register::E);
}

// SRL H
pub(super) fn instr_0xCB3C(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SRL_reg(cpu, Register::H);
}

// SRL L
pub(super) fn instr_0xCB3D(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SRL_reg(cpu, Register::L);
}

// SRL (HL)
pub(super) fn instr_0xCB3E(cpu: &mut CPU, mem: &mut VirtualMemory) {
    let addr = cpu.registers.read_pair(RegisterPair::HL);
    let val = mem.read(addr);
    let result = op_SRL(cpu, val);
    mem.write(addr, result);
}

// SRL A
pub(super) fn instr_0xCB3F(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SRL_reg(cpu, Register::A);
}

// BIT 0, B
pub(super) fn instr_0xCB40(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 0, Register::B);
}

// BIT 0, C
pub(super) fn instr_0xCB41(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 0, Register::C);
}

// BIT 0, D
pub(super) fn instr_0xCB42(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 0, Register::D);
}

// BIT 0, E
pub(super) fn instr_0xCB43(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 0, Register::E);
}

// BIT 0, H
pub(super) fn instr_0xCB44(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 0, Register::H);
}

// BIT 0, L
pub(super) fn instr_0xCB45(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 0, Register::L);
}

// BIT 0, (HL)
pub(super) fn instr_0xCB46(cpu: &mut CPU, mem: &mut VirtualMemory) {
    op_BIT_from_HLptr(cpu, 0, mem);
}

// BIT 0, A
pub(super) fn instr_0xCB47(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 0, Register::A);
}

// BIT 1, B
pub(super) fn instr_0xCB48(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 1, Register::B);
}

// BIT 1, C
pub(super) fn instr_0xCB49(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 1, Register::C);
}

// BIT 1, D
pub(super) fn instr_0xCB4A(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 1, Register::D);
}

// BIT 1, E
pub(super) fn instr_0xCB4B(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 1, Register::E);
}

// BIT 1, H
pub(super) fn instr_0xCB4C(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 1, Register::H);
}

// BIT 1, L
pub(super) fn instr_0xCB4D(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 1, Register::L);
}

// BIT 1, (HL)
pub(super) fn instr_0xCB4E(cpu: &mut CPU, mem: &mut VirtualMemory) {
    op_BIT_from_HLptr(cpu, 1, mem);
}

// BIT 1, A
pub(super) fn instr_0xCB4F(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 1, Register::A);
}

// BIT 2, B
pub(super) fn instr_0xCB50(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 2, Register::B);
}

// BIT 2, C
pub(super) fn instr_0xCB51(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 2, Register::C);
}

// BIT 2, D
pub(super) fn instr_0xCB52(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 2, Register::D);
}

// BIT 2, E
pub(super) fn instr_0xCB53(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 2, Register::E);
}

// BIT 2, H
pub(super) fn instr_0xCB54(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 2, Register::H);
}

// BIT 2, L
pub(super) fn instr_0xCB55(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 2, Register::L);
}

// BIT 2, (HL)
pub(super) fn instr_0xCB56(cpu: &mut CPU, mem: &mut VirtualMemory) {
    op_BIT_from_HLptr(cpu, 2, mem);
}

// BIT 2, A
pub(super) fn instr_0xCB57(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 2, Register::A);
}

// BIT 3, B
pub(super) fn instr_0xCB58(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 3, Register::B);
}

// BIT 3, C
pub(super) fn instr_0xCB59(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 3, Register::C);
}

// BIT 3, D
pub(super) fn instr_0xCB5A(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 3, Register::D);
}

// BIT 3, E
pub(super) fn instr_0xCB5B(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 3, Register::E);
}

// BIT 3, H
pub(super) fn instr_0xCB5C(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 3, Register::H);
}

// BIT 3, L
pub(super) fn instr_0xCB5D(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 3, Register::L);
}

// BIT 3, (HL)
pub(super) fn instr_0xCB5E(cpu: &mut CPU, mem: &mut VirtualMemory) {
    op_BIT_from_HLptr(cpu, 3, mem);
}

// BIT 3, A
pub(super) fn instr_0xCB5F(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 3, Register::A);
}

// BIT 4, B
pub(super) fn instr_0xCB60(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 4, Register::B);
}

// BIT 4, C
pub(super) fn instr_0xCB61(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 4, Register::C);
}

// BIT 4, D
pub(super) fn instr_0xCB62(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 4, Register::D);
}

// BIT 4, E
pub(super) fn instr_0xCB63(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 4, Register::E);
}

// BIT 4, H
pub(super) fn instr_0xCB64(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 4, Register::H);
}

// BIT 4, L
pub(super) fn instr_0xCB65(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 4, Register::L);
}

// BIT 4, (HL)
pub(super) fn instr_0xCB66(cpu: &mut CPU, mem: &mut VirtualMemory) {
    op_BIT_from_HLptr(cpu, 4, mem);
}

// BIT 4, A
pub(super) fn instr_0xCB67(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 4, Register::A);
}

// BIT 5, B
pub(super) fn instr_0xCB68(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 5, Register::B);
}

// BIT 5, C
pub(super) fn instr_0xCB69(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 5, Register::C);
}

// BIT 5, D
pub(super) fn instr_0xCB6A(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 5, Register::D);
}

// BIT 5, E
pub(super) fn instr_0xCB6B(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 5, Register::E);
}

// BIT 5, H
pub(super) fn instr_0xCB6C(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 5, Register::H);
}

// BIT 5, L
pub(super) fn instr_0xCB6D(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 5, Register::L);
}

// BIT 5, (HL)
pub(super) fn instr_0xCB6E(cpu: &mut CPU, mem: &mut VirtualMemory) {
    op_BIT_from_HLptr(cpu, 5, mem);
}

// BIT 5, A
pub(super) fn instr_0xCB6F(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 5, Register::A);
}

// BIT 6, B
pub(super) fn instr_0xCB70(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 6, Register::B);
}

// BIT 6, C
pub(super) fn instr_0xCB71(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 6, Register::C);
}

// BIT 6, D
pub(super) fn instr_0xCB72(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 6, Register::D);
}

// BIT 6, E
pub(super) fn instr_0xCB73(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 6, Register::E);
}

// BIT 6, H
pub(super) fn instr_0xCB74(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 6, Register::H);
}

// BIT 6, L
pub(super) fn instr_0xCB75(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 6, Register::L);
}

// BIT 6, (HL)
pub(super) fn instr_0xCB76(cpu: &mut CPU, mem: &mut VirtualMemory) {
    op_BIT_from_HLptr(cpu, 6, mem);
}

// BIT 6, A
pub(super) fn instr_0xCB77(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 6, Register::A);
}

// BIT 7, B
pub(super) fn instr_0xCB78(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 7, Register::B);
}

// BIT 7, C
pub(super) fn instr_0xCB79(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 7, Register::C);
}

// BIT 7, D
pub(super) fn instr_0xCB7A(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 7, Register::D);
}

// BIT 7, E
pub(super) fn instr_0xCB7B(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 7, Register::E);
}

// BIT 7, H
pub(super) fn instr_0xCB7C(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 7, Register::H);
}

// BIT 7, L
pub(super) fn instr_0xCB7D(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 7, Register::L);
}

// BIT 7, (HL)
pub(super) fn instr_0xCB7E(cpu: &mut CPU, mem: &mut VirtualMemory) {
    op_BIT_from_HLptr(cpu, 7, mem);
}

// BIT 7, A
pub(super) fn instr_0xCB7F(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_BIT_reg(cpu, 7, Register::A);
}

// RES 0, B
pub(super) fn instr_0xCB80(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 0, Register::B);
}

// RES 0, C
pub(super) fn instr_0xCB81(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 0, Register::C);
}

// RES 0, D
pub(super) fn instr_0xCB82(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 0, Register::D);
}

// RES 0, E
pub(super) fn instr_0xCB83(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 0, Register::E);
}

// RES 0, H
pub(super) fn instr_0xCB84(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 0, Register::H);
}

// RES 0, L
pub(super) fn instr_0xCB85(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 0, Register::L);
}

// RES 0, (HL)
pub(super) fn instr_0xCB86(cpu: &mut CPU, mem: &mut VirtualMemory) {
    op_RES_from_HLptr(cpu, 0, mem);
}

// RES 0, A
pub(super) fn instr_0xCB87(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 0, Register::A);
}

// RES 1, B
pub(super) fn instr_0xCB88(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 1, Register::B);
}

// RES 1, C
pub(super) fn instr_0xCB89(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 1, Register::C);
}

// RES 1, D
pub(super) fn instr_0xCB8A(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 1, Register::D);
}

// RES 1, E
pub(super) fn instr_0xCB8B(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 1, Register::E);
}

// RES 1, H
pub(super) fn instr_0xCB8C(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 1, Register::H);
}

// RES 1, L
pub(super) fn instr_0xCB8D(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 1, Register::L);
}

// RES 1, (HL)
pub(super) fn instr_0xCB8E(cpu: &mut CPU, mem: &mut VirtualMemory) {
    op_RES_from_HLptr(cpu, 1, mem);
}

// RES 1, A
pub(super) fn instr_0xCB8F(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 1, Register::A);
}

// RES 2, B
pub(super) fn instr_0xCB90(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 2, Register::B);
}

// RES 2, C
pub(super) fn instr_0xCB91(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 2, Register::C);
}

// RES 2, D
pub(super) fn instr_0xCB92(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 2, Register::D);
}

// RES 2, E
pub(super) fn instr_0xCB93(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 2, Register::E);
}

// RES 2, H
pub(super) fn instr_0xCB94(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 2, Register::H);
}

// RES 2, L
pub(super) fn instr_0xCB95(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 2, Register::L);
}

// RES 2, (HL)
pub(super) fn instr_0xCB96(cpu: &mut CPU, mem: &mut VirtualMemory) {
    op_RES_from_HLptr(cpu, 2, mem);
}

// RES 2, A
pub(super) fn instr_0xCB97(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 2, Register::A);
}

// RES 3, B
pub(super) fn instr_0xCB98(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 3, Register::B);
}

// RES 3, C
pub(super) fn instr_0xCB99(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 3, Register::C);
}

// RES 3, D
pub(super) fn instr_0xCB9A(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 3, Register::D);
}

// RES 3, E
pub(super) fn instr_0xCB9B(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 3, Register::E);
}

// RES 3, H
pub(super) fn instr_0xCB9C(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 3, Register::H);
}

// RES 3, L
pub(super) fn instr_0xCB9D(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 3, Register::L);
}

// RES 3, (HL)
pub(super) fn instr_0xCB9E(cpu: &mut CPU, mem: &mut VirtualMemory) {
    op_RES_from_HLptr(cpu, 3, mem);
}

// RES 3, A
pub(super) fn instr_0xCB9F(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 3, Register::A);
}

// RES 4, B
pub(super) fn instr_0xCBA0(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 4, Register::B);
}

// RES 4, C
pub(super) fn instr_0xCBA1(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 4, Register::C);
}

// RES 4, D
pub(super) fn instr_0xCBA2(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 4, Register::D);
}

// RES 4, E
pub(super) fn instr_0xCBA3(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 4, Register::E);
}

// RES 4, H
pub(super) fn instr_0xCBA4(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 4, Register::H);
}

// RES 4, L
pub(super) fn instr_0xCBA5(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 4, Register::L);
}

// RES 4, (HL)
pub(super) fn instr_0xCBA6(cpu: &mut CPU, mem: &mut VirtualMemory) {
    op_RES_from_HLptr(cpu, 4, mem);
}

// RES 4, A
pub(super) fn instr_0xCBA7(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 4, Register::A);
}

// RES 5, B
pub(super) fn instr_0xCBA8(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 5, Register::B);
}

// RES 5, C
pub(super) fn instr_0xCBA9(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 5, Register::C);
}

// RES 5, D
pub(super) fn instr_0xCBAA(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 5, Register::D);
}

// RES 5, E
pub(super) fn instr_0xCBAB(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 5, Register::E);
}

// RES 5, H
pub(super) fn instr_0xCBAC(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 5, Register::H);
}

// RES 5, L
pub(super) fn instr_0xCBAD(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 5, Register::L);
}

// RES 5, (HL)
pub(super) fn instr_0xCBAE(cpu: &mut CPU, mem: &mut VirtualMemory) {
    op_RES_from_HLptr(cpu, 5, mem);
}

// RES 5, A
pub(super) fn instr_0xCBAF(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 5, Register::A);
}

// RES 6, B
pub(super) fn instr_0xCBB0(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 6, Register::B);
}

// RES 6, C
pub(super) fn instr_0xCBB1(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 6, Register::C);
}

// RES 6, D
pub(super) fn instr_0xCBB2(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 6, Register::D);
}

// RES 6, E
pub(super) fn instr_0xCBB3(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 6, Register::E);
}

// RES 6, H
pub(super) fn instr_0xCBB4(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 6, Register::H);
}

// RES 6, L
pub(super) fn instr_0xCBB5(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 6, Register::L);
}

// RES 6, (HL)
pub(super) fn instr_0xCBB6(cpu: &mut CPU, mem: &mut VirtualMemory) {
    op_RES_from_HLptr(cpu, 6, mem);
}

// RES 6, A
pub(super) fn instr_0xCBB7(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 6, Register::A);
}

// RES 7, B
pub(super) fn instr_0xCBB8(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 7, Register::B);
}

// RES 7, C
pub(super) fn instr_0xCBB9(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 7, Register::C);
}

// RES 7, D
pub(super) fn instr_0xCBBA(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 7, Register::D);
}

// RES 7, E
pub(super) fn instr_0xCBBB(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 7, Register::E);
}

// RES 7, H
pub(super) fn instr_0xCBBC(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 7, Register::H);
}

// RES 7, L
pub(super) fn instr_0xCBBD(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 7, Register::L);
}

// RES 7, (HL)
pub(super) fn instr_0xCBBE(cpu: &mut CPU, mem: &mut VirtualMemory) {
    op_RES_from_HLptr(cpu, 7, mem);
}

// RES 7, A
pub(super) fn instr_0xCBBF(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_RES_reg(cpu, 7, Register::A);
}

// SET 0, B
pub(super) fn instr_0xCBC0(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 0, Register::B);
}

// SET 0, C
pub(super) fn instr_0xCBC1(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 0, Register::C);
}

// SET 0, D
pub(super) fn instr_0xCBC2(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 0, Register::D);
}

// SET 0, E
pub(super) fn instr_0xCBC3(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 0, Register::E);
}

// SET 0, H
pub(super) fn instr_0xCBC4(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 0, Register::H);
}

// SET 0, L
pub(super) fn instr_0xCBC5(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 0, Register::L);
}

// SET 0, (HL)
pub(super) fn instr_0xCBC6(cpu: &mut CPU, mem: &mut VirtualMemory) {
    op_SET_from_HLptr(cpu, 0, mem);
}

// SET 0, A
pub(super) fn instr_0xCBC7(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 0, Register::A);
}

// SET 1, B
pub(super) fn instr_0xCBC8(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 1, Register::B);
}

// SET 1, C
pub(super) fn instr_0xCBC9(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 1, Register::C);
}

// SET 1, D
pub(super) fn instr_0xCBCA(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 1, Register::D);
}

// SET 1, E
pub(super) fn instr_0xCBCB(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 1, Register::E);
}

// SET 1, H
pub(super) fn instr_0xCBCC(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 1, Register::H);
}

// SET 1, L
pub(super) fn instr_0xCBCD(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 1, Register::L);
}

// SET 1, (HL)
pub(super) fn instr_0xCBCE(cpu: &mut CPU, mem: &mut VirtualMemory) {
    op_SET_from_HLptr(cpu, 1, mem);
}

// SET 1, A
pub(super) fn instr_0xCBCF(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 1, Register::A);
}

// SET 2, B
pub(super) fn instr_0xCBD0(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 2, Register::B);
}

// SET 2, C
pub(super) fn instr_0xCBD1(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 2, Register::C);
}

// SET 2, D
pub(super) fn instr_0xCBD2(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 2, Register::D);
}

// SET 2, E
pub(super) fn instr_0xCBD3(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 2, Register::E);
}

// SET 2, H
pub(super) fn instr_0xCBD4(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 2, Register::H);
}

// SET 2, L
pub(super) fn instr_0xCBD5(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 2, Register::L);
}

// SET 2, (HL)
pub(super) fn instr_0xCBD6(cpu: &mut CPU, mem: &mut VirtualMemory) {
    op_SET_from_HLptr(cpu, 2, mem);
}

// SET 2, A
pub(super) fn instr_0xCBD7(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 2, Register::A);
}

// SET 3, B
pub(super) fn instr_0xCBD8(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 3, Register::B);
}

// SET 3, C
pub(super) fn instr_0xCBD9(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 3, Register::C);
}

// SET 3, D
pub(super) fn instr_0xCBDA(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 3, Register::D);
}

// SET 3, E
pub(super) fn instr_0xCBDB(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 3, Register::E);
}

// SET 3, H
pub(super) fn instr_0xCBDC(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 3, Register::H);
}

// SET 3, L
pub(super) fn instr_0xCBDD(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 3, Register::L);
}

// SET 3, (HL)
pub(super) fn instr_0xCBDE(cpu: &mut CPU, mem: &mut VirtualMemory) {
    op_SET_from_HLptr(cpu, 3, mem);
}

// SET 3, A
pub(super) fn instr_0xCBDF(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 3, Register::A);
}

// SET 4, B
pub(super) fn instr_0xCBE0(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 4, Register::B);
}

// SET 4, C
pub(super) fn instr_0xCBE1(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 4, Register::C);
}

// SET 4, D
pub(super) fn instr_0xCBE2(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 4, Register::D);
}

// SET 4, E
pub(super) fn instr_0xCBE3(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 4, Register::E);
}

// SET 4, H
pub(super) fn instr_0xCBE4(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 4, Register::H);
}

// SET 4, L
pub(super) fn instr_0xCBE5(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 4, Register::L);
}

// SET 4, (HL)
pub(super) fn instr_0xCBE6(cpu: &mut CPU, mem: &mut VirtualMemory) {
    op_SET_from_HLptr(cpu, 4, mem);
}

// SET 4, A
pub(super) fn instr_0xCBE7(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 4, Register::A);
}

// SET 5, B
pub(super) fn instr_0xCBE8(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 5, Register::B);
}

// SET 5, C
pub(super) fn instr_0xCBE9(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 5, Register::C);
}

// SET 5, D
pub(super) fn instr_0xCBEA(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 5, Register::D);
}

// SET 5, E
pub(super) fn instr_0xCBEB(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 5, Register::E);
}

// SET 5, H
pub(super) fn instr_0xCBEC(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 5, Register::H);
}

// SET 5, L
pub(super) fn instr_0xCBED(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 5, Register::L);
}

// SET 5, (HL)
pub(super) fn instr_0xCBEE(cpu: &mut CPU, mem: &mut VirtualMemory) {
    op_SET_from_HLptr(cpu, 5, mem);
}

// SET 5, A
pub(super) fn instr_0xCBEF(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 5, Register::A);
}

// SET 6, B
pub(super) fn instr_0xCBF0(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 6, Register::B);
}

// SET 6, C
pub(super) fn instr_0xCBF1(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 6, Register::C);
}

// SET 6, D
pub(super) fn instr_0xCBF2(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 6, Register::D);
}

// SET 6, E
pub(super) fn instr_0xCBF3(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 6, Register::E);
}

// SET 6, H
pub(super) fn instr_0xCBF4(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 6, Register::H);
}

// SET 6, L
pub(super) fn instr_0xCBF5(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 6, Register::L);
}

// SET 6, (HL)
pub(super) fn instr_0xCBF6(cpu: &mut CPU, mem: &mut VirtualMemory) {
    op_SET_from_HLptr(cpu, 6, mem);
}

// SET 6, A
pub(super) fn instr_0xCBF7(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 6, Register::A);
}

// SET 7, B
pub(super) fn instr_0xCBF8(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 7, Register::B);
}

// SET 7, C
pub(super) fn instr_0xCBF9(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 7, Register::C);
}

// SET 7, D
pub(super) fn instr_0xCBFA(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 7, Register::D);
}

// SET 7, E
pub(super) fn instr_0xCBFB(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 7, Register::E);
}

// SET 7, H
pub(super) fn instr_0xCBFC(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 7, Register::H);
}

// SET 7, L
pub(super) fn instr_0xCBFD(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 7, Register::L);
}

// SET 7, (HL)
pub(super) fn instr_0xCBFE(cpu: &mut CPU, mem: &mut VirtualMemory) {
    op_SET_from_HLptr(cpu, 7, mem);
}

// SET 7, A
pub(super) fn instr_0xCBFF(cpu: &mut CPU, _mem: &mut VirtualMemory) {
    op_SET_reg(cpu, 7, Register::A);
}
