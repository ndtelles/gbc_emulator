mod cpu;
mod delay_action;
mod dma_controller;
mod interrupt_controller;
mod lcd_controller;
mod render_engine;
mod timer_controller;
mod virtual_memory;

use std::sync::{Arc, Mutex};

use crate::gbc::cpu::CPU;
use crate::gbc::virtual_memory::VirtualMemory;

use self::delay_action::DelayedActions;
use self::dma_controller::DMAController;
use self::interrupt_controller::InterruptController;
use self::render_engine::Renderer;
use self::timer_controller::TimerController;

use color_eyre::eyre::Result;
use egui_extras::RetainedImage;

const MACHINE_CYCLES_PER_FRAME: u16 = 17556;

pub struct GBC {
    state: GBCState,
}

impl GBC {
    pub fn new(
        rom_data: Vec<u8>,
        display_buffer: Arc<Mutex<RetainedImage>>,
        gui_ctx: eframe::egui::Context,
    ) -> Result<Self> {
        Ok(Self {
            state: GBCState::new(rom_data, display_buffer, gui_ctx)?,
        })
    }

    pub fn run(&mut self) {
        loop {
            // LCD controller should be first since it controls what mode
            // everything following runs in.
            lcd_controller::tick(&mut self.state);
            // Interrupt controller should run before CPU so that interrupts flags get set in time
            // for any PPU mode updates
            interrupt_controller::tick(&mut self.state);
            delay_action::tick(&mut self.state);
            dma_controller::tick(&mut self.state);
            timer_controller::tick(&mut self.state);
            cpu::tick(&mut self.state);
            cpu::tick(&mut self.state);
            cpu::tick(&mut self.state);
            cpu::tick(&mut self.state);
            render_engine::tick(&mut self.state);
            render_engine::tick(&mut self.state);
            render_engine::tick(&mut self.state);
            render_engine::tick(&mut self.state);
            self.state.machine_cycle = (self.state.machine_cycle + 1) % MACHINE_CYCLES_PER_FRAME;
        }
    }
}

pub struct GBCState {
    cpu: CPU,
    mem: VirtualMemory,
    intr_ctrl: InterruptController,
    timer_ctrl: TimerController,
    dma_ctrl: DMAController,
    delayed_actions: DelayedActions,
    render_engine: Renderer,
    machine_cycle: u16,
}

impl GBCState {
    pub fn new(
        rom_data: Vec<u8>,
        display_buffer: Arc<Mutex<RetainedImage>>,
        gui_ctx: eframe::egui::Context,
    ) -> Result<Self> {
        Ok(Self {
            cpu: CPU::new(),
            mem: VirtualMemory::new(rom_data)?,
            intr_ctrl: InterruptController::new(),
            timer_ctrl: TimerController::new(),
            dma_ctrl: DMAController::new(),
            delayed_actions: DelayedActions::new(),
            render_engine: Renderer::new(display_buffer, gui_ctx),
            machine_cycle: 0,
        })
    }
}
