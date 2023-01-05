use super::{GBCState, virtual_memory::VirtualMemory};

const TRANSFER_CYCLES: u16 = 640;
const DMA_REG_ADDR: u16 = 0xFF46;

struct DMATransfer {
    cycles_until_finished: u16,
}

pub struct DMAController {
    current_transfer: Option<DMATransfer>,
}

impl DMAController {
    pub fn new() -> Self {
        Self {
            current_transfer: None
        }
    }
}

pub fn tick(state: &mut GBCState) {
    if state.dma_ctrl.current_transfer.is_none() {
        return;
    }
    let dma_ctrl = &mut state.dma_ctrl;
    let mem = &mut state.mem;
}

fn process_transfer(dma_ctrl: &mut DMAController, mem: &mut VirtualMemory) {

}
