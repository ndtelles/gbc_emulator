use std::collections::VecDeque;

use super::GBCState;

struct Pixel {
    color: u8,
    palette: u8,
    sprite_priority: u8,
    background_priority: bool,
}

struct Renderer {
    bg_fifo: VecDeque<Pixel>,
    obj_fifo: VecDeque<Pixel>,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            bg_fifo: VecDeque::with_capacity(8),
            obj_fifo: VecDeque::with_capacity(8),
        }
    }
}
