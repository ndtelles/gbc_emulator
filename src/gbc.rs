mod cpu;
mod delay_action;
mod dma_controller;
mod interrupt_controller;
mod virtual_memory;
mod render_engine;

use crate::gbc::cpu::CPU;
use crate::gbc::virtual_memory::VirtualMemory;

use self::delay_action::DelayedActions;
use self::dma_controller::DMAController;
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
            delay_action::tick(&mut self.state);
            dma_controller::tick(&mut self.state);
            cpu::execute(&mut self.state);
        }
    }
}

pub struct GBCState {
    cpu: CPU,
    mem: VirtualMemory,
    intr_ctrl: InterruptController,
    dma_ctrl: DMAController,
    delayed_actions: DelayedActions,
    machine_cycle: u16,
}

impl GBCState {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            mem: VirtualMemory::new(),
            intr_ctrl: InterruptController::new(),
            dma_ctrl: DMAController::new(),
            delayed_actions: DelayedActions::new(),
            machine_cycle: 0,
        }
    }
}
