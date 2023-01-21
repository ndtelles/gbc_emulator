#![allow(non_snake_case)]

use crate::gbc::{virtual_memory, GBCState};

use super::{
    op_helpers::*,
    register::{Register, RegisterMapMethods, RegisterPair}, consume_cycles,
};

// RLC B
pub(super) fn instr_0xCB00(state: &mut GBCState) {
    op_RLC_reg(state, Register::B);
    consume_cycles(state, 8);
}

// RLC C
pub(super) fn instr_0xCB01(state: &mut GBCState) {
    op_RLC_reg(state, Register::C);
    consume_cycles(state, 8);
}

// RLC D
pub(super) fn instr_0xCB02(state: &mut GBCState) {
    op_RLC_reg(state, Register::D);
    consume_cycles(state, 8);
}

// RLC E
pub(super) fn instr_0xCB03(state: &mut GBCState) {
    op_RLC_reg(state, Register::E);
    consume_cycles(state, 8);
}

// RLC H
pub(super) fn instr_0xCB04(state: &mut GBCState) {
    op_RLC_reg(state, Register::H);
    consume_cycles(state, 8);
}

// RLC L
pub(super) fn instr_0xCB05(state: &mut GBCState) {
    op_RLC_reg(state, Register::L);
    consume_cycles(state, 8);
}

// RLC (HL)
pub(super) fn instr_0xCB06(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    let result = op_RLC(state, val);
    virtual_memory::write(state, addr, result);
    consume_cycles(state, 16);
}

// RLC A
pub(super) fn instr_0xCB07(state: &mut GBCState) {
    op_RLC_reg(state, Register::A);
    consume_cycles(state, 8);
}

// RRC B
pub(super) fn instr_0xCB08(state: &mut GBCState) {
    op_RRC_reg(state, Register::B);
    consume_cycles(state, 8);
}

// RRC C
pub(super) fn instr_0xCB09(state: &mut GBCState) {
    op_RRC_reg(state, Register::C);
    consume_cycles(state, 8);
}

// RRC D
pub(super) fn instr_0xCB0A(state: &mut GBCState) {
    op_RRC_reg(state, Register::D);
    consume_cycles(state, 8);
}

// RRC E
pub(super) fn instr_0xCB0B(state: &mut GBCState) {
    op_RRC_reg(state, Register::E);
    consume_cycles(state, 8);
}

// RRC H
pub(super) fn instr_0xCB0C(state: &mut GBCState) {
    op_RRC_reg(state, Register::H);
    consume_cycles(state, 8);
}

// RRC L
pub(super) fn instr_0xCB0D(state: &mut GBCState) {
    op_RRC_reg(state, Register::L);
    consume_cycles(state, 8);
}

// RRC (HL)
pub(super) fn instr_0xCB0E(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    let result = op_RRC(state, val);
    virtual_memory::write(state, addr, result);
    consume_cycles(state, 16);
}

// RRC A
pub(super) fn instr_0xCB0F(state: &mut GBCState) {
    op_RRC_reg(state, Register::A);
    consume_cycles(state, 8);
}

// RL B
pub(super) fn instr_0xCB10(state: &mut GBCState) {
    op_RL_reg(state, Register::B);
    consume_cycles(state, 8);
}

// RL C
pub(super) fn instr_0xCB11(state: &mut GBCState) {
    op_RL_reg(state, Register::C);
    consume_cycles(state, 8);
}

// RL D
pub(super) fn instr_0xCB12(state: &mut GBCState) {
    op_RL_reg(state, Register::D);
    consume_cycles(state, 8);
}

// RL E
pub(super) fn instr_0xCB13(state: &mut GBCState) {
    op_RL_reg(state, Register::E);
    consume_cycles(state, 8);
}

// RL H
pub(super) fn instr_0xCB14(state: &mut GBCState) {
    op_RL_reg(state, Register::H);
    consume_cycles(state, 8);
}

// RL L
pub(super) fn instr_0xCB15(state: &mut GBCState) {
    op_RL_reg(state, Register::L);
    consume_cycles(state, 8);
}

// RL (HL)
pub(super) fn instr_0xCB16(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    let result = op_RL(state, val);
    virtual_memory::write(state, addr, result);
    consume_cycles(state, 16);
}

// RL A
pub(super) fn instr_0xCB17(state: &mut GBCState) {
    op_RL_reg(state, Register::A);
    consume_cycles(state, 8);
}

// RR B
pub(super) fn instr_0xCB18(state: &mut GBCState) {
    op_RR_reg(state, Register::B);
    consume_cycles(state, 8);
}

// RR C
pub(super) fn instr_0xCB19(state: &mut GBCState) {
    op_RR_reg(state, Register::C);
    consume_cycles(state, 8);
}

// RR D
pub(super) fn instr_0xCB1A(state: &mut GBCState) {
    op_RR_reg(state, Register::D);
    consume_cycles(state, 8);
}

// RR E
pub(super) fn instr_0xCB1B(state: &mut GBCState) {
    op_RR_reg(state, Register::E);
    consume_cycles(state, 8);
}

// RR H
pub(super) fn instr_0xCB1C(state: &mut GBCState) {
    op_RR_reg(state, Register::H);
    consume_cycles(state, 8);
}

// RR L
pub(super) fn instr_0xCB1D(state: &mut GBCState) {
    op_RR_reg(state, Register::L);
    consume_cycles(state, 8);
}

// RR (HL)
pub(super) fn instr_0xCB1E(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    let result = op_RR(state, val);
    virtual_memory::write(state, addr, result);
    consume_cycles(state, 16);
}

// RR A
pub(super) fn instr_0xCB1F(state: &mut GBCState) {
    op_RR_reg(state, Register::A);
    consume_cycles(state, 8);
}

// SLA B
pub(super) fn instr_0xCB20(state: &mut GBCState) {
    op_SLA_reg(state, Register::B);
    consume_cycles(state, 8);
}

// SLA C
pub(super) fn instr_0xCB21(state: &mut GBCState) {
    op_SLA_reg(state, Register::C);
    consume_cycles(state, 8);
}

// SLA D
pub(super) fn instr_0xCB22(state: &mut GBCState) {
    op_SLA_reg(state, Register::D);
    consume_cycles(state, 8);
}

// SLA E
pub(super) fn instr_0xCB23(state: &mut GBCState) {
    op_SLA_reg(state, Register::E);
    consume_cycles(state, 8);
}

// SLA H
pub(super) fn instr_0xCB24(state: &mut GBCState) {
    op_SLA_reg(state, Register::H);
    consume_cycles(state, 8);
}

// SLA L
pub(super) fn instr_0xCB25(state: &mut GBCState) {
    op_SLA_reg(state, Register::L);
    consume_cycles(state, 8);
}

// SLA (HL)
pub(super) fn instr_0xCB26(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    let result = op_SLA(state, val);
    virtual_memory::write(state, addr, result);
    consume_cycles(state, 16);
}

// SLA A
pub(super) fn instr_0xCB27(state: &mut GBCState) {
    op_SLA_reg(state, Register::A);
    consume_cycles(state, 8);
}

// SRA B
pub(super) fn instr_0xCB28(state: &mut GBCState) {
    op_SRA_reg(state, Register::B);
    consume_cycles(state, 8);
}

// SRA C
pub(super) fn instr_0xCB29(state: &mut GBCState) {
    op_SRA_reg(state, Register::C);
    consume_cycles(state, 8);
}

// SRA D
pub(super) fn instr_0xCB2A(state: &mut GBCState) {
    op_SRA_reg(state, Register::D);
    consume_cycles(state, 8);
}

// SRA E
pub(super) fn instr_0xCB2B(state: &mut GBCState) {
    op_SRA_reg(state, Register::E);
    consume_cycles(state, 8);
}

// SRA H
pub(super) fn instr_0xCB2C(state: &mut GBCState) {
    op_SRA_reg(state, Register::H);
    consume_cycles(state, 8);
}

// SRA L
pub(super) fn instr_0xCB2D(state: &mut GBCState) {
    op_SRA_reg(state, Register::L);
    consume_cycles(state, 8);
}

// SRA (HL)
pub(super) fn instr_0xCB2E(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    let result = op_SRA(state, val);
    virtual_memory::write(state, addr, result);
    consume_cycles(state, 16);
}

// SRA A
pub(super) fn instr_0xCB2F(state: &mut GBCState) {
    op_SRA_reg(state, Register::A);
    consume_cycles(state, 8);
}

// SWAP B
pub(super) fn instr_0xCB30(state: &mut GBCState) {
    op_SWAP_reg(state, Register::B);
    consume_cycles(state, 8);
}

// SWAP C
pub(super) fn instr_0xCB31(state: &mut GBCState) {
    op_SWAP_reg(state, Register::C);
    consume_cycles(state, 8);
}

// SWAP D
pub(super) fn instr_0xCB32(state: &mut GBCState) {
    op_SWAP_reg(state, Register::D);
    consume_cycles(state, 8);
}

// SWAP E
pub(super) fn instr_0xCB33(state: &mut GBCState) {
    op_SWAP_reg(state, Register::E);
    consume_cycles(state, 8);
}

// SWAP H
pub(super) fn instr_0xCB34(state: &mut GBCState) {
    op_SWAP_reg(state, Register::H);
    consume_cycles(state, 8);
}

// SWAP L
pub(super) fn instr_0xCB35(state: &mut GBCState) {
    op_SWAP_reg(state, Register::L);
    consume_cycles(state, 8);
}

// SWAP (HL)
pub(super) fn instr_0xCB36(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    let result = op_SWAP(state, val);
    virtual_memory::write(state, addr, result);
    consume_cycles(state, 16);
}

// SWAP A
pub(super) fn instr_0xCB37(state: &mut GBCState) {
    op_SWAP_reg(state, Register::A);
    consume_cycles(state, 8);
}

// SRL B
pub(super) fn instr_0xCB38(state: &mut GBCState) {
    op_SRL_reg(state, Register::B);
    consume_cycles(state, 8);
}

// SRL C
pub(super) fn instr_0xCB39(state: &mut GBCState) {
    op_SRL_reg(state, Register::C);
    consume_cycles(state, 8);
}

// SRL D
pub(super) fn instr_0xCB3A(state: &mut GBCState) {
    op_SRL_reg(state, Register::D);
    consume_cycles(state, 8);
}

// SRL E
pub(super) fn instr_0xCB3B(state: &mut GBCState) {
    op_SRL_reg(state, Register::E);
    consume_cycles(state, 8);
}

// SRL H
pub(super) fn instr_0xCB3C(state: &mut GBCState) {
    op_SRL_reg(state, Register::H);
    consume_cycles(state, 8);
}

// SRL L
pub(super) fn instr_0xCB3D(state: &mut GBCState) {
    op_SRL_reg(state, Register::L);
    consume_cycles(state, 8);
}

// SRL (HL)
pub(super) fn instr_0xCB3E(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    let result = op_SRL(state, val);
    virtual_memory::write(state, addr, result);
    consume_cycles(state, 16);
}

// SRL A
pub(super) fn instr_0xCB3F(state: &mut GBCState) {
    op_SRL_reg(state, Register::A);
    consume_cycles(state, 8);
}

// BIT 0, B
pub(super) fn instr_0xCB40(state: &mut GBCState) {
    op_BIT_reg(state, 0, Register::B);
    consume_cycles(state, 8);
}

// BIT 0, C
pub(super) fn instr_0xCB41(state: &mut GBCState) {
    op_BIT_reg(state, 0, Register::C);
    consume_cycles(state, 8);
}

// BIT 0, D
pub(super) fn instr_0xCB42(state: &mut GBCState) {
    op_BIT_reg(state, 0, Register::D);
    consume_cycles(state, 8);
}

// BIT 0, E
pub(super) fn instr_0xCB43(state: &mut GBCState) {
    op_BIT_reg(state, 0, Register::E);
    consume_cycles(state, 8);
}

// BIT 0, H
pub(super) fn instr_0xCB44(state: &mut GBCState) {
    op_BIT_reg(state, 0, Register::H);
    consume_cycles(state, 8);
}

// BIT 0, L
pub(super) fn instr_0xCB45(state: &mut GBCState) {
    op_BIT_reg(state, 0, Register::L);
    consume_cycles(state, 8);
}

// BIT 0, (HL)
pub(super) fn instr_0xCB46(state: &mut GBCState) {
    op_BIT_from_HLptr(state, 0);
    consume_cycles(state, 12);
}

// BIT 0, A
pub(super) fn instr_0xCB47(state: &mut GBCState) {
    op_BIT_reg(state, 0, Register::A);
    consume_cycles(state, 8);
}

// BIT 1, B
pub(super) fn instr_0xCB48(state: &mut GBCState) {
    op_BIT_reg(state, 1, Register::B);
    consume_cycles(state, 8);
}

// BIT 1, C
pub(super) fn instr_0xCB49(state: &mut GBCState) {
    op_BIT_reg(state, 1, Register::C);
    consume_cycles(state, 8);
}

// BIT 1, D
pub(super) fn instr_0xCB4A(state: &mut GBCState) {
    op_BIT_reg(state, 1, Register::D);
    consume_cycles(state, 8);
}

// BIT 1, E
pub(super) fn instr_0xCB4B(state: &mut GBCState) {
    op_BIT_reg(state, 1, Register::E);
    consume_cycles(state, 8);
}

// BIT 1, H
pub(super) fn instr_0xCB4C(state: &mut GBCState) {
    op_BIT_reg(state, 1, Register::H);
    consume_cycles(state, 8);
}

// BIT 1, L
pub(super) fn instr_0xCB4D(state: &mut GBCState) {
    op_BIT_reg(state, 1, Register::L);
    consume_cycles(state, 8);
}

// BIT 1, (HL)
pub(super) fn instr_0xCB4E(state: &mut GBCState) {
    op_BIT_from_HLptr(state, 1);
    consume_cycles(state, 12);
}

// BIT 1, A
pub(super) fn instr_0xCB4F(state: &mut GBCState) {
    op_BIT_reg(state, 1, Register::A);
    consume_cycles(state, 8);
}

// BIT 2, B
pub(super) fn instr_0xCB50(state: &mut GBCState) {
    op_BIT_reg(state, 2, Register::B);
    consume_cycles(state, 8);
}

// BIT 2, C
pub(super) fn instr_0xCB51(state: &mut GBCState) {
    op_BIT_reg(state, 2, Register::C);
    consume_cycles(state, 8);
}

// BIT 2, D
pub(super) fn instr_0xCB52(state: &mut GBCState) {
    op_BIT_reg(state, 2, Register::D);
    consume_cycles(state, 8);
}

// BIT 2, E
pub(super) fn instr_0xCB53(state: &mut GBCState) {
    op_BIT_reg(state, 2, Register::E);
    consume_cycles(state, 8);
}

// BIT 2, H
pub(super) fn instr_0xCB54(state: &mut GBCState) {
    op_BIT_reg(state, 2, Register::H);
    consume_cycles(state, 8);
}

// BIT 2, L
pub(super) fn instr_0xCB55(state: &mut GBCState) {
    op_BIT_reg(state, 2, Register::L);
    consume_cycles(state, 8);
}

// BIT 2, (HL)
pub(super) fn instr_0xCB56(state: &mut GBCState) {
    op_BIT_from_HLptr(state, 2);
    consume_cycles(state, 12);
}

// BIT 2, A
pub(super) fn instr_0xCB57(state: &mut GBCState) {
    op_BIT_reg(state, 2, Register::A);
    consume_cycles(state, 8);
}

// BIT 3, B
pub(super) fn instr_0xCB58(state: &mut GBCState) {
    op_BIT_reg(state, 3, Register::B);
    consume_cycles(state, 8);
}

// BIT 3, C
pub(super) fn instr_0xCB59(state: &mut GBCState) {
    op_BIT_reg(state, 3, Register::C);
    consume_cycles(state, 8);
}

// BIT 3, D
pub(super) fn instr_0xCB5A(state: &mut GBCState) {
    op_BIT_reg(state, 3, Register::D);
    consume_cycles(state, 8);
}

// BIT 3, E
pub(super) fn instr_0xCB5B(state: &mut GBCState) {
    op_BIT_reg(state, 3, Register::E);
    consume_cycles(state, 8);
}

// BIT 3, H
pub(super) fn instr_0xCB5C(state: &mut GBCState) {
    op_BIT_reg(state, 3, Register::H);
    consume_cycles(state, 8);
}

// BIT 3, L
pub(super) fn instr_0xCB5D(state: &mut GBCState) {
    op_BIT_reg(state, 3, Register::L);
    consume_cycles(state, 8);
}

// BIT 3, (HL)
pub(super) fn instr_0xCB5E(state: &mut GBCState) {
    op_BIT_from_HLptr(state, 3);
    consume_cycles(state, 12);
}

// BIT 3, A
pub(super) fn instr_0xCB5F(state: &mut GBCState) {
    op_BIT_reg(state, 3, Register::A);
    consume_cycles(state, 8);
}

// BIT 4, B
pub(super) fn instr_0xCB60(state: &mut GBCState) {
    op_BIT_reg(state, 4, Register::B);
    consume_cycles(state, 8);
}

// BIT 4, C
pub(super) fn instr_0xCB61(state: &mut GBCState) {
    op_BIT_reg(state, 4, Register::C);
    consume_cycles(state, 8);
}

// BIT 4, D
pub(super) fn instr_0xCB62(state: &mut GBCState) {
    op_BIT_reg(state, 4, Register::D);
    consume_cycles(state, 8);
}

// BIT 4, E
pub(super) fn instr_0xCB63(state: &mut GBCState) {
    op_BIT_reg(state, 4, Register::E);
    consume_cycles(state, 8);
}

// BIT 4, H
pub(super) fn instr_0xCB64(state: &mut GBCState) {
    op_BIT_reg(state, 4, Register::H);
    consume_cycles(state, 8);
}

// BIT 4, L
pub(super) fn instr_0xCB65(state: &mut GBCState) {
    op_BIT_reg(state, 4, Register::L);
    consume_cycles(state, 8);
}

// BIT 4, (HL)
pub(super) fn instr_0xCB66(state: &mut GBCState) {
    op_BIT_from_HLptr(state, 4);
    consume_cycles(state, 12);
}

// BIT 4, A
pub(super) fn instr_0xCB67(state: &mut GBCState) {
    op_BIT_reg(state, 4, Register::A);
    consume_cycles(state, 8);
}

// BIT 5, B
pub(super) fn instr_0xCB68(state: &mut GBCState) {
    op_BIT_reg(state, 5, Register::B);
    consume_cycles(state, 8);
}

// BIT 5, C
pub(super) fn instr_0xCB69(state: &mut GBCState) {
    op_BIT_reg(state, 5, Register::C);
    consume_cycles(state, 8);
}

// BIT 5, D
pub(super) fn instr_0xCB6A(state: &mut GBCState) {
    op_BIT_reg(state, 5, Register::D);
    consume_cycles(state, 8);
}

// BIT 5, E
pub(super) fn instr_0xCB6B(state: &mut GBCState) {
    op_BIT_reg(state, 5, Register::E);
    consume_cycles(state, 8);
}

// BIT 5, H
pub(super) fn instr_0xCB6C(state: &mut GBCState) {
    op_BIT_reg(state, 5, Register::H);
    consume_cycles(state, 8);
}

// BIT 5, L
pub(super) fn instr_0xCB6D(state: &mut GBCState) {
    op_BIT_reg(state, 5, Register::L);
    consume_cycles(state, 8);
}

// BIT 5, (HL)
pub(super) fn instr_0xCB6E(state: &mut GBCState) {
    op_BIT_from_HLptr(state, 5);
    consume_cycles(state, 12);
}

// BIT 5, A
pub(super) fn instr_0xCB6F(state: &mut GBCState) {
    op_BIT_reg(state, 5, Register::A);
    consume_cycles(state, 8);
}

// BIT 6, B
pub(super) fn instr_0xCB70(state: &mut GBCState) {
    op_BIT_reg(state, 6, Register::B);
    consume_cycles(state, 8);
}

// BIT 6, C
pub(super) fn instr_0xCB71(state: &mut GBCState) {
    op_BIT_reg(state, 6, Register::C);
    consume_cycles(state, 8);
}

// BIT 6, D
pub(super) fn instr_0xCB72(state: &mut GBCState) {
    op_BIT_reg(state, 6, Register::D);
    consume_cycles(state, 8);
}

// BIT 6, E
pub(super) fn instr_0xCB73(state: &mut GBCState) {
    op_BIT_reg(state, 6, Register::E);
    consume_cycles(state, 8);
}

// BIT 6, H
pub(super) fn instr_0xCB74(state: &mut GBCState) {
    op_BIT_reg(state, 6, Register::H);
    consume_cycles(state, 8);
}

// BIT 6, L
pub(super) fn instr_0xCB75(state: &mut GBCState) {
    op_BIT_reg(state, 6, Register::L);
    consume_cycles(state, 8);
}

// BIT 6, (HL)
pub(super) fn instr_0xCB76(state: &mut GBCState) {
    op_BIT_from_HLptr(state, 6);
    consume_cycles(state, 8);
}

// BIT 6, A
pub(super) fn instr_0xCB77(state: &mut GBCState) {
    op_BIT_reg(state, 6, Register::A);
    consume_cycles(state, 8);
}

// BIT 7, B
pub(super) fn instr_0xCB78(state: &mut GBCState) {
    op_BIT_reg(state, 7, Register::B);
    consume_cycles(state, 8);
}

// BIT 7, C
pub(super) fn instr_0xCB79(state: &mut GBCState) {
    op_BIT_reg(state, 7, Register::C);
    consume_cycles(state, 8);
}

// BIT 7, D
pub(super) fn instr_0xCB7A(state: &mut GBCState) {
    op_BIT_reg(state, 7, Register::D);
    consume_cycles(state, 8);
}

// BIT 7, E
pub(super) fn instr_0xCB7B(state: &mut GBCState) {
    op_BIT_reg(state, 7, Register::E);
    consume_cycles(state, 8);
}

// BIT 7, H
pub(super) fn instr_0xCB7C(state: &mut GBCState) {
    op_BIT_reg(state, 7, Register::H);
    consume_cycles(state, 8);
}

// BIT 7, L
pub(super) fn instr_0xCB7D(state: &mut GBCState) {
    op_BIT_reg(state, 7, Register::L);
    consume_cycles(state, 8);
}

// BIT 7, (HL)
pub(super) fn instr_0xCB7E(state: &mut GBCState) {
    op_BIT_from_HLptr(state, 7);
    consume_cycles(state, 12);
}

// BIT 7, A
pub(super) fn instr_0xCB7F(state: &mut GBCState) {
    op_BIT_reg(state, 7, Register::A);
    consume_cycles(state, 8);
}

// RES 0, B
pub(super) fn instr_0xCB80(state: &mut GBCState) {
    op_RES_reg(state, 0, Register::B);
    consume_cycles(state, 8);
}

// RES 0, C
pub(super) fn instr_0xCB81(state: &mut GBCState) {
    op_RES_reg(state, 0, Register::C);
    consume_cycles(state, 8);
}

// RES 0, D
pub(super) fn instr_0xCB82(state: &mut GBCState) {
    op_RES_reg(state, 0, Register::D);
    consume_cycles(state, 8);
}

// RES 0, E
pub(super) fn instr_0xCB83(state: &mut GBCState) {
    op_RES_reg(state, 0, Register::E);
    consume_cycles(state, 8);
}

// RES 0, H
pub(super) fn instr_0xCB84(state: &mut GBCState) {
    op_RES_reg(state, 0, Register::H);
    consume_cycles(state, 8);
}

// RES 0, L
pub(super) fn instr_0xCB85(state: &mut GBCState) {
    op_RES_reg(state, 0, Register::L);
    consume_cycles(state, 8);
}

// RES 0, (HL)
pub(super) fn instr_0xCB86(state: &mut GBCState) {
    op_RES_from_HLptr(state, 0);
    consume_cycles(state, 16);
}

// RES 0, A
pub(super) fn instr_0xCB87(state: &mut GBCState) {
    op_RES_reg(state, 0, Register::A);
    consume_cycles(state, 8);
}

// RES 1, B
pub(super) fn instr_0xCB88(state: &mut GBCState) {
    op_RES_reg(state, 1, Register::B);
    consume_cycles(state, 8);
}

// RES 1, C
pub(super) fn instr_0xCB89(state: &mut GBCState) {
    op_RES_reg(state, 1, Register::C);
    consume_cycles(state, 8);
}

// RES 1, D
pub(super) fn instr_0xCB8A(state: &mut GBCState) {
    op_RES_reg(state, 1, Register::D);
    consume_cycles(state, 8);
}

// RES 1, E
pub(super) fn instr_0xCB8B(state: &mut GBCState) {
    op_RES_reg(state, 1, Register::E);
    consume_cycles(state, 8);
}

// RES 1, H
pub(super) fn instr_0xCB8C(state: &mut GBCState) {
    op_RES_reg(state, 1, Register::H);
    consume_cycles(state, 8);
}

// RES 1, L
pub(super) fn instr_0xCB8D(state: &mut GBCState) {
    op_RES_reg(state, 1, Register::L);
    consume_cycles(state, 8);
}

// RES 1, (HL)
pub(super) fn instr_0xCB8E(state: &mut GBCState) {
    op_RES_from_HLptr(state, 1);
    consume_cycles(state, 16);
}

// RES 1, A
pub(super) fn instr_0xCB8F(state: &mut GBCState) {
    op_RES_reg(state, 1, Register::A);
    consume_cycles(state, 8);
}

// RES 2, B
pub(super) fn instr_0xCB90(state: &mut GBCState) {
    op_RES_reg(state, 2, Register::B);
    consume_cycles(state, 8);
}

// RES 2, C
pub(super) fn instr_0xCB91(state: &mut GBCState) {
    op_RES_reg(state, 2, Register::C);
    consume_cycles(state, 8);
}

// RES 2, D
pub(super) fn instr_0xCB92(state: &mut GBCState) {
    op_RES_reg(state, 2, Register::D);
    consume_cycles(state, 8);
}

// RES 2, E
pub(super) fn instr_0xCB93(state: &mut GBCState) {
    op_RES_reg(state, 2, Register::E);
    consume_cycles(state, 8);
}

// RES 2, H
pub(super) fn instr_0xCB94(state: &mut GBCState) {
    op_RES_reg(state, 2, Register::H);
    consume_cycles(state, 8);
}

// RES 2, L
pub(super) fn instr_0xCB95(state: &mut GBCState) {
    op_RES_reg(state, 2, Register::L);
    consume_cycles(state, 8);
}

// RES 2, (HL)
pub(super) fn instr_0xCB96(state: &mut GBCState) {
    op_RES_from_HLptr(state, 2);
    consume_cycles(state, 16);
}

// RES 2, A
pub(super) fn instr_0xCB97(state: &mut GBCState) {
    op_RES_reg(state, 2, Register::A);
    consume_cycles(state, 8);
}

// RES 3, B
pub(super) fn instr_0xCB98(state: &mut GBCState) {
    op_RES_reg(state, 3, Register::B);
    consume_cycles(state, 8);
}

// RES 3, C
pub(super) fn instr_0xCB99(state: &mut GBCState) {
    op_RES_reg(state, 3, Register::C);
    consume_cycles(state, 8);
}

// RES 3, D
pub(super) fn instr_0xCB9A(state: &mut GBCState) {
    op_RES_reg(state, 3, Register::D);
    consume_cycles(state, 8);
}

// RES 3, E
pub(super) fn instr_0xCB9B(state: &mut GBCState) {
    op_RES_reg(state, 3, Register::E);
    consume_cycles(state, 8);
}

// RES 3, H
pub(super) fn instr_0xCB9C(state: &mut GBCState) {
    op_RES_reg(state, 3, Register::H);
    consume_cycles(state, 8);
}

// RES 3, L
pub(super) fn instr_0xCB9D(state: &mut GBCState) {
    op_RES_reg(state, 3, Register::L);
    consume_cycles(state, 8);
}

// RES 3, (HL)
pub(super) fn instr_0xCB9E(state: &mut GBCState) {
    op_RES_from_HLptr(state, 3);
    consume_cycles(state, 16);
}

// RES 3, A
pub(super) fn instr_0xCB9F(state: &mut GBCState) {
    op_RES_reg(state, 3, Register::A);
    consume_cycles(state, 8);
}

// RES 4, B
pub(super) fn instr_0xCBA0(state: &mut GBCState) {
    op_RES_reg(state, 4, Register::B);
    consume_cycles(state, 8);
}

// RES 4, C
pub(super) fn instr_0xCBA1(state: &mut GBCState) {
    op_RES_reg(state, 4, Register::C);
    consume_cycles(state, 8);
}

// RES 4, D
pub(super) fn instr_0xCBA2(state: &mut GBCState) {
    op_RES_reg(state, 4, Register::D);
    consume_cycles(state, 8);
}

// RES 4, E
pub(super) fn instr_0xCBA3(state: &mut GBCState) {
    op_RES_reg(state, 4, Register::E);
    consume_cycles(state, 8);
}

// RES 4, H
pub(super) fn instr_0xCBA4(state: &mut GBCState) {
    op_RES_reg(state, 4, Register::H);
    consume_cycles(state, 8);
}

// RES 4, L
pub(super) fn instr_0xCBA5(state: &mut GBCState) {
    op_RES_reg(state, 4, Register::L);
    consume_cycles(state, 8);
}

// RES 4, (HL)
pub(super) fn instr_0xCBA6(state: &mut GBCState) {
    op_RES_from_HLptr(state, 4);
    consume_cycles(state, 16);
}

// RES 4, A
pub(super) fn instr_0xCBA7(state: &mut GBCState) {
    op_RES_reg(state, 4, Register::A);
    consume_cycles(state, 8);
}

// RES 5, B
pub(super) fn instr_0xCBA8(state: &mut GBCState) {
    op_RES_reg(state, 5, Register::B);
    consume_cycles(state, 8);
}

// RES 5, C
pub(super) fn instr_0xCBA9(state: &mut GBCState) {
    op_RES_reg(state, 5, Register::C);
    consume_cycles(state, 8);
}

// RES 5, D
pub(super) fn instr_0xCBAA(state: &mut GBCState) {
    op_RES_reg(state, 5, Register::D);
    consume_cycles(state, 8);
}

// RES 5, E
pub(super) fn instr_0xCBAB(state: &mut GBCState) {
    op_RES_reg(state, 5, Register::E);
    consume_cycles(state, 8);
}

// RES 5, H
pub(super) fn instr_0xCBAC(state: &mut GBCState) {
    op_RES_reg(state, 5, Register::H);
    consume_cycles(state, 8);
}

// RES 5, L
pub(super) fn instr_0xCBAD(state: &mut GBCState) {
    op_RES_reg(state, 5, Register::L);
    consume_cycles(state, 8);
}

// RES 5, (HL)
pub(super) fn instr_0xCBAE(state: &mut GBCState) {
    op_RES_from_HLptr(state, 5);
    consume_cycles(state, 16);
}

// RES 5, A
pub(super) fn instr_0xCBAF(state: &mut GBCState) {
    op_RES_reg(state, 5, Register::A);
    consume_cycles(state, 8);
}

// RES 6, B
pub(super) fn instr_0xCBB0(state: &mut GBCState) {
    op_RES_reg(state, 6, Register::B);
    consume_cycles(state, 8);
}

// RES 6, C
pub(super) fn instr_0xCBB1(state: &mut GBCState) {
    op_RES_reg(state, 6, Register::C);
    consume_cycles(state, 8);
}

// RES 6, D
pub(super) fn instr_0xCBB2(state: &mut GBCState) {
    op_RES_reg(state, 6, Register::D);
    consume_cycles(state, 8);
}

// RES 6, E
pub(super) fn instr_0xCBB3(state: &mut GBCState) {
    op_RES_reg(state, 6, Register::E);
    consume_cycles(state, 8);
}

// RES 6, H
pub(super) fn instr_0xCBB4(state: &mut GBCState) {
    op_RES_reg(state, 6, Register::H);
    consume_cycles(state, 8);
}

// RES 6, L
pub(super) fn instr_0xCBB5(state: &mut GBCState) {
    op_RES_reg(state, 6, Register::L);
    consume_cycles(state, 8);
}

// RES 6, (HL)
pub(super) fn instr_0xCBB6(state: &mut GBCState) {
    op_RES_from_HLptr(state, 6);
    consume_cycles(state, 16);
}

// RES 6, A
pub(super) fn instr_0xCBB7(state: &mut GBCState) {
    op_RES_reg(state, 6, Register::A);
    consume_cycles(state, 8);
}

// RES 7, B
pub(super) fn instr_0xCBB8(state: &mut GBCState) {
    op_RES_reg(state, 7, Register::B);
    consume_cycles(state, 8);
}

// RES 7, C
pub(super) fn instr_0xCBB9(state: &mut GBCState) {
    op_RES_reg(state, 7, Register::C);
    consume_cycles(state, 8);
}

// RES 7, D
pub(super) fn instr_0xCBBA(state: &mut GBCState) {
    op_RES_reg(state, 7, Register::D);
    consume_cycles(state, 8);
}

// RES 7, E
pub(super) fn instr_0xCBBB(state: &mut GBCState) {
    op_RES_reg(state, 7, Register::E);
    consume_cycles(state, 8);
}

// RES 7, H
pub(super) fn instr_0xCBBC(state: &mut GBCState) {
    op_RES_reg(state, 7, Register::H);
    consume_cycles(state, 8);
}

// RES 7, L
pub(super) fn instr_0xCBBD(state: &mut GBCState) {
    op_RES_reg(state, 7, Register::L);
    consume_cycles(state, 8);
}

// RES 7, (HL)
pub(super) fn instr_0xCBBE(state: &mut GBCState) {
    op_RES_from_HLptr(state, 7);
    consume_cycles(state, 16);
}

// RES 7, A
pub(super) fn instr_0xCBBF(state: &mut GBCState) {
    op_RES_reg(state, 7, Register::A);
    consume_cycles(state, 8);
}

// SET 0, B
pub(super) fn instr_0xCBC0(state: &mut GBCState) {
    op_SET_reg(state, 0, Register::B);
    consume_cycles(state, 8);
}

// SET 0, C
pub(super) fn instr_0xCBC1(state: &mut GBCState) {
    op_SET_reg(state, 0, Register::C);
    consume_cycles(state, 8);
}

// SET 0, D
pub(super) fn instr_0xCBC2(state: &mut GBCState) {
    op_SET_reg(state, 0, Register::D);
    consume_cycles(state, 8);
}

// SET 0, E
pub(super) fn instr_0xCBC3(state: &mut GBCState) {
    op_SET_reg(state, 0, Register::E);
    consume_cycles(state, 8);
}

// SET 0, H
pub(super) fn instr_0xCBC4(state: &mut GBCState) {
    op_SET_reg(state, 0, Register::H);
    consume_cycles(state, 8);
}

// SET 0, L
pub(super) fn instr_0xCBC5(state: &mut GBCState) {
    op_SET_reg(state, 0, Register::L);
    consume_cycles(state, 8);
}

// SET 0, (HL)
pub(super) fn instr_0xCBC6(state: &mut GBCState) {
    op_SET_from_HLptr(state, 0);
    consume_cycles(state, 16);
}

// SET 0, A
pub(super) fn instr_0xCBC7(state: &mut GBCState) {
    op_SET_reg(state, 0, Register::A);
    consume_cycles(state, 8);
}

// SET 1, B
pub(super) fn instr_0xCBC8(state: &mut GBCState) {
    op_SET_reg(state, 1, Register::B);
    consume_cycles(state, 8);
}

// SET 1, C
pub(super) fn instr_0xCBC9(state: &mut GBCState) {
    op_SET_reg(state, 1, Register::C);
    consume_cycles(state, 8);
}

// SET 1, D
pub(super) fn instr_0xCBCA(state: &mut GBCState) {
    op_SET_reg(state, 1, Register::D);
    consume_cycles(state, 8);
}

// SET 1, E
pub(super) fn instr_0xCBCB(state: &mut GBCState) {
    op_SET_reg(state, 1, Register::E);
    consume_cycles(state, 8);
}

// SET 1, H
pub(super) fn instr_0xCBCC(state: &mut GBCState) {
    op_SET_reg(state, 1, Register::H);
    consume_cycles(state, 8);
}

// SET 1, L
pub(super) fn instr_0xCBCD(state: &mut GBCState) {
    op_SET_reg(state, 1, Register::L);
    consume_cycles(state, 8);
}

// SET 1, (HL)
pub(super) fn instr_0xCBCE(state: &mut GBCState) {
    op_SET_from_HLptr(state, 1);
    consume_cycles(state, 16);
}

// SET 1, A
pub(super) fn instr_0xCBCF(state: &mut GBCState) {
    op_SET_reg(state, 1, Register::A);
    consume_cycles(state, 8);
}

// SET 2, B
pub(super) fn instr_0xCBD0(state: &mut GBCState) {
    op_SET_reg(state, 2, Register::B);
    consume_cycles(state, 8);
}

// SET 2, C
pub(super) fn instr_0xCBD1(state: &mut GBCState) {
    op_SET_reg(state, 2, Register::C);
    consume_cycles(state, 8);
}

// SET 2, D
pub(super) fn instr_0xCBD2(state: &mut GBCState) {
    op_SET_reg(state, 2, Register::D);
    consume_cycles(state, 8);
}

// SET 2, E
pub(super) fn instr_0xCBD3(state: &mut GBCState) {
    op_SET_reg(state, 2, Register::E);
    consume_cycles(state, 8);
}

// SET 2, H
pub(super) fn instr_0xCBD4(state: &mut GBCState) {
    op_SET_reg(state, 2, Register::H);
    consume_cycles(state, 8);
}

// SET 2, L
pub(super) fn instr_0xCBD5(state: &mut GBCState) {
    op_SET_reg(state, 2, Register::L);
    consume_cycles(state, 8);
}

// SET 2, (HL)
pub(super) fn instr_0xCBD6(state: &mut GBCState) {
    op_SET_from_HLptr(state, 2);
    consume_cycles(state, 16);
}

// SET 2, A
pub(super) fn instr_0xCBD7(state: &mut GBCState) {
    op_SET_reg(state, 2, Register::A);
    consume_cycles(state, 8);
}

// SET 3, B
pub(super) fn instr_0xCBD8(state: &mut GBCState) {
    op_SET_reg(state, 3, Register::B);
    consume_cycles(state, 8);
}

// SET 3, C
pub(super) fn instr_0xCBD9(state: &mut GBCState) {
    op_SET_reg(state, 3, Register::C);
    consume_cycles(state, 8);
}

// SET 3, D
pub(super) fn instr_0xCBDA(state: &mut GBCState) {
    op_SET_reg(state, 3, Register::D);
    consume_cycles(state, 8);
}

// SET 3, E
pub(super) fn instr_0xCBDB(state: &mut GBCState) {
    op_SET_reg(state, 3, Register::E);
    consume_cycles(state, 8);
}

// SET 3, H
pub(super) fn instr_0xCBDC(state: &mut GBCState) {
    op_SET_reg(state, 3, Register::H);
    consume_cycles(state, 8);
}

// SET 3, L
pub(super) fn instr_0xCBDD(state: &mut GBCState) {
    op_SET_reg(state, 3, Register::L);
    consume_cycles(state, 8);
}

// SET 3, (HL)
pub(super) fn instr_0xCBDE(state: &mut GBCState) {
    op_SET_from_HLptr(state, 3);
    consume_cycles(state, 16);
}

// SET 3, A
pub(super) fn instr_0xCBDF(state: &mut GBCState) {
    op_SET_reg(state, 3, Register::A);
    consume_cycles(state, 8);
}

// SET 4, B
pub(super) fn instr_0xCBE0(state: &mut GBCState) {
    op_SET_reg(state, 4, Register::B);
    consume_cycles(state, 8);
}

// SET 4, C
pub(super) fn instr_0xCBE1(state: &mut GBCState) {
    op_SET_reg(state, 4, Register::C);
    consume_cycles(state, 8);
}

// SET 4, D
pub(super) fn instr_0xCBE2(state: &mut GBCState) {
    op_SET_reg(state, 4, Register::D);
    consume_cycles(state, 8);
}

// SET 4, E
pub(super) fn instr_0xCBE3(state: &mut GBCState) {
    op_SET_reg(state, 4, Register::E);
    consume_cycles(state, 8);
}

// SET 4, H
pub(super) fn instr_0xCBE4(state: &mut GBCState) {
    op_SET_reg(state, 4, Register::H);
    consume_cycles(state, 8);
}

// SET 4, L
pub(super) fn instr_0xCBE5(state: &mut GBCState) {
    op_SET_reg(state, 4, Register::L);
    consume_cycles(state, 8);
}

// SET 4, (HL)
pub(super) fn instr_0xCBE6(state: &mut GBCState) {
    op_SET_from_HLptr(state, 4);
    consume_cycles(state, 16);
}

// SET 4, A
pub(super) fn instr_0xCBE7(state: &mut GBCState) {
    op_SET_reg(state, 4, Register::A);
    consume_cycles(state, 8);
}

// SET 5, B
pub(super) fn instr_0xCBE8(state: &mut GBCState) {
    op_SET_reg(state, 5, Register::B);
    consume_cycles(state, 8);
}

// SET 5, C
pub(super) fn instr_0xCBE9(state: &mut GBCState) {
    op_SET_reg(state, 5, Register::C);
    consume_cycles(state, 8);
}

// SET 5, D
pub(super) fn instr_0xCBEA(state: &mut GBCState) {
    op_SET_reg(state, 5, Register::D);
    consume_cycles(state, 8);
}

// SET 5, E
pub(super) fn instr_0xCBEB(state: &mut GBCState) {
    op_SET_reg(state, 5, Register::E);
    consume_cycles(state, 8);
}

// SET 5, H
pub(super) fn instr_0xCBEC(state: &mut GBCState) {
    op_SET_reg(state, 5, Register::H);
    consume_cycles(state, 8);
}

// SET 5, L
pub(super) fn instr_0xCBED(state: &mut GBCState) {
    op_SET_reg(state, 5, Register::L);
    consume_cycles(state, 8);
}

// SET 5, (HL)
pub(super) fn instr_0xCBEE(state: &mut GBCState) {
    op_SET_from_HLptr(state, 5);
    consume_cycles(state, 16);
}

// SET 5, A
pub(super) fn instr_0xCBEF(state: &mut GBCState) {
    op_SET_reg(state, 5, Register::A);
    consume_cycles(state, 8);
}

// SET 6, B
pub(super) fn instr_0xCBF0(state: &mut GBCState) {
    op_SET_reg(state, 6, Register::B);
    consume_cycles(state, 8);
}

// SET 6, C
pub(super) fn instr_0xCBF1(state: &mut GBCState) {
    op_SET_reg(state, 6, Register::C);
    consume_cycles(state, 8);
}

// SET 6, D
pub(super) fn instr_0xCBF2(state: &mut GBCState) {
    op_SET_reg(state, 6, Register::D);
    consume_cycles(state, 8);
}

// SET 6, E
pub(super) fn instr_0xCBF3(state: &mut GBCState) {
    op_SET_reg(state, 6, Register::E);
    consume_cycles(state, 8);
}

// SET 6, H
pub(super) fn instr_0xCBF4(state: &mut GBCState) {
    op_SET_reg(state, 6, Register::H);
    consume_cycles(state, 8);
}

// SET 6, L
pub(super) fn instr_0xCBF5(state: &mut GBCState) {
    op_SET_reg(state, 6, Register::L);
    consume_cycles(state, 8);
}

// SET 6, (HL)
pub(super) fn instr_0xCBF6(state: &mut GBCState) {
    op_SET_from_HLptr(state, 6);
    consume_cycles(state, 16);
}

// SET 6, A
pub(super) fn instr_0xCBF7(state: &mut GBCState) {
    op_SET_reg(state, 6, Register::A);
    consume_cycles(state, 8);
}

// SET 7, B
pub(super) fn instr_0xCBF8(state: &mut GBCState) {
    op_SET_reg(state, 7, Register::B);
    consume_cycles(state, 8);
}

// SET 7, C
pub(super) fn instr_0xCBF9(state: &mut GBCState) {
    op_SET_reg(state, 7, Register::C);
    consume_cycles(state, 8);
}

// SET 7, D
pub(super) fn instr_0xCBFA(state: &mut GBCState) {
    op_SET_reg(state, 7, Register::D);
    consume_cycles(state, 8);
}

// SET 7, E
pub(super) fn instr_0xCBFB(state: &mut GBCState) {
    op_SET_reg(state, 7, Register::E);
    consume_cycles(state, 8);
}

// SET 7, H
pub(super) fn instr_0xCBFC(state: &mut GBCState) {
    op_SET_reg(state, 7, Register::H);
    consume_cycles(state, 8);
}

// SET 7, L
pub(super) fn instr_0xCBFD(state: &mut GBCState) {
    op_SET_reg(state, 7, Register::L);
    consume_cycles(state, 8);
}

// SET 7, (HL)
pub(super) fn instr_0xCBFE(state: &mut GBCState) {
    op_SET_from_HLptr(state, 7);
    consume_cycles(state, 16);
}

// SET 7, A
pub(super) fn instr_0xCBFF(state: &mut GBCState) {
    op_SET_reg(state, 7, Register::A);
    consume_cycles(state, 8);
}
