mod cpu;
mod interrupt_controller;
mod memory;
mod render_engine;

use crate::gbc::cpu::CPU;
use crate::gbc::memory::VirtualMemory;

use self::interrupt_controller::InterruptController;

pub struct GBC {
    state: GBCState,
}

impl GBC {
    pub fn new() -> Self {
        Self {
            state: GBCState::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            cpu::execute(&mut self.state);
        }
    }
}

pub struct GBCState {
    cpu: CPU,
    mem: VirtualMemory,
    intr_ctrl: InterruptController,
    machine_cycle: u8,
}

impl GBCState {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            mem: VirtualMemory::new(),
            intr_ctrl: InterruptController::new(),
            machine_cycle: 0,
        }
    }
}
