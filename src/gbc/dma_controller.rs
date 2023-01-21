use std::{
    iter::{Peekable, StepBy, Zip},
    ops::Range,
};

use crate::util::{combine_high_low, index_bits};

use super::{
    virtual_memory::{self, OAM_ADDR, VRAM_DMA_REGISTER},
    GBCState, lcd_controller::{self, PPUMode},
};

const OAM_TRANSFER_BYTES: usize = 160;
const HDMA_REG_ADDR: u16 = 0xFF51;

struct DMATransfer {
    // Iterator for getting the src and dest address for next write
    iterator: Peekable<StepBy<Zip<Range<u16>, Range<u16>>>>,
}

impl DMATransfer {
    pub fn new(src_addr: u16, dest_addr: u16, length_bytes: usize, step_by_bytes: usize) -> Self {
        let src_range = src_addr..(src_addr + length_bytes as u16);
        let dest_range = dest_addr..(dest_addr + length_bytes as u16);
        let iterator = src_range.zip(dest_range).step_by(step_by_bytes).peekable();
        Self { iterator }
    }

    pub fn empty() -> Self {
        Self {
            iterator: (0..0).zip(0..0).step_by(1).peekable(),
        }
    }
}

pub struct DMAController {
    oam_transfer: DMATransfer,
    hblank_transfer: DMATransfer,
}

impl DMAController {
    pub fn new() -> Self {
        Self {
            oam_transfer: DMATransfer::empty(),
            hblank_transfer: DMATransfer::empty(),
        }
    }
}

pub fn tick(state: &mut GBCState) {
    process_oam_transfer(state);
}

/**
 * Trigger oam transfer when register 0xFF46 is written to
 */
pub fn trigger_oam_transfer(state: &mut GBCState, val: u8) {
    if val >= 0xC0 {
        return; // Invalid start address
    }

    // Not sure what should happen if OAM DMA is triggered again while
    // it's already running. In this case we will just restart the process.
    let src = (val as u16) << 8;
    state.dma_ctrl.oam_transfer = DMATransfer::new(src, OAM_ADDR, OAM_TRANSFER_BYTES, 1);
}

/**
 * Trigger either hblank or general purpose DMA transfer when register 0xFF55 is written to
 */
pub fn trigger_vram_transfer(state: &mut GBCState, val: u8) {
    // Hblank transfer cancelled
    if !index_bits(val, 7) && state.dma_ctrl.hblank_transfer.iterator.peek().is_some() {
        state.dma_ctrl.hblank_transfer = DMATransfer::empty();
        return;
    }

    let hdma_reg_vals = virtual_memory::read_bytes(state, HDMA_REG_ADDR, 4);
    // Destructure hdma register memory to vars
    let [src_high, src_low, dest_high, dest_low]: [_; 4] =
        hdma_reg_vals.as_ref().try_into().unwrap();

    if let 0x80..=0x9F | 0xC0.. = src_high {
        return; // Invalid src
    }

    // Src in range 0x0000-7FF0 or 0xA000-DFF0
    let src_addr = combine_high_low(src_high, src_low) & 0xFFF0;
    // Dest in range 0x8000 - 0x9FF0
    let dest_addr = (combine_high_low(dest_high, dest_low) & 0x1FF0) + 0x8000;
    // Val specifies length in 16 byte chunks
    let length_bytes = ((val as usize & 0x7F) + 1) * 16;

    match index_bits(val, 7) {
        // If bit 7 is high, start hblank transfer
        true => {
            state.dma_ctrl.hblank_transfer =
                DMATransfer::new(src_addr, dest_addr, length_bytes, 16);
        }
        // If bit 7 is low, start general purpose DMA transfer immediately
        false => {
            process_general_purpose_transfer(state, src_addr, dest_addr, length_bytes);
        }
    }
}

/**
 * Write one byte per machine cycle to OAM
 */
fn process_oam_transfer(state: &mut GBCState) {
    let next = state.dma_ctrl.oam_transfer.iterator.next();

    if let Some((src, dest)) = next {
        let val = virtual_memory::read(state, src);
        virtual_memory::write(state, dest, val);
    }
}

/**
 * Write 16 bytes per hblank
 */
pub fn process_hblank_transfer(state: &mut GBCState) {
    let next = state.dma_ctrl.hblank_transfer.iterator.next();
    if next.is_none() {
        return;
    }

    let (src, dest) = next.unwrap();
    let vals = virtual_memory::read_bytes(state, src, 16).into_owned();
    virtual_memory::write_bytes(state, dest, &vals);

    match state.dma_ctrl.hblank_transfer.iterator.peek() {
        // Write back remaining transfer length (in 16 byte chunks) to DMA register
        Some(_) => {
            let val = virtual_memory::read(state, VRAM_DMA_REGISTER);
            virtual_memory::write_without_triggers(state, VRAM_DMA_REGISTER, val - 1);
        }
        // Write back to DMA register that transfer has finished
        None => virtual_memory::write_without_triggers(state, VRAM_DMA_REGISTER, 0x00FF),
    }
}

/**
 * Halts program to copy data until transfer is complete
 */
fn process_general_purpose_transfer(
    state: &mut GBCState,
    src_addr: u16,
    dest_addr: u16,
    length_bytes: usize,
) {
    let vals = virtual_memory::read_bytes(state, src_addr, length_bytes).into_owned();
    virtual_memory::write_bytes(state, dest_addr, &vals);
}
