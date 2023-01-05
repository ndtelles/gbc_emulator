#![allow(non_snake_case)]

use crate::{
    gbc::{virtual_memory, GBCState},
    util::{add_and_get_carries, add_i8_to_u16, index_bitmap, subtract_and_get_borrows, Bytes},
};

use super::{
    instructions::map_CB_prefix_instruction,
    op_helpers::*,
    register::{FlagRegister, Register, RegisterMapMethods, RegisterPair},
};

/**
 * Instructions
 */

// NOP
pub(super) fn instr_0x00(_state: &mut GBCState) {}

// LD BC, u16
pub(super) fn instr_0x01(state: &mut GBCState) {
    op_LD_registerpair_from_u16(state, RegisterPair::BC);
}

// LD (BC), A
pub(super) fn instr_0x02(state: &mut GBCState) {
    op_LD_regpairptr_from_reg(state, RegisterPair::BC, Register::A);
}

// INC BC
pub(super) fn instr_0x03(state: &mut GBCState) {
    op_INC_regpair(state, RegisterPair::BC);
}

// INC B
pub(super) fn instr_0x04(state: &mut GBCState) {
    op_INC_reg(state, Register::B);
}

// DEC B
pub(super) fn instr_0x05(state: &mut GBCState) {
    op_DEC_reg(state, Register::B);
}

// LD B, u8
pub(super) fn instr_0x06(state: &mut GBCState) {
    op_LD_reg_from_u8(state, Register::B);
}

// RLCA
pub(super) fn instr_0x07(state: &mut GBCState) {
    let val = state.cpu.registers.read(Register::A);
    state.cpu.registers.set_flags(&FlagRegister {
        z: false,
        n: false,
        h: false,
        cy: index_bitmap(val, 7),
    });
    state.cpu.registers.write(Register::A, val.rotate_left(1));
}

// LD (u16), SP
pub(super) fn instr_0x08(state: &mut GBCState) {
    let addr = super::fetch_and_incr_pc_16(state);
    virtual_memory::write(state, addr, state.cpu.sp.low());
    virtual_memory::write(state, addr + 1, state.cpu.sp.high());
}

//  ADD HL, BC
pub(super) fn instr_0x09(state: &mut GBCState) {
    op_ADD_regpair(state, RegisterPair::BC);
}

// LD A, (BC)
pub(super) fn instr_0x0A(state: &mut GBCState) {
    op_LD_reg_from_regpairptr(state, Register::A, RegisterPair::BC);
}

// DEC BC
pub(super) fn instr_0x0B(state: &mut GBCState) {
    op_DEC_regpair(state, RegisterPair::BC);
}

// INC C
pub(super) fn instr_0x0C(state: &mut GBCState) {
    op_INC_reg(state, Register::C);
}

// DEC C
pub(super) fn instr_0x0D(state: &mut GBCState) {
    op_DEC_reg(state, Register::C);
}

// LD C, u8
pub(super) fn instr_0x0E(state: &mut GBCState) {
    op_LD_reg_from_u8(state, Register::C);
}

// RRCA
pub(super) fn instr_0x0F(state: &mut GBCState) {
    let val = state.cpu.registers.read(Register::A);
    state.cpu.registers.set_flags(&FlagRegister {
        z: false,
        n: false,
        h: false,
        cy: index_bitmap(val, 0),
    });
    state.cpu.registers.write(Register::A, val.rotate_right(1));
}

pub(super) fn instr_0x10(_state: &mut GBCState) {
    todo!();
}

// LD DE, u16
pub(super) fn instr_0x11(state: &mut GBCState) {
    op_LD_registerpair_from_u16(state, RegisterPair::DE);
}

// LD (DE), A
pub(super) fn instr_0x12(state: &mut GBCState) {
    op_LD_reg_from_regpairptr(state, Register::A, RegisterPair::DE);
}

// INC DE
pub(super) fn instr_0x13(state: &mut GBCState) {
    op_INC_regpair(state, RegisterPair::DE);
}

// INC D
pub(super) fn instr_0x14(state: &mut GBCState) {
    op_INC_reg(state, Register::D);
}

// DEC D
pub(super) fn instr_0x15(state: &mut GBCState) {
    op_DEC_reg(state, Register::D);
}

// LD D, u8
pub(super) fn instr_0x16(state: &mut GBCState) {
    op_LD_reg_from_u8(state, Register::D);
}

// RLA
pub(super) fn instr_0x17(state: &mut GBCState) {
    let val = state.cpu.registers.read(Register::A);
    let old_cy = state.cpu.registers.get_flags().cy;
    state.cpu.registers.set_flags(&FlagRegister {
        z: false,
        n: false,
        h: false,
        cy: index_bitmap(val, 7),
    });
    let result = (val << 1) | (old_cy as u8);
    state.cpu.registers.write(Register::A, result);
}

// JR i8
pub(super) fn instr_0x18(state: &mut GBCState) {
    let operand = super::fetch_and_incr_pc(state) as i8;
    state.cpu.pc = add_i8_to_u16(state.cpu.pc, operand).0;
}

// ADD HL, DE
pub(super) fn instr_0x19(state: &mut GBCState) {
    op_ADD_regpair(state, RegisterPair::DE);
}

// LD A, (DE)
pub(super) fn instr_0x1A(state: &mut GBCState) {
    op_LD_reg_from_regpairptr(state, Register::A, RegisterPair::DE);
}

// DEC DE
pub(super) fn instr_0x1B(state: &mut GBCState) {
    op_DEC_regpair(state, RegisterPair::DE);
}

// INC E
pub(super) fn instr_0x1C(state: &mut GBCState) {
    op_INC_reg(state, Register::E);
}

// DEC E
pub(super) fn instr_0x1D(state: &mut GBCState) {
    op_DEC_reg(state, Register::E);
}

// LD E, u8
pub(super) fn instr_0x1E(state: &mut GBCState) {
    op_LD_reg_from_u8(state, Register::E);
}

// RRA
pub(super) fn instr_0x1F(state: &mut GBCState) {
    let val = state.cpu.registers.read(Register::A);
    let old_cy = state.cpu.registers.get_flags().cy;
    state.cpu.registers.set_flags(&FlagRegister {
        z: false,
        n: false,
        h: false,
        cy: index_bitmap(val, 0),
    });
    let result = ((old_cy as u8) << 7) | (val >> 1);
    state.cpu.registers.write(Register::A, result);
}

// JR NZ, i8
pub(super) fn instr_0x20(state: &mut GBCState) {
    let operand = super::fetch_and_incr_pc(state) as i8;
    if !state.cpu.registers.get_flags().z {
        state.cpu.pc = add_i8_to_u16(state.cpu.pc, operand).0;
    }
}

// LD HL, u16
pub(super) fn instr_0x21(state: &mut GBCState) {
    op_LD_registerpair_from_u16(state, RegisterPair::HL);
}

// LD (HLI), A
pub(super) fn instr_0x22(state: &mut GBCState) {
    op_LD_regpairptr_from_reg(state, RegisterPair::HL, Register::A);
    let new_hl = state
        .cpu
        .registers
        .read_pair(RegisterPair::HL)
        .wrapping_add(1);
    state.cpu.registers.write_pair(RegisterPair::HL, new_hl);
}

// INC HL
pub(super) fn instr_0x23(state: &mut GBCState) {
    op_INC_regpair(state, RegisterPair::HL);
}

// INC H
pub(super) fn instr_0x24(state: &mut GBCState) {
    op_INC_reg(state, Register::H);
}

// DEC H
pub(super) fn instr_0x25(state: &mut GBCState) {
    op_DEC_reg(state, Register::H);
}

// LD H, u8
pub(super) fn instr_0x26(state: &mut GBCState) {
    op_LD_reg_from_u8(state, Register::H);
}

pub(super) fn instr_0x27(_state: &mut GBCState) {
    todo!();
}

// JR Z, i8
pub(super) fn instr_0x28(state: &mut GBCState) {
    let operand = super::fetch_and_incr_pc(state) as i8;
    if state.cpu.registers.get_flags().z {
        state.cpu.pc = add_i8_to_u16(state.cpu.pc, operand).0;
    }
}

// ADD HL, HL
pub(super) fn instr_0x29(state: &mut GBCState) {
    op_ADD_regpair(state, RegisterPair::HL);
}

// LD A, (HLI)
pub(super) fn instr_0x2A(state: &mut GBCState) {
    op_LD_reg_from_regpairptr(state, Register::A, RegisterPair::HL);
    let new_hl = state
        .cpu
        .registers
        .read_pair(RegisterPair::HL)
        .wrapping_add(1);
    state.cpu.registers.write_pair(RegisterPair::HL, new_hl);
}

// DEC HL
pub(super) fn instr_0x2B(state: &mut GBCState) {
    op_DEC_regpair(state, RegisterPair::HL);
}

// INC L
pub(super) fn instr_0x2C(state: &mut GBCState) {
    op_INC_reg(state, Register::L);
}

// DEC L
pub(super) fn instr_0x2D(state: &mut GBCState) {
    op_DEC_reg(state, Register::L);
}

// LD L, u8
pub(super) fn instr_0x2E(state: &mut GBCState) {
    op_LD_reg_from_u8(state, Register::L);
}

// CPL
pub(super) fn instr_0x2F(state: &mut GBCState) {
    let val = state.cpu.registers.read(Register::A);
    state.cpu.registers.write(Register::A, !val);
    let existing_flags = state.cpu.registers.get_flags();
    state.cpu.registers.set_flags(&FlagRegister {
        z: existing_flags.z,
        n: true,
        h: true,
        cy: existing_flags.cy,
    });
}

// JR NC, i8
pub(super) fn instr_0x30(state: &mut GBCState) {
    let operand = super::fetch_and_incr_pc(state) as i8;
    if !state.cpu.registers.get_flags().cy {
        state.cpu.pc = add_i8_to_u16(state.cpu.pc, operand).0;
    }
}

// LD SP, u16
pub(super) fn instr_0x31(state: &mut GBCState) {
    state.cpu.sp = super::fetch_and_incr_pc_16(state);
}

// LD (HLD), A
pub(super) fn instr_0x32(state: &mut GBCState) {
    op_LD_regpairptr_from_reg(state, RegisterPair::HL, Register::A);
    let new_hl = state
        .cpu
        .registers
        .read_pair(RegisterPair::HL)
        .wrapping_sub(1);
    state.cpu.registers.write_pair(RegisterPair::HL, new_hl);
}

// INC SP
pub(super) fn instr_0x33(state: &mut GBCState) {
    state.cpu.sp = state.cpu.sp.wrapping_add(1);
}

// INC (HL)
pub(super) fn instr_0x34(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let lhs = virtual_memory::read(state, addr);
    let (result, carries) = add_and_get_carries(lhs, 1);
    virtual_memory::write(state, addr, result);

    state.cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: false,
        h: index_bitmap(carries, 3),
        cy: state.cpu.registers.get_flags().cy,
    });
}

// DEC (HL)
pub(super) fn instr_0x35(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let lhs = virtual_memory::read(state, addr);
    let (result, borrows) = subtract_and_get_borrows(lhs, 1);
    virtual_memory::write(state, addr, result);

    state.cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: true,
        h: index_bitmap(borrows, 3),
        cy: state.cpu.registers.get_flags().cy,
    });
}

// LD (HL), u8
pub(super) fn instr_0x36(state: &mut GBCState) {
    op_LD_regpairptr_from_u8(state, RegisterPair::HL);
}

// SCF
pub(super) fn instr_0x37(state: &mut GBCState) {
    let existing_flags = state.cpu.registers.get_flags();
    state.cpu.registers.set_flags(&FlagRegister {
        z: existing_flags.z,
        n: false,
        h: false,
        cy: true,
    });
}

// JR C, i8
pub(super) fn instr_0x38(state: &mut GBCState) {
    let operand = super::fetch_and_incr_pc(state) as i8;
    if state.cpu.registers.get_flags().cy {
        state.cpu.pc = add_i8_to_u16(state.cpu.pc, operand).0;
    }
}

// ADD HL, SP
pub(super) fn instr_0x39(state: &mut GBCState) {
    let val = state.cpu.sp;
    op_ADD_u16(state, val);
}

// LD A, (HLD)
pub(super) fn instr_0x3A(state: &mut GBCState) {
    op_LD_reg_from_regpairptr(state, Register::A, RegisterPair::HL);
    let new_hl = state
        .cpu
        .registers
        .read_pair(RegisterPair::HL)
        .wrapping_sub(1);
    state.cpu.registers.write_pair(RegisterPair::HL, new_hl);
}

// DEC SP
pub(super) fn instr_0x3B(state: &mut GBCState) {
    state.cpu.sp = state.cpu.sp.wrapping_sub(1);
}

// INC A
pub(super) fn instr_0x3C(state: &mut GBCState) {
    op_INC_reg(state, Register::A);
}

// DEC A
pub(super) fn instr_0x3D(state: &mut GBCState) {
    op_DEC_reg(state, Register::A);
}

// LD A, u8
pub(super) fn instr_0x3E(state: &mut GBCState) {
    op_LD_reg_from_u8(state, Register::A);
}

// CCF
pub(super) fn instr_0x3F(state: &mut GBCState) {
    let existing_flags = state.cpu.registers.get_flags();
    state.cpu.registers.set_flags(&FlagRegister {
        z: existing_flags.z,
        n: false,
        h: false,
        cy: !existing_flags.cy,
    });
}

// LD B, B
pub(super) fn instr_0x40(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::B, Register::B);
}

// LD B, C
pub(super) fn instr_0x41(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::B, Register::C);
}

// LD B, D
pub(super) fn instr_0x42(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::B, Register::D);
}

// LD B, E
pub(super) fn instr_0x43(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::B, Register::E);
}

// LD B, H
pub(super) fn instr_0x44(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::B, Register::H);
}

// LD B, L
pub(super) fn instr_0x45(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::B, Register::L);
}

// LD B, (HL)
pub(super) fn instr_0x46(state: &mut GBCState) {
    op_LD_reg_from_regpairptr(state, Register::B, RegisterPair::HL);
}

// LD B, A
pub(super) fn instr_0x47(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::B, Register::A);
}

// LD C, B
pub(super) fn instr_0x48(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::C, Register::B);
}

// LD C, C
pub(super) fn instr_0x49(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::C, Register::C);
}

// LD C, D
pub(super) fn instr_0x4A(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::C, Register::D);
}

// LD C, E
pub(super) fn instr_0x4B(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::C, Register::E);
}

// LD C, H
pub(super) fn instr_0x4C(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::C, Register::H);
}

// LD C, L
pub(super) fn instr_0x4D(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::C, Register::L);
}

// LD C, (HL)
pub(super) fn instr_0x4E(state: &mut GBCState) {
    op_LD_reg_from_regpairptr(state, Register::C, RegisterPair::HL);
}

// LD C, A
pub(super) fn instr_0x4F(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::C, Register::A);
}

// LD D, B
pub(super) fn instr_0x50(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::D, Register::B);
}

// LD D, C
pub(super) fn instr_0x51(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::D, Register::C);
}

// LD D, D
pub(super) fn instr_0x52(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::D, Register::D);
}

// LD D, E
pub(super) fn instr_0x53(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::D, Register::E);
}

// LD D, H
pub(super) fn instr_0x54(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::D, Register::H);
}

// LD D, L
pub(super) fn instr_0x55(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::D, Register::L);
}

// LD D, (HL)
pub(super) fn instr_0x56(state: &mut GBCState) {
    op_LD_reg_from_regpairptr(state, Register::D, RegisterPair::HL);
}

// LD D, A
pub(super) fn instr_0x57(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::D, Register::A);
}

// LD E, B
pub(super) fn instr_0x58(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::E, Register::B);
}

// LD E, C
pub(super) fn instr_0x59(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::E, Register::C);
}

// LD E, D
pub(super) fn instr_0x5A(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::E, Register::D);
}

// LD E, E
pub(super) fn instr_0x5B(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::E, Register::E);
}

// LD E, H
pub(super) fn instr_0x5C(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::E, Register::H);
}

// LD E, L
pub(super) fn instr_0x5D(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::E, Register::L);
}

// LD E, (HL)
pub(super) fn instr_0x5E(state: &mut GBCState) {
    op_LD_reg_from_regpairptr(state, Register::E, RegisterPair::HL);
}

// LD E, A
pub(super) fn instr_0x5F(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::E, Register::A);
}

// LD H, B
pub(super) fn instr_0x60(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::H, Register::B);
}

// LD H, C
pub(super) fn instr_0x61(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::H, Register::C);
}

// LD H, D
pub(super) fn instr_0x62(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::H, Register::D);
}

// LD H, E
pub(super) fn instr_0x63(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::H, Register::E);
}

// LD H, H
pub(super) fn instr_0x64(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::H, Register::H);
}

// LD H, L
pub(super) fn instr_0x65(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::H, Register::L);
}

// LD H, (HL)
pub(super) fn instr_0x66(state: &mut GBCState) {
    op_LD_reg_from_regpairptr(state, Register::H, RegisterPair::HL);
}

// LD H, A
pub(super) fn instr_0x67(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::H, Register::A);
}

// LD L, B
pub(super) fn instr_0x68(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::L, Register::B);
}

// LD L, C
pub(super) fn instr_0x69(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::L, Register::C);
}

// LD L, D
pub(super) fn instr_0x6A(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::L, Register::D);
}

// LD L, E
pub(super) fn instr_0x6B(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::L, Register::E);
}

// LD L, H
pub(super) fn instr_0x6C(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::L, Register::H);
}

// LD L, L
pub(super) fn instr_0x6D(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::L, Register::L);
}

// LD L, (HL)
pub(super) fn instr_0x6E(state: &mut GBCState) {
    op_LD_reg_from_regpairptr(state, Register::L, RegisterPair::HL);
}

// LD L, A
pub(super) fn instr_0x6F(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::L, Register::A);
}

// LD (HL). B
pub(super) fn instr_0x70(state: &mut GBCState) {
    op_LD_regpairptr_from_reg(state, RegisterPair::HL, Register::B);
}

// LD (HL). C
pub(super) fn instr_0x71(state: &mut GBCState) {
    op_LD_regpairptr_from_reg(state, RegisterPair::HL, Register::C);
}

// LD (HL). D
pub(super) fn instr_0x72(state: &mut GBCState) {
    op_LD_regpairptr_from_reg(state, RegisterPair::HL, Register::D);
}

// LD (HL). E
pub(super) fn instr_0x73(state: &mut GBCState) {
    op_LD_regpairptr_from_reg(state, RegisterPair::HL, Register::E);
}

// LD (HL). H
pub(super) fn instr_0x74(state: &mut GBCState) {
    op_LD_regpairptr_from_reg(state, RegisterPair::HL, Register::H);
}

// LD (HL). L
pub(super) fn instr_0x75(state: &mut GBCState) {
    op_LD_regpairptr_from_reg(state, RegisterPair::HL, Register::L);
}

pub(super) fn instr_0x76(_state: &mut GBCState) {
    todo!();
}

// LD (HL), A
pub(super) fn instr_0x77(state: &mut GBCState) {
    op_LD_regpairptr_from_reg(state, RegisterPair::HL, Register::A);
}

// LD A, B
pub(super) fn instr_0x78(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::A, Register::B);
}

// LD A, C
pub(super) fn instr_0x79(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::A, Register::C);
}

// LD A, D
pub(super) fn instr_0x7A(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::A, Register::D);
}

// LD A, E
pub(super) fn instr_0x7B(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::A, Register::E);
}

// LD A, H
pub(super) fn instr_0x7C(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::A, Register::H);
}

// LD A, L
pub(super) fn instr_0x7D(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::A, Register::L);
}

// LD A, (HL)
pub(super) fn instr_0x7E(state: &mut GBCState) {
    op_LD_reg_from_regpairptr(state, Register::A, RegisterPair::HL);
}

// LD A, A
pub(super) fn instr_0x7F(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::A, Register::A);
}

// ADD A, B
pub(super) fn instr_0x80(state: &mut GBCState) {
    op_ADD_reg(state, Register::B);
}

// Add A, C
pub(super) fn instr_0x81(state: &mut GBCState) {
    op_ADD_reg(state, Register::C);
}

// Add A, D
pub(super) fn instr_0x82(state: &mut GBCState) {
    op_ADD_reg(state, Register::D);
}

// Add A, E
pub(super) fn instr_0x83(state: &mut GBCState) {
    op_ADD_reg(state, Register::E);
}

// Add A, H
pub(super) fn instr_0x84(state: &mut GBCState) {
    op_ADD_reg(state, Register::H);
}

// Add A, L
pub(super) fn instr_0x85(state: &mut GBCState) {
    op_ADD_reg(state, Register::L);
}

// ADD A, (HL)
pub(super) fn instr_0x86(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    op_ADD(state, val);
}

// Add A, A
pub(super) fn instr_0x87(state: &mut GBCState) {
    op_ADD_reg(state, Register::A);
}

// ADC A, B
pub(super) fn instr_0x88(state: &mut GBCState) {
    op_ADC_reg(state, Register::B);
}

// ADC A, C
pub(super) fn instr_0x89(state: &mut GBCState) {
    op_ADC_reg(state, Register::C);
}

// ADC A, D
pub(super) fn instr_0x8A(state: &mut GBCState) {
    op_ADC_reg(state, Register::D);
}

// ADC A, E
pub(super) fn instr_0x8B(state: &mut GBCState) {
    op_ADC_reg(state, Register::E);
}

// ADC A, H
pub(super) fn instr_0x8C(state: &mut GBCState) {
    op_ADC_reg(state, Register::H);
}

// ADC A, L
pub(super) fn instr_0x8D(state: &mut GBCState) {
    op_ADC_reg(state, Register::L);
}

// ADC A, (HL)
pub(super) fn instr_0x8E(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    op_ADC(state, val);
}

// ADC A, A
pub(super) fn instr_0x8F(state: &mut GBCState) {
    op_ADC_reg(state, Register::A);
}

// SUB B
pub(super) fn instr_0x90(state: &mut GBCState) {
    op_SUB_reg(state, Register::B);
}

// SUB C
pub(super) fn instr_0x91(state: &mut GBCState) {
    op_SUB_reg(state, Register::C);
}

// SUB D
pub(super) fn instr_0x92(state: &mut GBCState) {
    op_SUB_reg(state, Register::D);
}

// SUB E
pub(super) fn instr_0x93(state: &mut GBCState) {
    op_SUB_reg(state, Register::E);
}

// SUB H
pub(super) fn instr_0x94(state: &mut GBCState) {
    op_SUB_reg(state, Register::H);
}

// SUB L
pub(super) fn instr_0x95(state: &mut GBCState) {
    op_SUB_reg(state, Register::L);
}

// SUB (HL)
pub(super) fn instr_0x96(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    op_SUB(state, val);
}

// SUB A
pub(super) fn instr_0x97(state: &mut GBCState) {
    op_SUB_reg(state, Register::A);
}

// SBC A, B
pub(super) fn instr_0x98(state: &mut GBCState) {
    op_SBC_reg(state, Register::B);
}

// SBC A, C
pub(super) fn instr_0x99(state: &mut GBCState) {
    op_SBC_reg(state, Register::C);
}

// SBC A, D
pub(super) fn instr_0x9A(state: &mut GBCState) {
    op_SBC_reg(state, Register::D);
}

// SBC A, E
pub(super) fn instr_0x9B(state: &mut GBCState) {
    op_SBC_reg(state, Register::E);
}

// SBC A, H
pub(super) fn instr_0x9C(state: &mut GBCState) {
    op_SBC_reg(state, Register::H);
}

// SBC A, L
pub(super) fn instr_0x9D(state: &mut GBCState) {
    op_SBC_reg(state, Register::L);
}

// SBC A, (HL)
pub(super) fn instr_0x9E(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    op_SBC(state, val);
}

// SBC A, A
pub(super) fn instr_0x9F(state: &mut GBCState) {
    op_SBC_reg(state, Register::A);
}

// AND B
pub(super) fn instr_0xA0(state: &mut GBCState) {
    op_AND_reg(state, Register::B);
}

// AND C
pub(super) fn instr_0xA1(state: &mut GBCState) {
    op_AND_reg(state, Register::C);
}

// AND D
pub(super) fn instr_0xA2(state: &mut GBCState) {
    op_AND_reg(state, Register::D);
}

// AND E
pub(super) fn instr_0xA3(state: &mut GBCState) {
    op_AND_reg(state, Register::E);
}

// AND H
pub(super) fn instr_0xA4(state: &mut GBCState) {
    op_AND_reg(state, Register::H);
}

// AND L
pub(super) fn instr_0xA5(state: &mut GBCState) {
    op_AND_reg(state, Register::L);
}

// AND (HL)
pub(super) fn instr_0xA6(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    op_AND(state, val);
}

// AND A
pub(super) fn instr_0xA7(state: &mut GBCState) {
    op_AND_reg(state, Register::A);
}

// XOR B
pub(super) fn instr_0xA8(state: &mut GBCState) {
    op_XOR_reg(state, Register::B);
}

// XOR C
pub(super) fn instr_0xA9(state: &mut GBCState) {
    op_XOR_reg(state, Register::C);
}

// XOR D
pub(super) fn instr_0xAA(state: &mut GBCState) {
    op_XOR_reg(state, Register::D);
}

// XOR E
pub(super) fn instr_0xAB(state: &mut GBCState) {
    op_XOR_reg(state, Register::E);
}

// XOR H
pub(super) fn instr_0xAC(state: &mut GBCState) {
    op_XOR_reg(state, Register::H);
}

// XOR L
pub(super) fn instr_0xAD(state: &mut GBCState) {
    op_XOR_reg(state, Register::L);
}

// XOR (HL)
pub(super) fn instr_0xAE(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    op_XOR(state, val);
}

// XOR A
pub(super) fn instr_0xAF(state: &mut GBCState) {
    op_XOR_reg(state, Register::A);
}

// OR B
pub(super) fn instr_0xB0(state: &mut GBCState) {
    op_OR_reg(state, Register::B);
}

// OR C
pub(super) fn instr_0xB1(state: &mut GBCState) {
    op_OR_reg(state, Register::C);
}

// OR D
pub(super) fn instr_0xB2(state: &mut GBCState) {
    op_OR_reg(state, Register::D);
}

// OR E
pub(super) fn instr_0xB3(state: &mut GBCState) {
    op_OR_reg(state, Register::E);
}

// OR H
pub(super) fn instr_0xB4(state: &mut GBCState) {
    op_OR_reg(state, Register::H);
}

// OR L
pub(super) fn instr_0xB5(state: &mut GBCState) {
    op_OR_reg(state, Register::L);
}

// OR (HL)
pub(super) fn instr_0xB6(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    op_OR(state, val);
}

// OR A
pub(super) fn instr_0xB7(state: &mut GBCState) {
    op_OR_reg(state, Register::A);
}

// CP B
pub(super) fn instr_0xB8(state: &mut GBCState) {
    op_CP_reg(state, Register::B);
}

// CP C
pub(super) fn instr_0xB9(state: &mut GBCState) {
    op_CP_reg(state, Register::C);
}

// CP D
pub(super) fn instr_0xBA(state: &mut GBCState) {
    op_CP_reg(state, Register::D);
}

// CP E
pub(super) fn instr_0xBB(state: &mut GBCState) {
    op_CP_reg(state, Register::E);
}

// CP H
pub(super) fn instr_0xBC(state: &mut GBCState) {
    op_CP_reg(state, Register::H);
}

// CP L
pub(super) fn instr_0xBD(state: &mut GBCState) {
    op_CP_reg(state, Register::L);
}

// CP (HL)
pub(super) fn instr_0xBE(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    op_CP(state, val);
}

// CP A
pub(super) fn instr_0xBF(state: &mut GBCState) {
    op_CP_reg(state, Register::A);
}

// RET NZ
pub(super) fn instr_0xC0(state: &mut GBCState) {
    if !state.cpu.registers.get_flags().z {
        op_RET(state);
    }
}

// POP BC
pub(super) fn instr_0xC1(state: &mut GBCState) {
    op_POP_stack_to_regpair(state, RegisterPair::BC);
}

// JP NZ, u16
pub(super) fn instr_0xC2(state: &mut GBCState) {
    let new_pc = super::fetch_and_incr_pc_16(state);
    if !state.cpu.registers.get_flags().z {
        state.cpu.pc = new_pc;
    }
}

// JP u16
pub(super) fn instr_0xC3(state: &mut GBCState) {
    state.cpu.pc = super::fetch_and_incr_pc_16(state);
}

// CALL NZ, u16
pub(super) fn instr_0xC4(state: &mut GBCState) {
    let new_pc = super::fetch_and_incr_pc_16(state);
    if !state.cpu.registers.get_flags().z {
        state.cpu.sp -= 1;
        virtual_memory::write(state, state.cpu.sp, state.cpu.pc.high());
        state.cpu.sp -= 1;
        virtual_memory::write(state, state.cpu.sp, state.cpu.pc.low());
        state.cpu.pc = new_pc;
    }
}

// PUSH BC
pub(super) fn instr_0xC5(state: &mut GBCState) {
    op_PUSH_stack_from_regpair(state, RegisterPair::BC);
}

// ADD A, u8
pub(super) fn instr_0xC6(state: &mut GBCState) {
    let src_val = super::fetch_and_incr_pc(state) as u8;
    op_ADD(state, src_val);
}

// RST 0
pub(super) fn instr_0xC7(state: &mut GBCState) {
    op_RST(state, 0x0000);
}

// RET Z
pub(super) fn instr_0xC8(state: &mut GBCState) {
    if state.cpu.registers.get_flags().z {
        op_RET(state);
    }
}

// RET
pub(super) fn instr_0xC9(state: &mut GBCState) {
    op_RET(state);
}

// JP Z, u16
pub(super) fn instr_0xCA(state: &mut GBCState) {
    let new_pc = super::fetch_and_incr_pc_16(state);
    if state.cpu.registers.get_flags().z {
        state.cpu.pc = new_pc;
    }
}

// Prefix for second instruction set
pub(super) fn instr_0xCB(state: &mut GBCState) {
    let instr = super::fetch_and_incr_pc(state);
    let instruction_impl = map_CB_prefix_instruction(instr);
    instruction_impl(state);
}

// CALL Z, u16
pub(super) fn instr_0xCC(state: &mut GBCState) {
    let new_pc = super::fetch_and_incr_pc_16(state);
    if state.cpu.registers.get_flags().z {
        state.cpu.sp -= 1;
        virtual_memory::write(state, state.cpu.sp, state.cpu.pc.high());
        state.cpu.sp -= 1;
        virtual_memory::write(state, state.cpu.sp, state.cpu.pc.low());
        state.cpu.pc = new_pc;
    }
}

// CALL u16
pub(super) fn instr_0xCD(state: &mut GBCState) {
    let new_pc = super::fetch_and_incr_pc_16(state);
    state.cpu.sp -= 1;
    virtual_memory::write(state, state.cpu.sp, state.cpu.pc.high());
    state.cpu.sp -= 1;
    virtual_memory::write(state, state.cpu.sp, state.cpu.pc.low());
    state.cpu.pc = new_pc;
}

// ADC A, u8
pub(super) fn instr_0xCE(state: &mut GBCState) {
    let val = super::fetch_and_incr_pc(state);
    op_ADC(state, val);
}

// RST 1
pub(super) fn instr_0xCF(state: &mut GBCState) {
    op_RST(state, 0x0008);
}

// RET NC
pub(super) fn instr_0xD0(state: &mut GBCState) {
    if !state.cpu.registers.get_flags().cy {
        op_RET(state);
    }
}

// POP DE
pub(super) fn instr_0xD1(state: &mut GBCState) {
    op_POP_stack_to_regpair(state, RegisterPair::DE);
}

// JP NC, u16
pub(super) fn instr_0xD2(state: &mut GBCState) {
    let new_pc = super::fetch_and_incr_pc_16(state);
    if !state.cpu.registers.get_flags().cy {
        state.cpu.pc = new_pc;
    }
}

// Invalid Opcode
pub(super) fn instr_0xD3(_state: &mut GBCState) {
    unimplemented!();
}

// CALL NC, u16
pub(super) fn instr_0xD4(state: &mut GBCState) {
    let new_pc = super::fetch_and_incr_pc_16(state);
    if !state.cpu.registers.get_flags().cy {
        state.cpu.sp -= 1;
        virtual_memory::write(state, state.cpu.sp, state.cpu.pc.high());
        state.cpu.sp -= 1;
        virtual_memory::write(state, state.cpu.sp, state.cpu.pc.low());
        state.cpu.pc = new_pc;
    }
}

// PUSH DE
pub(super) fn instr_0xD5(state: &mut GBCState) {
    op_PUSH_stack_from_regpair(state, RegisterPair::DE);
}

// SUB u8
pub(super) fn instr_0xD6(state: &mut GBCState) {
    let val = super::fetch_and_incr_pc(state);
    op_SUB(state, val);
}

// RST 2
pub(super) fn instr_0xD7(state: &mut GBCState) {
    op_RST(state, 0x0010);
}

// RET C
pub(super) fn instr_0xD8(state: &mut GBCState) {
    if state.cpu.registers.get_flags().cy {
        op_RET(state);
    }
}

// RETI
pub(super) fn instr_0xD9(state: &mut GBCState) {
    op_RET(state);
    state.intr_ctrl.enable_interrupts();
}

// JP C, u16
pub(super) fn instr_0xDA(state: &mut GBCState) {
    let new_pc = super::fetch_and_incr_pc_16(state);
    if state.cpu.registers.get_flags().cy {
        state.cpu.pc = new_pc;
    }
}

// Invalid Opcode
pub(super) fn instr_0xDB(_state: &mut GBCState) {
    unimplemented!();
}

// CALL C, u16
pub(super) fn instr_0xDC(state: &mut GBCState) {
    let new_pc = super::fetch_and_incr_pc_16(state);
    if state.cpu.registers.get_flags().cy {
        state.cpu.sp -= 1;
        virtual_memory::write(state, state.cpu.sp, state.cpu.pc.high());
        state.cpu.sp -= 1;
        virtual_memory::write(state, state.cpu.sp, state.cpu.pc.low());
        state.cpu.pc = new_pc;
    }
}

// Invalid Opcode
pub(super) fn instr_0xDD(_state: &mut GBCState) {
    unimplemented!();
}

// SBC A, u8
pub(super) fn instr_0xDE(state: &mut GBCState) {
    let val = super::fetch_and_incr_pc(state);
    op_SBC(state, val);
}

// RST 3
pub(super) fn instr_0xDF(state: &mut GBCState) {
    op_RST(state, 0x0018);
}

// LD (u8), A
pub(super) fn instr_0xE0(state: &mut GBCState) {
    op_LD_u8ptr_from_reg(state, Register::A);
}

// POP HL
pub(super) fn instr_0xE1(state: &mut GBCState) {
    op_POP_stack_to_regpair(state, RegisterPair::HL);
}

// LD (C), A
pub(super) fn instr_0xE2(state: &mut GBCState) {
    op_LD_regpptr_from_reg(state, Register::C, Register::A);
}

// Invalid Opcode
pub(super) fn instr_0xE3(_state: &mut GBCState) {
    unimplemented!();
}

// Invalid Opcode
pub(super) fn instr_0xE4(_state: &mut GBCState) {
    unimplemented!();
}

// PUSH HL
pub(super) fn instr_0xE5(state: &mut GBCState) {
    op_PUSH_stack_from_regpair(state, RegisterPair::HL);
}

// AND u8
pub(super) fn instr_0xE6(state: &mut GBCState) {
    let val = super::fetch_and_incr_pc(state);
    op_AND(state, val);
}

// RST 4
pub(super) fn instr_0xE7(state: &mut GBCState) {
    op_RST(state, 0x0020);
}

// ADD SP, e
pub(super) fn instr_0xE8(state: &mut GBCState) {
    let rhs = super::fetch_and_incr_pc(state) as u16;
    let (result, carries) = add_and_get_carries(state.cpu.sp, rhs);
    state.cpu.sp = result;

    state.cpu.registers.set_flags(&FlagRegister {
        z: false,
        n: false,
        // https://stackoverflow.com/a/57978555
        h: index_bitmap(carries, 3),
        cy: index_bitmap(carries, 15),
    });
}

// JP HL
pub(super) fn instr_0xE9(state: &mut GBCState) {
    state.cpu.pc = state.cpu.registers.read_pair(RegisterPair::HL);
}

// LD (u16), A
pub(super) fn instr_0xEA(state: &mut GBCState) {
    op_LD_u16ptr_from_reg(state, Register::A);
}

// Invalid Opcode
pub(super) fn instr_0xEB(_state: &mut GBCState) {
    unimplemented!();
}

// Invalid Opcode
pub(super) fn instr_0xEC(_state: &mut GBCState) {
    unimplemented!();
}

// Invalid Opcode
pub(super) fn instr_0xED(_state: &mut GBCState) {
    unimplemented!();
}

// XOR u8
pub(super) fn instr_0xEE(state: &mut GBCState) {
    let val = super::fetch_and_incr_pc(state);
    op_XOR(state, val);
}

// RST 5
pub(super) fn instr_0xEF(state: &mut GBCState) {
    op_RST(state, 0x0028);
}

// LD A, (u8)
pub(super) fn instr_0xF0(state: &mut GBCState) {
    op_LD_reg_from_u8ptr(state, Register::A);
}

// POP AF
pub(super) fn instr_0xF1(state: &mut GBCState) {
    op_POP_stack_to_regpair(state, RegisterPair::AF);
}

// LD A, (C)
pub(super) fn instr_0xF2(state: &mut GBCState) {
    op_LD_reg_from_regptr(state, Register::A, Register::C);
}

// DI
pub(super) fn instr_0xF3(state: &mut GBCState) {
    state.intr_ctrl.disable_interrupts();
}

// Invalid Opcode
pub(super) fn instr_0xF4(_state: &mut GBCState) {
    unimplemented!();
}

// PUSH AF
pub(super) fn instr_0xF5(state: &mut GBCState) {
    op_PUSH_stack_from_regpair(state, RegisterPair::AF);
}

// OR u8
pub(super) fn instr_0xF6(state: &mut GBCState) {
    let val = super::fetch_and_incr_pc(state);
    op_OR(state, val);
}

// RST 6
pub(super) fn instr_0xF7(state: &mut GBCState) {
    op_RST(state, 0x0030);
}

// LDHL SP, i8
pub(super) fn instr_0xF8(state: &mut GBCState) {
    // Be careful of data types and sign extensions in this operation!
    let operand = super::fetch_and_incr_pc(state) as i8;
    let (result, carries_or_borrows) = add_i8_to_u16(state.cpu.sp, operand);
    state.cpu.registers.write_pair(RegisterPair::HL, result);
    state.cpu.registers.set_flags(&FlagRegister {
        z: false,
        n: false,
        h: index_bitmap(carries_or_borrows, 11),
        cy: index_bitmap(carries_or_borrows, 15),
    });
}

// LD SP, HL
pub(super) fn instr_0xF9(state: &mut GBCState) {
    state.cpu.sp = state.cpu.registers.read_pair(RegisterPair::HL);
}

// LD A, (u16)
pub(super) fn instr_0xFA(state: &mut GBCState) {
    op_LD_reg_from_u16ptr(state, Register::A);
}

// EI
pub(super) fn instr_0xFB(state: &mut GBCState) {
    state.intr_ctrl.enable_interrupts();
}

// Invalid Opcode
pub(super) fn instr_0xFC(_state: &mut GBCState) {
    unimplemented!();
}

// Invalid Opcode
pub(super) fn instr_0xFD(_state: &mut GBCState) {
    unimplemented!();
}

// CP u8
pub(super) fn instr_0xFE(state: &mut GBCState) {
    let val = super::fetch_and_incr_pc(state);
    op_CP(state, val);
}

// RST 7
pub(super) fn instr_0xFF(state: &mut GBCState) {
    op_RST(state, 0x0038);
}
