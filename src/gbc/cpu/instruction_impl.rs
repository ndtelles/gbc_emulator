#![allow(non_snake_case)]

use tracing::trace;

use crate::{
    gbc::{interrupt_controller, virtual_memory, GBCState},
    util::{add_and_get_carries, add_i8_to_u16, index_bits, subtract_and_get_borrows, Bytes},
};

use super::{
    call, consume_cycles,
    instructions::map_CB_prefix_instruction,
    op_helpers::*,
    register::{FlagRegister, Register, RegisterMapMethods, RegisterPair},
};

/**
 * Instructions
 */

// NOP
pub(super) fn instr_0x00(state: &mut GBCState) {
    consume_cycles(state, 4);
}

// LD BC, u16
pub(super) fn instr_0x01(state: &mut GBCState) {
    op_LD_registerpair_from_u16(state, RegisterPair::BC);
    consume_cycles(state, 12);
}

// LD (BC), A
pub(super) fn instr_0x02(state: &mut GBCState) {
    op_LD_regpairptr_from_reg(state, RegisterPair::BC, Register::A);
    consume_cycles(state, 8);
}

// INC BC
pub(super) fn instr_0x03(state: &mut GBCState) {
    op_INC_regpair(state, RegisterPair::BC);
    consume_cycles(state, 8);
}

// INC B
pub(super) fn instr_0x04(state: &mut GBCState) {
    op_INC_reg(state, Register::B);
    consume_cycles(state, 4);
}

// DEC B
pub(super) fn instr_0x05(state: &mut GBCState) {
    op_DEC_reg(state, Register::B);
    consume_cycles(state, 4);
    trace!(
        "Register B {} {}",
        state.cpu.registers.read(Register::B),
        state.cpu.registers.get_flags().z
    );
}

// LD B, u8
pub(super) fn instr_0x06(state: &mut GBCState) {
    op_LD_reg_from_u8(state, Register::B);
    consume_cycles(state, 8);
}

// RLCA
pub(super) fn instr_0x07(state: &mut GBCState) {
    let val = state.cpu.registers.read(Register::A);
    state.cpu.registers.set_flags(&FlagRegister {
        z: false,
        n: false,
        h: false,
        cy: index_bits(val, 7),
    });
    state.cpu.registers.write(Register::A, val.rotate_left(1));
    consume_cycles(state, 4);
}

// LD (u16), SP
pub(super) fn instr_0x08(state: &mut GBCState) {
    let addr = super::fetch_and_incr_pc_16(state);
    virtual_memory::write(state, addr, state.cpu.sp.low());
    virtual_memory::write(state, addr + 1, state.cpu.sp.high());
    consume_cycles(state, 28);
}

//  ADD HL, BC
pub(super) fn instr_0x09(state: &mut GBCState) {
    op_ADD_regpair(state, RegisterPair::BC);
    consume_cycles(state, 8);
}

// LD A, (BC)
pub(super) fn instr_0x0A(state: &mut GBCState) {
    op_LD_reg_from_regpairptr(state, Register::A, RegisterPair::BC);
    consume_cycles(state, 8);
}

// DEC BC
pub(super) fn instr_0x0B(state: &mut GBCState) {
    op_DEC_regpair(state, RegisterPair::BC);
    consume_cycles(state, 8);
}

// INC C
pub(super) fn instr_0x0C(state: &mut GBCState) {
    op_INC_reg(state, Register::C);
    consume_cycles(state, 4);
}

// DEC C
pub(super) fn instr_0x0D(state: &mut GBCState) {
    op_DEC_reg(state, Register::C);
    consume_cycles(state, 4);
    trace!(
        "Register C {} {}",
        state.cpu.registers.read(Register::C),
        state.cpu.registers.get_flags().z
    );
}

// LD C, u8
pub(super) fn instr_0x0E(state: &mut GBCState) {
    op_LD_reg_from_u8(state, Register::C);
    consume_cycles(state, 8);
}

// RRCA
pub(super) fn instr_0x0F(state: &mut GBCState) {
    let val = state.cpu.registers.read(Register::A);
    state.cpu.registers.set_flags(&FlagRegister {
        z: false,
        n: false,
        h: false,
        cy: index_bits(val, 0),
    });
    state.cpu.registers.write(Register::A, val.rotate_right(1));
    consume_cycles(state, 4);
}

pub(super) fn instr_0x10(state: &mut GBCState) {
    todo!();
    consume_cycles(state, 4);
}

// LD DE, u16
pub(super) fn instr_0x11(state: &mut GBCState) {
    op_LD_registerpair_from_u16(state, RegisterPair::DE);
    consume_cycles(state, 12);
}

// LD (DE), A
pub(super) fn instr_0x12(state: &mut GBCState) {
    op_LD_reg_from_regpairptr(state, Register::A, RegisterPair::DE);
    consume_cycles(state, 8);
}

// INC DE
pub(super) fn instr_0x13(state: &mut GBCState) {
    op_INC_regpair(state, RegisterPair::DE);
    consume_cycles(state, 8);
}

// INC D
pub(super) fn instr_0x14(state: &mut GBCState) {
    op_INC_reg(state, Register::D);
    consume_cycles(state, 4);
}

// DEC D
pub(super) fn instr_0x15(state: &mut GBCState) {
    op_DEC_reg(state, Register::D);
    consume_cycles(state, 4);
}

// LD D, u8
pub(super) fn instr_0x16(state: &mut GBCState) {
    op_LD_reg_from_u8(state, Register::D);
    consume_cycles(state, 8);
}

// RLA
pub(super) fn instr_0x17(state: &mut GBCState) {
    let val = state.cpu.registers.read(Register::A);
    let old_cy = state.cpu.registers.get_flags().cy;
    state.cpu.registers.set_flags(&FlagRegister {
        z: false,
        n: false,
        h: false,
        cy: index_bits(val, 7),
    });
    let result = (val << 1) | (old_cy as u8);
    state.cpu.registers.write(Register::A, result);
    consume_cycles(state, 4);
}

// JR i8
pub(super) fn instr_0x18(state: &mut GBCState) {
    let operand = super::fetch_and_incr_pc(state) as i8;
    state.cpu.pc = add_i8_to_u16(state.cpu.pc, operand).0;
    consume_cycles(state, 12);
}

// ADD HL, DE
pub(super) fn instr_0x19(state: &mut GBCState) {
    op_ADD_regpair(state, RegisterPair::DE);
    consume_cycles(state, 8);
}

// LD A, (DE)
pub(super) fn instr_0x1A(state: &mut GBCState) {
    op_LD_reg_from_regpairptr(state, Register::A, RegisterPair::DE);
    consume_cycles(state, 8);
}

// DEC DE
pub(super) fn instr_0x1B(state: &mut GBCState) {
    op_DEC_regpair(state, RegisterPair::DE);
    consume_cycles(state, 8);
}

// INC E
pub(super) fn instr_0x1C(state: &mut GBCState) {
    op_INC_reg(state, Register::E);
    consume_cycles(state, 4);
}

// DEC E
pub(super) fn instr_0x1D(state: &mut GBCState) {
    op_DEC_reg(state, Register::E);
    consume_cycles(state, 4);
}

// LD E, u8
pub(super) fn instr_0x1E(state: &mut GBCState) {
    op_LD_reg_from_u8(state, Register::E);
    consume_cycles(state, 8);
}

// RRA
pub(super) fn instr_0x1F(state: &mut GBCState) {
    let val = state.cpu.registers.read(Register::A);
    let old_cy = state.cpu.registers.get_flags().cy;
    state.cpu.registers.set_flags(&FlagRegister {
        z: false,
        n: false,
        h: false,
        cy: index_bits(val, 0),
    });
    let result = ((old_cy as u8) << 7) | (val >> 1);
    state.cpu.registers.write(Register::A, result);
    consume_cycles(state, 4);
}

// JR NZ, i8
pub(super) fn instr_0x20(state: &mut GBCState) {
    let operand = super::fetch_and_incr_pc(state) as i8;
    if !state.cpu.registers.get_flags().z {
        state.cpu.pc = add_i8_to_u16(state.cpu.pc, operand).0;
        consume_cycles(state, 12);
        return;
    }
    consume_cycles(state, 8);
}

// LD HL, u16
pub(super) fn instr_0x21(state: &mut GBCState) {
    op_LD_registerpair_from_u16(state, RegisterPair::HL);
    consume_cycles(state, 12);
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
    consume_cycles(state, 8);
}

// INC HL
pub(super) fn instr_0x23(state: &mut GBCState) {
    op_INC_regpair(state, RegisterPair::HL);
    consume_cycles(state, 8);
}

// INC H
pub(super) fn instr_0x24(state: &mut GBCState) {
    op_INC_reg(state, Register::H);
    consume_cycles(state, 4);
}

// DEC H
pub(super) fn instr_0x25(state: &mut GBCState) {
    op_DEC_reg(state, Register::H);
    consume_cycles(state, 4);
}

// LD H, u8
pub(super) fn instr_0x26(state: &mut GBCState) {
    op_LD_reg_from_u8(state, Register::H);
    consume_cycles(state, 8);
}

pub(super) fn instr_0x27(state: &mut GBCState) {
    todo!();
    consume_cycles(state, 4);
}

// JR Z, i8
pub(super) fn instr_0x28(state: &mut GBCState) {
    let operand = super::fetch_and_incr_pc(state) as i8;
    if state.cpu.registers.get_flags().z {
        state.cpu.pc = add_i8_to_u16(state.cpu.pc, operand).0;
        consume_cycles(state, 12);
        return;
    }
    consume_cycles(state, 8);
}

// ADD HL, HL
pub(super) fn instr_0x29(state: &mut GBCState) {
    op_ADD_regpair(state, RegisterPair::HL);
    consume_cycles(state, 8);
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
    consume_cycles(state, 8);
}

// DEC HL
pub(super) fn instr_0x2B(state: &mut GBCState) {
    op_DEC_regpair(state, RegisterPair::HL);
    consume_cycles(state, 8);
}

// INC L
pub(super) fn instr_0x2C(state: &mut GBCState) {
    op_INC_reg(state, Register::L);
    consume_cycles(state, 4);
}

// DEC L
pub(super) fn instr_0x2D(state: &mut GBCState) {
    op_DEC_reg(state, Register::L);
    consume_cycles(state, 4);
}

// LD L, u8
pub(super) fn instr_0x2E(state: &mut GBCState) {
    op_LD_reg_from_u8(state, Register::L);
    consume_cycles(state, 8);
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
    consume_cycles(state, 4);
}

// JR NC, i8
pub(super) fn instr_0x30(state: &mut GBCState) {
    let operand = super::fetch_and_incr_pc(state) as i8;
    if !state.cpu.registers.get_flags().cy {
        state.cpu.pc = add_i8_to_u16(state.cpu.pc, operand).0;
        consume_cycles(state, 12);
        return;
    }
    consume_cycles(state, 8);
}

// LD SP, u16
pub(super) fn instr_0x31(state: &mut GBCState) {
    state.cpu.sp = super::fetch_and_incr_pc_16(state);
    consume_cycles(state, 12);
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
    consume_cycles(state, 8);
}

// INC SP
pub(super) fn instr_0x33(state: &mut GBCState) {
    state.cpu.sp = state.cpu.sp.wrapping_add(1);
    consume_cycles(state, 8);
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
        h: index_bits(carries, 3),
        cy: state.cpu.registers.get_flags().cy,
    });
    consume_cycles(state, 12);
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
        h: index_bits(borrows, 3),
        cy: state.cpu.registers.get_flags().cy,
    });
    consume_cycles(state, 12);
}

// LD (HL), u8
pub(super) fn instr_0x36(state: &mut GBCState) {
    op_LD_regpairptr_from_u8(state, RegisterPair::HL);
    consume_cycles(state, 12);
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
    consume_cycles(state, 4);
}

// JR C, i8
pub(super) fn instr_0x38(state: &mut GBCState) {
    let operand = super::fetch_and_incr_pc(state) as i8;
    if state.cpu.registers.get_flags().cy {
        state.cpu.pc = add_i8_to_u16(state.cpu.pc, operand).0;
        consume_cycles(state, 12);
        return;
    }
    consume_cycles(state, 8);
}

// ADD HL, SP
pub(super) fn instr_0x39(state: &mut GBCState) {
    let val = state.cpu.sp;
    op_ADD_u16(state, val);
    consume_cycles(state, 8);
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
    consume_cycles(state, 8);
}

// DEC SP
pub(super) fn instr_0x3B(state: &mut GBCState) {
    state.cpu.sp = state.cpu.sp.wrapping_sub(1);
    consume_cycles(state, 8);
}

// INC A
pub(super) fn instr_0x3C(state: &mut GBCState) {
    op_INC_reg(state, Register::A);
    consume_cycles(state, 4);
}

// DEC A
pub(super) fn instr_0x3D(state: &mut GBCState) {
    op_DEC_reg(state, Register::A);
    consume_cycles(state, 4);
}

// LD A, u8
pub(super) fn instr_0x3E(state: &mut GBCState) {
    op_LD_reg_from_u8(state, Register::A);
    consume_cycles(state, 8);
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
    consume_cycles(state, 4);
}

// LD B, B
pub(super) fn instr_0x40(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::B, Register::B);
    consume_cycles(state, 4);
}

// LD B, C
pub(super) fn instr_0x41(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::B, Register::C);
    consume_cycles(state, 4);
}

// LD B, D
pub(super) fn instr_0x42(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::B, Register::D);
    consume_cycles(state, 4);
}

// LD B, E
pub(super) fn instr_0x43(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::B, Register::E);
    consume_cycles(state, 4);
}

// LD B, H
pub(super) fn instr_0x44(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::B, Register::H);
    consume_cycles(state, 4);
}

// LD B, L
pub(super) fn instr_0x45(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::B, Register::L);
    consume_cycles(state, 4);
}

// LD B, (HL)
pub(super) fn instr_0x46(state: &mut GBCState) {
    op_LD_reg_from_regpairptr(state, Register::B, RegisterPair::HL);
    consume_cycles(state, 8);
}

// LD B, A
pub(super) fn instr_0x47(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::B, Register::A);
    consume_cycles(state, 4);
}

// LD C, B
pub(super) fn instr_0x48(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::C, Register::B);
    consume_cycles(state, 4);
}

// LD C, C
pub(super) fn instr_0x49(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::C, Register::C);
    consume_cycles(state, 4);
}

// LD C, D
pub(super) fn instr_0x4A(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::C, Register::D);
    consume_cycles(state, 4);
}

// LD C, E
pub(super) fn instr_0x4B(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::C, Register::E);
    consume_cycles(state, 4);
}

// LD C, H
pub(super) fn instr_0x4C(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::C, Register::H);
    consume_cycles(state, 4);
}

// LD C, L
pub(super) fn instr_0x4D(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::C, Register::L);
    consume_cycles(state, 4);
}

// LD C, (HL)
pub(super) fn instr_0x4E(state: &mut GBCState) {
    op_LD_reg_from_regpairptr(state, Register::C, RegisterPair::HL);
    consume_cycles(state, 8);
}

// LD C, A
pub(super) fn instr_0x4F(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::C, Register::A);
    consume_cycles(state, 4);
}

// LD D, B
pub(super) fn instr_0x50(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::D, Register::B);
    consume_cycles(state, 4);
}

// LD D, C
pub(super) fn instr_0x51(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::D, Register::C);
    consume_cycles(state, 4);
}

// LD D, D
pub(super) fn instr_0x52(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::D, Register::D);
    consume_cycles(state, 4);
}

// LD D, E
pub(super) fn instr_0x53(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::D, Register::E);
    consume_cycles(state, 4);
}

// LD D, H
pub(super) fn instr_0x54(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::D, Register::H);
    consume_cycles(state, 4);
}

// LD D, L
pub(super) fn instr_0x55(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::D, Register::L);
    consume_cycles(state, 4);
}

// LD D, (HL)
pub(super) fn instr_0x56(state: &mut GBCState) {
    op_LD_reg_from_regpairptr(state, Register::D, RegisterPair::HL);
    consume_cycles(state, 8);
}

// LD D, A
pub(super) fn instr_0x57(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::D, Register::A);
    consume_cycles(state, 4);
}

// LD E, B
pub(super) fn instr_0x58(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::E, Register::B);
    consume_cycles(state, 4);
}

// LD E, C
pub(super) fn instr_0x59(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::E, Register::C);
    consume_cycles(state, 4);
}

// LD E, D
pub(super) fn instr_0x5A(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::E, Register::D);
    consume_cycles(state, 4);
}

// LD E, E
pub(super) fn instr_0x5B(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::E, Register::E);
    consume_cycles(state, 4);
}

// LD E, H
pub(super) fn instr_0x5C(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::E, Register::H);
    consume_cycles(state, 4);
}

// LD E, L
pub(super) fn instr_0x5D(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::E, Register::L);
    consume_cycles(state, 4);
}

// LD E, (HL)
pub(super) fn instr_0x5E(state: &mut GBCState) {
    op_LD_reg_from_regpairptr(state, Register::E, RegisterPair::HL);
    consume_cycles(state, 8);
}

// LD E, A
pub(super) fn instr_0x5F(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::E, Register::A);
    consume_cycles(state, 4);
}

// LD H, B
pub(super) fn instr_0x60(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::H, Register::B);
    consume_cycles(state, 4);
}

// LD H, C
pub(super) fn instr_0x61(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::H, Register::C);
    consume_cycles(state, 4);
}

// LD H, D
pub(super) fn instr_0x62(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::H, Register::D);
    consume_cycles(state, 4);
}

// LD H, E
pub(super) fn instr_0x63(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::H, Register::E);
    consume_cycles(state, 4);
}

// LD H, H
pub(super) fn instr_0x64(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::H, Register::H);
    consume_cycles(state, 4);
}

// LD H, L
pub(super) fn instr_0x65(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::H, Register::L);
    consume_cycles(state, 4);
}

// LD H, (HL)
pub(super) fn instr_0x66(state: &mut GBCState) {
    op_LD_reg_from_regpairptr(state, Register::H, RegisterPair::HL);
    consume_cycles(state, 8);
}

// LD H, A
pub(super) fn instr_0x67(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::H, Register::A);
    consume_cycles(state, 4);
}

// LD L, B
pub(super) fn instr_0x68(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::L, Register::B);
    consume_cycles(state, 4);
}

// LD L, C
pub(super) fn instr_0x69(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::L, Register::C);
    consume_cycles(state, 4);
}

// LD L, D
pub(super) fn instr_0x6A(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::L, Register::D);
    consume_cycles(state, 4);
}

// LD L, E
pub(super) fn instr_0x6B(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::L, Register::E);
    consume_cycles(state, 4);
}

// LD L, H
pub(super) fn instr_0x6C(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::L, Register::H);
    consume_cycles(state, 4);
}

// LD L, L
pub(super) fn instr_0x6D(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::L, Register::L);
    consume_cycles(state, 4);
}

// LD L, (HL)
pub(super) fn instr_0x6E(state: &mut GBCState) {
    op_LD_reg_from_regpairptr(state, Register::L, RegisterPair::HL);
    consume_cycles(state, 8);
}

// LD L, A
pub(super) fn instr_0x6F(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::L, Register::A);
    consume_cycles(state, 4);
}

// LD (HL). B
pub(super) fn instr_0x70(state: &mut GBCState) {
    op_LD_regpairptr_from_reg(state, RegisterPair::HL, Register::B);
    consume_cycles(state, 8);
}

// LD (HL). C
pub(super) fn instr_0x71(state: &mut GBCState) {
    op_LD_regpairptr_from_reg(state, RegisterPair::HL, Register::C);
    consume_cycles(state, 8);
}

// LD (HL). D
pub(super) fn instr_0x72(state: &mut GBCState) {
    op_LD_regpairptr_from_reg(state, RegisterPair::HL, Register::D);
    consume_cycles(state, 8);
}

// LD (HL). E
pub(super) fn instr_0x73(state: &mut GBCState) {
    op_LD_regpairptr_from_reg(state, RegisterPair::HL, Register::E);
    consume_cycles(state, 8);
}

// LD (HL). H
pub(super) fn instr_0x74(state: &mut GBCState) {
    op_LD_regpairptr_from_reg(state, RegisterPair::HL, Register::H);
    consume_cycles(state, 8);
}

// LD (HL). L
pub(super) fn instr_0x75(state: &mut GBCState) {
    op_LD_regpairptr_from_reg(state, RegisterPair::HL, Register::L);
    consume_cycles(state, 8);
}

// HALT
pub(super) fn instr_0x76(state: &mut GBCState) {
    state.cpu.halted = true;
    consume_cycles(state, 4);
}

// LD (HL), A
pub(super) fn instr_0x77(state: &mut GBCState) {
    op_LD_regpairptr_from_reg(state, RegisterPair::HL, Register::A);
    consume_cycles(state, 8);
}

// LD A, B
pub(super) fn instr_0x78(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::A, Register::B);
    consume_cycles(state, 4);
}

// LD A, C
pub(super) fn instr_0x79(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::A, Register::C);
    consume_cycles(state, 4);
}

// LD A, D
pub(super) fn instr_0x7A(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::A, Register::D);
    consume_cycles(state, 4);
}

// LD A, E
pub(super) fn instr_0x7B(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::A, Register::E);
    consume_cycles(state, 4);
}

// LD A, H
pub(super) fn instr_0x7C(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::A, Register::H);
    consume_cycles(state, 4);
}

// LD A, L
pub(super) fn instr_0x7D(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::A, Register::L);
    consume_cycles(state, 4);
}

// LD A, (HL)
pub(super) fn instr_0x7E(state: &mut GBCState) {
    op_LD_reg_from_regpairptr(state, Register::A, RegisterPair::HL);
    consume_cycles(state, 8);
}

// LD A, A
pub(super) fn instr_0x7F(state: &mut GBCState) {
    op_LD_reg_from_reg(state, Register::A, Register::A);
    consume_cycles(state, 4);
}

// ADD A, B
pub(super) fn instr_0x80(state: &mut GBCState) {
    op_ADD_reg(state, Register::B);
    consume_cycles(state, 4);
}

// Add A, C
pub(super) fn instr_0x81(state: &mut GBCState) {
    op_ADD_reg(state, Register::C);
    consume_cycles(state, 4);
}

// Add A, D
pub(super) fn instr_0x82(state: &mut GBCState) {
    op_ADD_reg(state, Register::D);
    consume_cycles(state, 4);
}

// Add A, E
pub(super) fn instr_0x83(state: &mut GBCState) {
    op_ADD_reg(state, Register::E);
    consume_cycles(state, 4);
}

// Add A, H
pub(super) fn instr_0x84(state: &mut GBCState) {
    op_ADD_reg(state, Register::H);
    consume_cycles(state, 4);
}

// Add A, L
pub(super) fn instr_0x85(state: &mut GBCState) {
    op_ADD_reg(state, Register::L);
    consume_cycles(state, 4);
}

// ADD A, (HL)
pub(super) fn instr_0x86(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    op_ADD(state, val);
    consume_cycles(state, 8);
}

// Add A, A
pub(super) fn instr_0x87(state: &mut GBCState) {
    op_ADD_reg(state, Register::A);
    consume_cycles(state, 4);
}

// ADC A, B
pub(super) fn instr_0x88(state: &mut GBCState) {
    op_ADC_reg(state, Register::B);
    consume_cycles(state, 4);
}

// ADC A, C
pub(super) fn instr_0x89(state: &mut GBCState) {
    op_ADC_reg(state, Register::C);
    consume_cycles(state, 4);
}

// ADC A, D
pub(super) fn instr_0x8A(state: &mut GBCState) {
    op_ADC_reg(state, Register::D);
    consume_cycles(state, 4);
}

// ADC A, E
pub(super) fn instr_0x8B(state: &mut GBCState) {
    op_ADC_reg(state, Register::E);
    consume_cycles(state, 4);
}

// ADC A, H
pub(super) fn instr_0x8C(state: &mut GBCState) {
    op_ADC_reg(state, Register::H);
    consume_cycles(state, 4);
}

// ADC A, L
pub(super) fn instr_0x8D(state: &mut GBCState) {
    op_ADC_reg(state, Register::L);
    consume_cycles(state, 4);
}

// ADC A, (HL)
pub(super) fn instr_0x8E(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    op_ADC(state, val);
    consume_cycles(state, 8);
}

// ADC A, A
pub(super) fn instr_0x8F(state: &mut GBCState) {
    op_ADC_reg(state, Register::A);
    consume_cycles(state, 4);
}

// SUB B
pub(super) fn instr_0x90(state: &mut GBCState) {
    op_SUB_reg(state, Register::B);
    consume_cycles(state, 4);
}

// SUB C
pub(super) fn instr_0x91(state: &mut GBCState) {
    op_SUB_reg(state, Register::C);
    consume_cycles(state, 4);
}

// SUB D
pub(super) fn instr_0x92(state: &mut GBCState) {
    op_SUB_reg(state, Register::D);
    consume_cycles(state, 4);
}

// SUB E
pub(super) fn instr_0x93(state: &mut GBCState) {
    op_SUB_reg(state, Register::E);
    consume_cycles(state, 4);
}

// SUB H
pub(super) fn instr_0x94(state: &mut GBCState) {
    op_SUB_reg(state, Register::H);
    consume_cycles(state, 4);
}

// SUB L
pub(super) fn instr_0x95(state: &mut GBCState) {
    op_SUB_reg(state, Register::L);
    consume_cycles(state, 4);
}

// SUB (HL)
pub(super) fn instr_0x96(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    op_SUB(state, val);
    consume_cycles(state, 8);
}

// SUB A
pub(super) fn instr_0x97(state: &mut GBCState) {
    op_SUB_reg(state, Register::A);
    consume_cycles(state, 4);
}

// SBC A, B
pub(super) fn instr_0x98(state: &mut GBCState) {
    op_SBC_reg(state, Register::B);
    consume_cycles(state, 4);
}

// SBC A, C
pub(super) fn instr_0x99(state: &mut GBCState) {
    op_SBC_reg(state, Register::C);
    consume_cycles(state, 4);
}

// SBC A, D
pub(super) fn instr_0x9A(state: &mut GBCState) {
    op_SBC_reg(state, Register::D);
    consume_cycles(state, 4);
}

// SBC A, E
pub(super) fn instr_0x9B(state: &mut GBCState) {
    op_SBC_reg(state, Register::E);
    consume_cycles(state, 4);
}

// SBC A, H
pub(super) fn instr_0x9C(state: &mut GBCState) {
    op_SBC_reg(state, Register::H);
    consume_cycles(state, 4);
}

// SBC A, L
pub(super) fn instr_0x9D(state: &mut GBCState) {
    op_SBC_reg(state, Register::L);
    consume_cycles(state, 4);
}

// SBC A, (HL)
pub(super) fn instr_0x9E(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    op_SBC(state, val);
    consume_cycles(state, 8);
}

// SBC A, A
pub(super) fn instr_0x9F(state: &mut GBCState) {
    op_SBC_reg(state, Register::A);
    consume_cycles(state, 4);
}

// AND B
pub(super) fn instr_0xA0(state: &mut GBCState) {
    op_AND_reg(state, Register::B);
    consume_cycles(state, 4);
}

// AND C
pub(super) fn instr_0xA1(state: &mut GBCState) {
    op_AND_reg(state, Register::C);
    consume_cycles(state, 4);
}

// AND D
pub(super) fn instr_0xA2(state: &mut GBCState) {
    op_AND_reg(state, Register::D);
    consume_cycles(state, 4);
}

// AND E
pub(super) fn instr_0xA3(state: &mut GBCState) {
    op_AND_reg(state, Register::E);
    consume_cycles(state, 4);
}

// AND H
pub(super) fn instr_0xA4(state: &mut GBCState) {
    op_AND_reg(state, Register::H);
    consume_cycles(state, 4);
}

// AND L
pub(super) fn instr_0xA5(state: &mut GBCState) {
    op_AND_reg(state, Register::L);
    consume_cycles(state, 4);
}

// AND (HL)
pub(super) fn instr_0xA6(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    op_AND(state, val);
    consume_cycles(state, 8);
}

// AND A
pub(super) fn instr_0xA7(state: &mut GBCState) {
    op_AND_reg(state, Register::A);
    consume_cycles(state, 4);
}

// XOR B
pub(super) fn instr_0xA8(state: &mut GBCState) {
    op_XOR_reg(state, Register::B);
    consume_cycles(state, 4);
}

// XOR C
pub(super) fn instr_0xA9(state: &mut GBCState) {
    op_XOR_reg(state, Register::C);
    consume_cycles(state, 4);
}

// XOR D
pub(super) fn instr_0xAA(state: &mut GBCState) {
    op_XOR_reg(state, Register::D);
    consume_cycles(state, 4);
}

// XOR E
pub(super) fn instr_0xAB(state: &mut GBCState) {
    op_XOR_reg(state, Register::E);
    consume_cycles(state, 4);
}

// XOR H
pub(super) fn instr_0xAC(state: &mut GBCState) {
    op_XOR_reg(state, Register::H);
    consume_cycles(state, 4);
}

// XOR L
pub(super) fn instr_0xAD(state: &mut GBCState) {
    op_XOR_reg(state, Register::L);
    consume_cycles(state, 4);
}

// XOR (HL)
pub(super) fn instr_0xAE(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    op_XOR(state, val);
    consume_cycles(state, 8);
}

// XOR A
pub(super) fn instr_0xAF(state: &mut GBCState) {
    op_XOR_reg(state, Register::A);
    consume_cycles(state, 4);
}

// OR B
pub(super) fn instr_0xB0(state: &mut GBCState) {
    op_OR_reg(state, Register::B);
    consume_cycles(state, 4);
}

// OR C
pub(super) fn instr_0xB1(state: &mut GBCState) {
    op_OR_reg(state, Register::C);
    consume_cycles(state, 4);
}

// OR D
pub(super) fn instr_0xB2(state: &mut GBCState) {
    op_OR_reg(state, Register::D);
    consume_cycles(state, 4);
}

// OR E
pub(super) fn instr_0xB3(state: &mut GBCState) {
    op_OR_reg(state, Register::E);
    consume_cycles(state, 4);
}

// OR H
pub(super) fn instr_0xB4(state: &mut GBCState) {
    op_OR_reg(state, Register::H);
    consume_cycles(state, 4);
}

// OR L
pub(super) fn instr_0xB5(state: &mut GBCState) {
    op_OR_reg(state, Register::L);
    consume_cycles(state, 4);
}

// OR (HL)
pub(super) fn instr_0xB6(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    op_OR(state, val);
    consume_cycles(state, 8);
}

// OR A
pub(super) fn instr_0xB7(state: &mut GBCState) {
    op_OR_reg(state, Register::A);
    consume_cycles(state, 4);
}

// CP B
pub(super) fn instr_0xB8(state: &mut GBCState) {
    op_CP_reg(state, Register::B);
    consume_cycles(state, 4);
}

// CP C
pub(super) fn instr_0xB9(state: &mut GBCState) {
    op_CP_reg(state, Register::C);
    consume_cycles(state, 4);
}

// CP D
pub(super) fn instr_0xBA(state: &mut GBCState) {
    op_CP_reg(state, Register::D);
    consume_cycles(state, 4);
}

// CP E
pub(super) fn instr_0xBB(state: &mut GBCState) {
    op_CP_reg(state, Register::E);
    consume_cycles(state, 4);
}

// CP H
pub(super) fn instr_0xBC(state: &mut GBCState) {
    op_CP_reg(state, Register::H);
    consume_cycles(state, 4);
}

// CP L
pub(super) fn instr_0xBD(state: &mut GBCState) {
    op_CP_reg(state, Register::L);
    consume_cycles(state, 4);
}

// CP (HL)
pub(super) fn instr_0xBE(state: &mut GBCState) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    op_CP(state, val);
    consume_cycles(state, 8);
}

// CP A
pub(super) fn instr_0xBF(state: &mut GBCState) {
    op_CP_reg(state, Register::A);
    consume_cycles(state, 4);
}

// RET NZ
pub(super) fn instr_0xC0(state: &mut GBCState) {
    if !state.cpu.registers.get_flags().z {
        op_RET(state);
        consume_cycles(state, 20);
        return;
    }
    consume_cycles(state, 8);
}

// POP BC
pub(super) fn instr_0xC1(state: &mut GBCState) {
    op_POP_stack_to_regpair(state, RegisterPair::BC);
    consume_cycles(state, 12);
}

// JP NZ, u16
pub(super) fn instr_0xC2(state: &mut GBCState) {
    let new_pc = super::fetch_and_incr_pc_16(state);
    if !state.cpu.registers.get_flags().z {
        state.cpu.pc = new_pc;
        consume_cycles(state, 16);
        return;
    }
    consume_cycles(state, 12);
}

// JP u16
pub(super) fn instr_0xC3(state: &mut GBCState) {
    state.cpu.pc = super::fetch_and_incr_pc_16(state);
    consume_cycles(state, 16);
}

// CALL NZ, u16
pub(super) fn instr_0xC4(state: &mut GBCState) {
    let new_pc = super::fetch_and_incr_pc_16(state);
    if !state.cpu.registers.get_flags().z {
        call(state, new_pc);
        consume_cycles(state, 24);
        return;
    }
    consume_cycles(state, 12);
}

// PUSH BC
pub(super) fn instr_0xC5(state: &mut GBCState) {
    op_PUSH_stack_from_regpair(state, RegisterPair::BC);
    consume_cycles(state, 16);
}

// ADD A, u8
pub(super) fn instr_0xC6(state: &mut GBCState) {
    let src_val = super::fetch_and_incr_pc(state) as u8;
    op_ADD(state, src_val);
    consume_cycles(state, 8);
}

// RST 0
pub(super) fn instr_0xC7(state: &mut GBCState) {
    op_RST(state, 0x0000);
    consume_cycles(state, 16);
}

// RET Z
pub(super) fn instr_0xC8(state: &mut GBCState) {
    if state.cpu.registers.get_flags().z {
        op_RET(state);
        consume_cycles(state, 20);
        return;
    }
    consume_cycles(state, 8);
}

// RET
pub(super) fn instr_0xC9(state: &mut GBCState) {
    op_RET(state);
    consume_cycles(state, 16);
}

// JP Z, u16
pub(super) fn instr_0xCA(state: &mut GBCState) {
    let new_pc = super::fetch_and_incr_pc_16(state);
    if state.cpu.registers.get_flags().z {
        state.cpu.pc = new_pc;
        consume_cycles(state, 16);
        return;
    }
    consume_cycles(state, 12);
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
        call(state, new_pc);
        consume_cycles(state, 24);
        return;
    }
    consume_cycles(state, 12);
}

// CALL u16
pub(super) fn instr_0xCD(state: &mut GBCState) {
    let new_pc = super::fetch_and_incr_pc_16(state);
    call(state, new_pc);
    consume_cycles(state, 24);
}

// ADC A, u8
pub(super) fn instr_0xCE(state: &mut GBCState) {
    let val = super::fetch_and_incr_pc(state);
    op_ADC(state, val);
    consume_cycles(state, 8);
}

// RST 1
pub(super) fn instr_0xCF(state: &mut GBCState) {
    op_RST(state, 0x0008);
    consume_cycles(state, 16);
}

// RET NC
pub(super) fn instr_0xD0(state: &mut GBCState) {
    if !state.cpu.registers.get_flags().cy {
        op_RET(state);
        consume_cycles(state, 20);
        return;
    }
    consume_cycles(state, 8);
}

// POP DE
pub(super) fn instr_0xD1(state: &mut GBCState) {
    op_POP_stack_to_regpair(state, RegisterPair::DE);
    consume_cycles(state, 12);
}

// JP NC, u16
pub(super) fn instr_0xD2(state: &mut GBCState) {
    let new_pc = super::fetch_and_incr_pc_16(state);
    if !state.cpu.registers.get_flags().cy {
        state.cpu.pc = new_pc;
        consume_cycles(state, 16);
        return;
    }
    consume_cycles(state, 12);
}

// Invalid Opcode
pub(super) fn instr_0xD3(_state: &mut GBCState) {
    unimplemented!();
}

// CALL NC, u16
pub(super) fn instr_0xD4(state: &mut GBCState) {
    let new_pc = super::fetch_and_incr_pc_16(state);
    if !state.cpu.registers.get_flags().cy {
        call(state, new_pc);
        consume_cycles(state, 24);
        return;
    }
    consume_cycles(state, 12);
}

// PUSH DE
pub(super) fn instr_0xD5(state: &mut GBCState) {
    op_PUSH_stack_from_regpair(state, RegisterPair::DE);
    consume_cycles(state, 16);
}

// SUB u8
pub(super) fn instr_0xD6(state: &mut GBCState) {
    let val = super::fetch_and_incr_pc(state);
    op_SUB(state, val);
    consume_cycles(state, 8);
}

// RST 2
pub(super) fn instr_0xD7(state: &mut GBCState) {
    op_RST(state, 0x0010);
    consume_cycles(state, 16);
}

// RET C
pub(super) fn instr_0xD8(state: &mut GBCState) {
    if state.cpu.registers.get_flags().cy {
        op_RET(state);
        consume_cycles(state, 20);
        return;
    }
    consume_cycles(state, 8);
}

// RETI
pub(super) fn instr_0xD9(state: &mut GBCState) {
    op_RET(state);
    interrupt_controller::enable_interrupts(state);
    consume_cycles(state, 16);
}

// JP C, u16
pub(super) fn instr_0xDA(state: &mut GBCState) {
    let new_pc = super::fetch_and_incr_pc_16(state);
    if state.cpu.registers.get_flags().cy {
        state.cpu.pc = new_pc;
        consume_cycles(state, 16);
        return;
    }
    consume_cycles(state, 12);
}

// Invalid Opcode
pub(super) fn instr_0xDB(_state: &mut GBCState) {
    unimplemented!();
}

// CALL C, u16
pub(super) fn instr_0xDC(state: &mut GBCState) {
    let new_pc = super::fetch_and_incr_pc_16(state);
    if state.cpu.registers.get_flags().cy {
        call(state, new_pc);
        consume_cycles(state, 24);
        return;
    }
    consume_cycles(state, 12);
}

// Invalid Opcode
pub(super) fn instr_0xDD(_state: &mut GBCState) {
    unimplemented!();
}

// SBC A, u8
pub(super) fn instr_0xDE(state: &mut GBCState) {
    let val = super::fetch_and_incr_pc(state);
    op_SBC(state, val);
    consume_cycles(state, 8);
}

// RST 3
pub(super) fn instr_0xDF(state: &mut GBCState) {
    op_RST(state, 0x0018);
    consume_cycles(state, 16);
}

// LD (u8), A
pub(super) fn instr_0xE0(state: &mut GBCState) {
    op_LD_u8ptr_from_reg(state, Register::A);
    consume_cycles(state, 12);
}

// POP HL
pub(super) fn instr_0xE1(state: &mut GBCState) {
    op_POP_stack_to_regpair(state, RegisterPair::HL);
    consume_cycles(state, 12);
}

// LD (C), A
pub(super) fn instr_0xE2(state: &mut GBCState) {
    op_LD_regpptr_from_reg(state, Register::C, Register::A);
    consume_cycles(state, 8);
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
    consume_cycles(state, 16);
}

// AND u8
pub(super) fn instr_0xE6(state: &mut GBCState) {
    let val = super::fetch_and_incr_pc(state);
    op_AND(state, val);
    consume_cycles(state, 8);
}

// RST 4
pub(super) fn instr_0xE7(state: &mut GBCState) {
    op_RST(state, 0x0020);
    consume_cycles(state, 16);
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
        h: index_bits(carries, 3),
        cy: index_bits(carries, 15),
    });
    consume_cycles(state, 16);
}

// JP HL
pub(super) fn instr_0xE9(state: &mut GBCState) {
    state.cpu.pc = state.cpu.registers.read_pair(RegisterPair::HL);
    consume_cycles(state, 4);
}

// LD (u16), A
pub(super) fn instr_0xEA(state: &mut GBCState) {
    op_LD_u16ptr_from_reg(state, Register::A);
    consume_cycles(state, 16);
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
    consume_cycles(state, 8);
}

// RST 5
pub(super) fn instr_0xEF(state: &mut GBCState) {
    op_RST(state, 0x0028);
    consume_cycles(state, 16);
}

// LD A, (u8)
pub(super) fn instr_0xF0(state: &mut GBCState) {
    op_LD_reg_from_u8ptr(state, Register::A);
    consume_cycles(state, 12);
}

// POP AF
pub(super) fn instr_0xF1(state: &mut GBCState) {
    op_POP_stack_to_regpair(state, RegisterPair::AF);
    consume_cycles(state, 12);
}

// LD A, (C)
pub(super) fn instr_0xF2(state: &mut GBCState) {
    op_LD_reg_from_regptr(state, Register::A, Register::C);
    consume_cycles(state, 8);
}

// DI
pub(super) fn instr_0xF3(state: &mut GBCState) {
    interrupt_controller::disable_interrupts(state);
    consume_cycles(state, 4);
}

// Invalid Opcode
pub(super) fn instr_0xF4(_state: &mut GBCState) {
    unimplemented!();
}

// PUSH AF
pub(super) fn instr_0xF5(state: &mut GBCState) {
    op_PUSH_stack_from_regpair(state, RegisterPair::AF);
    consume_cycles(state, 16);
}

// OR u8
pub(super) fn instr_0xF6(state: &mut GBCState) {
    let val = super::fetch_and_incr_pc(state);
    op_OR(state, val);
    consume_cycles(state, 8);
}

// RST 6
pub(super) fn instr_0xF7(state: &mut GBCState) {
    op_RST(state, 0x0030);
    consume_cycles(state, 16);
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
        h: index_bits(carries_or_borrows, 11),
        cy: index_bits(carries_or_borrows, 15),
    });
    consume_cycles(state, 12);
}

// LD SP, HL
pub(super) fn instr_0xF9(state: &mut GBCState) {
    state.cpu.sp = state.cpu.registers.read_pair(RegisterPair::HL);
    consume_cycles(state, 8);
}

// LD A, (u16)
pub(super) fn instr_0xFA(state: &mut GBCState) {
    op_LD_reg_from_u16ptr(state, Register::A);
    consume_cycles(state, 16);
}

// EI
pub(super) fn instr_0xFB(state: &mut GBCState) {
    interrupt_controller::enable_interrupts(state);
    consume_cycles(state, 4);
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
    consume_cycles(state, 8);
}

// RST 7
pub(super) fn instr_0xFF(state: &mut GBCState) {
    op_RST(state, 0x0038);
    consume_cycles(state, 16);
}
