use super::{
    virtual_memory::{OAM_ADDR, self},
    GBCState,
};

const OAM_TRANSFER_MACHINE_CYCLES: u16 = 160;
const OAM_TRANSFER_LENGTH: usize = 0xA0;

struct DMATransfer {
    cycles_until_finished: u16,
    src: u16,
    dest: u16,
    length: usize,
}

pub struct DMAController {
    current_transfer: Option<DMATransfer>,
}

impl DMAController {
    pub fn new() -> Self {
        Self {
            current_transfer: None,
        }
    }
}

pub fn trigger_oam_transfer(state: &mut GBCState, val: u8) {
    if state.dma_ctrl.current_transfer.is_some() {
        return; // DMA transfer already running
    }

    let src = (val as u16) << 8;
    state.dma_ctrl.current_transfer = Some(DMATransfer {
        cycles_until_finished: OAM_TRANSFER_MACHINE_CYCLES,
        src,
        dest: OAM_ADDR,
        length: OAM_TRANSFER_LENGTH,
    });
}

pub fn tick(state: &mut GBCState) {
    if state.dma_ctrl.current_transfer.is_none() {
        return;
    }

    let transfer = state.dma_ctrl.current_transfer.as_mut().unwrap();
    if transfer.cycles_until_finished != 0 {
        transfer.cycles_until_finished -= 1;
        return;
    }
    
    let src = transfer.src;
    let dest = transfer.dest;
    let length = transfer.length;

    let vals = virtual_memory::read_len(state, src, length).into_owned();
    virtual_memory::write_len(state, dest, &vals);
}
