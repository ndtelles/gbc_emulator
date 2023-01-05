use crate::util::{reset_bit, set_bit};

use super::virtual_memory::VirtualMemory;

const INTERRUPT_REQUEST_ADDR: u16 = 0xFF0F;
const INTERRUPT_ENABLE_ADDR: u16 = 0xFFFF;

pub struct InterruptController {
    interrupt_master_enable: bool,
}

impl InterruptController {
    pub fn new() -> Self {
        Self {
            interrupt_master_enable: false,
        }
    }

    pub fn enable_interrupts(&mut self) {
        self.interrupt_master_enable = true;
    }

    pub fn disable_interrupts(&mut self) {
        self.interrupt_master_enable = false;
    }
}

// Interrupt flags. The enum value corresponds to the flag's bit index in the flag byte
pub enum InterruptFlag {
    P10P13TerminalNegativeEdge = 4,
    SerialTransferComplete = 3,
    TimerOverflow = 2,
    LcdcStatusInterrupt = 1,
    VerticalBlanking = 0,
}

// pub fn set_interrupt_request_flag(flag: InterruptFlag, mem: &mut VirtualMemory) {
//     let flags = mem.read(INTERRUPT_REQUEST_ADDR);
//     mem.write(INTERRUPT_REQUEST_ADDR, set_bit(flags, flag as usize));
// }

// pub fn reset_interrupt_request_flag(flag: InterruptFlag, mem: &mut VirtualMemory) {
//     let flags = mem.read(INTERRUPT_REQUEST_ADDR);
//     mem.write(INTERRUPT_REQUEST_ADDR, reset_bit(flags, flag as usize));
// }

// pub fn set_interrupt_enable_flag(flag: InterruptFlag, mem: &mut VirtualMemory) {
//     let flags = mem.read(INTERRUPT_ENABLE_ADDR);
//     mem.write(INTERRUPT_ENABLE_ADDR, set_bit(flags, flag as usize));
// }

// pub fn reset_interrupt_enable_flag(flag: InterruptFlag, mem: &mut VirtualMemory) {
//     let flags = mem.read(INTERRUPT_ENABLE_ADDR);
//     mem.write(INTERRUPT_ENABLE_ADDR, reset_bit(flags, flag as usize));
// }
