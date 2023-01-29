use int_enum::IntEnum;

use crate::util::{reset_bit, set_bit};

use super::{
    delay_action,
    lcd_controller::{self, PPUMode},
    virtual_memory, GBCState,
};

pub const INTERRUPT_REQUEST_ADDR: u16 = 0xFF0F;
pub const INTERRUPT_ENABLE_ADDR: u16 = 0xFFFF;

pub struct InterruptController {
    interrupt_master_enable: bool,
}

impl InterruptController {
    pub fn new() -> Self {
        Self {
            interrupt_master_enable: false,
        }
    }
}

// Interrupt flags. The enum value corresponds to the flag's bit index in the flag byte
#[repr(u8)]
#[derive(Clone, Copy, IntEnum)]
pub enum InterruptFlag {
    VerticalBlanking = 0,
    LcdcStatusInterrupt = 1,
    TimerOverflow = 2,
    SerialTransferComplete = 3,
    Joypad = 4,
}

impl InterruptFlag {
    pub fn handler_address(&self) -> u16 {
        match self {
            InterruptFlag::VerticalBlanking => 0x0040,
            InterruptFlag::LcdcStatusInterrupt => 0x0048,
            InterruptFlag::TimerOverflow => 0x0050,
            InterruptFlag::SerialTransferComplete => 0x0058,
            InterruptFlag::Joypad => 0x0060,
        }
    }
}

pub fn tick(state: &mut GBCState) {
    let stat = lcd_controller::get_lcd_status_register(state);
    if (stat.ppu_mode == PPUMode::HBlank && stat.hblank_interrupt_source)
        || (stat.ppu_mode == PPUMode::OAMScan && stat.oam_stat_interrupt_source)
        || (stat.ppu_mode == PPUMode::VBlank && stat.vblank_interrupt_source)
        || (stat.lyc_match_ly && stat.lyc_match_ly_interrupt_source)
    {
        set_interrupt_request_flag(state, InterruptFlag::LcdcStatusInterrupt);
    }

    if stat.ppu_mode == PPUMode::VBlank {
        set_interrupt_request_flag(state, InterruptFlag::VerticalBlanking);
    }
}

pub fn enable_interrupts(state: &mut GBCState) {
    delay_action::schedule(
        state,
        |state| state.intr_ctrl.interrupt_master_enable = true,
        1,
    );
}

pub fn disable_interrupts(state: &mut GBCState) {
    delay_action::schedule(
        state,
        |state| state.intr_ctrl.interrupt_master_enable = false,
        1,
    );
}

pub fn set_interrupt_request_flag(state: &mut GBCState, flag: InterruptFlag) {
    let flags = virtual_memory::read(state, INTERRUPT_REQUEST_ADDR);
    virtual_memory::write_without_triggers(state, INTERRUPT_REQUEST_ADDR, set_bit(flags, flag as usize));
}

pub fn reset_interrupt_request_flag(state: &mut GBCState, flag: InterruptFlag) {
    let flags = virtual_memory::read(state, INTERRUPT_REQUEST_ADDR);
    virtual_memory::write_without_triggers(
        state,
        INTERRUPT_REQUEST_ADDR,
        reset_bit(flags, flag as usize),
    );
}

pub fn set_interrupt_enable_flag(state: &mut GBCState, flag: InterruptFlag) {
    let flags = virtual_memory::read(state, INTERRUPT_ENABLE_ADDR);
    virtual_memory::write(state, INTERRUPT_ENABLE_ADDR, set_bit(flags, flag as usize));
}

pub fn reset_interrupt_enable_flag(state: &mut GBCState, flag: InterruptFlag) {
    let flags = virtual_memory::read(state, INTERRUPT_ENABLE_ADDR);
    virtual_memory::write(
        state,
        INTERRUPT_ENABLE_ADDR,
        reset_bit(flags, flag as usize),
    );
}

pub fn get_active_interrupt(state: &GBCState) -> Option<InterruptFlag> {
    if !state.intr_ctrl.interrupt_master_enable {
        return None;
    }

    let active_interrupts = virtual_memory::read(state, INTERRUPT_ENABLE_ADDR)
        & virtual_memory::read(state, INTERRUPT_REQUEST_ADDR);
    if active_interrupts == 0 {
        return None;
    }
    // The lowest bit interrupt has the highest priority
    let lowest_active_bit = active_interrupts.trailing_zeros() as u8;
    InterruptFlag::from_int(lowest_active_bit).ok()
}
