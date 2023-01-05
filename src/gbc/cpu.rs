mod cb_instruction_impl;
mod instruction_impl;
mod instructions;
mod op_helpers;
mod register;

use self::instructions::map_instruction;
use self::register::{RegisterMap, RegisterMapMethods};
use super::{GBCState, virtual_memory};

const PROGRAM_START_ADDR: u16 = 0x0100;

pub struct CPU {
    registers: RegisterMap,
    pc: u16,
    sp: u16,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: RegisterMap::new(),
            // Start of user program
            pc: PROGRAM_START_ADDR,
            // End of stack RAM (stack starts at end)
            sp: 0xFFFE,
        }
    }
}

// Fetch next 8 bits at program counter
fn fetch_and_incr_pc(state: &mut GBCState) -> u8 {
    let data = virtual_memory::read(state, state.cpu.pc);
    state.cpu.pc += 1;
    data
}

// Fetch next 16 bits (little endian) at program counter. Return as big endian
fn fetch_and_incr_pc_16(state: &mut GBCState) -> u16 {
    fetch_and_incr_pc(state) as u16 | ((fetch_and_incr_pc(state) as u16) << 8)
}

pub fn execute(state: &mut GBCState) {
    let instruction = fetch_and_incr_pc(state);
    let instruction_impl = map_instruction(instruction);
    instruction_impl(state);
}
