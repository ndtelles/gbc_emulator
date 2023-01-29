use crate::util::index_bits;

use super::{virtual_memory, GBCState, interrupt_controller::{self, InterruptFlag}};

pub const DIVIDER_REGISTER: u16 = 0xFF04;
const TIMER_COUNTER_REGISTER: u16 = 0xFF05;
const TIMER_MODULO_REGISTER: u16 = 0xFF06;
pub const TIMER_CONTROL_REGISTER: u16 = 0xFF07;

const MCYCLES_PER_DIVIDER_UPDATE: u16 = 64;

pub struct TimerController {
    // Cycles left until next div update
    div_update_countdown: u16,
    timer_enabled: bool,
    // Cycles left until next timer update
    timer_update_countdown: u16,
    mcyles_per_timer_countdown: u16
}
impl TimerController {
    pub fn new() -> Self {
        Self {
            div_update_countdown: MCYCLES_PER_DIVIDER_UPDATE,
            timer_enabled: false,
            timer_update_countdown: 128,
            mcyles_per_timer_countdown: 128,
        }
    }

    fn reset_div_update_countdown(&mut self) {
        self.div_update_countdown = MCYCLES_PER_DIVIDER_UPDATE;
    }

    fn reset_timer_update_countdown(&mut self) {
        self.timer_update_countdown = self.mcyles_per_timer_countdown;
    }
}

pub fn tick(state: &mut GBCState) {
    handle_divider(state);
    handle_timer(state);
}

fn handle_divider(state: &mut GBCState) {
    state.timer_ctrl.div_update_countdown -= 1;
    if state.timer_ctrl.div_update_countdown == 0 {
        let curr_val = virtual_memory::read(state, DIVIDER_REGISTER);
        virtual_memory::write_without_triggers(state, DIVIDER_REGISTER, curr_val.wrapping_add(1));
        state.timer_ctrl.reset_div_update_countdown();
    }
}

fn handle_timer(state: &mut GBCState) {
    if !state.timer_ctrl.timer_enabled {
        return;
    }

    state.timer_ctrl.timer_update_countdown -= 1;
    if state.timer_ctrl.timer_update_countdown == 0 {
        let curr_val = virtual_memory::read(state, TIMER_COUNTER_REGISTER);
        if curr_val == 0xFF {
            // Timer overflow
            let tma = virtual_memory::read(state, TIMER_MODULO_REGISTER);
            virtual_memory::write_without_triggers(state, TIMER_COUNTER_REGISTER, tma);
            interrupt_controller::set_interrupt_request_flag(state, InterruptFlag::TimerOverflow);
        } else {
            virtual_memory::write_without_triggers(state, TIMER_COUNTER_REGISTER, curr_val + 1);
        }
        
        state.timer_ctrl.reset_timer_update_countdown();
    }
}

pub fn set_timer_control_register(state: &mut GBCState, val: u8) {
    state.timer_ctrl.timer_enabled = index_bits(val, 2);
    state.timer_ctrl.mcyles_per_timer_countdown = clock_select_bits_to_mcycles(val & 0x03);
}

fn clock_select_bits_to_mcycles(select_bits: u8) -> u16 {
    match select_bits {
        0x00 => 128,
        0x01 => 4,
        0x02 => 16,
        0x03 => 64,
        _ => panic!("Invalid clock select bit"),
    }
}
