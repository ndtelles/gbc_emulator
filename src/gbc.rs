mod cpu;
mod memory;

use crate::gbc::cpu::CPU;
use crate::gbc::memory::VirtualMemory;

pub struct GBC {
    cpu: CPU,
    mem: VirtualMemory,
}

impl GBC {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            mem: VirtualMemory::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.execute(&mut self.mem)
        }
    }
}
