mod cpu;
mod delay_action;
mod dma_controller;
mod interrupt_controller;
mod lcd_controller;
mod render_engine;
mod virtual_memory;

use crate::gbc::cpu::CPU;
use crate::gbc::virtual_memory::VirtualMemory;

use self::delay_action::DelayedActions;
use self::dma_controller::DMAController;
use self::interrupt_controller::InterruptController;
use self::render_engine::Renderer;

pub struct GBC {
    state: GBCState,
}

impl GBC {
    pub fn new(rom_data: Vec<u8>) -> Self {
        Self {
            state: GBCState::new(rom_data),
        }
    }

    pub fn run(&mut self) {
        loop {
            render_engine::tick(&mut self.state);
            delay_action::tick(&mut self.state);
            dma_controller::tick(&mut self.state);
            cpu::tick(&mut self.state);
        }
    }
}

pub struct GBCState {
    cpu: CPU,
    mem: VirtualMemory,
    intr_ctrl: InterruptController,
    dma_ctrl: DMAController,
    delayed_actions: DelayedActions,
    render_engine: Renderer,
    machine_cycle: u16,
}

impl GBCState {
    pub fn new(rom_data: Vec<u8>) -> Self {
        Self {
            cpu: CPU::new(),
            mem: VirtualMemory::new(rom_data),
            intr_ctrl: InterruptController::new(),
            dma_ctrl: DMAController::new(),
            delayed_actions: DelayedActions::new(),
            render_engine: Renderer::new(),
            machine_cycle: 0,
        }
    }
}
