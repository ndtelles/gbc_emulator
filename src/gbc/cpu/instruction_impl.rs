use crate::gbc::memory::VirtualMemory;

use super::{
    register::{Register, RegisterMapMethods, RegisterPair},
    CPU,
};

#[allow(non_snake_case)]
impl CPU {
    // Load value from register to another register
    fn ld_reg_from_reg(&mut self, dest: Register, src: Register) {
        let val = self.registers.read(src);
        self.registers.write(dest, val);
    }

    // Load value from register pointer to another register
    fn ld_reg_from_regptr(&mut self, dest: Register, src: Register, mem: &VirtualMemory) {
        let addr = 0xFF00 | (self.registers.read(src) as u16);
        let val = mem.read(addr);
        self.registers.write(dest, val);
    }

    // Load immediate u8 value from PC to register
    fn ld_reg_from_u8(&mut self, dest: Register, mem: &VirtualMemory) {
        let val = self.fetch(mem);
        self.registers.write(dest, val);
    }

    // Load immediate u8 pointer from PC to register
    fn ld_reg_from_u8ptr(&mut self, dest: Register, mem: &VirtualMemory) {
        let addr = 0xFF00 | (self.fetch(mem) as u16);
        let val = mem.read(addr);
        self.registers.write(dest, val);
    }

    // Load immediate u16 pointer from PC to register
    fn ld_reg_from_u16ptr(&mut self, dest: Register, mem: &VirtualMemory) {
        let addr = self.fetch_16(mem);
        let val = mem.read(addr);
        self.registers.write(dest, val);
    }

    // Load register to immediate u8 pointer from PC
    fn ld_u8ptr_from_reg(&mut self, src: Register, mem: &mut VirtualMemory) {
        let val = self.registers.read(src);
        let addr = 0xFF00 | (self.fetch(mem) as u16);
        mem.write(addr, val);
    }

    // Load register to immediate u16 pointer from PC
    fn ld_u16ptr_from_reg(&mut self, src: Register, mem: &mut VirtualMemory) {
        let val = self.registers.read(src);
        let addr = self.fetch_16(mem);
        mem.write(addr, val);
    }

    // Load immediate u8 value from PC to register pair pointer
    fn ld_regpairptr_from_u8(&mut self, dest: RegisterPair, mem: &mut VirtualMemory) {
        let val = self.fetch(mem);
        let addr = self.registers.read_pair(dest);
        mem.write(addr, val);
    }

    // Load value at register pair pointer to register
    fn ld_reg_from_regpairptr(&mut self, dest: Register, src: RegisterPair, mem: &VirtualMemory) {
        let val = mem.read(self.registers.read_pair(src));
        self.registers.write(dest, val);
    }

    // Load value from register to register pair pointer
    fn ld_regpairptr_from_reg(&self, dest: RegisterPair, src: Register, mem: &mut VirtualMemory) {
        let val = self.registers.read(src);
        let addr = self.registers.read_pair(dest);
        mem.write(addr, val);
    }

    // Load value from register to register pointer
    fn ld_regpptr_from_reg(&self, dest: Register, src: Register, mem: &mut VirtualMemory) {
        let val = self.registers.read(src);
        let addr = 0xFF00 | (self.registers.read(dest) as u16);
        mem.write(addr, val);
    }

    // NOP
    pub(super) fn instr_0x00(&mut self, _mem: &mut VirtualMemory) {}

    pub(super) fn instr_0x01(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    // LD (BC), A
    pub(super) fn instr_0x02(&mut self, mem: &mut VirtualMemory) {
        self.ld_regpairptr_from_reg(RegisterPair::BC, Register::A, mem);
    }

    pub(super) fn instr_0x03(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x04(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x05(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    // LD B, u8
    pub(super) fn instr_0x06(&mut self, mem: &mut VirtualMemory) {
        self.ld_reg_from_u8(Register::B, mem);
    }

    pub(super) fn instr_0x07(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x08(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x09(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    // LD A, (BC)
    pub(super) fn instr_0x0A(&mut self, mem: &mut VirtualMemory) {
        self.ld_reg_from_regpairptr(Register::A, RegisterPair::BC, mem);
    }

    pub(super) fn instr_0x0B(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x0C(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x0D(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    // LD C, u8
    pub(super) fn instr_0x0E(&mut self, mem: &mut VirtualMemory) {
        self.ld_reg_from_u8(Register::C, mem);
    }

    pub(super) fn instr_0x0F(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x10(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x11(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    // LD (DE), A
    pub(super) fn instr_0x12(&mut self, mem: &mut VirtualMemory) {
        self.ld_reg_from_regpairptr(Register::A, RegisterPair::DE, mem);
    }

    pub(super) fn instr_0x13(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x14(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x15(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    // LD D, u8
    pub(super) fn instr_0x16(&mut self, mem: &mut VirtualMemory) {
        self.ld_reg_from_u8(Register::D, mem);
    }

    pub(super) fn instr_0x17(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x18(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x19(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    // LD A, (DE)
    pub(super) fn instr_0x1A(&mut self, mem: &mut VirtualMemory) {
        self.ld_reg_from_regpairptr(Register::A, RegisterPair::DE, mem);
    }

    pub(super) fn instr_0x1B(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x1C(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x1D(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    // LD E, u8
    pub(super) fn instr_0x1E(&mut self, mem: &mut VirtualMemory) {
        self.ld_reg_from_u8(Register::E, mem);
    }

    pub(super) fn instr_0x1F(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x20(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x21(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    // LD (HLI), A
    pub(super) fn instr_0x22(&mut self, mem: &mut VirtualMemory) {
        self.ld_regpairptr_from_reg(RegisterPair::HL, Register::A, mem);
        let new_hl = self.registers.read_pair(RegisterPair::HL).wrapping_add(1);
        self.registers.write_pair(RegisterPair::HL, new_hl);
    }

    pub(super) fn instr_0x23(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x24(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x25(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    // LD H, u8
    pub(super) fn instr_0x26(&mut self, mem: &mut VirtualMemory) {
        self.ld_reg_from_u8(Register::H, mem);
    }

    pub(super) fn instr_0x27(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x28(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x29(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    // LD A, (HLI)
    pub(super) fn instr_0x2A(&mut self, mem: &mut VirtualMemory) {
        self.ld_reg_from_regpairptr(Register::A, RegisterPair::HL, mem);
        let new_hl = self.registers.read_pair(RegisterPair::HL).wrapping_add(1);
        self.registers.write_pair(RegisterPair::HL, new_hl);
    }

    pub(super) fn instr_0x2B(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x2C(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x2D(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    // LD L, u8
    pub(super) fn instr_0x2E(&mut self, mem: &mut VirtualMemory) {
        self.ld_reg_from_u8(Register::L, mem);
    }

    pub(super) fn instr_0x2F(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x30(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x31(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    // LD (HLD), A
    pub(super) fn instr_0x32(&mut self, mem: &mut VirtualMemory) {
        self.ld_regpairptr_from_reg(RegisterPair::HL, Register::A, mem);
        let new_hl = self.registers.read_pair(RegisterPair::HL).wrapping_sub(1);
        self.registers.write_pair(RegisterPair::HL, new_hl);
    }

    pub(super) fn instr_0x33(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x34(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x35(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    // LD (HL), u8
    pub(super) fn instr_0x36(&mut self, mem: &mut VirtualMemory) {
        self.ld_regpairptr_from_u8(RegisterPair::HL, mem);
    }

    pub(super) fn instr_0x37(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x38(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x39(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    // LD A, (HLD)
    pub(super) fn instr_0x3A(&mut self, mem: &mut VirtualMemory) {
        self.ld_reg_from_regpairptr(Register::A, RegisterPair::HL, mem);
        let new_hl = self.registers.read_pair(RegisterPair::HL).wrapping_sub(1);
        self.registers.write_pair(RegisterPair::HL, new_hl);
    }

    pub(super) fn instr_0x3B(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x3C(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x3D(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    // LD A, u8
    pub(super) fn instr_0x3E(&mut self, mem: &mut VirtualMemory) {
        self.ld_reg_from_u8(Register::A, mem);
    }

    pub(super) fn instr_0x3F(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    // LD B, B
    pub(super) fn instr_0x40(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::B, Register::B);
    }

    // LD B, C
    pub(super) fn instr_0x41(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::B, Register::C);
    }

    // LD B, D
    pub(super) fn instr_0x42(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::B, Register::D);
    }

    // LD B, E
    pub(super) fn instr_0x43(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::B, Register::E);
    }

    // LD B, H
    pub(super) fn instr_0x44(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::B, Register::H);
    }

    // LD B, L
    pub(super) fn instr_0x45(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::B, Register::L);
    }

    // LD B, (HL)
    pub(super) fn instr_0x46(&mut self, mem: &mut VirtualMemory) {
        self.ld_reg_from_regpairptr(Register::B, RegisterPair::HL, mem);
    }

    // LD B, A
    pub(super) fn instr_0x47(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::B, Register::A);
    }

    // LD C, B
    pub(super) fn instr_0x48(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::C, Register::B);
    }

    // LD C, C
    pub(super) fn instr_0x49(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::C, Register::C);
    }

    // LD C, D
    pub(super) fn instr_0x4A(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::C, Register::D);
    }

    // LD C, E
    pub(super) fn instr_0x4B(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::C, Register::E);
    }

    // LD C, H
    pub(super) fn instr_0x4C(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::C, Register::H);
    }

    // LD C, L
    pub(super) fn instr_0x4D(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::C, Register::L);
    }

    // LD C, (HL)
    pub(super) fn instr_0x4E(&mut self, mem: &mut VirtualMemory) {
        self.ld_reg_from_regpairptr(Register::C, RegisterPair::HL, mem);
    }

    // LD C, A
    pub(super) fn instr_0x4F(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::C, Register::A);
    }

    // LD D, B
    pub(super) fn instr_0x50(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::D, Register::B);
    }

    // LD D, C
    pub(super) fn instr_0x51(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::D, Register::C);
    }

    // LD D, D
    pub(super) fn instr_0x52(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::D, Register::D);
    }

    // LD D, E
    pub(super) fn instr_0x53(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::D, Register::E);
    }

    // LD D, H
    pub(super) fn instr_0x54(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::D, Register::H);
    }

    // LD D, L
    pub(super) fn instr_0x55(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::D, Register::L);
    }

    // LD D, (HL)
    pub(super) fn instr_0x56(&mut self, mem: &mut VirtualMemory) {
        self.ld_reg_from_regpairptr(Register::D, RegisterPair::HL, mem);
    }

    // LD D, A
    pub(super) fn instr_0x57(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::D, Register::A);
    }

    // LD E, B
    pub(super) fn instr_0x58(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::E, Register::B);
    }

    // LD E, C
    pub(super) fn instr_0x59(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::E, Register::C);
    }

    // LD E, D
    pub(super) fn instr_0x5A(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::E, Register::D);
    }

    // LD E, E
    pub(super) fn instr_0x5B(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::E, Register::E);
    }

    // LD E, H
    pub(super) fn instr_0x5C(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::E, Register::H);
    }

    // LD E, L
    pub(super) fn instr_0x5D(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::E, Register::L);
    }

    // LD E, (HL)
    pub(super) fn instr_0x5E(&mut self, mem: &mut VirtualMemory) {
        self.ld_reg_from_regpairptr(Register::E, RegisterPair::HL, mem);
    }

    // LD E, A
    pub(super) fn instr_0x5F(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::E, Register::A);
    }

    // LD H, B
    pub(super) fn instr_0x60(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::H, Register::B);
    }

    // LD H, C
    pub(super) fn instr_0x61(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::H, Register::C);
    }

    // LD H, D
    pub(super) fn instr_0x62(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::H, Register::D);
    }

    // LD H, E
    pub(super) fn instr_0x63(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::H, Register::E);
    }

    // LD H, H
    pub(super) fn instr_0x64(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::H, Register::H);
    }

    // LD H, L
    pub(super) fn instr_0x65(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::H, Register::L);
    }

    // LD H, (HL)
    pub(super) fn instr_0x66(&mut self, mem: &mut VirtualMemory) {
        self.ld_reg_from_regpairptr(Register::H, RegisterPair::HL, mem);
    }

    // LD H, A
    pub(super) fn instr_0x67(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::H, Register::A);
    }

    // LD L, B
    pub(super) fn instr_0x68(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::L, Register::B);
    }

    // LD L, C
    pub(super) fn instr_0x69(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::L, Register::C);
    }

    // LD L, D
    pub(super) fn instr_0x6A(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::L, Register::D);
    }

    // LD L, E
    pub(super) fn instr_0x6B(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::L, Register::E);
    }

    // LD L, H
    pub(super) fn instr_0x6C(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::L, Register::H);
    }

    // LD L, L
    pub(super) fn instr_0x6D(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::L, Register::L);
    }

    // LD L, (HL)
    pub(super) fn instr_0x6E(&mut self, mem: &mut VirtualMemory) {
        self.ld_reg_from_regpairptr(Register::L, RegisterPair::HL, mem);
    }

    // LD L, A
    pub(super) fn instr_0x6F(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::L, Register::A);
    }

    // LD (HL). B
    pub(super) fn instr_0x70(&mut self, mem: &mut VirtualMemory) {
        self.ld_regpairptr_from_reg(RegisterPair::HL, Register::B, mem);
    }

    // LD (HL). C
    pub(super) fn instr_0x71(&mut self, mem: &mut VirtualMemory) {
        self.ld_regpairptr_from_reg(RegisterPair::HL, Register::C, mem);
    }

    // LD (HL). D
    pub(super) fn instr_0x72(&mut self, mem: &mut VirtualMemory) {
        self.ld_regpairptr_from_reg(RegisterPair::HL, Register::D, mem);
    }

    // LD (HL). E
    pub(super) fn instr_0x73(&mut self, mem: &mut VirtualMemory) {
        self.ld_regpairptr_from_reg(RegisterPair::HL, Register::E, mem);
    }

    // LD (HL). H
    pub(super) fn instr_0x74(&mut self, mem: &mut VirtualMemory) {
        self.ld_regpairptr_from_reg(RegisterPair::HL, Register::H, mem);
    }

    // LD (HL). L
    pub(super) fn instr_0x75(&mut self, mem: &mut VirtualMemory) {
        self.ld_regpairptr_from_reg(RegisterPair::HL, Register::L, mem);
    }

    pub(super) fn instr_0x76(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    // LD (HL), A
    pub(super) fn instr_0x77(&mut self, mem: &mut VirtualMemory) {
        self.ld_regpairptr_from_reg(RegisterPair::HL, Register::A, mem);
    }

    // LD A, B
    pub(super) fn instr_0x78(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::A, Register::B);
    }

    // LD A, C
    pub(super) fn instr_0x79(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::A, Register::C);
    }

    // LD A, D
    pub(super) fn instr_0x7A(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::A, Register::D);
    }

    // LD A, E
    pub(super) fn instr_0x7B(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::A, Register::E);
    }

    // LD A, H
    pub(super) fn instr_0x7C(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::A, Register::H);
    }

    // LD A, L
    pub(super) fn instr_0x7D(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::A, Register::L);
    }

    // LD A, (HL)
    pub(super) fn instr_0x7E(&mut self, mem: &mut VirtualMemory) {
        self.ld_reg_from_regpairptr(Register::A, RegisterPair::HL, mem);
    }

    // LD A, A
    pub(super) fn instr_0x7F(&mut self, _mem: &mut VirtualMemory) {
        self.ld_reg_from_reg(Register::A, Register::A);
    }

    pub(super) fn instr_0x80(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x81(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x82(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x83(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x84(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x85(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x86(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x87(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x88(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x89(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x8A(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x8B(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x8C(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x8D(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x8E(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x8F(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x90(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x91(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x92(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x93(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x94(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x95(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x96(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x97(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x98(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x99(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x9A(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x9B(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x9C(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x9D(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x9E(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0x9F(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xA0(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xA1(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xA2(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xA3(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xA4(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xA5(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xA6(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xA7(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xA8(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xA9(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xAA(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xAB(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xAC(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xAD(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xAE(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xAF(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xB0(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xB1(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xB2(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xB3(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xB4(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xB5(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xB6(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xB7(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xB8(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xB9(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xBA(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xBB(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xBC(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xBD(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xBE(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xBF(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xC0(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xC1(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xC2(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xC3(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xC4(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xC5(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xC6(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xC7(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xC8(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xC9(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCA(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCC(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCD(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCE(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCF(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xD0(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xD1(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xD2(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xD3(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xD4(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xD5(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xD6(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xD7(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xD8(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xD9(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xDA(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xDB(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xDC(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xDD(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xDE(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xDF(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    // LD (u8), A
    pub(super) fn instr_0xE0(&mut self, mem: &mut VirtualMemory) {
        self.ld_u8ptr_from_reg(Register::A, mem);
    }

    pub(super) fn instr_0xE1(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    // LD (C), A
    pub(super) fn instr_0xE2(&mut self, mem: &mut VirtualMemory) {
        self.ld_regpptr_from_reg(Register::C, Register::A, mem);
    }

    pub(super) fn instr_0xE3(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xE4(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xE5(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xE6(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xE7(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xE8(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xE9(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    // LD (u16), A
    pub(super) fn instr_0xEA(&mut self, mem: &mut VirtualMemory) {
        self.ld_u16ptr_from_reg(Register::A, mem);
    }

    pub(super) fn instr_0xEB(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xEC(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xED(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xEE(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xEF(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    // LD A, (u8)
    pub(super) fn instr_0xF0(&mut self, mem: &mut VirtualMemory) {
        self.ld_reg_from_u8ptr(Register::A, mem);
    }

    pub(super) fn instr_0xF1(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    // LD A, (C)
    pub(super) fn instr_0xF2(&mut self, mem: &mut VirtualMemory) {
        self.ld_reg_from_regptr(Register::A, Register::C, mem);
    }

    pub(super) fn instr_0xF3(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xF4(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xF5(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xF6(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xF7(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xF8(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xF9(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    // LD A, (u16)
    pub(super) fn instr_0xFA(&mut self, mem: &mut VirtualMemory) {
        self.ld_reg_from_u16ptr(Register::A, mem);
    }

    pub(super) fn instr_0xFB(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xFC(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xFD(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xFE(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xFF(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }
}
