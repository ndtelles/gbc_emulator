#![allow(non_snake_case)]

use crate::gbc::{virtual_memory, GBCState};

use super::{
    op_helpers::*,
    register::{Register, RegisterMapMethods, RegisterPair},
};

// RLC B
pub(super) fn instr_0xCB00(state: &mut GBCState) {
    op_RLC_reg(state, Register::B);
}

// RLC C
pub(super) fn instr_0xCB01(state: &mut GBCState) {
    op_RLC_reg(state, Register::C);
}

// RLC D
pub(super) fn instr_0xCB02(state: &mut GBCState) {
    op_RLC_reg(state, Register::D);
}

// RLC E
pub(super) fn instr_0xCB03(state: &mut GBCState) {
    op_RLC_reg(state, Register::E);
}

// RLC H
pub(super) fn instr_0xCB04(state: &mut GBCState) {
    op_RLC_reg(state, Register::H);
}

// RLC L
pub(super) fn instr_0xCB05(state: &mut GBCState) {
    op_RLC_reg(state, Register::L);
}

// RLC (HL)
pub(super) fn instr_0xCB06(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    let result = op_RLC(state, val);
    virtual_memory::write(state, addr, result);
}

// RLC A
pub(super) fn instr_0xCB07(state: &mut GBCState) {
    op_RLC_reg(state, Register::A);
}

// RRC B
pub(super) fn instr_0xCB08(state: &mut GBCState) {
    op_RRC_reg(state, Register::B);
}

// RRC C
pub(super) fn instr_0xCB09(state: &mut GBCState) {
    op_RRC_reg(state, Register::C);
}

// RRC D
pub(super) fn instr_0xCB0A(state: &mut GBCState) {
    op_RRC_reg(state, Register::D);
}

// RRC E
pub(super) fn instr_0xCB0B(state: &mut GBCState) {
    op_RRC_reg(state, Register::E);
}

// RRC H
pub(super) fn instr_0xCB0C(state: &mut GBCState) {
    op_RRC_reg(state, Register::H);
}

// RRC L
pub(super) fn instr_0xCB0D(state: &mut GBCState) {
    op_RRC_reg(state, Register::L);
}

// RRC (HL)
pub(super) fn instr_0xCB0E(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    let result = op_RRC(state, val);
    virtual_memory::write(state, addr, result);
}

// RRC A
pub(super) fn instr_0xCB0F(state: &mut GBCState) {
    op_RRC_reg(state, Register::A);
}

// RL B
pub(super) fn instr_0xCB10(state: &mut GBCState) {
    op_RL_reg(state, Register::B);
}

// RL C
pub(super) fn instr_0xCB11(state: &mut GBCState) {
    op_RL_reg(state, Register::C);
}

// RL D
pub(super) fn instr_0xCB12(state: &mut GBCState) {
    op_RL_reg(state, Register::D);
}

// RL E
pub(super) fn instr_0xCB13(state: &mut GBCState) {
    op_RL_reg(state, Register::E);
}

// RL H
pub(super) fn instr_0xCB14(state: &mut GBCState) {
    op_RL_reg(state, Register::H);
}

// RL L
pub(super) fn instr_0xCB15(state: &mut GBCState) {
    op_RL_reg(state, Register::L);
}

// RL (HL)
pub(super) fn instr_0xCB16(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    let result = op_RL(state, val);
    virtual_memory::write(state, addr, result);
}

// RL A
pub(super) fn instr_0xCB17(state: &mut GBCState) {
    op_RL_reg(state, Register::A);
}

// RR B
pub(super) fn instr_0xCB18(state: &mut GBCState) {
    op_RR_reg(state, Register::B);
}

// RR C
pub(super) fn instr_0xCB19(state: &mut GBCState) {
    op_RR_reg(state, Register::C);
}

// RR D
pub(super) fn instr_0xCB1A(state: &mut GBCState) {
    op_RR_reg(state, Register::D);
}

// RR E
pub(super) fn instr_0xCB1B(state: &mut GBCState) {
    op_RR_reg(state, Register::E);
}

// RR H
pub(super) fn instr_0xCB1C(state: &mut GBCState) {
    op_RR_reg(state, Register::H);
}

// RR L
pub(super) fn instr_0xCB1D(state: &mut GBCState) {
    op_RR_reg(state, Register::L);
}

// RR (HL)
pub(super) fn instr_0xCB1E(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    let result = op_RR(state, val);
    virtual_memory::write(state, addr, result);
}

// RR A
pub(super) fn instr_0xCB1F(state: &mut GBCState) {
    op_RR_reg(state, Register::A);
}

// SLA B
pub(super) fn instr_0xCB20(state: &mut GBCState) {
    op_SLA_reg(state, Register::B);
}

// SLA C
pub(super) fn instr_0xCB21(state: &mut GBCState) {
    op_SLA_reg(state, Register::C);
}

// SLA D
pub(super) fn instr_0xCB22(state: &mut GBCState) {
    op_SLA_reg(state, Register::D);
}

// SLA E
pub(super) fn instr_0xCB23(state: &mut GBCState) {
    op_SLA_reg(state, Register::E);
}

// SLA H
pub(super) fn instr_0xCB24(state: &mut GBCState) {
    op_SLA_reg(state, Register::H);
}

// SLA L
pub(super) fn instr_0xCB25(state: &mut GBCState) {
    op_SLA_reg(state, Register::L);
}

// SLA (HL)
pub(super) fn instr_0xCB26(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    let result = op_SLA(state, val);
    virtual_memory::write(state, addr, result);
}

// SLA A
pub(super) fn instr_0xCB27(state: &mut GBCState) {
    op_SLA_reg(state, Register::A);
}

// SRA B
pub(super) fn instr_0xCB28(state: &mut GBCState) {
    op_SRA_reg(state, Register::B);
}

// SRA C
pub(super) fn instr_0xCB29(state: &mut GBCState) {
    op_SRA_reg(state, Register::C);
}

// SRA D
pub(super) fn instr_0xCB2A(state: &mut GBCState) {
    op_SRA_reg(state, Register::D);
}

// SRA E
pub(super) fn instr_0xCB2B(state: &mut GBCState) {
    op_SRA_reg(state, Register::E);
}

// SRA H
pub(super) fn instr_0xCB2C(state: &mut GBCState) {
    op_SRA_reg(state, Register::H);
}

// SRA L
pub(super) fn instr_0xCB2D(state: &mut GBCState) {
    op_SRA_reg(state, Register::L);
}

// SRA (HL)
pub(super) fn instr_0xCB2E(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    let result = op_SRA(state, val);
    virtual_memory::write(state, addr, result);
}

// SRA A
pub(super) fn instr_0xCB2F(state: &mut GBCState) {
    op_SRA_reg(state, Register::A);
}

// SWAP B
pub(super) fn instr_0xCB30(state: &mut GBCState) {
    op_SWAP_reg(state, Register::B);
}

// SWAP C
pub(super) fn instr_0xCB31(state: &mut GBCState) {
    op_SWAP_reg(state, Register::C);
}

// SWAP D
pub(super) fn instr_0xCB32(state: &mut GBCState) {
    op_SWAP_reg(state, Register::D);
}

// SWAP E
pub(super) fn instr_0xCB33(state: &mut GBCState) {
    op_SWAP_reg(state, Register::E);
}

// SWAP H
pub(super) fn instr_0xCB34(state: &mut GBCState) {
    op_SWAP_reg(state, Register::H);
}

// SWAP L
pub(super) fn instr_0xCB35(state: &mut GBCState) {
    op_SWAP_reg(state, Register::L);
}

// SWAP (HL)
pub(super) fn instr_0xCB36(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    let result = op_SWAP(state, val);
    virtual_memory::write(state, addr, result);
}

// SWAP A
pub(super) fn instr_0xCB37(state: &mut GBCState) {
    op_SWAP_reg(state, Register::A);
}

// SRL B
pub(super) fn instr_0xCB38(state: &mut GBCState) {
    op_SRL_reg(state, Register::B);
}

// SRL C
pub(super) fn instr_0xCB39(state: &mut GBCState) {
    op_SRL_reg(state, Register::C);
}

// SRL D
pub(super) fn instr_0xCB3A(state: &mut GBCState) {
    op_SRL_reg(state, Register::D);
}

// SRL E
pub(super) fn instr_0xCB3B(state: &mut GBCState) {
    op_SRL_reg(state, Register::E);
}

// SRL H
pub(super) fn instr_0xCB3C(state: &mut GBCState) {
    op_SRL_reg(state, Register::H);
}

// SRL L
pub(super) fn instr_0xCB3D(state: &mut GBCState) {
    op_SRL_reg(state, Register::L);
}

// SRL (HL)
pub(super) fn instr_0xCB3E(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    let result = op_SRL(state, val);
    virtual_memory::write(state, addr, result);
}

// SRL A
pub(super) fn instr_0xCB3F(state: &mut GBCState) {
    op_SRL_reg(state, Register::A);
}

// BIT 0, B
pub(super) fn instr_0xCB40(state: &mut GBCState) {
    op_BIT_reg(state, 0, Register::B);
}

// BIT 0, C
pub(super) fn instr_0xCB41(state: &mut GBCState) {
    op_BIT_reg(state, 0, Register::C);
}

// BIT 0, D
pub(super) fn instr_0xCB42(state: &mut GBCState) {
    op_BIT_reg(state, 0, Register::D);
}

// BIT 0, E
pub(super) fn instr_0xCB43(state: &mut GBCState) {
    op_BIT_reg(state, 0, Register::E);
}

// BIT 0, H
pub(super) fn instr_0xCB44(state: &mut GBCState) {
    op_BIT_reg(state, 0, Register::H);
}

// BIT 0, L
pub(super) fn instr_0xCB45(state: &mut GBCState) {
    op_BIT_reg(state, 0, Register::L);
}

// BIT 0, (HL)
pub(super) fn instr_0xCB46(state: &mut GBCState) {
    op_BIT_from_HLptr(state, 0);
}

// BIT 0, A
pub(super) fn instr_0xCB47(state: &mut GBCState) {
    op_BIT_reg(state, 0, Register::A);
}

// BIT 1, B
pub(super) fn instr_0xCB48(state: &mut GBCState) {
    op_BIT_reg(state, 1, Register::B);
}

// BIT 1, C
pub(super) fn instr_0xCB49(state: &mut GBCState) {
    op_BIT_reg(state, 1, Register::C);
}

// BIT 1, D
pub(super) fn instr_0xCB4A(state: &mut GBCState) {
    op_BIT_reg(state, 1, Register::D);
}

// BIT 1, E
pub(super) fn instr_0xCB4B(state: &mut GBCState) {
    op_BIT_reg(state, 1, Register::E);
}

// BIT 1, H
pub(super) fn instr_0xCB4C(state: &mut GBCState) {
    op_BIT_reg(state, 1, Register::H);
}

// BIT 1, L
pub(super) fn instr_0xCB4D(state: &mut GBCState) {
    op_BIT_reg(state, 1, Register::L);
}

// BIT 1, (HL)
pub(super) fn instr_0xCB4E(state: &mut GBCState) {
    op_BIT_from_HLptr(state, 1);
}

// BIT 1, A
pub(super) fn instr_0xCB4F(state: &mut GBCState) {
    op_BIT_reg(state, 1, Register::A);
}

// BIT 2, B
pub(super) fn instr_0xCB50(state: &mut GBCState) {
    op_BIT_reg(state, 2, Register::B);
}

// BIT 2, C
pub(super) fn instr_0xCB51(state: &mut GBCState) {
    op_BIT_reg(state, 2, Register::C);
}

// BIT 2, D
pub(super) fn instr_0xCB52(state: &mut GBCState) {
    op_BIT_reg(state, 2, Register::D);
}

// BIT 2, E
pub(super) fn instr_0xCB53(state: &mut GBCState) {
    op_BIT_reg(state, 2, Register::E);
}

// BIT 2, H
pub(super) fn instr_0xCB54(state: &mut GBCState) {
    op_BIT_reg(state, 2, Register::H);
}

// BIT 2, L
pub(super) fn instr_0xCB55(state: &mut GBCState) {
    op_BIT_reg(state, 2, Register::L);
}

// BIT 2, (HL)
pub(super) fn instr_0xCB56(state: &mut GBCState) {
    op_BIT_from_HLptr(state, 2);
}

// BIT 2, A
pub(super) fn instr_0xCB57(state: &mut GBCState) {
    op_BIT_reg(state, 2, Register::A);
}

// BIT 3, B
pub(super) fn instr_0xCB58(state: &mut GBCState) {
    op_BIT_reg(state, 3, Register::B);
}

// BIT 3, C
pub(super) fn instr_0xCB59(state: &mut GBCState) {
    op_BIT_reg(state, 3, Register::C);
}

// BIT 3, D
pub(super) fn instr_0xCB5A(state: &mut GBCState) {
    op_BIT_reg(state, 3, Register::D);
}

// BIT 3, E
pub(super) fn instr_0xCB5B(state: &mut GBCState) {
    op_BIT_reg(state, 3, Register::E);
}

// BIT 3, H
pub(super) fn instr_0xCB5C(state: &mut GBCState) {
    op_BIT_reg(state, 3, Register::H);
}

// BIT 3, L
pub(super) fn instr_0xCB5D(state: &mut GBCState) {
    op_BIT_reg(state, 3, Register::L);
}

// BIT 3, (HL)
pub(super) fn instr_0xCB5E(state: &mut GBCState) {
    op_BIT_from_HLptr(state, 3);
}

// BIT 3, A
pub(super) fn instr_0xCB5F(state: &mut GBCState) {
    op_BIT_reg(state, 3, Register::A);
}

// BIT 4, B
pub(super) fn instr_0xCB60(state: &mut GBCState) {
    op_BIT_reg(state, 4, Register::B);
}

// BIT 4, C
pub(super) fn instr_0xCB61(state: &mut GBCState) {
    op_BIT_reg(state, 4, Register::C);
}

// BIT 4, D
pub(super) fn instr_0xCB62(state: &mut GBCState) {
    op_BIT_reg(state, 4, Register::D);
}

// BIT 4, E
pub(super) fn instr_0xCB63(state: &mut GBCState) {
    op_BIT_reg(state, 4, Register::E);
}

// BIT 4, H
pub(super) fn instr_0xCB64(state: &mut GBCState) {
    op_BIT_reg(state, 4, Register::H);
}

// BIT 4, L
pub(super) fn instr_0xCB65(state: &mut GBCState) {
    op_BIT_reg(state, 4, Register::L);
}

// BIT 4, (HL)
pub(super) fn instr_0xCB66(state: &mut GBCState) {
    op_BIT_from_HLptr(state, 4);
}

// BIT 4, A
pub(super) fn instr_0xCB67(state: &mut GBCState) {
    op_BIT_reg(state, 4, Register::A);
}

// BIT 5, B
pub(super) fn instr_0xCB68(state: &mut GBCState) {
    op_BIT_reg(state, 5, Register::B);
}

// BIT 5, C
pub(super) fn instr_0xCB69(state: &mut GBCState) {
    op_BIT_reg(state, 5, Register::C);
}

// BIT 5, D
pub(super) fn instr_0xCB6A(state: &mut GBCState) {
    op_BIT_reg(state, 5, Register::D);
}

// BIT 5, E
pub(super) fn instr_0xCB6B(state: &mut GBCState) {
    op_BIT_reg(state, 5, Register::E);
}

// BIT 5, H
pub(super) fn instr_0xCB6C(state: &mut GBCState) {
    op_BIT_reg(state, 5, Register::H);
}

// BIT 5, L
pub(super) fn instr_0xCB6D(state: &mut GBCState) {
    op_BIT_reg(state, 5, Register::L);
}

// BIT 5, (HL)
pub(super) fn instr_0xCB6E(state: &mut GBCState) {
    op_BIT_from_HLptr(state, 5);
}

// BIT 5, A
pub(super) fn instr_0xCB6F(state: &mut GBCState) {
    op_BIT_reg(state, 5, Register::A);
}

// BIT 6, B
pub(super) fn instr_0xCB70(state: &mut GBCState) {
    op_BIT_reg(state, 6, Register::B);
}

// BIT 6, C
pub(super) fn instr_0xCB71(state: &mut GBCState) {
    op_BIT_reg(state, 6, Register::C);
}

// BIT 6, D
pub(super) fn instr_0xCB72(state: &mut GBCState) {
    op_BIT_reg(state, 6, Register::D);
}

// BIT 6, E
pub(super) fn instr_0xCB73(state: &mut GBCState) {
    op_BIT_reg(state, 6, Register::E);
}

// BIT 6, H
pub(super) fn instr_0xCB74(state: &mut GBCState) {
    op_BIT_reg(state, 6, Register::H);
}

// BIT 6, L
pub(super) fn instr_0xCB75(state: &mut GBCState) {
    op_BIT_reg(state, 6, Register::L);
}

// BIT 6, (HL)
pub(super) fn instr_0xCB76(state: &mut GBCState) {
    op_BIT_from_HLptr(state, 6);
}

// BIT 6, A
pub(super) fn instr_0xCB77(state: &mut GBCState) {
    op_BIT_reg(state, 6, Register::A);
}

// BIT 7, B
pub(super) fn instr_0xCB78(state: &mut GBCState) {
    op_BIT_reg(state, 7, Register::B);
}

// BIT 7, C
pub(super) fn instr_0xCB79(state: &mut GBCState) {
    op_BIT_reg(state, 7, Register::C);
}

// BIT 7, D
pub(super) fn instr_0xCB7A(state: &mut GBCState) {
    op_BIT_reg(state, 7, Register::D);
}

// BIT 7, E
pub(super) fn instr_0xCB7B(state: &mut GBCState) {
    op_BIT_reg(state, 7, Register::E);
}

// BIT 7, H
pub(super) fn instr_0xCB7C(state: &mut GBCState) {
    op_BIT_reg(state, 7, Register::H);
}

// BIT 7, L
pub(super) fn instr_0xCB7D(state: &mut GBCState) {
    op_BIT_reg(state, 7, Register::L);
}

// BIT 7, (HL)
pub(super) fn instr_0xCB7E(state: &mut GBCState) {
    op_BIT_from_HLptr(state, 7);
}

// BIT 7, A
pub(super) fn instr_0xCB7F(state: &mut GBCState) {
    op_BIT_reg(state, 7, Register::A);
}

// RES 0, B
pub(super) fn instr_0xCB80(state: &mut GBCState) {
    op_RES_reg(state, 0, Register::B);
}

// RES 0, C
pub(super) fn instr_0xCB81(state: &mut GBCState) {
    op_RES_reg(state, 0, Register::C);
}

// RES 0, D
pub(super) fn instr_0xCB82(state: &mut GBCState) {
    op_RES_reg(state, 0, Register::D);
}

// RES 0, E
pub(super) fn instr_0xCB83(state: &mut GBCState) {
    op_RES_reg(state, 0, Register::E);
}

// RES 0, H
pub(super) fn instr_0xCB84(state: &mut GBCState) {
    op_RES_reg(state, 0, Register::H);
}

// RES 0, L
pub(super) fn instr_0xCB85(state: &mut GBCState) {
    op_RES_reg(state, 0, Register::L);
}

// RES 0, (HL)
pub(super) fn instr_0xCB86(state: &mut GBCState) {
    op_RES_from_HLptr(state, 0);
}

// RES 0, A
pub(super) fn instr_0xCB87(state: &mut GBCState) {
    op_RES_reg(state, 0, Register::A);
}

// RES 1, B
pub(super) fn instr_0xCB88(state: &mut GBCState) {
    op_RES_reg(state, 1, Register::B);
}

// RES 1, C
pub(super) fn instr_0xCB89(state: &mut GBCState) {
    op_RES_reg(state, 1, Register::C);
}

// RES 1, D
pub(super) fn instr_0xCB8A(state: &mut GBCState) {
    op_RES_reg(state, 1, Register::D);
}

// RES 1, E
pub(super) fn instr_0xCB8B(state: &mut GBCState) {
    op_RES_reg(state, 1, Register::E);
}

// RES 1, H
pub(super) fn instr_0xCB8C(state: &mut GBCState) {
    op_RES_reg(state, 1, Register::H);
}

// RES 1, L
pub(super) fn instr_0xCB8D(state: &mut GBCState) {
    op_RES_reg(state, 1, Register::L);
}

// RES 1, (HL)
pub(super) fn instr_0xCB8E(state: &mut GBCState) {
    op_RES_from_HLptr(state, 1);
}

// RES 1, A
pub(super) fn instr_0xCB8F(state: &mut GBCState) {
    op_RES_reg(state, 1, Register::A);
}

// RES 2, B
pub(super) fn instr_0xCB90(state: &mut GBCState) {
    op_RES_reg(state, 2, Register::B);
}

// RES 2, C
pub(super) fn instr_0xCB91(state: &mut GBCState) {
    op_RES_reg(state, 2, Register::C);
}

// RES 2, D
pub(super) fn instr_0xCB92(state: &mut GBCState) {
    op_RES_reg(state, 2, Register::D);
}

// RES 2, E
pub(super) fn instr_0xCB93(state: &mut GBCState) {
    op_RES_reg(state, 2, Register::E);
}

// RES 2, H
pub(super) fn instr_0xCB94(state: &mut GBCState) {
    op_RES_reg(state, 2, Register::H);
}

// RES 2, L
pub(super) fn instr_0xCB95(state: &mut GBCState) {
    op_RES_reg(state, 2, Register::L);
}

// RES 2, (HL)
pub(super) fn instr_0xCB96(state: &mut GBCState) {
    op_RES_from_HLptr(state, 2);
}

// RES 2, A
pub(super) fn instr_0xCB97(state: &mut GBCState) {
    op_RES_reg(state, 2, Register::A);
}

// RES 3, B
pub(super) fn instr_0xCB98(state: &mut GBCState) {
    op_RES_reg(state, 3, Register::B);
}

// RES 3, C
pub(super) fn instr_0xCB99(state: &mut GBCState) {
    op_RES_reg(state, 3, Register::C);
}

// RES 3, D
pub(super) fn instr_0xCB9A(state: &mut GBCState) {
    op_RES_reg(state, 3, Register::D);
}

// RES 3, E
pub(super) fn instr_0xCB9B(state: &mut GBCState) {
    op_RES_reg(state, 3, Register::E);
}

// RES 3, H
pub(super) fn instr_0xCB9C(state: &mut GBCState) {
    op_RES_reg(state, 3, Register::H);
}

// RES 3, L
pub(super) fn instr_0xCB9D(state: &mut GBCState) {
    op_RES_reg(state, 3, Register::L);
}

// RES 3, (HL)
pub(super) fn instr_0xCB9E(state: &mut GBCState) {
    op_RES_from_HLptr(state, 3);
}

// RES 3, A
pub(super) fn instr_0xCB9F(state: &mut GBCState) {
    op_RES_reg(state, 3, Register::A);
}

// RES 4, B
pub(super) fn instr_0xCBA0(state: &mut GBCState) {
    op_RES_reg(state, 4, Register::B);
}

// RES 4, C
pub(super) fn instr_0xCBA1(state: &mut GBCState) {
    op_RES_reg(state, 4, Register::C);
}

// RES 4, D
pub(super) fn instr_0xCBA2(state: &mut GBCState) {
    op_RES_reg(state, 4, Register::D);
}

// RES 4, E
pub(super) fn instr_0xCBA3(state: &mut GBCState) {
    op_RES_reg(state, 4, Register::E);
}

// RES 4, H
pub(super) fn instr_0xCBA4(state: &mut GBCState) {
    op_RES_reg(state, 4, Register::H);
}

// RES 4, L
pub(super) fn instr_0xCBA5(state: &mut GBCState) {
    op_RES_reg(state, 4, Register::L);
}

// RES 4, (HL)
pub(super) fn instr_0xCBA6(state: &mut GBCState) {
    op_RES_from_HLptr(state, 4);
}

// RES 4, A
pub(super) fn instr_0xCBA7(state: &mut GBCState) {
    op_RES_reg(state, 4, Register::A);
}

// RES 5, B
pub(super) fn instr_0xCBA8(state: &mut GBCState) {
    op_RES_reg(state, 5, Register::B);
}

// RES 5, C
pub(super) fn instr_0xCBA9(state: &mut GBCState) {
    op_RES_reg(state, 5, Register::C);
}

// RES 5, D
pub(super) fn instr_0xCBAA(state: &mut GBCState) {
    op_RES_reg(state, 5, Register::D);
}

// RES 5, E
pub(super) fn instr_0xCBAB(state: &mut GBCState) {
    op_RES_reg(state, 5, Register::E);
}

// RES 5, H
pub(super) fn instr_0xCBAC(state: &mut GBCState) {
    op_RES_reg(state, 5, Register::H);
}

// RES 5, L
pub(super) fn instr_0xCBAD(state: &mut GBCState) {
    op_RES_reg(state, 5, Register::L);
}

// RES 5, (HL)
pub(super) fn instr_0xCBAE(state: &mut GBCState) {
    op_RES_from_HLptr(state, 5);
}

// RES 5, A
pub(super) fn instr_0xCBAF(state: &mut GBCState) {
    op_RES_reg(state, 5, Register::A);
}

// RES 6, B
pub(super) fn instr_0xCBB0(state: &mut GBCState) {
    op_RES_reg(state, 6, Register::B);
}

// RES 6, C
pub(super) fn instr_0xCBB1(state: &mut GBCState) {
    op_RES_reg(state, 6, Register::C);
}

// RES 6, D
pub(super) fn instr_0xCBB2(state: &mut GBCState) {
    op_RES_reg(state, 6, Register::D);
}

// RES 6, E
pub(super) fn instr_0xCBB3(state: &mut GBCState) {
    op_RES_reg(state, 6, Register::E);
}

// RES 6, H
pub(super) fn instr_0xCBB4(state: &mut GBCState) {
    op_RES_reg(state, 6, Register::H);
}

// RES 6, L
pub(super) fn instr_0xCBB5(state: &mut GBCState) {
    op_RES_reg(state, 6, Register::L);
}

// RES 6, (HL)
pub(super) fn instr_0xCBB6(state: &mut GBCState) {
    op_RES_from_HLptr(state, 6);
}

// RES 6, A
pub(super) fn instr_0xCBB7(state: &mut GBCState) {
    op_RES_reg(state, 6, Register::A);
}

// RES 7, B
pub(super) fn instr_0xCBB8(state: &mut GBCState) {
    op_RES_reg(state, 7, Register::B);
}

// RES 7, C
pub(super) fn instr_0xCBB9(state: &mut GBCState) {
    op_RES_reg(state, 7, Register::C);
}

// RES 7, D
pub(super) fn instr_0xCBBA(state: &mut GBCState) {
    op_RES_reg(state, 7, Register::D);
}

// RES 7, E
pub(super) fn instr_0xCBBB(state: &mut GBCState) {
    op_RES_reg(state, 7, Register::E);
}

// RES 7, H
pub(super) fn instr_0xCBBC(state: &mut GBCState) {
    op_RES_reg(state, 7, Register::H);
}

// RES 7, L
pub(super) fn instr_0xCBBD(state: &mut GBCState) {
    op_RES_reg(state, 7, Register::L);
}

// RES 7, (HL)
pub(super) fn instr_0xCBBE(state: &mut GBCState) {
    op_RES_from_HLptr(state, 7);
}

// RES 7, A
pub(super) fn instr_0xCBBF(state: &mut GBCState) {
    op_RES_reg(state, 7, Register::A);
}

// SET 0, B
pub(super) fn instr_0xCBC0(state: &mut GBCState) {
    op_SET_reg(state, 0, Register::B);
}

// SET 0, C
pub(super) fn instr_0xCBC1(state: &mut GBCState) {
    op_SET_reg(state, 0, Register::C);
}

// SET 0, D
pub(super) fn instr_0xCBC2(state: &mut GBCState) {
    op_SET_reg(state, 0, Register::D);
}

// SET 0, E
pub(super) fn instr_0xCBC3(state: &mut GBCState) {
    op_SET_reg(state, 0, Register::E);
}

// SET 0, H
pub(super) fn instr_0xCBC4(state: &mut GBCState) {
    op_SET_reg(state, 0, Register::H);
}

// SET 0, L
pub(super) fn instr_0xCBC5(state: &mut GBCState) {
    op_SET_reg(state, 0, Register::L);
}

// SET 0, (HL)
pub(super) fn instr_0xCBC6(state: &mut GBCState) {
    op_SET_from_HLptr(state, 0);
}

// SET 0, A
pub(super) fn instr_0xCBC7(state: &mut GBCState) {
    op_SET_reg(state, 0, Register::A);
}

// SET 1, B
pub(super) fn instr_0xCBC8(state: &mut GBCState) {
    op_SET_reg(state, 1, Register::B);
}

// SET 1, C
pub(super) fn instr_0xCBC9(state: &mut GBCState) {
    op_SET_reg(state, 1, Register::C);
}

// SET 1, D
pub(super) fn instr_0xCBCA(state: &mut GBCState) {
    op_SET_reg(state, 1, Register::D);
}

// SET 1, E
pub(super) fn instr_0xCBCB(state: &mut GBCState) {
    op_SET_reg(state, 1, Register::E);
}

// SET 1, H
pub(super) fn instr_0xCBCC(state: &mut GBCState) {
    op_SET_reg(state, 1, Register::H);
}

// SET 1, L
pub(super) fn instr_0xCBCD(state: &mut GBCState) {
    op_SET_reg(state, 1, Register::L);
}

// SET 1, (HL)
pub(super) fn instr_0xCBCE(state: &mut GBCState) {
    op_SET_from_HLptr(state, 1);
}

// SET 1, A
pub(super) fn instr_0xCBCF(state: &mut GBCState) {
    op_SET_reg(state, 1, Register::A);
}

// SET 2, B
pub(super) fn instr_0xCBD0(state: &mut GBCState) {
    op_SET_reg(state, 2, Register::B);
}

// SET 2, C
pub(super) fn instr_0xCBD1(state: &mut GBCState) {
    op_SET_reg(state, 2, Register::C);
}

// SET 2, D
pub(super) fn instr_0xCBD2(state: &mut GBCState) {
    op_SET_reg(state, 2, Register::D);
}

// SET 2, E
pub(super) fn instr_0xCBD3(state: &mut GBCState) {
    op_SET_reg(state, 2, Register::E);
}

// SET 2, H
pub(super) fn instr_0xCBD4(state: &mut GBCState) {
    op_SET_reg(state, 2, Register::H);
}

// SET 2, L
pub(super) fn instr_0xCBD5(state: &mut GBCState) {
    op_SET_reg(state, 2, Register::L);
}

// SET 2, (HL)
pub(super) fn instr_0xCBD6(state: &mut GBCState) {
    op_SET_from_HLptr(state, 2);
}

// SET 2, A
pub(super) fn instr_0xCBD7(state: &mut GBCState) {
    op_SET_reg(state, 2, Register::A);
}

// SET 3, B
pub(super) fn instr_0xCBD8(state: &mut GBCState) {
    op_SET_reg(state, 3, Register::B);
}

// SET 3, C
pub(super) fn instr_0xCBD9(state: &mut GBCState) {
    op_SET_reg(state, 3, Register::C);
}

// SET 3, D
pub(super) fn instr_0xCBDA(state: &mut GBCState) {
    op_SET_reg(state, 3, Register::D);
}

// SET 3, E
pub(super) fn instr_0xCBDB(state: &mut GBCState) {
    op_SET_reg(state, 3, Register::E);
}

// SET 3, H
pub(super) fn instr_0xCBDC(state: &mut GBCState) {
    op_SET_reg(state, 3, Register::H);
}

// SET 3, L
pub(super) fn instr_0xCBDD(state: &mut GBCState) {
    op_SET_reg(state, 3, Register::L);
}

// SET 3, (HL)
pub(super) fn instr_0xCBDE(state: &mut GBCState) {
    op_SET_from_HLptr(state, 3);
}

// SET 3, A
pub(super) fn instr_0xCBDF(state: &mut GBCState) {
    op_SET_reg(state, 3, Register::A);
}

// SET 4, B
pub(super) fn instr_0xCBE0(state: &mut GBCState) {
    op_SET_reg(state, 4, Register::B);
}

// SET 4, C
pub(super) fn instr_0xCBE1(state: &mut GBCState) {
    op_SET_reg(state, 4, Register::C);
}

// SET 4, D
pub(super) fn instr_0xCBE2(state: &mut GBCState) {
    op_SET_reg(state, 4, Register::D);
}

// SET 4, E
pub(super) fn instr_0xCBE3(state: &mut GBCState) {
    op_SET_reg(state, 4, Register::E);
}

// SET 4, H
pub(super) fn instr_0xCBE4(state: &mut GBCState) {
    op_SET_reg(state, 4, Register::H);
}

// SET 4, L
pub(super) fn instr_0xCBE5(state: &mut GBCState) {
    op_SET_reg(state, 4, Register::L);
}

// SET 4, (HL)
pub(super) fn instr_0xCBE6(state: &mut GBCState) {
    op_SET_from_HLptr(state, 4);
}

// SET 4, A
pub(super) fn instr_0xCBE7(state: &mut GBCState) {
    op_SET_reg(state, 4, Register::A);
}

// SET 5, B
pub(super) fn instr_0xCBE8(state: &mut GBCState) {
    op_SET_reg(state, 5, Register::B);
}

// SET 5, C
pub(super) fn instr_0xCBE9(state: &mut GBCState) {
    op_SET_reg(state, 5, Register::C);
}

// SET 5, D
pub(super) fn instr_0xCBEA(state: &mut GBCState) {
    op_SET_reg(state, 5, Register::D);
}

// SET 5, E
pub(super) fn instr_0xCBEB(state: &mut GBCState) {
    op_SET_reg(state, 5, Register::E);
}

// SET 5, H
pub(super) fn instr_0xCBEC(state: &mut GBCState) {
    op_SET_reg(state, 5, Register::H);
}

// SET 5, L
pub(super) fn instr_0xCBED(state: &mut GBCState) {
    op_SET_reg(state, 5, Register::L);
}

// SET 5, (HL)
pub(super) fn instr_0xCBEE(state: &mut GBCState) {
    op_SET_from_HLptr(state, 5);
}

// SET 5, A
pub(super) fn instr_0xCBEF(state: &mut GBCState) {
    op_SET_reg(state, 5, Register::A);
}

// SET 6, B
pub(super) fn instr_0xCBF0(state: &mut GBCState) {
    op_SET_reg(state, 6, Register::B);
}

// SET 6, C
pub(super) fn instr_0xCBF1(state: &mut GBCState) {
    op_SET_reg(state, 6, Register::C);
}

// SET 6, D
pub(super) fn instr_0xCBF2(state: &mut GBCState) {
    op_SET_reg(state, 6, Register::D);
}

// SET 6, E
pub(super) fn instr_0xCBF3(state: &mut GBCState) {
    op_SET_reg(state, 6, Register::E);
}

// SET 6, H
pub(super) fn instr_0xCBF4(state: &mut GBCState) {
    op_SET_reg(state, 6, Register::H);
}

// SET 6, L
pub(super) fn instr_0xCBF5(state: &mut GBCState) {
    op_SET_reg(state, 6, Register::L);
}

// SET 6, (HL)
pub(super) fn instr_0xCBF6(state: &mut GBCState) {
    op_SET_from_HLptr(state, 6);
}

// SET 6, A
pub(super) fn instr_0xCBF7(state: &mut GBCState) {
    op_SET_reg(state, 6, Register::A);
}

// SET 7, B
pub(super) fn instr_0xCBF8(state: &mut GBCState) {
    op_SET_reg(state, 7, Register::B);
}

// SET 7, C
pub(super) fn instr_0xCBF9(state: &mut GBCState) {
    op_SET_reg(state, 7, Register::C);
}

// SET 7, D
pub(super) fn instr_0xCBFA(state: &mut GBCState) {
    op_SET_reg(state, 7, Register::D);
}

// SET 7, E
pub(super) fn instr_0xCBFB(state: &mut GBCState) {
    op_SET_reg(state, 7, Register::E);
}

// SET 7, H
pub(super) fn instr_0xCBFC(state: &mut GBCState) {
    op_SET_reg(state, 7, Register::H);
}

// SET 7, L
pub(super) fn instr_0xCBFD(state: &mut GBCState) {
    op_SET_reg(state, 7, Register::L);
}

// SET 7, (HL)
pub(super) fn instr_0xCBFE(state: &mut GBCState) {
    op_SET_from_HLptr(state, 7);
}

// SET 7, A
pub(super) fn instr_0xCBFF(state: &mut GBCState) {
    op_SET_reg(state, 7, Register::A);
}
