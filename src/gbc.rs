mod cpu;
mod delay_action;
mod dma_controller;
mod interrupt_controller;
mod lcd_controller;
mod render_engine;
mod virtual_memory;

use std::sync::mpsc::Sender;

use crate::gbc::cpu::CPU;
use crate::gbc::virtual_memory::VirtualMemory;

use self::delay_action::DelayedActions;
use self::dma_controller::DMAController;
use self::interrupt_controller::InterruptController;
use self::render_engine::Renderer;

use color_eyre::eyre::Result;
use egui_extras::RetainedImage;

const MACHINE_CYCLES_PER_FRAME: u16 = 17556;

pub struct GBC {
    state: GBCState,
}

impl GBC {
    pub fn new(rom_data: Vec<u8>, img_publisher: Sender<RetainedImage>) -> Result<Self> {
        Ok(Self {
            state: GBCState::new(rom_data, img_publisher)?,
        })
    }

    pub fn run(&mut self) {
        loop {
            delay_action::tick(&mut self.state);
            dma_controller::tick(&mut self.state);
            cpu::tick(&mut self.state);
            render_engine::tick(&mut self.state);
            // self.state.machine_cycle = (self.state.machine_cycle + 1) % MACHINE_CYCLES_PER_FRAME;
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
    pub fn new(rom_data: Vec<u8>, img_publisher: Sender<RetainedImage>) -> Result<Self> {
        Ok(Self {
            cpu: CPU::new(),
            mem: VirtualMemory::new(rom_data)?,
            intr_ctrl: InterruptController::new(),
            dma_ctrl: DMAController::new(),
            delayed_actions: DelayedActions::new(),
            render_engine: Renderer::new(img_publisher),
            machine_cycle: 0,
        })
    }
}
