use crate::{
    gbc::memory::VirtualMemory,
    util::{add_and_get_carries, index_bitmap, subtract_and_get_borrows},
};

use super::{
    register::{FlagRegister, Register, RegisterMapMethods, RegisterPair},
    CPU,
};

#[allow(non_snake_case)]
impl CPU {
    /**
     * 8-bit Transfer Helpers
     */
    // Load value from register to another register
    pub(super) fn op_LD_reg_from_reg(&mut self, dest: Register, src: Register) {
        let val = self.registers.read(src);
        self.registers.write(dest, val);
    }

    // Load value from register pointer to another register
    pub(super) fn op_LD_reg_from_regptr(
        &mut self,
        dest: Register,
        src: Register,
        mem: &VirtualMemory,
    ) {
        let addr = 0xFF00 | (self.registers.read(src) as u16);
        let val = mem.read(addr);
        self.registers.write(dest, val);
    }

    // Load immediate u8 value from PC to register
    pub(super) fn op_LD_reg_from_u8(&mut self, dest: Register, mem: &VirtualMemory) {
        let val = self.fetch_and_incr_pc(mem);
        self.registers.write(dest, val);
    }

    // Load immediate u8 pointer from PC to register
    pub(super) fn op_LD_reg_from_u8ptr(&mut self, dest: Register, mem: &VirtualMemory) {
        let addr = 0xFF00 | (self.fetch_and_incr_pc(mem) as u16);
        let val = mem.read(addr);
        self.registers.write(dest, val);
    }

    // Load immediate u16 pointer from PC to register
    pub(super) fn op_LD_reg_from_u16ptr(&mut self, dest: Register, mem: &VirtualMemory) {
        let addr = self.fetch_and_incr_pc_16(mem);
        let val = mem.read(addr);
        self.registers.write(dest, val);
    }

    // Load register to immediate u8 pointer from PC
    pub(super) fn op_LD_u8ptr_from_reg(&mut self, src: Register, mem: &mut VirtualMemory) {
        let val = self.registers.read(src);
        let addr = 0xFF00 | (self.fetch_and_incr_pc(mem) as u16);
        mem.write(addr, val);
    }

    // Load register to immediate u16 pointer from PC
    pub(super) fn op_LD_u16ptr_from_reg(&mut self, src: Register, mem: &mut VirtualMemory) {
        let val = self.registers.read(src);
        let addr = self.fetch_and_incr_pc_16(mem);
        mem.write(addr, val);
    }

    // Load immediate u8 value from PC to register pair pointer
    pub(super) fn op_LD_regpairptr_from_u8(&mut self, dest: RegisterPair, mem: &mut VirtualMemory) {
        let val = self.fetch_and_incr_pc(mem);
        let addr = self.registers.read_pair(dest);
        mem.write(addr, val);
    }

    // Load value at register pair pointer to register
    pub(super) fn op_LD_reg_from_regpairptr(
        &mut self,
        dest: Register,
        src: RegisterPair,
        mem: &VirtualMemory,
    ) {
        let val = mem.read(self.registers.read_pair(src));
        self.registers.write(dest, val);
    }

    // Load value from register to register pair pointer
    pub(super) fn op_LD_regpairptr_from_reg(
        &self,
        dest: RegisterPair,
        src: Register,
        mem: &mut VirtualMemory,
    ) {
        let val = self.registers.read(src);
        let addr = self.registers.read_pair(dest);
        mem.write(addr, val);
    }

    // Load value from register to register pointer
    pub(super) fn op_LD_regpptr_from_reg(
        &self,
        dest: Register,
        src: Register,
        mem: &mut VirtualMemory,
    ) {
        let val = self.registers.read(src);
        let addr = 0xFF00 | (self.registers.read(dest) as u16);
        mem.write(addr, val);
    }

    /**
     * 16-bit Transfer Helpers
     */
    // Load immediate u16 value from PC to register pair
    pub(super) fn op_LD_registerpair_from_u16(&mut self, dest: RegisterPair, mem: &VirtualMemory) {
        let val = self.fetch_and_incr_pc_16(mem);
        self.registers.write_pair(dest, val);
    }

    // Push value from register pair to stack
    pub(super) fn op_PUSH_stack_from_regpair(
        &mut self,
        src: RegisterPair,
        mem: &mut VirtualMemory,
    ) {
        let val = self.registers.read_pair(src);
        self.sp -= 1;
        // Write high byte
        mem.write(self.sp, (val >> 8) as u8);
        self.sp -= 1;
        // Write low byte
        mem.write(self.sp, val as u8);
    }

    // Pop value from stack to register pair
    pub(super) fn op_POP_stack_to_regpair(&mut self, dest: RegisterPair, mem: &VirtualMemory) {
        let val_low = mem.read(self.sp) as u16;
        self.sp += 1;
        let val_high = (mem.read(self.sp) as u16) << 8;
        self.sp += 1;
        let val = val_high | val_low;
        self.registers.write_pair(dest, val);
    }

    /**
     * 8-bit Arithmetic and Logical Operation Helpers
     */
    // Add value to register A and set flags
    pub(super) fn op_ADD(&mut self, val: u8) {
        let a_val = self.registers.read(Register::A);
        let (sum, carries) = add_and_get_carries(a_val, val);
        self.registers.write(Register::A, sum);

        self.registers.set_flags(&FlagRegister {
            z: sum == 0,
            n: false,
            h: index_bitmap(carries, 3),
            cy: index_bitmap(carries, 7),
        });
    }

    // Add register to register A and set flags
    pub(super) fn op_ADD_reg(&mut self, reg: Register) {
        let val = self.registers.read(reg);
        self.op_ADD(val);
    }

    // Add value and carry bit to register A and set flags
    pub(super) fn op_ADC(&mut self, val: u8) {
        let carry = self.registers.get_flags().cy as u8;
        self.op_ADD(val.wrapping_add(carry));
    }

    // Add value and carry bit to register A and set flags
    pub(super) fn op_ADC_reg(&mut self, reg: Register) {
        let val = self.registers.read(reg);
        self.op_ADC(val);
    }

    // Subtract value from register A and set flags
    pub(super) fn op_SUB(&mut self, val: u8) {
        let a_val = self.registers.read(Register::A);
        let (diff, borrows) = subtract_and_get_borrows(a_val, val);
        self.registers.write(Register::A, diff);

        self.registers.set_flags(&FlagRegister {
            z: diff == 0,
            n: true,
            h: index_bitmap(borrows, 3),
            cy: index_bitmap(borrows, 7),
        });
    }

    // Subtract register from register A and set flags
    pub(super) fn op_SUB_reg(&mut self, reg: Register) {
        let val = self.registers.read(reg);
        self.op_SUB(val);
    }

    // Subtract value and carry bit from register A and set flags
    pub(super) fn op_SBC(&mut self, val: u8) {
        let carry = self.registers.get_flags().cy as u8;
        self.op_SUB(val.wrapping_add(carry));
    }

    // Subtract register and carry bit from register A and set flags
    pub(super) fn op_SBC_reg(&mut self, reg: Register) {
        let val = self.registers.read(reg);
        self.op_SBC(val);
    }

    // Logic AND value with register A and set flags
    pub(super) fn op_AND(&mut self, val: u8) {
        let result = self.registers.read(Register::A) & val;
        self.registers.write(Register::A, result);

        self.registers.set_flags(&FlagRegister {
            z: result == 0,
            n: false,
            h: true,
            cy: false,
        });
    }

    // Logic AND register with register A and set flags
    pub(super) fn op_AND_reg(&mut self, reg: Register) {
        let val = self.registers.read(reg);
        self.op_AND(val);
    }

    // Logic OR value with register A and set flags
    pub(super) fn op_OR(&mut self, val: u8) {
        let result = self.registers.read(Register::A) | val;
        self.registers.write(Register::A, result);

        self.registers.set_flags(&FlagRegister {
            z: result == 0,
            n: false,
            h: false,
            cy: false,
        });
    }

    // Logic OR register with register A and set flags
    pub(super) fn op_OR_reg(&mut self, reg: Register) {
        let val = self.registers.read(reg);
        self.op_OR(val);
    }

    // Logic XOR value with register A and set flags
    pub(super) fn op_XOR(&mut self, val: u8) {
        let result = self.registers.read(Register::A) ^ val;
        self.registers.write(Register::A, result);

        self.registers.set_flags(&FlagRegister {
            z: result == 0,
            n: false,
            h: false,
            cy: false,
        });
    }

    // Logic XOR register with register A and set flags
    pub(super) fn op_XOR_reg(&mut self, reg: Register) {
        let val = self.registers.read(reg);
        self.op_XOR(val);
    }

    // Compare value with Register A and set flags (Doesn't affect value of register A)
    pub(super) fn op_CP(&mut self, val: u8) {
        let lhs = self.registers.read(Register::A);
        let (result, borrows) = subtract_and_get_borrows(lhs, val);

        self.registers.set_flags(&FlagRegister {
            z: result == 0,
            n: true,
            h: index_bitmap(borrows, 3),
            cy: index_bitmap(borrows, 7),
        });
    }

    // Compare register with register A and set flags
    pub(super) fn op_CP_reg(&mut self, reg: Register) {
        let val = self.registers.read(reg);
        self.op_CP(val);
    }

    // Increment register
    pub(super) fn op_INC_reg(&mut self, reg: Register) {
        let lhs = self.registers.read(reg);
        let (result, carries) = add_and_get_carries(lhs, 1);
        self.registers.write(reg, result);

        self.registers.set_flags(&FlagRegister {
            z: result == 0,
            n: false,
            h: index_bitmap(carries, 3),
            cy: self.registers.get_flags().cy,
        });
    }

    // Decrement register
    pub(super) fn op_DEC_reg(&mut self, reg: Register) {
        let lhs = self.registers.read(reg);
        let (result, borrows) = subtract_and_get_borrows(lhs, 1);
        self.registers.write(reg, result);

        self.registers.set_flags(&FlagRegister {
            z: result == 0,
            n: true,
            h: index_bitmap(borrows, 3),
            cy: self.registers.get_flags().cy,
        });
    }

    /**
     * 16-bit Arithmetic Operation Helpers
     */
    // Add value to register pair HL and set flags
    pub(super) fn op_ADD_u16(&mut self, rhs: u16) {
        let lhs = self.registers.read_pair(RegisterPair::HL);
        let (result, carries) = add_and_get_carries(lhs, rhs);
        self.registers.write_pair(RegisterPair::HL, result);

        self.registers.set_flags(&FlagRegister {
            z: self.registers.get_flags().z,
            n: false,
            h: index_bitmap(carries, 11),
            cy: index_bitmap(carries, 15),
        });
    }

    // Add register pair to register pair HL and set flags
    pub(super) fn op_ADD_regpair(&mut self, pair: RegisterPair) {
        let rhs = self.registers.read_pair(pair);
        self.op_ADD_u16(rhs)
    }

    // Increment register pair
    pub(super) fn op_INC_regpair(&mut self, pair: RegisterPair) {
        let lhs = self.registers.read_pair(pair);
        let result = lhs.wrapping_add(1);
        self.registers.write_pair(pair, result);
    }

    // Decrement register pair
    pub(super) fn op_DEC_regpair(&mut self, pair: RegisterPair) {
        let lhs = self.registers.read_pair(pair);
        let result = lhs.wrapping_sub(1);
        self.registers.write_pair(pair, result);
    }

    /**
     * Rotate Shift Instructions
     */
    // Rotate register left. Don't include carry in rotated value
    pub(super) fn op_RLC_reg(&mut self, reg: Register) {
        let val = self.registers.read(reg);
        let result = val.rotate_left(1);
        self.registers.set_flags(&FlagRegister {
            z: result == 0,
            n: false,
            h: false,
            cy: index_bitmap(val, 7),
        });
        self.registers.write(reg, result);
    }

    // Rotate register left. Include carry in rotated value
    pub(super) fn op_RL_reg(&mut self, reg: Register) {
        let val = self.registers.read(reg);
        let old_cy = self.registers.get_flags().cy;
        let result = (val << 1) | (old_cy as u8);

        self.registers.set_flags(&FlagRegister {
            z: result == 0,
            n: false,
            h: false,
            cy: index_bitmap(val, 7),
        });
        self.registers.write(reg, result);
    }
}
