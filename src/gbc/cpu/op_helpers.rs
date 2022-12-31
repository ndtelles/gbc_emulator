#![allow(non_snake_case)]

use crate::{
    gbc::memory::VirtualMemory,
    util::{add_and_get_carries, index_bitmap, reset_bit, set_bit, subtract_and_get_borrows},
};

use super::{
    register::{FlagRegister, Register, RegisterMapMethods, RegisterPair},
    CPU,
};

/**
 * 8-bit Transfer Helpers
 */
// Load value from register to another register
pub(super) fn op_LD_reg_from_reg(cpu: &mut CPU, dest: Register, src: Register) {
    let val = cpu.registers.read(src);
    cpu.registers.write(dest, val);
}

// Load value from register pointer to another register
pub(super) fn op_LD_reg_from_regptr(
    cpu: &mut CPU,
    dest: Register,
    src: Register,
    mem: &VirtualMemory,
) {
    let addr = 0xFF00 | (cpu.registers.read(src) as u16);
    let val = mem.read(addr);
    cpu.registers.write(dest, val);
}

// Load immediate u8 value from PC to register
pub(super) fn op_LD_reg_from_u8(cpu: &mut CPU, dest: Register, mem: &VirtualMemory) {
    let val = super::fetch_and_incr_pc(cpu, mem);
    cpu.registers.write(dest, val);
}

// Load immediate u8 pointer from PC to register
pub(super) fn op_LD_reg_from_u8ptr(cpu: &mut CPU, dest: Register, mem: &VirtualMemory) {
    let addr = 0xFF00 | (super::fetch_and_incr_pc(cpu, mem) as u16);
    let val = mem.read(addr);
    cpu.registers.write(dest, val);
}

// Load immediate u16 pointer from PC to register
pub(super) fn op_LD_reg_from_u16ptr(cpu: &mut CPU, dest: Register, mem: &VirtualMemory) {
    let addr = super::fetch_and_incr_pc_16(cpu, mem);
    let val = mem.read(addr);
    cpu.registers.write(dest, val);
}

// Load register to immediate u8 pointer from PC
pub(super) fn op_LD_u8ptr_from_reg(cpu: &mut CPU, src: Register, mem: &mut VirtualMemory) {
    let val = cpu.registers.read(src);
    let addr = 0xFF00 | (super::fetch_and_incr_pc(cpu, mem) as u16);
    mem.write(addr, val);
}

// Load register to immediate u16 pointer from PC
pub(super) fn op_LD_u16ptr_from_reg(cpu: &mut CPU, src: Register, mem: &mut VirtualMemory) {
    let val = cpu.registers.read(src);
    let addr = super::fetch_and_incr_pc_16(cpu, mem);
    mem.write(addr, val);
}

// Load immediate u8 value from PC to register pair pointer
pub(super) fn op_LD_regpairptr_from_u8(cpu: &mut CPU, dest: RegisterPair, mem: &mut VirtualMemory) {
    let val = super::fetch_and_incr_pc(cpu, mem);
    let addr = cpu.registers.read_pair(dest);
    mem.write(addr, val);
}

// Load value at register pair pointer to register
pub(super) fn op_LD_reg_from_regpairptr(
    cpu: &mut CPU,
    dest: Register,
    src: RegisterPair,
    mem: &VirtualMemory,
) {
    let val = mem.read(cpu.registers.read_pair(src));
    cpu.registers.write(dest, val);
}

// Load value from register to register pair pointer
pub(super) fn op_LD_regpairptr_from_reg(
    cpu: &CPU,
    dest: RegisterPair,
    src: Register,
    mem: &mut VirtualMemory,
) {
    let val = cpu.registers.read(src);
    let addr = cpu.registers.read_pair(dest);
    mem.write(addr, val);
}

// Load value from register to register pointer
pub(super) fn op_LD_regpptr_from_reg(
    cpu: &CPU,
    dest: Register,
    src: Register,
    mem: &mut VirtualMemory,
) {
    let val = cpu.registers.read(src);
    let addr = 0xFF00 | (cpu.registers.read(dest) as u16);
    mem.write(addr, val);
}

/**
 * 16-bit Transfer Helpers
 */
// Load immediate u16 value from PC to register pair
pub(super) fn op_LD_registerpair_from_u16(cpu: &mut CPU, dest: RegisterPair, mem: &VirtualMemory) {
    let val = super::fetch_and_incr_pc_16(cpu, mem);
    cpu.registers.write_pair(dest, val);
}

// Push value from register pair to stack
pub(super) fn op_PUSH_stack_from_regpair(
    cpu: &mut CPU,
    src: RegisterPair,
    mem: &mut VirtualMemory,
) {
    let val = cpu.registers.read_pair(src);
    cpu.sp -= 1;
    // Write high byte
    mem.write(cpu.sp, (val >> 8) as u8);
    cpu.sp -= 1;
    // Write low byte
    mem.write(cpu.sp, val as u8);
}

// Pop value from stack to register pair
pub(super) fn op_POP_stack_to_regpair(cpu: &mut CPU, dest: RegisterPair, mem: &VirtualMemory) {
    let val_low = mem.read(cpu.sp) as u16;
    cpu.sp += 1;
    let val_high = (mem.read(cpu.sp) as u16) << 8;
    cpu.sp += 1;
    let val = val_high | val_low;
    cpu.registers.write_pair(dest, val);
}

/**
 * 8-bit Arithmetic and Logical Operation Helpers
 */
// Add value to register A and set flags
pub(super) fn op_ADD(cpu: &mut CPU, val: u8) {
    let a_val = cpu.registers.read(Register::A);
    let (sum, carries) = add_and_get_carries(a_val, val);
    cpu.registers.write(Register::A, sum);

    cpu.registers.set_flags(&FlagRegister {
        z: sum == 0,
        n: false,
        h: index_bitmap(carries, 3),
        cy: index_bitmap(carries, 7),
    });
}

// Add register to register A and set flags
pub(super) fn op_ADD_reg(cpu: &mut CPU, reg: Register) {
    let val = cpu.registers.read(reg);
    op_ADD(cpu, val);
}

// Add value and carry bit to register A and set flags
pub(super) fn op_ADC(cpu: &mut CPU, val: u8) {
    let carry = cpu.registers.get_flags().cy as u8;
    op_ADD(cpu, val.wrapping_add(carry));
}

// Add value and carry bit to register A and set flags
pub(super) fn op_ADC_reg(cpu: &mut CPU, reg: Register) {
    let val = cpu.registers.read(reg);
    op_ADC(cpu, val);
}

// Subtract value from register A and set flags
pub(super) fn op_SUB(cpu: &mut CPU, val: u8) {
    let a_val = cpu.registers.read(Register::A);
    let (diff, borrows) = subtract_and_get_borrows(a_val, val);
    cpu.registers.write(Register::A, diff);

    cpu.registers.set_flags(&FlagRegister {
        z: diff == 0,
        n: true,
        h: index_bitmap(borrows, 3),
        cy: index_bitmap(borrows, 7),
    });
}

// Subtract register from register A and set flags
pub(super) fn op_SUB_reg(cpu: &mut CPU, reg: Register) {
    let val = cpu.registers.read(reg);
    op_SUB(cpu, val);
}

// Subtract value and carry bit from register A and set flags
pub(super) fn op_SBC(cpu: &mut CPU, val: u8) {
    let carry = cpu.registers.get_flags().cy as u8;
    op_SUB(cpu, val.wrapping_add(carry));
}

// Subtract register and carry bit from register A and set flags
pub(super) fn op_SBC_reg(cpu: &mut CPU, reg: Register) {
    let val = cpu.registers.read(reg);
    op_SBC(cpu, val);
}

// Logic AND value with register A and set flags
pub(super) fn op_AND(cpu: &mut CPU, val: u8) {
    let result = cpu.registers.read(Register::A) & val;
    cpu.registers.write(Register::A, result);

    cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: false,
        h: true,
        cy: false,
    });
}

// Logic AND register with register A and set flags
pub(super) fn op_AND_reg(cpu: &mut CPU, reg: Register) {
    let val = cpu.registers.read(reg);
    op_AND(cpu, val);
}

// Logic OR value with register A and set flags
pub(super) fn op_OR(cpu: &mut CPU, val: u8) {
    let result = cpu.registers.read(Register::A) | val;
    cpu.registers.write(Register::A, result);

    cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: false,
        h: false,
        cy: false,
    });
}

// Logic OR register with register A and set flags
pub(super) fn op_OR_reg(cpu: &mut CPU, reg: Register) {
    let val = cpu.registers.read(reg);
    op_OR(cpu, val);
}

// Logic XOR value with register A and set flags
pub(super) fn op_XOR(cpu: &mut CPU, val: u8) {
    let result = cpu.registers.read(Register::A) ^ val;
    cpu.registers.write(Register::A, result);

    cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: false,
        h: false,
        cy: false,
    });
}

// Logic XOR register with register A and set flags
pub(super) fn op_XOR_reg(cpu: &mut CPU, reg: Register) {
    let val = cpu.registers.read(reg);
    op_XOR(cpu, val);
}

// Compare value with Register A and set flags (Doesn't affect value of register A)
pub(super) fn op_CP(cpu: &mut CPU, val: u8) {
    let lhs = cpu.registers.read(Register::A);
    let (result, borrows) = subtract_and_get_borrows(lhs, val);

    cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: true,
        h: index_bitmap(borrows, 3),
        cy: index_bitmap(borrows, 7),
    });
}

// Compare register with register A and set flags
pub(super) fn op_CP_reg(cpu: &mut CPU, reg: Register) {
    let val = cpu.registers.read(reg);
    op_CP(cpu, val);
}

// Increment register
pub(super) fn op_INC_reg(cpu: &mut CPU, reg: Register) {
    let lhs = cpu.registers.read(reg);
    let (result, carries) = add_and_get_carries(lhs, 1);
    cpu.registers.write(reg, result);

    cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: false,
        h: index_bitmap(carries, 3),
        cy: cpu.registers.get_flags().cy,
    });
}

// Decrement register
pub(super) fn op_DEC_reg(cpu: &mut CPU, reg: Register) {
    let lhs = cpu.registers.read(reg);
    let (result, borrows) = subtract_and_get_borrows(lhs, 1);
    cpu.registers.write(reg, result);

    cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: true,
        h: index_bitmap(borrows, 3),
        cy: cpu.registers.get_flags().cy,
    });
}

/**
 * 16-bit Arithmetic Operation Helpers
 */
// Add value to register pair HL and set flags
pub(super) fn op_ADD_u16(cpu: &mut CPU, rhs: u16) {
    let lhs = cpu.registers.read_pair(RegisterPair::HL);
    let (result, carries) = add_and_get_carries(lhs, rhs);
    cpu.registers.write_pair(RegisterPair::HL, result);

    cpu.registers.set_flags(&FlagRegister {
        z: cpu.registers.get_flags().z,
        n: false,
        h: index_bitmap(carries, 11),
        cy: index_bitmap(carries, 15),
    });
}

// Add register pair to register pair HL and set flags
pub(super) fn op_ADD_regpair(cpu: &mut CPU, pair: RegisterPair) {
    let rhs = cpu.registers.read_pair(pair);
    op_ADD_u16(cpu, rhs)
}

// Increment register pair
pub(super) fn op_INC_regpair(cpu: &mut CPU, pair: RegisterPair) {
    let lhs = cpu.registers.read_pair(pair);
    let result = lhs.wrapping_add(1);
    cpu.registers.write_pair(pair, result);
}

// Decrement register pair
pub(super) fn op_DEC_regpair(cpu: &mut CPU, pair: RegisterPair) {
    let lhs = cpu.registers.read_pair(pair);
    let result = lhs.wrapping_sub(1);
    cpu.registers.write_pair(pair, result);
}

/**
 * Rotate Shift Instructions
 */
pub(super) fn op_RLC(cpu: &mut CPU, val: u8) -> u8 {
    let result = val.rotate_left(1);
    cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: false,
        h: false,
        cy: index_bitmap(val, 7),
    });
    result
}

// Rotate register left. Don't include carry in rotated value
pub(super) fn op_RLC_reg(cpu: &mut CPU, reg: Register) {
    let val = cpu.registers.read(reg);
    let result = op_RLC(cpu, val);
    cpu.registers.write(reg, result);
}

pub(super) fn op_RL(cpu: &mut CPU, val: u8) -> u8 {
    let old_cy = cpu.registers.get_flags().cy;
    let result = (val << 1) | (old_cy as u8);
    cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: false,
        h: false,
        cy: index_bitmap(val, 7),
    });
    result
}

// Rotate register left. Include carry in rotated value
pub(super) fn op_RL_reg(cpu: &mut CPU, reg: Register) {
    let val = cpu.registers.read(reg);
    let result = op_RL(cpu, val);
    cpu.registers.write(reg, result);
}

pub(super) fn op_RRC(cpu: &mut CPU, val: u8) -> u8 {
    let result = val.rotate_right(1);
    cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: false,
        h: false,
        cy: index_bitmap(val, 0),
    });
    result
}

// Rotate register right. Don't include carry in rotated value
pub(super) fn op_RRC_reg(cpu: &mut CPU, reg: Register) {
    let val = cpu.registers.read(reg);
    let result = op_RRC(cpu, val);
    cpu.registers.write(reg, result);
}

pub(super) fn op_RR(cpu: &mut CPU, val: u8) -> u8 {
    let old_cy = cpu.registers.get_flags().cy;
    let result = ((old_cy as u8) << 7) | (val >> 1);
    cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: false,
        h: false,
        cy: index_bitmap(val, 0),
    });
    result
}

// Rotate register right. Include carry in rotated value
pub(super) fn op_RR_reg(cpu: &mut CPU, reg: Register) {
    let val = cpu.registers.read(reg);
    let result = op_RR(cpu, val);
    cpu.registers.write(reg, result);
}

pub(super) fn op_SLA(cpu: &mut CPU, val: u8) -> u8 {
    let result = val << 1;
    cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: false,
        h: false,
        cy: index_bitmap(val, 7),
    });
    result
}

// Shift register left
pub(super) fn op_SLA_reg(cpu: &mut CPU, reg: Register) {
    let val = cpu.registers.read(reg);
    let result = op_SLA(cpu, val);
    cpu.registers.write(reg, result);
}

pub(super) fn op_SRA(cpu: &mut CPU, val: u8) -> u8 {
    let result = (val & 0x80) | (val >> 1);
    cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: false,
        h: false,
        cy: index_bitmap(val, 0),
    });
    result
}

// Shift register right
pub(super) fn op_SRA_reg(cpu: &mut CPU, reg: Register) {
    let val = cpu.registers.read(reg);
    let result = op_SRA(cpu, val);
    cpu.registers.write(reg, result);
}

pub(super) fn op_SRL(cpu: &mut CPU, val: u8) -> u8 {
    let result = val >> 1;
    cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: false,
        h: false,
        cy: index_bitmap(val, 0),
    });
    result
}

// Shift register right
pub(super) fn op_SRL_reg(cpu: &mut CPU, reg: Register) {
    let val = cpu.registers.read(reg);
    let result = op_SRL(cpu, val);
    cpu.registers.write(reg, result);
}

pub(super) fn op_SWAP(cpu: &mut CPU, val: u8) -> u8 {
    let result = (val << 4) | (val >> 4);
    cpu.registers.set_flags(&FlagRegister {
        z: result == 0,
        n: false,
        h: false,
        cy: false,
    });
    result
}

// Swap higher and lower order bits
pub(super) fn op_SWAP_reg(cpu: &mut CPU, reg: Register) {
    let val = cpu.registers.read(reg);
    let result = op_SWAP(cpu, val);
    cpu.registers.write(reg, result);
}

// Writes to flag Z the complement of the contents of the specified bit
fn op_BIT(cpu: &mut CPU, bit: usize, val: u8) {
    cpu.registers.set_flags(&FlagRegister {
        z: !index_bitmap(val, bit),
        n: false,
        h: true,
        cy: cpu.registers.get_flags().cy,
    });
}

pub(super) fn op_BIT_reg(cpu: &mut CPU, bit: usize, reg: Register) {
    let val = cpu.registers.read(reg);
    op_BIT(cpu, bit, val);
}

pub(super) fn op_BIT_from_HLptr(cpu: &mut CPU, bit: usize, mem: &VirtualMemory) {
    let addr = cpu.registers.read_pair(RegisterPair::HL);
    let val = mem.read(addr);
    op_BIT(cpu, bit, val);
}

pub(super) fn op_SET_reg(cpu: &mut CPU, bit: usize, reg: Register) {
    let val = cpu.registers.read(reg);
    cpu.registers.write(reg, set_bit(val, bit));
}

pub(super) fn op_SET_from_HLptr(cpu: &CPU, bit: usize, mem: &mut VirtualMemory) {
    let addr = cpu.registers.read_pair(RegisterPair::HL);
    let val = mem.read(addr);
    mem.write(addr, set_bit(val, bit));
}

pub(super) fn op_RES_reg(cpu: &mut CPU, bit: usize, reg: Register) {
    let val = cpu.registers.read(reg);
    cpu.registers.write(reg, reset_bit(val, bit));
}

pub(super) fn op_RES_from_HLptr(cpu: &mut CPU, bit: usize, mem: &mut VirtualMemory) {
    let addr = cpu.registers.read_pair(RegisterPair::HL);
    let val = mem.read(addr);
    mem.write(addr, reset_bit(val, bit));
}
