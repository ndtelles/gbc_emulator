mod cb_instruction_impl;
mod instruction_impl;
mod instructions;
mod op_helpers;
mod register;

use crate::util::{combine_high_low, Bytes};

use self::instructions::map_instruction;
use self::register::{RegisterMap, RegisterMapMethods};
use super::interrupt_controller::{
    self, InterruptFlag, INTERRUPT_ENABLE_ADDR, INTERRUPT_REQUEST_ADDR,
};
use super::{virtual_memory, GBCState};

const PROGRAM_START_ADDR: u16 = 0x0100;

pub struct CPU {
    registers: RegisterMap,
    pc: u16,
    sp: u16,
    halted: bool,
    busy_cycles: u8,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: RegisterMap::new(),
            // Start of user program
            pc: PROGRAM_START_ADDR,
            // End of stack RAM (stack starts at end)
            sp: 0xFFFE,
            halted: false,
            busy_cycles: 0,
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
    let low = fetch_and_incr_pc(state);
    let high = fetch_and_incr_pc(state);
    combine_high_low(high, low)
}

// Indicates if the CPU should wake up from a halt
fn cpu_should_wake(state: &GBCState) -> bool {
    // CPU only wakes up if there is an enabled and requested interrupt.
    // IME does not need to be set.
    virtual_memory::read(state, INTERRUPT_ENABLE_ADDR)
        & virtual_memory::read(state, INTERRUPT_REQUEST_ADDR)
        > 0
}

// Mark cpu as busy for x cycles
fn consume_cycles(state: &mut GBCState, cycles: u8) {
    state.cpu.busy_cycles = cycles;
}

// Call a method by moving current PC to SP and setting PC
fn call(state: &mut GBCState, new_pc: u16) {
    state.cpu.sp -= 1;
    virtual_memory::write(state, state.cpu.sp, state.cpu.pc.high());
    state.cpu.sp -= 1;
    virtual_memory::write(state, state.cpu.sp, state.cpu.pc.low());
    state.cpu.pc = new_pc;
}

fn handle_interrupt(state: &mut GBCState, intr: InterruptFlag) {
    interrupt_controller::reset_interrupt_request_flag(state, intr);
    interrupt_controller::disable_interrupts(state);
    call(state, intr.handler_address());
    consume_cycles(state, 5);
}

pub fn execute(state: &mut GBCState) {
    if state.cpu.busy_cycles > 0 {
        // CPU has been marked as already busy this cycle
        state.cpu.busy_cycles -= 1;
        return;
    }

    if let Some(intr) = interrupt_controller::get_active_interrupt(state) {
        handle_interrupt(state, intr);
        return;
    }

    if state.cpu.halted {
        if !cpu_should_wake(state) {
            return; // Stay halted
        }
        state.cpu.halted = false;
    }

    let instruction = fetch_and_incr_pc(state);
    let instruction_impl = map_instruction(instruction);
    instruction_impl(state);
}
