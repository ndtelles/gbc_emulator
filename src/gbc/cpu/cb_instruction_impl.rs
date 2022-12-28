use crate::gbc::memory::VirtualMemory;

use super::{
    register::{Register, RegisterMapMethods, RegisterPair},
    CPU,
};

#[allow(non_snake_case)]
impl CPU {
    // RLC B
    pub(super) fn instr_0xCB00(&mut self, _mem: &mut VirtualMemory) {
        self.op_RLC_reg(Register::B);
    }

    // RLC C
    pub(super) fn instr_0xCB01(&mut self, _mem: &mut VirtualMemory) {
        self.op_RLC_reg(Register::C);
    }

    // RLC D
    pub(super) fn instr_0xCB02(&mut self, _mem: &mut VirtualMemory) {
        self.op_RLC_reg(Register::D);
    }

    // RLC E
    pub(super) fn instr_0xCB03(&mut self, _mem: &mut VirtualMemory) {
        self.op_RLC_reg(Register::E);
    }

    // RLC H
    pub(super) fn instr_0xCB04(&mut self, _mem: &mut VirtualMemory) {
        self.op_RLC_reg(Register::H);
    }

    // RLC L
    pub(super) fn instr_0xCB05(&mut self, _mem: &mut VirtualMemory) {
        self.op_RLC_reg(Register::L);
    }

    // RLC (HL)
    pub(super) fn instr_0xCB06(&mut self, mem: &mut VirtualMemory) {
        let addr = self.registers.read_pair(RegisterPair::HL);
        let val = mem.read(addr);
        let result = self.op_RLC(val);
        mem.write(addr, result);
    }

    // RLC A
    pub(super) fn instr_0xCB07(&mut self, _mem: &mut VirtualMemory) {
        self.op_RLC_reg(Register::A);
    }

    // RRC B
    pub(super) fn instr_0xCB08(&mut self, _mem: &mut VirtualMemory) {
        self.op_RRC_reg(Register::B);
    }

    // RRC C
    pub(super) fn instr_0xCB09(&mut self, _mem: &mut VirtualMemory) {
        self.op_RRC_reg(Register::C);
    }

    // RRC D
    pub(super) fn instr_0xCB0A(&mut self, _mem: &mut VirtualMemory) {
        self.op_RRC_reg(Register::D);
    }

    // RRC E
    pub(super) fn instr_0xCB0B(&mut self, _mem: &mut VirtualMemory) {
        self.op_RRC_reg(Register::E);
    }

    // RRC H
    pub(super) fn instr_0xCB0C(&mut self, _mem: &mut VirtualMemory) {
        self.op_RRC_reg(Register::H);
    }

    // RRC L
    pub(super) fn instr_0xCB0D(&mut self, _mem: &mut VirtualMemory) {
        self.op_RRC_reg(Register::L);
    }

    // RRC (HL)
    pub(super) fn instr_0xCB0E(&mut self, mem: &mut VirtualMemory) {
        let addr = self.registers.read_pair(RegisterPair::HL);
        let val = mem.read(addr);
        let result = self.op_RRC(val);
        mem.write(addr, result);
    }

    // RRC A
    pub(super) fn instr_0xCB0F(&mut self, _mem: &mut VirtualMemory) {
        self.op_RRC_reg(Register::A);
    }

    // RL B
    pub(super) fn instr_0xCB10(&mut self, _mem: &mut VirtualMemory) {
        self.op_RL_reg(Register::B);
    }

    // RL C
    pub(super) fn instr_0xCB11(&mut self, _mem: &mut VirtualMemory) {
        self.op_RL_reg(Register::C);
    }

    // RL D
    pub(super) fn instr_0xCB12(&mut self, _mem: &mut VirtualMemory) {
        self.op_RL_reg(Register::D);
    }

    // RL E
    pub(super) fn instr_0xCB13(&mut self, _mem: &mut VirtualMemory) {
        self.op_RL_reg(Register::E);
    }

    // RL H
    pub(super) fn instr_0xCB14(&mut self, _mem: &mut VirtualMemory) {
        self.op_RL_reg(Register::H);
    }

    // RL L
    pub(super) fn instr_0xCB15(&mut self, _mem: &mut VirtualMemory) {
        self.op_RL_reg(Register::L);
    }

    // RL (HL)
    pub(super) fn instr_0xCB16(&mut self, mem: &mut VirtualMemory) {
        let addr = self.registers.read_pair(RegisterPair::HL);
        let val = mem.read(addr);
        let result = self.op_RL(val);
        mem.write(addr, result);
    }

    // RL A
    pub(super) fn instr_0xCB17(&mut self, _mem: &mut VirtualMemory) {
        self.op_RL_reg(Register::A);
    }

    // RR B
    pub(super) fn instr_0xCB18(&mut self, _mem: &mut VirtualMemory) {
        self.op_RR_reg(Register::B);
    }

    // RR C
    pub(super) fn instr_0xCB19(&mut self, _mem: &mut VirtualMemory) {
        self.op_RR_reg(Register::C);
    }

    // RR D
    pub(super) fn instr_0xCB1A(&mut self, _mem: &mut VirtualMemory) {
        self.op_RR_reg(Register::D);
    }

    // RR E
    pub(super) fn instr_0xCB1B(&mut self, _mem: &mut VirtualMemory) {
        self.op_RR_reg(Register::E);
    }

    // RR H
    pub(super) fn instr_0xCB1C(&mut self, _mem: &mut VirtualMemory) {
        self.op_RR_reg(Register::H);
    }

    // RR L
    pub(super) fn instr_0xCB1D(&mut self, _mem: &mut VirtualMemory) {
        self.op_RR_reg(Register::L);
    }

    // RR (HL)
    pub(super) fn instr_0xCB1E(&mut self, mem: &mut VirtualMemory) {
        let addr = self.registers.read_pair(RegisterPair::HL);
        let val = mem.read(addr);
        let result = self.op_RR(val);
        mem.write(addr, result);
    }

    // RR A
    pub(super) fn instr_0xCB1F(&mut self, _mem: &mut VirtualMemory) {
        self.op_RR_reg(Register::A);
    }

    // SLA B
    pub(super) fn instr_0xCB20(&mut self, _mem: &mut VirtualMemory) {
        self.op_SLA_reg(Register::B);
    }

    // SLA C
    pub(super) fn instr_0xCB21(&mut self, _mem: &mut VirtualMemory) {
        self.op_SLA_reg(Register::C);
    }

    // SLA D
    pub(super) fn instr_0xCB22(&mut self, _mem: &mut VirtualMemory) {
        self.op_SLA_reg(Register::D);
    }

    // SLA E
    pub(super) fn instr_0xCB23(&mut self, _mem: &mut VirtualMemory) {
        self.op_SLA_reg(Register::E);
    }

    // SLA H
    pub(super) fn instr_0xCB24(&mut self, _mem: &mut VirtualMemory) {
        self.op_SLA_reg(Register::H);
    }

    // SLA L
    pub(super) fn instr_0xCB25(&mut self, _mem: &mut VirtualMemory) {
        self.op_SLA_reg(Register::L);
    }

    // SLA (HL)
    pub(super) fn instr_0xCB26(&mut self, mem: &mut VirtualMemory) {
        let addr = self.registers.read_pair(RegisterPair::HL);
        let val = mem.read(addr);
        let result = self.op_SLA(val);
        mem.write(addr, result);
    }

    // SLA A
    pub(super) fn instr_0xCB27(&mut self, _mem: &mut VirtualMemory) {
        self.op_SLA_reg(Register::A);
    }

    // SRA B
    pub(super) fn instr_0xCB28(&mut self, _mem: &mut VirtualMemory) {
        self.op_SRA_reg(Register::B);
    }

    // SRA C
    pub(super) fn instr_0xCB29(&mut self, _mem: &mut VirtualMemory) {
        self.op_SRA_reg(Register::C);
    }

    // SRA D
    pub(super) fn instr_0xCB2A(&mut self, _mem: &mut VirtualMemory) {
        self.op_SRA_reg(Register::D);
    }

    // SRA E
    pub(super) fn instr_0xCB2B(&mut self, _mem: &mut VirtualMemory) {
        self.op_SRA_reg(Register::E);
    }

    // SRA H
    pub(super) fn instr_0xCB2C(&mut self, _mem: &mut VirtualMemory) {
        self.op_SRA_reg(Register::H);
    }

    // SRA L
    pub(super) fn instr_0xCB2D(&mut self, _mem: &mut VirtualMemory) {
        self.op_SRA_reg(Register::L);
    }

    // SRA (HL)
    pub(super) fn instr_0xCB2E(&mut self, mem: &mut VirtualMemory) {
        let addr = self.registers.read_pair(RegisterPair::HL);
        let val = mem.read(addr);
        let result = self.op_SRA(val);
        mem.write(addr, result);
    }

    // SRA A
    pub(super) fn instr_0xCB2F(&mut self, _mem: &mut VirtualMemory) {
        self.op_SRA_reg(Register::A);
    }

    // SWAP B
    pub(super) fn instr_0xCB30(&mut self, _mem: &mut VirtualMemory) {
        self.op_SWAP_reg(Register::B);
    }

    // SWAP C
    pub(super) fn instr_0xCB31(&mut self, _mem: &mut VirtualMemory) {
        self.op_SWAP_reg(Register::C);
    }

    // SWAP D
    pub(super) fn instr_0xCB32(&mut self, _mem: &mut VirtualMemory) {
        self.op_SWAP_reg(Register::D);
    }

    // SWAP E
    pub(super) fn instr_0xCB33(&mut self, _mem: &mut VirtualMemory) {
        self.op_SWAP_reg(Register::E);
    }

    // SWAP H
    pub(super) fn instr_0xCB34(&mut self, _mem: &mut VirtualMemory) {
        self.op_SWAP_reg(Register::H);
    }

    // SWAP L
    pub(super) fn instr_0xCB35(&mut self, _mem: &mut VirtualMemory) {
        self.op_SWAP_reg(Register::L);
    }

    // SWAP (HL)
    pub(super) fn instr_0xCB36(&mut self, mem: &mut VirtualMemory) {
        let addr = self.registers.read_pair(RegisterPair::HL);
        let val = mem.read(addr);
        let result = self.op_SWAP(val);
        mem.write(addr, result);
    }

    // SWAP A
    pub(super) fn instr_0xCB37(&mut self, _mem: &mut VirtualMemory) {
        self.op_SWAP_reg(Register::A);
    }

    // SRL B
    pub(super) fn instr_0xCB38(&mut self, _mem: &mut VirtualMemory) {
        self.op_SRL_reg(Register::B);
    }

    // SRL C
    pub(super) fn instr_0xCB39(&mut self, _mem: &mut VirtualMemory) {
        self.op_SRL_reg(Register::C);
    }

    // SRL D
    pub(super) fn instr_0xCB3A(&mut self, _mem: &mut VirtualMemory) {
        self.op_SRL_reg(Register::D);
    }

    // SRL E
    pub(super) fn instr_0xCB3B(&mut self, _mem: &mut VirtualMemory) {
        self.op_SRL_reg(Register::E);
    }

    // SRL H
    pub(super) fn instr_0xCB3C(&mut self, _mem: &mut VirtualMemory) {
        self.op_SRL_reg(Register::H);
    }

    // SRL L
    pub(super) fn instr_0xCB3D(&mut self, _mem: &mut VirtualMemory) {
        self.op_SRL_reg(Register::L);
    }

    // SRL (HL)
    pub(super) fn instr_0xCB3E(&mut self, mem: &mut VirtualMemory) {
        let addr = self.registers.read_pair(RegisterPair::HL);
        let val = mem.read(addr);
        let result = self.op_SRL(val);
        mem.write(addr, result);
    }

    // SRL A
    pub(super) fn instr_0xCB3F(&mut self, _mem: &mut VirtualMemory) {
        self.op_SRL_reg(Register::A);
    }

    // BIT 0, B
    pub(super) fn instr_0xCB40(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(0, Register::B);
    }

    // BIT 0, C
    pub(super) fn instr_0xCB41(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(0, Register::C);
    }

    // BIT 0, D
    pub(super) fn instr_0xCB42(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(0, Register::D);
    }

    // BIT 0, E
    pub(super) fn instr_0xCB43(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(0, Register::E);
    }

    // BIT 0, H
    pub(super) fn instr_0xCB44(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(0, Register::H);
    }

    // BIT 0, L
    pub(super) fn instr_0xCB45(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(0, Register::L);
    }

    // BIT 0, (HL)
    pub(super) fn instr_0xCB46(&mut self, mem: &mut VirtualMemory) {
        self.op_BIT_from_HLptr(0, mem);
    }

    // BIT 0, A
    pub(super) fn instr_0xCB47(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(0, Register::A);
    }

    // BIT 1, B
    pub(super) fn instr_0xCB48(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(1, Register::B);
    }

    // BIT 1, C
    pub(super) fn instr_0xCB49(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(1, Register::C);
    }

    // BIT 1, D
    pub(super) fn instr_0xCB4A(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(1, Register::D);
    }

    // BIT 1, E
    pub(super) fn instr_0xCB4B(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(1, Register::E);
    }

    // BIT 1, H
    pub(super) fn instr_0xCB4C(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(1, Register::H);
    }

    // BIT 1, L
    pub(super) fn instr_0xCB4D(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(1, Register::L);
    }

    // BIT 1, (HL)
    pub(super) fn instr_0xCB4E(&mut self, mem: &mut VirtualMemory) {
        self.op_BIT_from_HLptr(1, mem);
    }

    // BIT 1, A
    pub(super) fn instr_0xCB4F(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(1, Register::A);
    }

    // BIT 2, B
    pub(super) fn instr_0xCB50(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(2, Register::B);
    }

    // BIT 2, C
    pub(super) fn instr_0xCB51(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(2, Register::C);
    }

    // BIT 2, D
    pub(super) fn instr_0xCB52(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(2, Register::D);
    }

    // BIT 2, E
    pub(super) fn instr_0xCB53(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(2, Register::E);
    }

    // BIT 2, H
    pub(super) fn instr_0xCB54(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(2, Register::H);
    }

    // BIT 2, L
    pub(super) fn instr_0xCB55(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(2, Register::L);
    }

    // BIT 2, (HL)
    pub(super) fn instr_0xCB56(&mut self, mem: &mut VirtualMemory) {
        self.op_BIT_from_HLptr(2, mem);
    }

    // BIT 2, A
    pub(super) fn instr_0xCB57(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(2, Register::A);
    }

    // BIT 3, B
    pub(super) fn instr_0xCB58(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(3, Register::B);
    }

    // BIT 3, C
    pub(super) fn instr_0xCB59(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(3, Register::C);
    }

    // BIT 3, D
    pub(super) fn instr_0xCB5A(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(3, Register::D);
    }

    // BIT 3, E
    pub(super) fn instr_0xCB5B(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(3, Register::E);
    }

    // BIT 3, H
    pub(super) fn instr_0xCB5C(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(3, Register::H);
    }

    // BIT 3, L
    pub(super) fn instr_0xCB5D(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(3, Register::L);
    }

    // BIT 3, (HL)
    pub(super) fn instr_0xCB5E(&mut self, mem: &mut VirtualMemory) {
        self.op_BIT_from_HLptr(3, mem);
    }

    // BIT 3, A
    pub(super) fn instr_0xCB5F(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(3, Register::A);
    }

    // BIT 4, B
    pub(super) fn instr_0xCB60(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(4, Register::B);
    }

    // BIT 4, C
    pub(super) fn instr_0xCB61(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(4, Register::C);
    }

    // BIT 4, D
    pub(super) fn instr_0xCB62(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(4, Register::D);
    }

    // BIT 4, E
    pub(super) fn instr_0xCB63(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(4, Register::E);
    }

    // BIT 4, H
    pub(super) fn instr_0xCB64(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(4, Register::H);
    }

    // BIT 4, L
    pub(super) fn instr_0xCB65(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(4, Register::L);
    }

    // BIT 4, (HL)
    pub(super) fn instr_0xCB66(&mut self, mem: &mut VirtualMemory) {
        self.op_BIT_from_HLptr(4, mem);
    }

    // BIT 4, A
    pub(super) fn instr_0xCB67(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(4, Register::A);
    }

    // BIT 5, B
    pub(super) fn instr_0xCB68(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(5, Register::B);
    }

    // BIT 5, C
    pub(super) fn instr_0xCB69(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(5, Register::C);
    }

    // BIT 5, D
    pub(super) fn instr_0xCB6A(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(5, Register::D);
    }

    // BIT 5, E
    pub(super) fn instr_0xCB6B(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(5, Register::E);
    }

    // BIT 5, H
    pub(super) fn instr_0xCB6C(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(5, Register::H);
    }

    // BIT 5, L
    pub(super) fn instr_0xCB6D(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(5, Register::L);
    }

    // BIT 5, (HL)
    pub(super) fn instr_0xCB6E(&mut self, mem: &mut VirtualMemory) {
        self.op_BIT_from_HLptr(5, mem);
    }

    // BIT 5, A
    pub(super) fn instr_0xCB6F(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(5, Register::A);
    }

    // BIT 6, B
    pub(super) fn instr_0xCB70(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(6, Register::B);
    }

    // BIT 6, C
    pub(super) fn instr_0xCB71(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(6, Register::C);
    }

    // BIT 6, D
    pub(super) fn instr_0xCB72(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(6, Register::D);
    }

    // BIT 6, E
    pub(super) fn instr_0xCB73(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(6, Register::E);
    }

    // BIT 6, H
    pub(super) fn instr_0xCB74(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(6, Register::H);
    }

    // BIT 6, L
    pub(super) fn instr_0xCB75(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(6, Register::L);
    }

    // BIT 6, (HL)
    pub(super) fn instr_0xCB76(&mut self, mem: &mut VirtualMemory) {
        self.op_BIT_from_HLptr(6, mem);
    }

    // BIT 6, A
    pub(super) fn instr_0xCB77(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(6, Register::A);
    }

    // BIT 7, B
    pub(super) fn instr_0xCB78(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(7, Register::B);
    }

    // BIT 7, C
    pub(super) fn instr_0xCB79(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(7, Register::C);
    }

    // BIT 7, D
    pub(super) fn instr_0xCB7A(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(7, Register::D);
    }

    // BIT 7, E
    pub(super) fn instr_0xCB7B(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(7, Register::E);
    }

    // BIT 7, H
    pub(super) fn instr_0xCB7C(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(7, Register::H);
    }

    // BIT 7, L
    pub(super) fn instr_0xCB7D(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(7, Register::L);
    }

    // BIT 7, (HL)
    pub(super) fn instr_0xCB7E(&mut self, mem: &mut VirtualMemory) {
        self.op_BIT_from_HLptr(7, mem);
    }

    // BIT 7, A
    pub(super) fn instr_0xCB7F(&mut self, _mem: &mut VirtualMemory) {
        self.op_BIT_reg(7, Register::A);
    }

    pub(super) fn instr_0xCB80(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB81(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB82(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB83(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB84(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB85(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB86(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB87(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB88(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB89(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB8A(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB8B(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB8C(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB8D(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB8E(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB8F(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB90(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB91(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB92(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB93(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB94(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB95(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB96(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB97(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB98(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB99(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB9A(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB9B(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB9C(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB9D(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB9E(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCB9F(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBA0(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBA1(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBA2(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBA3(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBA4(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBA5(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBA6(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBA7(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBA8(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBA9(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBAA(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBAB(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBAC(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBAD(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBAE(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBAF(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBB0(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBB1(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBB2(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBB3(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBB4(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBB5(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBB6(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBB7(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBB8(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBB9(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBBA(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBBB(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBBC(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBBD(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBBE(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBBF(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBC0(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBC1(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBC2(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBC3(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBC4(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBC5(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBC6(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBC7(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBC8(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBC9(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBCA(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBCB(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBCC(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBCD(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBCE(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBCF(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBD0(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBD1(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBD2(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBD3(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBD4(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBD5(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBD6(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBD7(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBD8(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBD9(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBDA(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBDB(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBDC(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBDD(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBDE(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBDF(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBE0(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBE1(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBE2(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBE3(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBE4(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBE5(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBE6(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBE7(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBE8(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBE9(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBEA(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBEB(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBEC(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBED(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBEE(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBEF(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBF0(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBF1(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBF2(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBF3(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBF4(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBF5(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBF6(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBF7(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBF8(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBF9(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBFA(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBFB(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBFC(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBFD(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBFE(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }

    pub(super) fn instr_0xCBFF(&mut self, _mem: &mut VirtualMemory) {
        todo!();
    }
}
