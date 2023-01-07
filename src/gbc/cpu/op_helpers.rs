#![allow(non_snake_case)]

use crate::{
    gbc::{virtual_memory, GBCState},
    util::{add_and_get_carries, index_bits, reset_bit, set_bit, subtract_and_get_borrows, Bytes, combine_high_low},
};

use super::register::{FlagRegister, Register, RegisterMapMethods, RegisterPair};

/**
 * 8-bit Transfer Helpers
 */
// Load value from register to another register
pub(super) fn op_LD_reg_from_reg(state: &mut GBCState, dest: Register, src: Register) {
    let val = state.cpu.registers.read(src);
    state.cpu.registers.write(dest, val);
}

// Load value from register pointer to another register
pub(super) fn op_LD_reg_from_regptr(state: &mut GBCState, dest: Register, src: Register) {
    let addr = 0xFF00 | (state.cpu.registers.read(src) as u16);
    let val = virtual_memory::read(state, addr);
    state.cpu.registers.write(dest, val);
}

// Load immediate u8 value from PC to register
pub(super) fn op_LD_reg_from_u8(state: &mut GBCState, dest: Register) {
    let val = super::fetch_and_incr_pc(state);
    state.cpu.registers.write(dest, val);
}

// Load immediate u8 pointer from PC to register
pub(super) fn op_LD_reg_from_u8ptr(state: &mut GBCState, dest: Register) {
    let addr = 0xFF00 | (super::fetch_and_incr_pc(state) as u16);
    let val = virtual_memory::read(state, addr);
    state.cpu.registers.write(dest, val);
}

// Load immediate u16 pointer from PC to register
pub(super) fn op_LD_reg_from_u16ptr(state: &mut GBCState, dest: Register) {
    let addr = super::fetch_and_incr_pc_16(state);
    let val = virtual_memory::read(state, addr);
    state.cpu.registers.write(dest, val);
}

// Load register to immediate u8 pointer from PC
pub(super) fn op_LD_u8ptr_from_reg(state: &mut GBCState, src: Register) {
    let val = state.cpu.registers.read(src);
    let addr = 0xFF00 | (super::fetch_and_incr_pc(state) as u16);
    virtual_memory::write(state, addr, val);
}

// Load register to immediate u16 pointer from PC
pub(super) fn op_LD_u16ptr_from_reg(state: &mut GBCState, src: Register) {
    let val = state.cpu.registers.read(src);
    let addr = super::fetch_and_incr_pc_16(state);
    virtual_memory::write(state, addr, val);
}

// Load immediate u8 value from PC to register pair pointer
pub(super) fn op_LD_regpairptr_from_u8(state: &mut GBCState, dest: RegisterPair) {
    let val = super::fetch_and_incr_pc(state);
    let addr = state.cpu.registers.read_pair(dest);
    virtual_memory::write(state, addr, val);
}

// Load value at register pair pointer to register
pub(super) fn op_LD_reg_from_regpairptr(state: &mut GBCState, dest: Register, src: RegisterPair) {
    let val = virtual_memory::read(state, state.cpu.registers.read_pair(src));
    state.cpu.registers.write(dest, val);
}

// Load value from register to register pair pointer
pub(super) fn op_LD_regpairptr_from_reg(state: &mut GBCState, dest: RegisterPair, src: Register) {
    let val = state.cpu.registers.read(src);
    let addr = state.cpu.registers.read_pair(dest);
    virtual_memory::write(state, addr, val);
}

// Load value from register to register pointer
pub(super) fn op_LD_regpptr_from_reg(state: &mut GBCState, dest: Register, src: Register) {
    let val = state.cpu.registers.read(src);
    let addr = 0xFF00 | (state.cpu.registers.read(dest) as u16);
    virtual_memory::write(state, addr, val);
}

/**
 * 16-bit Transfer Helpers
 */
// Load immediate u16 value from PC to register pair
pub(super) fn op_LD_registerpair_from_u16(state: &mut GBCState, dest: RegisterPair) {
    let val = super::fetch_and_incr_pc_16(state);
    state.cpu.registers.write_pair(dest, val);
}

// Push value from register pair to stack
pub(super) fn op_PUSH_stack_from_regpair(state: &mut GBCState, src: RegisterPair) {
    let val = state.cpu.registers.read_pair(src);
    state.cpu.sp -= 1;
    // Write high byte
    virtual_memory::write(state, state.cpu.sp, (val >> 8) as u8);
    state.cpu.sp -= 1;
    // Write low byte
    virtual_memory::write(state, state.cpu.sp, val as u8);
}

// Pop value from stack to register pair
pub(super) fn op_POP_stack_to_regpair(state: &mut GBCState, dest: RegisterPair) {
    let val_low = virtual_memory::read(state, state.cpu.sp);
    state.cpu.sp += 1;
    let val_high = virtual_memory::read(state, state.cpu.sp);
    state.cpu.sp += 1;
    let val = combine_high_low(val_high, val_low);
    state.cpu.registers.write_pair(dest, val);
}

/**
 * 8-bit Arithmetic and Logical Operation Helpers
 */
// Add value to register A and set flags
pub(super) fn op_ADD(state: &mut GBCState, val: u8) {
    let a_val = state.cpu.registers.read(Register::A);
    let (sum, carries) = add_and_get_carries(a_val, val);
    state.cpu.registers.write(Register::A, sum);

    state.cpu.registers.set_flags(&FlagRegister {
        z: sum == 0,
        n: false,
        h: index_bits(carries, 3),
        cy: index_bits(carries, 7),
    });
}

// Add register to register A and set flags
pub(super) fn op_ADD_reg(state: &mut GBCState, reg: Register) {
    let val = state.cpu.registers.read(reg);
    op_ADD(state, val);
}

// Add value and carry bit to register A and set flags
pub(super) fn op_ADC(state: &mut GBCState, val: u8) {
    let carry = state.cpu.registers.get_flags().cy as u8;
    op_ADD(state, val.wrapping_add(carry));
}

// Add value and carry bit to register A and set flags
pub(super) fn op_ADC_reg(state: &mut GBCState, reg: Register) {
    let val = state.cpu.registers.read(reg);
    op_ADC(state, val);
}

// Subtract value from register A and set flags
pub(super) fn op_SUB(state: &mut GBCState, val: u8) {
    let a_val = state.cpu.registers.read(Register::A);
    let (diff, borrows) = subtract_and_get_borrows(a_val, val);
    state.cpu.registers.write(Register::A, diff);

    state.cpu.registers.set_flags(&FlagRegister {
        z: diff == 0,
        n: true,
        h: index_bits(borrows, 3),
        cy: index_bits(borrows, 7),
    });
}

// Subtract register from register A and set flags
pub(super) fn op_SUB_reg(state: &mut GBCState, reg: Register) {
    let val = state.cpu.registers.read(reg);
    op_SUB(state, val);
}

// Subtract value and carry bit from register A and set flags
pub(super) fn op_SBC(state: &mut GBCState, val: u8) {
    let carry = state.cpu.registers.get_flags().cy as u8;
    op_SUB(state, val.wrapping_add(carry));
}

// Subtract register and carry bit from register A and set flags
pub(super) fn op_SBC_reg(state: &mut GBCState, reg: Register) {
    let val = state.cpu.registers.read(reg);
    op_SBC(state, val);
}

// Logic AND value with register A and set flags
pub(super) fn op_AND(state: &mut GBCState, val: u8) {
    let result = state.cpu.registers.read(Register::A) & val;
    state.cpu.registers.write(Register::A, result);

    state.cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: false,
        h: true,
        cy: false,
    });
}

// Logic AND register with register A and set flags
pub(super) fn op_AND_reg(state: &mut GBCState, reg: Register) {
    let val = state.cpu.registers.read(reg);
    op_AND(state, val);
}

// Logic OR value with register A and set flags
pub(super) fn op_OR(state: &mut GBCState, val: u8) {
    let result = state.cpu.registers.read(Register::A) | val;
    state.cpu.registers.write(Register::A, result);

    state.cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: false,
        h: false,
        cy: false,
    });
}

// Logic OR register with register A and set flags
pub(super) fn op_OR_reg(state: &mut GBCState, reg: Register) {
    let val = state.cpu.registers.read(reg);
    op_OR(state, val);
}

// Logic XOR value with register A and set flags
pub(super) fn op_XOR(state: &mut GBCState, val: u8) {
    let result = state.cpu.registers.read(Register::A) ^ val;
    state.cpu.registers.write(Register::A, result);

    state.cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: false,
        h: false,
        cy: false,
    });
}

// Logic XOR register with register A and set flags
pub(super) fn op_XOR_reg(state: &mut GBCState, reg: Register) {
    let val = state.cpu.registers.read(reg);
    op_XOR(state, val);
}

// Compare value with Register A and set flags (Doesn't affect value of register A)
pub(super) fn op_CP(state: &mut GBCState, val: u8) {
    let lhs = state.cpu.registers.read(Register::A);
    let (result, borrows) = subtract_and_get_borrows(lhs, val);

    state.cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: true,
        h: index_bits(borrows, 3),
        cy: index_bits(borrows, 7),
    });
}

// Compare register with register A and set flags
pub(super) fn op_CP_reg(state: &mut GBCState, reg: Register) {
    let val = state.cpu.registers.read(reg);
    op_CP(state, val);
}

// Increment register
pub(super) fn op_INC_reg(state: &mut GBCState, reg: Register) {
    let lhs = state.cpu.registers.read(reg);
    let (result, carries) = add_and_get_carries(lhs, 1);
    state.cpu.registers.write(reg, result);

    state.cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: false,
        h: index_bits(carries, 3),
        cy: state.cpu.registers.get_flags().cy,
    });
}

// Decrement register
pub(super) fn op_DEC_reg(state: &mut GBCState, reg: Register) {
    let lhs = state.cpu.registers.read(reg);
    let (result, borrows) = subtract_and_get_borrows(lhs, 1);
    state.cpu.registers.write(reg, result);

    state.cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: true,
        h: index_bits(borrows, 3),
        cy: state.cpu.registers.get_flags().cy,
    });
}

/**
 * 16-bit Arithmetic Operation Helpers
 */
// Add value to register pair HL and set flags
pub(super) fn op_ADD_u16(state: &mut GBCState, rhs: u16) {
    let lhs = state.cpu.registers.read_pair(RegisterPair::HL);
    let (result, carries) = add_and_get_carries(lhs, rhs);
    state.cpu.registers.write_pair(RegisterPair::HL, result);

    state.cpu.registers.set_flags(&FlagRegister {
        z: state.cpu.registers.get_flags().z,
        n: false,
        h: index_bits(carries, 11),
        cy: index_bits(carries, 15),
    });
}

// Add register pair to register pair HL and set flags
pub(super) fn op_ADD_regpair(state: &mut GBCState, pair: RegisterPair) {
    let rhs = state.cpu.registers.read_pair(pair);
    op_ADD_u16(state, rhs)
}

// Increment register pair
pub(super) fn op_INC_regpair(state: &mut GBCState, pair: RegisterPair) {
    let lhs = state.cpu.registers.read_pair(pair);
    let result = lhs.wrapping_add(1);
    state.cpu.registers.write_pair(pair, result);
}

// Decrement register pair
pub(super) fn op_DEC_regpair(state: &mut GBCState, pair: RegisterPair) {
    let lhs = state.cpu.registers.read_pair(pair);
    let result = lhs.wrapping_sub(1);
    state.cpu.registers.write_pair(pair, result);
}

/**
 * Rotate Shift Instructions
 */
pub(super) fn op_RLC(state: &mut GBCState, val: u8) -> u8 {
    let result = val.rotate_left(1);
    state.cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: false,
        h: false,
        cy: index_bits(val, 7),
    });
    result
}

// Rotate register left. Don't include carry in rotated value
pub(super) fn op_RLC_reg(state: &mut GBCState, reg: Register) {
    let val = state.cpu.registers.read(reg);
    let result = op_RLC(state, val);
    state.cpu.registers.write(reg, result);
}

pub(super) fn op_RL(state: &mut GBCState, val: u8) -> u8 {
    let old_cy = state.cpu.registers.get_flags().cy;
    let result = (val << 1) | (old_cy as u8);
    state.cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: false,
        h: false,
        cy: index_bits(val, 7),
    });
    result
}

// Rotate register left. Include carry in rotated value
pub(super) fn op_RL_reg(state: &mut GBCState, reg: Register) {
    let val = state.cpu.registers.read(reg);
    let result = op_RL(state, val);
    state.cpu.registers.write(reg, result);
}

pub(super) fn op_RRC(state: &mut GBCState, val: u8) -> u8 {
    let result = val.rotate_right(1);
    state.cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: false,
        h: false,
        cy: index_bits(val, 0),
    });
    result
}

// Rotate register right. Don't include carry in rotated value
pub(super) fn op_RRC_reg(state: &mut GBCState, reg: Register) {
    let val = state.cpu.registers.read(reg);
    let result = op_RRC(state, val);
    state.cpu.registers.write(reg, result);
}

pub(super) fn op_RR(state: &mut GBCState, val: u8) -> u8 {
    let old_cy = state.cpu.registers.get_flags().cy;
    let result = ((old_cy as u8) << 7) | (val >> 1);
    state.cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: false,
        h: false,
        cy: index_bits(val, 0),
    });
    result
}

// Rotate register right. Include carry in rotated value
pub(super) fn op_RR_reg(state: &mut GBCState, reg: Register) {
    let val = state.cpu.registers.read(reg);
    let result = op_RR(state, val);
    state.cpu.registers.write(reg, result);
}

pub(super) fn op_SLA(state: &mut GBCState, val: u8) -> u8 {
    let result = val << 1;
    state.cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: false,
        h: false,
        cy: index_bits(val, 7),
    });
    result
}

// Shift register left
pub(super) fn op_SLA_reg(state: &mut GBCState, reg: Register) {
    let val = state.cpu.registers.read(reg);
    let result = op_SLA(state, val);
    state.cpu.registers.write(reg, result);
}

pub(super) fn op_SRA(state: &mut GBCState, val: u8) -> u8 {
    let result = (val & 0x80) | (val >> 1);
    state.cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: false,
        h: false,
        cy: index_bits(val, 0),
    });
    result
}

// Shift register right
pub(super) fn op_SRA_reg(state: &mut GBCState, reg: Register) {
    let val = state.cpu.registers.read(reg);
    let result = op_SRA(state, val);
    state.cpu.registers.write(reg, result);
}

pub(super) fn op_SRL(state: &mut GBCState, val: u8) -> u8 {
    let result = val >> 1;
    state.cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: false,
        h: false,
        cy: index_bits(val, 0),
    });
    result
}

// Shift register right
pub(super) fn op_SRL_reg(state: &mut GBCState, reg: Register) {
    let val = state.cpu.registers.read(reg);
    let result = op_SRL(state, val);
    state.cpu.registers.write(reg, result);
}

pub(super) fn op_SWAP(state: &mut GBCState, val: u8) -> u8 {
    let result = (val << 4) | (val >> 4);
    state.cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: false,
        h: false,
        cy: false,
    });
    result
}

// Swap higher and lower order bits
pub(super) fn op_SWAP_reg(state: &mut GBCState, reg: Register) {
    let val = state.cpu.registers.read(reg);
    let result = op_SWAP(state, val);
    state.cpu.registers.write(reg, result);
}

// Writes to flag Z the complement of the contents of the specified bit
fn op_BIT(state: &mut GBCState, bit: usize, val: u8) {
    state.cpu.registers.set_flags(&FlagRegister {
        z: !index_bits(val, bit),
        n: false,
        h: true,
        cy: state.cpu.registers.get_flags().cy,
    });
}

pub(super) fn op_BIT_reg(state: &mut GBCState, bit: usize, reg: Register) {
    let val = state.cpu.registers.read(reg);
    op_BIT(state, bit, val);
}

pub(super) fn op_BIT_from_HLptr(state: &mut GBCState, bit: usize) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    op_BIT(state, bit, val);
}

pub(super) fn op_SET_reg(state: &mut GBCState, bit: usize, reg: Register) {
    let val = state.cpu.registers.read(reg);
    state.cpu.registers.write(reg, set_bit(val, bit));
}

pub(super) fn op_SET_from_HLptr(state: &mut GBCState, bit: usize) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    virtual_memory::write(state, addr, set_bit(val, bit));
}

pub(super) fn op_RES_reg(state: &mut GBCState, bit: usize, reg: Register) {
    let val = state.cpu.registers.read(reg);
    state.cpu.registers.write(reg, reset_bit(val, bit));
}

pub(super) fn op_RES_from_HLptr(state: &mut GBCState, bit: usize) {
    let addr = state.cpu.registers.read_pair(RegisterPair::HL);
    let val = virtual_memory::read(state, addr);
    virtual_memory::write(state, addr, reset_bit(val, bit));
}

pub(super) fn op_RET(state: &mut GBCState) {
    let pc_low = virtual_memory::read(state, state.cpu.sp);
    state.cpu.sp += 1;
    let pc_high = virtual_memory::read(state, state.cpu.sp);
    state.cpu.sp += 1;
    state.cpu.pc = combine_high_low(pc_high, pc_low);
}

pub(super) fn op_RST(state: &mut GBCState, new_pc: u16) {
    state.cpu.sp -= 1;
    virtual_memory::write(state, state.cpu.sp, state.cpu.pc.high());
    state.cpu.sp -= 1;
    virtual_memory::write(state, state.cpu.sp, state.cpu.pc.low());
    state.cpu.pc = new_pc;
}
